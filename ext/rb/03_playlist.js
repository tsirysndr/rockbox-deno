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
} from "ext:core/ops";

export async function getResumeInfo() {
  await op_playlist_get_resume_info();
}

export async function getTrackInfo() {
  await op_playlist_get_track_info();
}

export async function getFirstIndex() {
  await op_playlist_get_first_index();
}

export async function getDisplayIndex() {
  await op_playlist_get_display_index();
}

export async function amount() {
  await op_playlist_amount();
}

export async function playlistResume() {
  await op_playlist_resume();
}

export async function resumeTrack(startIndex, crc, offset, elapsed) {
  await op_playlist_resume_track(startIndex, crc, offset, elapsed);
}

export async function setModified() {
  await op_playlist_set_modified();
}

export async function start(startIndex, elapsed, offset) {
  await op_playlist_start(startIndex, elapsed, offset);
}

export async function sync() {
  await op_playlist_sync();
}

export async function removeAllTracks() {
  await op_playlist_remove_all_tracks();
}

export async function removeTracks(params) {
  await op_playlist_remove_tracks(params);
}

export async function createPlaylist(params) {
  await op_create_playlist(params);
}

export async function insertTracks(params) {
  await op_playlist_insert_tracks(params);
}

export async function insertDirectory(params) {
  await op_playlist_insert_directory(params);
}

export async function insertPlaylist(params) {
  await op_insert_playlist(params);
}

export async function insertAlbum(params) {
  await op_playlist_insert_album(params);
}

export async function insertArtistTracks(params) {
  await op_playlist_insert_artist_tracks(params);
}

export async function shufflePlaylist(startIndex) {
  await op_shuffle_playlist(startIndex);
}
