use deno_core::{error::AnyError, op2};
use serde::{Deserialize, Serialize};

use crate::{
  api::rockbox::v1alpha1::{
    playback_service_client::PlaybackServiceClient, CurrentTrackRequest,
    FastForwardRewindRequest, FlushAndReloadTracksRequest,
    GetFilePositionRequest, HardStopRequest, NextTrackRequest, PauseRequest,
    PlayAlbumRequest, PlayAllTracksRequest, PlayArtistTracksRequest,
    PlayDirectoryRequest, PlayLikedTracksRequest, PlayPlaylistRequest,
    PlayRequest, PlayTrackRequest, PreviousRequest, ResumeRequest,
    StatusRequest,
  },
  build_url,
  types::{CurrentTrack, NextTrack},
};

#[derive(Serialize, Deserialize)]
pub struct PlayAllTracksParams {
  pub shuffle: Option<bool>,
  pub position: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayLikedTracksParams {
  pub shuffle: Option<bool>,
  pub position: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayDirectoryParams {
  pub path: String,
  pub shuffle: Option<bool>,
  pub recurse: Option<bool>,
  pub position: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayPlaylistParams {
  pub playlist_id: String,
  pub shuffle: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayArtistTracksParams {
  pub artist_id: String,
  pub shuffle: Option<bool>,
  pub position: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayAlbumParams {
  pub album_id: String,
  pub shuffle: Option<bool>,
  pub position: Option<i32>,
}

#[op2(async)]
pub async fn op_play(elapsed: i32, offset: i32) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client
    .play(PlayRequest {
      elapsed: elapsed as i64,
      offset: offset as i64,
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_pause() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client.pause(PauseRequest {}).await?;
  Ok(())
}

#[op2(async)]
pub async fn op_resume() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client.resume(ResumeRequest {}).await?;
  Ok(())
}

#[op2(async)]
pub async fn op_next() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client.next_track(NextTrackRequest {}).await?;
  Ok(())
}

#[op2(async)]
pub async fn op_previous() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client.previous(PreviousRequest {}).await?;
  Ok(())
}

#[op2(async)]
pub async fn op_fast_forward_rewind(new_time: i32) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client
    .fast_forward_rewind(FastForwardRewindRequest { new_time })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_status() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client.status(StatusRequest {}).await?;
  Ok(())
}

#[op2(async)]
#[serde]
pub async fn op_current_track() -> Result<CurrentTrack, AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  let response = client
    .current_track(CurrentTrackRequest {})
    .await?
    .into_inner();
  Ok(response.into())
}

#[op2(async)]
#[serde]
pub async fn op_next_track() -> Result<NextTrack, AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  let response = client.next_track(NextTrackRequest {}).await?.into_inner();
  Ok(response.into())
}

#[op2(async)]
pub async fn op_flush_and_reload_tracks() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client
    .flush_and_reload_tracks(FlushAndReloadTracksRequest {})
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_get_file_position() -> Result<i32, AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  let response = client
    .get_file_position(GetFilePositionRequest {})
    .await?
    .into_inner();
  Ok(response.position)
}

#[op2(async)]
pub async fn op_hard_stop() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client.hard_stop(HardStopRequest {}).await?;
  Ok(())
}

#[op2(async)]
pub async fn op_play_album(
  #[serde] params: PlayAlbumParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client
    .play_album(PlayAlbumRequest {
      album_id: params.album_id,
      shuffle: params.shuffle,
      position: params.position,
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_play_artist_tracks(
  #[serde] params: PlayArtistTracksParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client
    .play_artist_tracks(PlayArtistTracksRequest {
      artist_id: params.artist_id,
      shuffle: params.shuffle,
      position: params.position,
    })
    .await?;

  Ok(())
}

#[op2(async)]
pub async fn op_play_playlist(
  #[serde] params: PlayPlaylistParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client
    .play_playlist(PlayPlaylistRequest {
      playlist_id: params.playlist_id,
      shuffle: params.shuffle,
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_play_directory(
  #[serde] params: PlayDirectoryParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client
    .play_directory(PlayDirectoryRequest {
      path: params.path,
      shuffle: params.shuffle,
      recurse: params.recurse,
      position: params.position,
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_play_track(#[string] path: String) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client.play_track(PlayTrackRequest { path }).await?;
  Ok(())
}

#[op2(async)]
pub async fn op_play_liked_tracks(
  #[serde] params: PlayLikedTracksParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client
    .play_liked_tracks(PlayLikedTracksRequest {
      shuffle: params.shuffle,
      position: params.position,
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_play_all_tracks(
  #[serde] params: PlayAllTracksParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaybackServiceClient::connect(url).await?;
  client
    .play_all_tracks(PlayAllTracksRequest {
      shuffle: params.shuffle,
      position: params.position,
    })
    .await?;
  Ok(())
}
