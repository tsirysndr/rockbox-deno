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
    title: string;
    artist: string;
    year: number;
    yearString: string;
    albumArt?: string;
    md5: string;
    artistId: string;
    tracks: Track[];
  }

  export interface Artist {
    id: string;
    name: string;
    bio?: string;
    image?: string;
    albums: Album[];
    tracks: Track[];
  }

  export interface Track {
    id: string;
    path: string;
    artist: string;
    album: string;
    albumArtist: string;
    bitrate: number;
    composer: string;
    discNumber: number;
    filesize: number;
    frequency: number;
    length: number;
    trackNumber: number;
    year: number;
    yearString: string;
    genre: string;
    md5: string;
    albumArt?: string;
    artist_id?: string;
    album_id?: string;
    genre_id?: string;
    createdAt: string;
    updatedAt: string;
  }

  export interface CurrentTrack {
    id: string;
    title: string;
    artist: string;
    album: string;
    genre: string;
    disc: string;
    trackString: string;
    yearString: string;
    composer: string;
    comment: string;
    albumArtist: string;
    grouping: number;
    discnum: number;
    tracknum: number;
    layer: number;
    year: number;
    bitrate: number;
    frequency: number;
    filesize: number;
    length: number;
    elapsed: number;
    path: string;
    albumArt?: string;
    albumId: string;
    artistId: string;
  }

  export interface NextTrack {
    path: string;
    title: string;
    artist: string;
    album: string;
    genre: string;
    disc: string;
    trackString: string;
    yearString: string;
    composer: string;
    comment: string;
    albumArtist: string;
    grouping: number;
    discnum: number;
    tracknum: number;
    layer: number;
    year: number;
    bitrate: number;
    frequency: number;
    length: number;
    elapsed: number;
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

  export interface RemoveTracksOptions {
    positions: number[];
  }

  export interface CreatePlaylistOptions {
    name: string;
    tracks: string[];
  }

  export interface InsertTracksOptions {
    position: number;
    tracks: string[];
    shuffle?: boolean;
  }

  export interface InsertDirectoryOptions {
    directory: string;
    position: number;
    shuffle?: boolean;
    recurse?: boolean;
  }

  export interface InsertPlaylistOptions {
    position: number;
  }

  export interface InsertAlbumOptions {
    albumId: string;
    position: number;
    shuffle?: boolean;
  }

  export interface InsertArtistTracksOptions {
    artistId: string;
    position: number;
    shuffle?: boolean;
  }

  export interface ReplaygainSettings {
    noclip: boolean;
    type: number;
    preamp: number;
  }

  export interface EqBandSetting {
    cutoff: number;
    q: number;
    gain: number;
  }

  export interface Settings {
    volume: number;
    balance: number;
    bass: number;
    treble: number;
    channelConfig: number;
    stereoWidth: number;
    bassCutoff: number;
    trebleCutoff: number;
    crossfade: number;
    crossfadeFadeInDelay: number;
    crossfadeFadeOutDelay: number;
    crossfadeFadeInDuration: number;
    crossfadeFadeOutDuration: number;
    crossfadeFadeOutMixmode: number;
    ReplaygainSettings: ReplaygainSettings;
    crossfeed: number;
    crossfeedDirectGain: number;
    crossfeedCrossGain: number;
    crossfeedHfAttenuation: number;
    crossfeedHfCutoff: number;
    eqEnabled: boolean;
    eqPrecut: number;
    eqBandSettings: EqBandSetting[];
    beep: number;
    keyclick: number;
    keyclickRepeats: number;
    ditheringEnabled: boolean;
    timestretchEnabled: boolean;
    partyMode: boolean;
    volumeType: number;
    playerName: string;
    musicDir: string;
    repeatMode: number;
    playlistShuffle: boolean;
  }

  export interface PlayAlbumOptions {
    albumId: string;
    shuffle?: boolean;
  }

  export interface PlayAllTracksOptions {
    shuffle?: boolean;
    position?: number;
  }

  export interface PlayArtistTracksOptions {
    artistId: string;
    shuffle?: boolean;
    position?: number;
  }

  export interface PlayDirectoryOptions {
    path: string;
    shuffle?: boolean;
    recurse?: boolean;
    position?: number;
  }

  export interface PlayLikedTracksOptions {
    shuffle?: boolean;
    position?: number;
  }

  export interface PlayPlaylistOptions {
    playlistId: string;
    shuffle?: boolean;
  }

  export const browse: {
    tree: {
      getEntries(path?: string): Promise<Entry[]>;
    };
  };

  export type SearchResults = {
    albums: Album[];
    artists: Artist[];
    tracks: Track[];
  };

  export interface SettingsOptions {
    musicSir?: string;
    playlistShuffle?: boolean;
    repeatMode?: number;
    bass?: number;
    treble?: number;
    bassCuttoff?: number;
    trebleCutoff?: number;
    crossfade?: number;
    fadeOnStop?: number;
    fadeInDelay?: number;
    fadeInDuration?: number;
    fadeOutDelay?: number;
    fadeOutDuration?: number;
    fadeOutMixmode?: number;
    balance?: number;
    stereoWidth?: number;
    stereoswMode?: number;
    surrfoundEnabled?: number;
    surroundBalance?: number;
    surroundFx1?: number;
    surroundFx2?: number;
    partyMode?: boolean;
    channelConfig?: number;
    playerName?: string;
    eqEnabled?: boolean;
    eqBandSettings?: EqBandSetting[];
    replaygainSettins?: ReplaygainSettings;
  }

  export interface CurrentPlaylist {
    index: number;
    amount: number;
    maxPlaylistSize: number;
    firstIndex: number;
    lastInsertPos: number;
    seed: number;
    lastShuffledStart: number;
    tracks: Track[];
  }

  export const library: {
    getAlbums(): Promise<Album[]>;
    getArtists(): Promise<Artist[]>;
    getTracks(): Promise<Track[]>;
    getAlbum(id: string): Promise<Album>;
    getArtist(id: string): Promise<Artist>;
    getTrack(id: string): Promise<Track>;
    likeTrack(id: string): Promise<void>;
    unlikeTrack(id: string): Promise<void>;
    scanLibrary(path?: string): Promise<void>;
    search(query: string): Promise<SearchResults>;
  };
  export const playback: {
    play(elapsed: number, offset: number): Promise<void>;
    pause(): Promise<void>;
    resume(): Promise<void>;
    next(): Promise<void>;
    previous(): Promise<void>;
    fastForwardRewind(newTime: number): Promise<void>;
    status(): Promise<number>;
    currentTrack(): Promise<CurrentTrack | null>;
    nextTrack(): Promise<NextTrack | null>;
    flushAndReloadTracks(): Promise<void>;
    getFilePosition(): Promise<number>;
    hardStop(): Promise<void>;
    playAlbum(options: PlayAlbumOptions): Promise<void>;
    playAllTracks(options: PlayAllTracksOptions): Promise<void>;
    playArtistTracks(options: PlayArtistTracksOptions): Promise<void>;
    playDirectory(options: PlayDirectoryOptions): Promise<void>;
    playLikedTracks(options: PlayLikedTracksOptions): Promise<void>;
    playPlaylist(options: PlayPlaylistOptions): Promise<void>;
    playTrack(path: string): Promise<void>;
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
    start(
      startIndex?: number,
      elapsed?: number,
      offset?: number,
    ): Promise<void>;
    sync(): Promise<void>;
    removeAllTracks(): Promise<void>;
    removeTracks(options: RemoveTracksOptions): Promise<void>;
    createPlaylist(options: CreatePlaylistOptions): Promise<void>;
    insertTrack(options: InsertTracksOptions): Promise<void>;
    insertDirectory(options: InsertDirectoryOptions): Promise<void>;
    insertPlaylist(options: InsertPlaylistOptions): Promise<void>;
    insertAlbum(options: InsertAlbumOptions): Promise<void>;
    insertArtistTracks(options: InsertArtistTracksOptions): Promise<void>;
    shufflePlaylist(startIndex?: number): Promise<void>;
    getCurrentPlaylist(): Promise<CurrentPlaylist>;
  };
  export const settings: {
    getGlobalSettings: () => Promise<Settings>;
    saveSettings: (settings: SettingsOptions) => Promise<void>;
  };
  export const sound: {
    adjustVolume: (steps: number) => Promise<void>;
  };
  export const system: {
    getGlobalStatus: () => Promise<SystemStatus>;
    getRockboxVersion: () => Promise<string>;
  };
}
