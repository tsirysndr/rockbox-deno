import {
  op_get_album,
  op_get_albums,
  op_get_artist,
  op_get_artists,
  op_get_liked_albums,
  op_get_liked_tracks,
  op_get_track,
  op_get_tracks,
  op_like_track,
  op_scan_library,
  op_search,
  op_unlike_track,
} from "ext:core/ops";

export async function getAlbums() {
  await op_get_albums();
}

export async function getArtists() {
  await op_get_artists();
}

export async function getTracks() {
  await op_get_tracks();
}

export async function getAlbum(id) {
  await op_get_album(id);
}

export async function getArtist(id) {
  await op_get_artist(id);
}

export async function getTrack(id) {
  await op_get_track(id);
}

export async function likeTrack(id) {
  await op_like_track(id);
}

export async function unlikeTrack(id) {
  await op_unlike_track(id);
}

export async function getLikedTracks() {
  await op_get_liked_tracks();
}

export async function getLikedAlbums() {
  await op_get_liked_albums();
}

export async function scanLibrary(path) {
  await op_scan_library(path);
}

export async function search(query) {
  await op_search(query);
}
