use deno_core::op2;

#[op2(async)]
pub async fn op_playlist_get_resume_info() {}

#[op2(async)]
pub async fn op_playlist_get_track_info() {}

#[op2(async)]
pub async fn op_playlist_get_first_index() {}

#[op2(async)]
pub async fn op_playlist_get_display_index() {}

#[op2(async)]
pub async fn op_playlist_amount() {}

#[op2(async)]
pub async fn op_playlist_resume() {}

#[op2(async)]
pub async fn op_playlist_resume_track() {}

#[op2(async)]
pub async fn op_playlist_set_modified() {}

#[op2(async)]
pub async fn op_playlist_start() {}

#[op2(async)]
pub async fn op_playlist_sync() {}

#[op2(async)]
pub async fn op_playlist_remove_all_tracks() {}

#[op2(async)]
pub async fn op_playlist_remove_tracks() {}

#[op2(async)]
pub async fn op_create_playlist() {}

#[op2(async)]
pub async fn op_playlist_insert_tracks() {}

#[op2(async)]
pub async fn op_playlist_insert_directory() {}

#[op2(async)]
pub async fn op_insert_playlist() {}

#[op2(async)]
pub async fn op_shuffle_playlist() {}
