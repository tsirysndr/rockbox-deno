mod args;
mod cache;
mod cdp;
mod emit;
mod errors;
mod factory;
mod file_fetcher;
mod graph_container;
mod graph_util;
mod http_util;
mod js;
mod jsr;
mod lsp;
mod module_loader;
mod node;
mod npm;
mod ops;
mod resolver;
mod shared;
mod standalone;
mod task_runner;
mod tools;
mod tsc;
mod util;
mod version;
mod worker;

pub use deno_runtime::UNSTABLE_GRANULAR_FLAGS;

use crate::args::flags_from_vec;
use crate::args::DenoSubcommand;
use crate::args::Flags;
use crate::util::display;
use crate::util::v8::get_v8_flags_from_env;
use crate::util::v8::init_v8_flags;
use args::TaskFlags;
use deno_core::anyhow::Context;
use deno_core::error::AnyError;
use deno_core::error::JsError;
use deno_core::futures::FutureExt;
use deno_core::unsync::JoinHandle;
use deno_npm::resolution::SnapshotFromLockfileError;
use deno_resolver::npm::ByonmResolvePkgFolderFromDenoReqError;
use deno_resolver::npm::ResolvePkgFolderFromDenoReqError;
use deno_runtime::fmt_errors::format_js_error;
use deno_runtime::tokio_util::create_and_run_current_thread_with_maybe_metrics;
use deno_runtime::WorkerExecutionMode;
use deno_terminal::colors;
use factory::CliFactory;
use standalone::MODULE_NOT_FOUND;
use standalone::UNSUPPORTED_SCHEME;
use std::env;
use std::future::Future;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;

/// Ensures that all subcommands return an i32 exit code and an [`AnyError`] error type.
trait SubcommandOutput {
  fn output(self) -> Result<i32, AnyError>;
}

impl SubcommandOutput for Result<i32, AnyError> {
  fn output(self) -> Result<i32, AnyError> {
    self
  }
}

impl SubcommandOutput for Result<(), AnyError> {
  fn output(self) -> Result<i32, AnyError> {
    self.map(|_| 0)
  }
}

impl SubcommandOutput for Result<(), std::io::Error> {
  fn output(self) -> Result<i32, AnyError> {
    self.map(|_| 0).map_err(|e| e.into())
  }
}

#[inline(always)]
fn spawn_subcommand<F: Future<Output = T> + 'static, T: SubcommandOutput>(
  f: F,
) -> JoinHandle<Result<i32, AnyError>> {
  // the boxed_local() is important in order to get windows to not blow the stack in debug
  deno_core::unsync::spawn(
    async move { f.map(|r| r.output()).await }.boxed_local(),
  )
}

fn exit_with_message(message: &str, code: i32) -> ! {
  log::error!(
    "{}: {}",
    colors::red_bold("error"),
    message.trim_start_matches("error: ")
  );
  deno_runtime::exit(code);
}

fn exit_for_error(error: AnyError) -> ! {
  let mut error_string = format!("{error:?}");
  let mut error_code = 1;

  if let Some(e) = error.downcast_ref::<JsError>() {
    error_string = format_js_error(e);
  } else if let Some(SnapshotFromLockfileError::IntegrityCheckFailed(e)) =
    error.downcast_ref::<SnapshotFromLockfileError>()
  {
    error_string = e.to_string();
    error_code = 10;
  }

  exit_with_message(&error_string, error_code);
}

fn unwrap_or_exit<T>(result: Result<T, AnyError>) -> T {
  match result {
    Ok(value) => value,
    Err(error) => {
      let mut error_string = format!("{:?}", error);

      if let Some(e) = error.downcast_ref::<JsError>() {
        error_string = format_js_error(e);
      }

      exit_with_message(&error_string, 1);
    }
  }
}

fn unwrap_or_err<T>(result: Result<T, AnyError>) -> Result<T, AnyError> {
  match result {
    Ok(value) => Ok(value),
    Err(error) => {
      let mut error_string = format!("{:?}", error);

      if let Some(e) = error.downcast_ref::<JsError>() {
        error_string = format_js_error(e);
      }

      log::error!(
        "{}: {}",
        colors::red_bold("error"),
        &error_string.trim_start_matches("error: ")
      );
      Err(error)
    }
  }
}

