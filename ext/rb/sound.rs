use deno_core::{error::AnyError, op2};

use crate::{
  api::rockbox::v1alpha1::sound_service_client::SoundServiceClient, build_url,
};

#[op2(async)]
pub async fn op_adjust_volume() -> Result<(), AnyError> {
  let url = build_url();
  let mut client = SoundServiceClient::connect(url).await?;
  Ok(())
}
