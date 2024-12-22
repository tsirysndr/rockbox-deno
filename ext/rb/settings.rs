use deno_core::{error::AnyError, op2};

use crate::types::EqBandSetting;
use crate::types::ReplaygainSettings;
use crate::{
  api::rockbox::v1alpha1::{
    settings_service_client::SettingsServiceClient, GetGlobalSettingsRequest,
    SaveSettingsRequest,
  },
  build_url,
  types::Settings,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SaveSettingsParams {
  pub music_dir: Option<String>,
  pub playlist_shuffle: Option<bool>,
  pub repeat_mode: Option<i32>,
  pub bass: Option<i32>,
  pub treble: Option<i32>,
  pub bass_cutoff: Option<i32>,
  pub treble_cutoff: Option<i32>,
  pub crossfade: Option<i32>,
  pub fade_on_stop: Option<bool>,
  pub fade_in_delay: Option<i32>,
  pub fade_in_duration: Option<i32>,
  pub fade_out_delay: Option<i32>,
  pub fade_out_duration: Option<i32>,
  pub fade_out_mixmode: Option<i32>,
  pub balance: Option<i32>,
  pub stereo_width: Option<i32>,
  pub stereosw_mode: Option<i32>,
  pub surround_enabled: Option<i32>,
  pub surround_balance: Option<i32>,
  pub surround_fx1: Option<i32>,
  pub surround_fx2: Option<i32>,
  pub party_mode: Option<bool>,
  pub channel_config: Option<i32>,
  pub player_name: Option<String>,
  pub eq_enabled: Option<bool>,
  pub eq_band_settings: Vec<EqBandSetting>,
  pub replaygain_settings: Option<ReplaygainSettings>,
}

#[op2(async)]
#[serde]
pub async fn op_get_global_settings() -> Result<Settings, AnyError> {
  let url = build_url();
  let mut client = SettingsServiceClient::connect(url).await?;
  let response = client
    .get_global_settings(GetGlobalSettingsRequest {})
    .await?
    .into_inner();
  Ok(response.into())
}

#[op2(async)]
pub async fn op_save_settings(
  #[serde] params: SaveSettingsParams,
) -> Result<(), AnyError> {
  let url = build_url();
  let mut client = SettingsServiceClient::connect(url).await?;
  let _response = client
    .save_settings(SaveSettingsRequest {
      music_dir: params.music_dir,
      playlist_shuffle: params.playlist_shuffle,
      repeat_mode: params.repeat_mode,
      bass: params.bass,
      treble: params.treble,
      bass_cutoff: params.bass_cutoff,
      treble_cutoff: params.treble_cutoff,
      crossfade: params.crossfade,
      fade_on_stop: params.fade_on_stop,
      fade_in_delay: params.fade_in_delay,
      fade_in_duration: params.fade_in_duration,
      fade_out_delay: params.fade_out_delay,
      fade_out_duration: params.fade_out_duration,
      fade_out_mixmode: params.fade_out_mixmode,
      balance: params.balance,
      stereo_width: params.stereo_width,
      stereosw_mode: params.stereosw_mode,
      surround_enabled: params.surround_enabled,
      surround_balance: params.surround_balance,
      surround_fx1: params.surround_fx1,
      surround_fx2: params.surround_fx2,
      party_mode: params.party_mode,
      channel_config: params.channel_config,
      player_name: params.player_name,
      eq_enabled: params.eq_enabled,
      eq_band_settings: params
        .eq_band_settings
        .into_iter()
        .map(|v| v.into())
        .collect(),
      replaygain_settings: params.replaygain_settings.map(|v| v.into()),
    })
    .await?;
  Ok(())
}
