import * as browse from "ext:deno_rb/00_browse.js";
import * as library from "ext:deno_rb/01_library.js";
import * as playback from "ext:deno_rb/02_playback.js";
import * as playlist from "ext:deno_rb/03_playlist.js";
import * as settings from "ext:deno_rb/04_settings.js";
import * as sound from "ext:deno_rb/05_sound.js";
import * as system from "ext:deno_rb/06_system.js";

const rockboxNs = {
  browse: browse.default,
  library,
  playback,
  playlist,
  settings,
  sound,
  system,
};

export { rockboxNs };
