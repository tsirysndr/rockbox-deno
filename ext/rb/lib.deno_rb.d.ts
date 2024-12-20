/// <reference no-default-lib="true" />
/// <reference lib="esnext" />
/// <reference lib="esnext.disposable" />

declare namespace Deno {
  export interface Entry {
    name: string;
    attr: number;
    timeWrite: number;
    customaction: string;
  }

  export interface Album {
    id: string;
  }

  export interface Artist {
    id: string;
  }

  export interface Track {
    id: string;
  }

  export interface Mp3Entry {
    path: string;
  }

  export interface SystemStatus {
    resumeIndex: number;
    resumeCrc32: number;
    resumeElapsed: number;
    resumeOffset: number;
    runtime: number;
    topruntime: number;
    dircacheSize: number;
    lastScreen: number;
    viewerIconCount: number;
    lastVolumeChange: number;
  }

  export const rb: {
    browse: {
      getEntries(path?: string): Promise<Entry[]>;
    };
    library: {
      getAlbum(id: string): Promise<Album>;
      getArtist(id: string): Promise<Artist>;
      getTrack(id: string): Promise<Track>;
    };
    playback: {
      play(elapsed: number, offset: number): Promise<void>;
      pause(): Promise<void>;
      resume(): Promise<void>;
      next(): Promise<void>;
      previous(): Promise<void>;
      fastForwardRewind(): Promise<void>;
      status(): Promise<number>;
      currentTrack(): Promise<Mp3Entry | null>;
      nextTrack(): Promise<Mp3Entry | null>;
      flushAndReloadTracks(): Promise<void>;
      getFilePosition(): Promise<number>;
      hardStop(): Promise<void>;
    };
    playlist: {
      getResumeInfo(): Promise<void>;
      getTrackInfo(): Promise<void>;
      getFirstIndex(): Promise<number>;
      getDisplayIndex(): Promise<number>;
      amount(): Promise<number>;
      playlistResume(): Promise<void>;
      resumeTrack(): Promise<void>;
      setModified(): Promise<void>;
      start(): Promise<void>;
      sync(): Promise<void>;
      removeAllTracks(): Promise<void>;
      createPlaylist(): Promise<void>;
      insertTrack(): Promise<void>;
      insertDirectory(): Promise<void>;
      insertPlaylist(): Promise<void>;
      shufflePlaylist(): Promise<void>;
    };
    settings: {
      getGlobalSettings: () => Promise<void>;
    };
    sound: {
      adjustVolume: (steps: number) => Promise<void>;
    };
    system: {
      getGlobalStatus: () => Promise<void>;
      getRockboxVersion: () => Promise<void>;
    };
  };
}
