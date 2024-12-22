import {
  op_current_track,
  op_fast_forward_rewind,
  op_flush_and_reload_tracks,
  op_get_file_position,
  op_hard_stop,
  op_next,
  op_next_track,
  op_pause,
  op_play,
  op_play_album,
  op_play_all_tracks,
  op_play_artist_tracks,
  op_play_directory,
  op_play_liked_tracks,
  op_play_playlist,
  op_play_track,
  op_previous,
  op_resume,
  op_status,
} from "ext:core/ops";

export async function play(elapsed, offset) {
  await op_play(elapsed, offset);
}

export async function pause() {
  await op_pause();
}

export async function resume() {
  await op_resume();
}

export async function next() {
  await op_next();
}

export async function previous() {
  await op_previous();
}

export async function fastForwardRewind(newTime) {
  await op_fast_forward_rewind(newTime);
}

export async function status() {
  await op_status();
}

export async function currentTrack() {
  await op_current_track();
}

export async function nextTrack() {
  await op_next_track();
}

export async function flushAndReloadTracks() {
  await op_flush_and_reload_tracks();
}

export async function getFilePosition() {
  await op_get_file_position();
}

export async function hardStop() {
  await op_hard_stop();
}

export async function playAlbum(options) {
  await op_play_album(options);
}

export async function playAllTracks(options) {
  await op_play_all_tracks(options);
}

export async function playArtistTracks(options) {
  await op_play_artist_tracks(options);
}

export async function playDirectory(options) {
  await op_play_directory(options);
}

export async function playLikedTracks(options) {
  await op_play_liked_tracks(options);
}

export async function playPlaylist(options) {
  await op_play_playlist(options);
}

export async function playTrack(path) {
  await op_play_track(path);
}