fn resolve_flags_and_init(
  args: Vec<std::ffi::OsString>,
) -> Result<Flags, AnyError> {
  let flags = match flags_from_vec(args) {
    Ok(flags) => flags,
    Err(err @ clap::Error { .. })
      if err.kind() == clap::error::ErrorKind::DisplayVersion =>
    {
      // Ignore results to avoid BrokenPipe errors.
      util::logger::init(None, None);
      let _ = err.print();
      deno_runtime::exit(0);
    }
    Err(err) => {
      util::logger::init(None, None);
      exit_for_error(AnyError::from(err))
    }
  };

  deno_telemetry::init(crate::args::otel_runtime_config())?;
  util::logger::init(flags.log_level, Some(flags.otel_config()));

  // TODO(bartlomieju): remove in Deno v2.5 and hard error then.
  if flags.unstable_config.legacy_flag_enabled {
    log::warn!(
      "⚠️  {}",
      colors::yellow(
        "The `--unstable` flag has been removed in Deno 2.0. Use granular `--unstable-*` flags instead.\nLearn more at: https://docs.deno.com/runtime/manual/tools/unstable_flags"
      )
    );
  }

  let default_v8_flags = match flags.subcommand {
    // Using same default as VSCode:
    // https://github.com/microsoft/vscode/blob/48d4ba271686e8072fc6674137415bc80d936bc7/extensions/typescript-language-features/src/configuration/configuration.ts#L213-L214
    DenoSubcommand::Lsp => vec!["--max-old-space-size=3072".to_string()],
    _ => {
      // TODO(bartlomieju): I think this can be removed as it's handled by `deno_core`
      // and its settings.
      // deno_ast removes TypeScript `assert` keywords, so this flag only affects JavaScript
      // TODO(petamoriken): Need to check TypeScript `assert` keywords in deno_ast
      vec!["--no-harmony-import-assertions".to_string()]
    }
  };

  init_v8_flags(&default_v8_flags, &flags.v8_flags, get_v8_flags_from_env());
  // TODO(bartlomieju): remove last argument once Deploy no longer needs it
  deno_core::JsRuntime::init_platform(
    None, /* import assertions enabled */ false,
  );

  Ok(flags)
}

