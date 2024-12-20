/// <reference no-default-lib="true" />
/// <reference lib="esnext" />
/// <reference lib="esnext.disposable" />

declare namespace Rb {
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

  export const browse: {
    tree: {
      getEntries(path?: string): Promise<Entry[]>;
    };
  };

  export const library: {
    getAlbums(): Promise<Album[]>;
    getArtists(): Promise<Artist[]>;
    getTracks(): Promise<Track[]>;
    getAlbum(id: string): Promise<Album>;
    getArtist(id: string): Promise<Artist>;
    getTrack(id: string): Promise<Track>;
    likeTrack(id: string): Promise<void>;
    unlikeTrack(id: string): Promise<void>;
  };
  export const playback: {
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
  export const playlist: {
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
  export const settings: {
    getGlobalSettings: () => Promise<void>;
  };
  export const sound: {
    adjustVolume: (steps: number) => Promise<void>;
  };
  export const system: {
    getGlobalStatus: () => Promise<void>;
    getRockboxVersion: () => Promise<void>;
  };
}
