import {
  op_create_playlist,
  op_insert_playlist,
  op_playlist_amount,
  op_playlist_get_display_index,
  op_playlist_get_first_index,
  op_playlist_get_resume_info,
  op_playlist_get_track_info,
  op_playlist_insert_album,
  op_playlist_insert_artist_tracks,
  op_playlist_insert_directory,
  op_playlist_insert_tracks,
  op_playlist_remove_all_tracks,
  op_playlist_remove_tracks,
  op_playlist_resume,
  op_playlist_resume_track,
  op_playlist_set_modified,
  op_playlist_start,
  op_playlist_sync,
  op_shuffle_playlist,
  op_playlist_get_current,
} from "ext:core/ops";

export function getResumeInfo() {
  return op_playlist_get_resume_info();
}

export function getTrackInfo() {
  return op_playlist_get_track_info();
}

export function getFirstIndex() {
  return op_playlist_get_first_index();
}

export function getDisplayIndex() {
  return op_playlist_get_display_index();
}

export function amount() {
  return op_playlist_amount();
}

export function playlistResume() {
  return op_playlist_resume();
}

export function resumeTrack(startIndex, crc, offset, elapsed) {
  return op_playlist_resume_track(startIndex, crc, offset, elapsed);
}

export function setModified() {
  return op_playlist_set_modified();
}

export function start(startIndex, elapsed, offset) {
  return op_playlist_start(startIndex, elapsed, offset);
}

export function sync() {
  return op_playlist_sync();
}

export function removeAllTracks() {
  return op_playlist_remove_all_tracks();
}

export function removeTracks(params) {
  return op_playlist_remove_tracks(params);
}

export function createPlaylist(params) {
  return op_create_playlist(params);
}

export function insertTracks(params) {
  return op_playlist_insert_tracks(params);
}

export function insertDirectory(params) {
  return op_playlist_insert_directory(params);
}

export function insertPlaylist(params) {
  return op_insert_playlist(params);
}

export function insertAlbum(params) {
  return op_playlist_insert_album(params);
}

export function insertArtistTracks(params) {
  return op_playlist_insert_artist_tracks(params);
}

export function shufflePlaylist(startIndex) {
  return op_shuffle_playlist(startIndex);
}

export function getCurrentPlaylist() {
  return op_playlist_get_current();
}
