pub mod browse;
pub mod library;
pub mod playback;
pub mod playlist;
pub mod settings;
pub mod sound;
pub mod system;
pub mod types;

use browse::*;
use library::*;
use playback::*;
use playlist::*;
use settings::*;
use sound::*;
use system::*;

use std::path::PathBuf;

deno_core::extension!(
  deno_rb,
  ops = [
    // browse
    op_rockbox_browse,
    op_tree_get_entries,
    // library
    op_get_albums,
    op_get_artists,
    op_get_tracks,
    op_get_album,
    op_get_artist,
    op_get_track,
    op_like_track,
    op_unlike_track,
    // playback
    op_play,
    op_pause,
    op_resume,
    op_next,
    op_previous,
    op_fast_forward_rewind,
    op_status,
    op_current_track,
    op_next_track,
    op_flush_and_reload_tracks,
    op_get_file_position,
    op_hard_stop,
    // playlist
    op_playlist_get_resume_info,
    op_playlist_get_track_info,
    op_playlist_get_first_index,
    op_playlist_get_display_index,
    op_playlist_amount,
    op_playlist_resume,
    op_playlist_resume_track,
    op_playlist_set_modified,
    op_playlist_start,
    op_playlist_sync,
    op_playlist_remove_all_tracks,
    op_playlist_remove_tracks,
    op_create_playlist,
    op_playlist_insert_tracks,
    op_playlist_insert_directory,
    op_insert_playlist,
    op_shuffle_playlist,
    // settings
    op_get_global_settings,
    op_save_settings,
    // sound
    op_adjust_volume,
    // system
    op_get_global_status,
    op_get_rockbox_version,
  ],
  esm = [
    "00_browse.js",
    "01_library.js",
    "02_playback.js",
    "03_playlist.js",
    "04_settings.js",
    "05_sound.js",
    "06_system.js",
  ],
);

pub fn get_declaration() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("lib.deno_rb.d.ts")
}