async fn run_subcommand(flags: Arc<Flags>) -> Result<i32, AnyError> {
  let handle = match flags.subcommand.clone() {
    DenoSubcommand::Add(add_flags) => spawn_subcommand(async {
      tools::registry::add(
        flags,
        add_flags,
        tools::registry::AddCommandName::Add,
      )
      .await
    }),
    DenoSubcommand::Remove(remove_flags) => spawn_subcommand(async {
      tools::registry::remove(flags, remove_flags).await
    }),
    DenoSubcommand::Doc(doc_flags) => {
      spawn_subcommand(async { tools::doc::doc(flags, doc_flags).await })
    }
    DenoSubcommand::Eval(eval_flags) => spawn_subcommand(async {
      tools::run::eval_command(flags, eval_flags).await
    }),
    DenoSubcommand::Cache(cache_flags) => spawn_subcommand(async move {
      tools::installer::install_from_entrypoints(flags, &cache_flags.files)
        .await
    }),
    DenoSubcommand::Check(check_flags) => {
      spawn_subcommand(
        async move { tools::check::check(flags, check_flags).await },
      )
    }
    DenoSubcommand::Clean => {
      spawn_subcommand(async move { tools::clean::clean() })
    }
    DenoSubcommand::Coverage(coverage_flags) => spawn_subcommand(async {
      tools::coverage::cover_files(flags, coverage_flags)
    }),
    DenoSubcommand::Fmt(fmt_flags) => {
      spawn_subcommand(
        async move { tools::fmt::format(flags, fmt_flags).await },
      )
    }
    DenoSubcommand::Init(init_flags) => {
      spawn_subcommand(async { tools::init::init_project(init_flags).await })
    }
    DenoSubcommand::Info(info_flags) => {
      spawn_subcommand(async { tools::info::info(flags, info_flags).await })
    }
    DenoSubcommand::Lint(lint_flags) => spawn_subcommand(async {
      if lint_flags.rules {
        tools::lint::print_rules_list(
          lint_flags.json,
          lint_flags.maybe_rules_tags,
        );
        Ok(())
      } else {
        tools::lint::lint(flags, lint_flags).await
      }
    }),
    DenoSubcommand::Run(run_flags) => spawn_subcommand(async move {
      if run_flags.is_stdin() {
        tools::run::run_from_stdin(flags.clone()).await
      } else {
        let result = tools::run::run_script(
          WorkerExecutionMode::Run,
          flags.clone(),
          run_flags.watch,
        )
        .await;

        match result {
          Ok(v) => Ok(v),
          Err(script_err) => {
            if let Some(ResolvePkgFolderFromDenoReqError::Byonm(
              ByonmResolvePkgFolderFromDenoReqError::UnmatchedReq(_),
            )) =
              script_err.downcast_ref::<ResolvePkgFolderFromDenoReqError>()
            {
              if flags.node_modules_dir.is_none() {
                let mut flags = flags.deref().clone();
                let watch = match &flags.subcommand {
                  DenoSubcommand::Run(run_flags) => run_flags.watch.clone(),
                  _ => unreachable!(),
                };
                flags.node_modules_dir =
                  Some(deno_config::deno_json::NodeModulesDirMode::None);
                // use the current lockfile, but don't write it out
                if flags.frozen_lockfile.is_none() {
                  flags.internal.lockfile_skip_write = true;
                }
                return tools::run::run_script(
                  WorkerExecutionMode::Run,
                  Arc::new(flags),
                  watch,
                )
                .await;
              }
            }
            let script_err_msg = script_err.to_string();
            if script_err_msg.starts_with(MODULE_NOT_FOUND)
              || script_err_msg.starts_with(UNSUPPORTED_SCHEME)
            {
              if run_flags.bare {
                let mut cmd = args::clap_root();
                cmd.build();
                let command_names = cmd
                  .get_subcommands()
                  .map(|command| command.get_name())
                  .collect::<Vec<_>>();
                let suggestions =
                  args::did_you_mean(&run_flags.script, command_names);
                if !suggestions.is_empty() {
                  let mut error =
                    clap::error::Error::<clap::error::DefaultFormatter>::new(
                      clap::error::ErrorKind::InvalidSubcommand,
                    )
                    .with_cmd(&cmd);
                  error.insert(
                    clap::error::ContextKind::SuggestedSubcommand,
                    clap::error::ContextValue::Strings(suggestions),
                  );

                  Err(error.into())
                } else {
                  Err(script_err)
                }
              } else {
                let mut new_flags = flags.deref().clone();
                let task_flags = TaskFlags {
                  cwd: None,
                  task: Some(run_flags.script.clone()),
                  is_run: true,
                  recursive: false,
                  filter: None,
                  eval: false,
                };
                new_flags.subcommand = DenoSubcommand::Task(task_flags.clone());
                let result = tools::task::execute_script(
                  Arc::new(new_flags),
                  task_flags.clone(),
                )
                .await;
                match result {
                  Ok(v) => Ok(v),
                  Err(_) => {
                    // Return script error for backwards compatibility.
                    Err(script_err)
                  }
                }
              }
            } else {
              Err(script_err)
            }
          }
        }
      }
    }),
    DenoSubcommand::Repl(repl_flags) => {
      spawn_subcommand(async move { tools::repl::run(flags, repl_flags).await })
    }
    DenoSubcommand::Serve(serve_flags) => {
      spawn_subcommand(
        async move { tools::serve::serve(flags, serve_flags).await },
      )
    }
    DenoSubcommand::Task(task_flags) => spawn_subcommand(async {
      tools::task::execute_script(flags, task_flags).await
    }),
    DenoSubcommand::Test(test_flags) => {
      spawn_subcommand(async {
        if let Some(ref coverage_dir) = test_flags.coverage_dir {
          if test_flags.clean {
            let _ = std::fs::remove_dir_all(coverage_dir);
          }
          std::fs::create_dir_all(coverage_dir)
            .with_context(|| format!("Failed creating: {coverage_dir}"))?;
          // this is set in order to ensure spawned processes use the same
          // coverage directory
          env::set_var(
            "DENO_UNSTABLE_COVERAGE_DIR",
            PathBuf::from(coverage_dir).canonicalize()?,
          );
        }

        if test_flags.watch.is_some() {
          tools::test::run_tests_with_watch(flags, test_flags).await
        } else {
          tools::test::run_tests(flags, test_flags).await
        }
      })
    }
    DenoSubcommand::Uninstall(uninstall_flags) => spawn_subcommand(async {
      tools::installer::uninstall(flags, uninstall_flags).await
    }),
    _ => exit_with_message("Unimplemented subcommand", 1),
  };

  handle.await?
}

pub(crate) fn unstable_exit_cb(feature: &str, api_name: &str) {
  log::error!(
    "Unstable API '{api_name}'. The `--unstable-{}` flag must be provided.",
    feature
  );
  deno_runtime::exit(70);
}

pub fn cli(args: Vec<std::ffi::OsString>) -> Result<(), AnyError> {
  let future = async move {
    let flags = resolve_flags_and_init(args)?;
    run_subcommand(Arc::new(flags)).await
  };
  unwrap_or_err(create_and_run_current_thread_with_maybe_metrics(future))?;
  Ok(())
}
