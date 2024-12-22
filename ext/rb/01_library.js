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

export function getAlbums() {
  return op_get_albums();
}

export function getArtists() {
  return op_get_artists();
}

export function getTracks() {
  return op_get_tracks();
}

export function getAlbum(id) {
  return op_get_album(id);
}

export function getArtist(id) {
  return op_get_artist(id);
}

export function getTrack(id) {
  return op_get_track(id);
}

export function likeTrack(id) {
  return op_like_track(id);
}

export function unlikeTrack(id) {
  return op_unlike_track(id);
}

export function getLikedTracks() {
  return op_get_liked_tracks();
}

export function getLikedAlbums() {
  return op_get_liked_albums();
}

export function scanLibrary(path) {
  return op_scan_library(path);
}

export function search(query) {
  return op_search(query);
}
