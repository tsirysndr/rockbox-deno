use deno_core::{error::AnyError, op2};
use serde::{Deserialize, Serialize};

use crate::{
  api::rockbox::v1alpha1::{
    playlist_service_client::PlaylistServiceClient, AmountRequest,
    CreatePlaylistRequest, GetCurrentRequest, GetDisplayIndexRequest,
    GetFirstIndexRequest, GetResumeInfoRequest, GetTrackInfoRequest,
    InsertAlbumRequest, InsertArtistTracksRequest, InsertDirectoryRequest,
    InsertPlaylistRequest, InsertTracksRequest, PlaylistResumeRequest,
    RemoveAllTracksRequest, RemoveTracksRequest, ResumeTrackRequest,
    SetModifiedRequest, ShufflePlaylistRequest, StartRequest, SyncRequest,
  },
  build_url,
  types::CurrentPlaylist,
};

#[derive(Serialize, Deserialize)]
pub struct RemoveTracksParams {
  pub positions: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePlaylistParams {
  name: String,
  tracks: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct InsertTracksParams {
  pub position: i32,
  pub tracks: Vec<String>,
  pub playlist_id: Option<String>,
  pub shuffle: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct InsertDirectoryParams {
  pub directory: String,
  pub position: i32,
  pub shuffle: Option<bool>,
  pub recurse: Option<bool>,
  pub playlist_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct InsertAlbumParams {
  pub album_id: String,
  pub position: i32,
  pub shuffle: Option<bool>,
  pub playlist_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct InsertArtistTracksParams {
  pub artist_id: String,
  pub position: i32,
  pub shuffle: Option<bool>,
  pub playlist_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct InsertPlaylistParams {
  pub position: i32,
  pub playlist_id: String,
  pub shuffle: Option<bool>,
  pub target_playlist_id: Option<String>,
}

#[op2(async)]
pub async fn op_playlist_get_resume_info() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  let _response = client
    .get_resume_info(GetResumeInfoRequest {})
    .await?
    .into_inner();
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_get_track_info() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  let _response = client
    .get_track_info(GetTrackInfoRequest {})
    .await?
    .into_inner();
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_get_first_index() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  let _response = client
    .get_first_index(GetFirstIndexRequest {})
    .await?
    .into_inner();
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_get_display_index() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  let _response = client
    .get_display_index(GetDisplayIndexRequest {})
    .await?
    .into_inner();
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_amount() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  let _response = client.amount(AmountRequest {}).await?.into_inner();
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_resume() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client.playlist_resume(PlaylistResumeRequest {}).await?;
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_resume_track(
  start_index: i32,
  crc: u32,
  offset: u32,
  elapsed: u32,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client
    .resume_track(ResumeTrackRequest {
      start_index,
      crc,
      offset: offset as u64,
      elapsed: elapsed as u64,
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_set_modified() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client.set_modified(SetModifiedRequest {}).await?;
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_start(
  start_index: Option<i32>,
  elapsed: Option<i32>,
  offset: Option<i32>,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client
    .start(StartRequest {
      start_index,
      elapsed,
      offset,
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_sync() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client.sync(SyncRequest {}).await?;
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_remove_all_tracks() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client.remove_all_tracks(RemoveAllTracksRequest {}).await?;
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_remove_tracks(
  #[serde] params: RemoveTracksParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client
    .remove_tracks(RemoveTracksRequest {
      positions: params.positions,
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_create_playlist(
  #[serde] params: CreatePlaylistParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client
    .create_playlist(CreatePlaylistRequest {
      name: params.name,
      tracks: params.tracks,
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_insert_tracks(
  #[serde] params: InsertTracksParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client
    .insert_tracks(InsertTracksRequest {
      position: params.position,
      tracks: params.tracks,
      playlist_id: params.playlist_id,
      shuffle: params.shuffle,
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_insert_directory(
  #[serde] params: InsertDirectoryParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client
    .insert_directory(InsertDirectoryRequest {
      directory: params.directory,
      position: params.position,
      shuffle: params.shuffle,
      recurse: params.recurse,
      playlist_id: params.playlist_id,
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_insert_album(
  #[serde] params: InsertAlbumParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client
    .insert_album(InsertAlbumRequest {
      album_id: params.album_id,
      position: params.position,
      shuffle: params.shuffle,
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_playlist_insert_artist_tracks(
  #[serde] params: InsertArtistTracksParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client
    .insert_artist_tracks(InsertArtistTracksRequest {
      artist_id: params.artist_id,
      position: params.position,
      shuffle: params.shuffle,
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_insert_playlist(
  #[serde] params: InsertPlaylistParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client
    .insert_playlist(InsertPlaylistRequest {
      playlist_id: params.playlist_id,
      position: params.position,
      shuffle: params.shuffle,
      target_playlist_id: params
        .target_playlist_id
        .unwrap_or("current".to_string()),
    })
    .await?;
  Ok(())
}

#[op2(async)]
pub async fn op_shuffle_playlist(
  start_index: Option<i32>,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  client
    .shuffle_playlist(ShufflePlaylistRequest {
      start_index: start_index.unwrap_or(0),
    })
    .await?;
  Ok(())
}

#[op2(async)]
#[serde]
pub async fn op_playlist_get_curent() -> Result<CurrentPlaylist, AnyError> {
  let url = build_url();
  let mut client = PlaylistServiceClient::connect(url).await?;
  let response = client.get_current(GetCurrentRequest {}).await?.into_inner();
  Ok(response.into())
}
