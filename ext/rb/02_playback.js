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

export function play(elapsed, offset) {
  return op_play(elapsed, offset);
}

export function pause() {
  return op_pause();
}

export function resume() {
  return op_resume();
}

export function next() {
  return op_next();
}

export function previous() {
  return op_previous();
}

export function fastForwardRewind(newTime) {
  return op_fast_forward_rewind(newTime);
}

export function status() {
  return op_status();
}

export function currentTrack() {
  return op_current_track();
}

export function nextTrack() {
  return op_next_track();
}

export function flushAndReloadTracks() {
  return op_flush_and_reload_tracks();
}

export function getFilePosition() {
  return op_get_file_position();
}

export function hardStop() {
  return op_hard_stop();
}

export function playAlbum(options) {
  return op_play_album(options);
}

export function playAllTracks(options) {
  return op_play_all_tracks(options);
}

export function playArtistTracks(options) {
  return op_play_artist_tracks(options);
}

export function playDirectory(options) {
  return op_play_directory(options);
}

export function playLikedTracks(options) {
  return op_play_liked_tracks(options);
}

export function playPlaylist(options) {
  return op_play_playlist(options);
}

export function playTrack(path) {
  return op_play_track(path);
}
