use deno_core::op2;

#[op2(async)]
pub async fn op_get_global_settings() {}

#[op2(async)]
pub async fn op_save_settings() {}
