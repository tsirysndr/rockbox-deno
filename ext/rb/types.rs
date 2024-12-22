use serde::{Deserialize, Serialize};

use crate::api::rockbox::v1alpha1::SearchResponse;

#[derive(Serialize, Deserialize)]
pub struct Entry {
  pub name: String,
  pub attr: i32,
  pub time_write: u32,
  pub customaction: i32,
}

impl From<crate::api::rockbox::v1alpha1::Entry> for Entry {
  fn from(entry: crate::api::rockbox::v1alpha1::Entry) -> Self {
    Self {
      name: entry.name,
      attr: entry.attr,
      time_write: entry.time_write,
      customaction: entry.customaction,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Album {
  pub id: String,
  pub title: String,
  pub artist: String,
  pub year: u32,
  pub year_string: String,
  pub album_art: Option<String>,
  pub md5: String,
  pub artist_id: String,
  pub tracks: Vec<Track>,
}

impl From<crate::api::rockbox::v1alpha1::Album> for Album {
  fn from(album: crate::api::rockbox::v1alpha1::Album) -> Self {
    Self {
      id: album.id,
      title: album.title,
      artist: album.artist,
      year: album.year,
      year_string: album.year_string,
      album_art: album.album_art,
      md5: album.md5,
      artist_id: album.artist_id,
      tracks: album.tracks.into_iter().map(Track::from).collect(),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Artist {
  pub id: String,
  pub name: String,
  pub bio: Option<String>,
  pub image: Option<String>,
  pub albums: Vec<Album>,
  pub tracks: Vec<Track>,
}

impl From<crate::api::rockbox::v1alpha1::Artist> for Artist {
  fn from(artist: crate::api::rockbox::v1alpha1::Artist) -> Self {
    Self {
      id: artist.id,
      name: artist.name,
      bio: artist.bio,
      image: artist.image,
      albums: artist.albums.into_iter().map(Album::from).collect(),
      tracks: artist.tracks.into_iter().map(Track::from).collect(),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Track {
  pub id: String,
  pub path: String,
  pub title: String,
  pub artist: String,
  pub album: String,
  pub album_artist: String,
  pub bitrate: u32,
  pub composer: String,
  pub disc_number: u32,
  pub filesize: u32,
  pub frequency: u32,
  pub length: u32,
  pub track_number: u32,
  pub year: u32,
  pub year_string: String,
  pub genre: String,
  pub md5: String,
  pub album_art: Option<String>,
  pub artist_id: Option<String>,
  pub album_id: Option<String>,
  pub genre_id: Option<String>,
  pub created_at: String,
  pub updated_at: String,
}

impl From<crate::api::rockbox::v1alpha1::Track> for Track {
  fn from(track: crate::api::rockbox::v1alpha1::Track) -> Self {
    Self {
      id: track.id,
      path: track.path,
      title: track.title,
      artist: track.artist,
      album: track.album,
      album_artist: track.album_artist,
      bitrate: track.bitrate,
      composer: track.composer,
      disc_number: track.disc_number,
      filesize: track.filesize,
      frequency: track.frequency,
      length: track.length,
      track_number: track.track_number,
      year: track.year,
      year_string: track.year_string,
      genre: track.genre,
      md5: track.md5,
      album_art: track.album_art,
      artist_id: track.artist_id,
      album_id: track.album_id,
      genre_id: track.genre_id,
      created_at: track.created_at,
      updated_at: track.updated_at,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct CurrentTrack {
  pub title: String,
  pub artist: String,
  pub album: String,
  pub genre: String,
  pub disc: String,
  pub track_string: String,
  pub year_string: String,
  pub composer: String,
  pub comment: String,
  pub album_artist: String,
  pub grouping: String,
  pub discnum: i32,
  pub tracknum: i32,
  pub layer: i32,
  pub year: i32,
  pub bitrate: u32,
  pub frequency: u64,
  pub filesize: u64,
  pub length: u64,
  pub elapsed: u64,
  pub path: String,
  pub album_art: Option<String>,
  pub album_id: String,
  pub artist_id: String,
  pub id: String,
}

impl From<crate::api::rockbox::v1alpha1::CurrentTrackResponse>
  for CurrentTrack
{
  fn from(track: crate::api::rockbox::v1alpha1::CurrentTrackResponse) -> Self {
    Self {
      title: track.title,
      artist: track.artist,
      album: track.album,
      genre: track.genre,
      disc: track.disc,
      track_string: track.track_string,
      year_string: track.year_string,
      composer: track.composer,
      comment: track.comment,
      album_artist: track.album_artist,
      grouping: track.grouping,
      discnum: track.discnum,
      tracknum: track.tracknum,
      layer: track.layer,
      year: track.year,
      bitrate: track.bitrate,
      frequency: track.frequency,
      filesize: track.filesize,
      length: track.length,
      elapsed: track.elapsed,
      path: track.path,
      album_art: track.album_art,
      album_id: track.album_id,
      artist_id: track.artist_id,
      id: track.id,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct NextTrack {
  pub title: String,
  pub artist: String,
  pub album: String,
  pub genre: String,
  pub disc: String,
  pub track_string: String,
  pub year_string: String,
  pub composer: String,
  pub comment: String,
  pub album_artist: String,
  pub grouping: String,
  pub discnum: i32,
  pub tracknum: i32,
  pub layer: i32,
  pub year: i32,
  pub bitrate: u32,
  pub frequency: u64,
  pub filesize: u64,
  pub length: u64,
  pub elapsed: u64,
  pub path: String,
}

impl From<crate::api::rockbox::v1alpha1::NextTrackResponse> for NextTrack {
  fn from(track: crate::api::rockbox::v1alpha1::NextTrackResponse) -> Self {
    Self {
      title: track.title,
      artist: track.artist,
      album: track.album,
      genre: track.genre,
      disc: track.disc,
      track_string: track.track_string,
      year_string: track.year_string,
      composer: track.composer,
      comment: track.comment,
      album_artist: track.album_artist,
      grouping: track.grouping,
      discnum: track.discnum,
      tracknum: track.tracknum,
      layer: track.layer,
      year: track.year,
      bitrate: track.bitrate,
      frequency: track.frequency,
      filesize: track.filesize,
      length: track.length,
      elapsed: track.elapsed,
      path: track.path,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct EqBandSetting {
  pub cutoff: i32,
  pub q: i32,
  pub gain: i32,
}

impl Into<crate::api::rockbox::v1alpha1::EqBandSetting> for EqBandSetting {
  fn into(self) -> crate::api::rockbox::v1alpha1::EqBandSetting {
    crate::api::rockbox::v1alpha1::EqBandSetting {
      cutoff: self.cutoff,
      q: self.q,
      gain: self.gain,
    }
  }
}

impl From<crate::api::rockbox::v1alpha1::EqBandSetting> for EqBandSetting {
  fn from(
    eq_band_setting: crate::api::rockbox::v1alpha1::EqBandSetting,
  ) -> Self {
    Self {
      cutoff: eq_band_setting.cutoff,
      q: eq_band_setting.q,
      gain: eq_band_setting.gain,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct ReplaygainSettings {
  pub noclip: bool,
  pub r#type: i32,
  pub preamp: i32,
}

impl Into<crate::api::rockbox::v1alpha1::ReplaygainSettings>
  for ReplaygainSettings
{
  fn into(self) -> crate::api::rockbox::v1alpha1::ReplaygainSettings {
    crate::api::rockbox::v1alpha1::ReplaygainSettings {
      noclip: self.noclip,
      r#type: self.r#type,
      preamp: self.preamp,
    }
  }
}

impl From<crate::api::rockbox::v1alpha1::ReplaygainSettings>
  for ReplaygainSettings
{
  fn from(
    replaygain_settings: crate::api::rockbox::v1alpha1::ReplaygainSettings,
  ) -> Self {
    Self {
      noclip: replaygain_settings.noclip,
      r#type: replaygain_settings.r#type,
      preamp: replaygain_settings.preamp,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
  pub volume: i32,
  pub balance: i32,
  pub bass: i32,
  pub treble: i32,
  pub channel_config: i32,
  pub stereo_width: i32,
  pub bass_cutoff: i32,
  pub treble_cutoff: i32,
  pub crossfade: i32,
  pub crossfade_fade_in_delay: i32,
  pub crossfade_fade_out_delay: i32,
  pub crossfade_fade_in_duration: i32,
  pub crossfade_fade_out_duration: i32,
  pub crossfade_fade_out_mixmode: i32,
  pub replaygain_settings: Option<ReplaygainSettings>,
  pub crossfeed: i32,
  pub crossfeed_direct_gain: u32,
  pub crossfeed_cross_gain: u32,
  pub crossfeed_hf_attenuation: u32,
  pub crossfeed_hf_cutoff: u32,
  pub eq_enabled: bool,
  pub eq_precut: u32,
  pub eq_band_settings: Vec<EqBandSetting>,
  pub beep: i32,
  pub keyclick: i32,
  pub keyclick_repeats: i32,
  pub dithering_enabled: bool,
  pub timestretch_enabled: bool,
  pub party_mode: bool,
  pub player_name: String,
  pub music_dir: String,
  pub repeat_mode: i32,
  pub playlist_shuffle: bool,
}

impl From<crate::api::rockbox::v1alpha1::GetGlobalSettingsResponse>
  for Settings
{
  fn from(
    settings: crate::api::rockbox::v1alpha1::GetGlobalSettingsResponse,
  ) -> Self {
    Self {
      volume: settings.volume,
      balance: settings.balance,
      bass: settings.bass,
      treble: settings.treble,
      channel_config: settings.channel_config,
      stereo_width: settings.stereo_width,
      bass_cutoff: settings.bass_cutoff,
      treble_cutoff: settings.treble_cutoff,
      crossfade: settings.crossfade,
      crossfade_fade_in_delay: settings.crossfade_fade_in_delay,
      crossfade_fade_out_delay: settings.crossfade_fade_out_delay,
      crossfade_fade_in_duration: settings.crossfade_fade_in_duration,
      crossfade_fade_out_duration: settings.crossfade_fade_out_duration,
      crossfade_fade_out_mixmode: settings.crossfade_fade_out_mixmode,
      replaygain_settings: settings.replaygain_settings.map(|x| x.into()),
      crossfeed: settings.crossfeed,
      crossfeed_direct_gain: settings.crossfeed_direct_gain,
      crossfeed_cross_gain: settings.crossfeed_cross_gain,
      crossfeed_hf_attenuation: settings.crossfeed_hf_attenuation,
      crossfeed_hf_cutoff: settings.crossfeed_hf_cutoff,
      eq_enabled: settings.eq_enabled,
      eq_precut: settings.eq_precut,
      eq_band_settings: settings
        .eq_band_settings
        .into_iter()
        .map(|x| x.into())
        .collect(),
      beep: settings.beep,
      keyclick: settings.keyclick,
      keyclick_repeats: settings.keyclick_repeats,
      dithering_enabled: settings.dithering_enabled,
      timestretch_enabled: settings.timestretch_enabled,
      party_mode: settings.party_mode,
      player_name: settings.player_name,
      music_dir: settings.music_dir,
      repeat_mode: settings.repeat_mode,
      playlist_shuffle: settings.playlist_shuffle,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct SystemStatus {
  pub resume_index: i32,
  pub resume_crc32: u32,
  pub resume_elapsed: u32,
  pub resume_offset: u32,
  pub runtime: i32,
  pub topruntime: i32,
  pub dircache_size: i32,
  pub last_screen: i32,
  pub viewer_icon_count: i32,
  pub last_volume_change: i32,
}

impl From<crate::api::rockbox::v1alpha1::GetGlobalStatusResponse>
  for SystemStatus
{
  fn from(
    status: crate::api::rockbox::v1alpha1::GetGlobalStatusResponse,
  ) -> Self {
    Self {
      resume_index: status.resume_index,
      resume_crc32: status.resume_crc32,
      resume_elapsed: status.resume_elapsed,
      resume_offset: status.resume_offset,
      runtime: status.runtime,
      topruntime: status.topruntime,
      dircache_size: status.dircache_size,
      last_screen: status.last_screen,
      viewer_icon_count: status.viewer_icon_count,
      last_volume_change: status.last_volume_change,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct CurrentPlaylist {
  pub index: i32,
  pub amount: i32,
  pub max_playlist_size: i32,
  pub first_index: i32,
  pub last_insert_pos: i32,
  pub seed: i32,
  pub last_shuffled_start: i32,
  pub tracks: Vec<CurrentTrack>,
}

impl From<crate::api::rockbox::v1alpha1::GetCurrentResponse>
  for CurrentPlaylist
{
  fn from(current: crate::api::rockbox::v1alpha1::GetCurrentResponse) -> Self {
    Self {
      index: current.index,
      amount: current.amount,
      max_playlist_size: current.max_playlist_size,
      first_index: current.first_index,
      last_insert_pos: current.last_insert_pos,
      seed: current.seed,
      last_shuffled_start: current.last_shuffled_start,
      tracks: current.tracks.into_iter().map(CurrentTrack::from).collect(),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct SearchResults {
  pub albums: Vec<Album>,
  pub tracks: Vec<Track>,
  pub artists: Vec<Artist>,
}

impl From<crate::api::rockbox::v1alpha1::SearchResponse> for SearchResults {
  fn from(response: SearchResponse) -> Self {
    Self {
      albums: response.albums.into_iter().map(Album::from).collect(),
      artists: response.artists.into_iter().map(Artist::from).collect(),
      tracks: response.tracks.into_iter().map(Track::from).collect(),
    }
  }
}
