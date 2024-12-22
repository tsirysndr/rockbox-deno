use deno_core::{error::AnyError, op2};

use crate::{
  api::rockbox::v1alpha1::{
    library_service_client::LibraryServiceClient, GetAlbumRequest,
    GetAlbumsRequest, GetArtistRequest, GetArtistsRequest,
    GetLikedAlbumsRequest, GetLikedTracksRequest, GetTrackRequest,
    GetTracksRequest, LikeAlbumRequest, LikeTrackRequest, ScanLibraryRequest,
    SearchRequest, UnlikeAlbumRequest, UnlikeTrackRequest,
  },
  build_url,
  types::SearchResults,
};

use crate::types::{Album, Artist, Track};

#[op2(async)]
#[serde]
pub async fn op_get_albums() -> Result<Vec<Album>, AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let response = client.get_albums(GetAlbumsRequest {}).await?.into_inner();
  Ok(response.albums.into_iter().map(Album::from).collect())
}

#[op2(async)]
#[serde]
pub async fn op_get_artists() -> Result<Vec<Artist>, AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let response = client.get_artists(GetArtistsRequest {}).await?.into_inner();
  Ok(response.artists.into_iter().map(Artist::from).collect())
}

#[op2(async)]
#[serde]
pub async fn op_get_tracks() -> Result<Vec<Track>, AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let response = client.get_tracks(GetTracksRequest {}).await?.into_inner();
  Ok(response.tracks.into_iter().map(Track::from).collect())
}

#[op2(async)]
#[serde]
pub async fn op_get_album(
  #[string] id: String,
) -> Result<Option<Album>, AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let response = client.get_album(GetAlbumRequest { id }).await?.into_inner();
  Ok(response.album.map(Album::from))
}

#[op2(async)]
#[serde]
pub async fn op_get_artist(
  #[string] id: String,
) -> Result<Option<Artist>, AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let response = client
    .get_artist(GetArtistRequest { id })
    .await?
    .into_inner();
  Ok(response.artist.map(Artist::from))
}

#[op2(async)]
#[serde]
pub async fn op_get_track(
  #[string] id: String,
) -> Result<Option<Track>, AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let response = client.get_track(GetTrackRequest { id }).await?.into_inner();
  Ok(response.track.map(Track::from))
}

#[op2(async)]
pub async fn op_like_track(#[string] id: String) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let _response = client
    .like_track(LikeTrackRequest { id })
    .await?
    .into_inner();
  Ok(())
}

#[op2(async)]
pub async fn op_unlike_track(#[string] id: String) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let _response = client
    .unlike_track(UnlikeTrackRequest { id })
    .await?
    .into_inner();
  Ok(())
}

#[op2(async)]
pub async fn op_like_album(#[string] id: String) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let _response = client
    .like_album(LikeAlbumRequest { id })
    .await?
    .into_inner();
  Ok(())
}

#[op2(async)]
pub async fn op_unlike_album(#[string] id: String) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let _response = client
    .unlike_album(UnlikeAlbumRequest { id })
    .await?
    .into_inner();
  Ok(())
}

#[op2(async)]
#[serde]
pub async fn op_get_liked_tracks() -> Result<Vec<Track>, AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let response = client
    .get_liked_tracks(GetLikedTracksRequest {})
    .await?
    .into_inner();
  Ok(response.tracks.into_iter().map(Track::from).collect())
}

#[op2(async)]
#[serde]
pub async fn op_get_liked_albums() -> Result<Vec<Album>, AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let response = client
    .get_liked_albums(GetLikedAlbumsRequest {})
    .await?
    .into_inner();
  Ok(response.albums.into_iter().map(Album::from).collect())
}

#[op2(async)]
pub async fn op_scan_library(
  #[string] path: Option<String>,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let _response = client
    .scan_library(ScanLibraryRequest { path })
    .await?
    .into_inner();
  Ok(())
}

#[op2(async)]
#[serde]
pub async fn op_search(
  #[string] term: String,
) -> Result<SearchResults, AnyError> {
  let url = build_url();
  let mut client = LibraryServiceClient::connect(url).await?;
  let response = client.search(SearchRequest { term }).await?.into_inner();
  Ok(response.into())
}
