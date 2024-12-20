use deno_core::op2;

#[op2(async)]
pub async fn op_play() {}

#[op2(async)]
pub async fn op_pause() {}

#[op2(async)]
pub async fn op_resume() {}

#[op2(async)]
pub async fn op_next() {}

#[op2(async)]
pub async fn op_previous() {}

#[op2(async)]
pub async fn op_fast_forward_rewind() {}

#[op2(async)]
pub async fn op_status() {}

#[op2(async)]
pub async fn op_current_track() {}

#[op2(async)]
pub async fn op_next_track() {}

#[op2(async)]
pub async fn op_flush_and_reload_tracks() {}

#[op2(async)]
pub async fn op_get_file_position() {}

#[op2(async)]
pub async fn op_hard_stop() {}
