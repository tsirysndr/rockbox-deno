use deno_core::op2;

#[op2(async)]
pub async fn op_get_albums() {}

#[op2(async)]
pub async fn op_get_artists() {}

#[op2(async)]
pub async fn op_get_tracks() {}

#[op2(async)]
pub async fn op_get_album() {}

#[op2(async)]
pub async fn op_get_artist() {}

#[op2(async)]
pub async fn op_get_track() {}

#[op2(async)]
pub async fn op_like_track() {}

#[op2(async)]
pub async fn op_unlike_track() {}
