use deno_core::{error::AnyError, op2};

use crate::{
  api::rockbox::v1alpha1::{
    system_service_client::SystemServiceClient, GetGlobalStatusRequest,
    GetRockboxVersionRequest,
  },
  build_url,
  types::SystemStatus,
};

#[op2(async)]
#[serde]
pub async fn op_get_global_status() -> Result<SystemStatus, AnyError> {
  let url = build_url();
  let mut client = SystemServiceClient::connect(url).await?;
  let response = client
    .get_global_status(GetGlobalStatusRequest {})
    .await?
    .into_inner();
  Ok(response.into())
}

#[op2(async)]
#[string]
pub async fn op_get_rockbox_version() -> Result<String, AnyError> {
  let url = build_url();
  let mut client = SystemServiceClient::connect(url).await?;
  let response = client
    .get_rockbox_version(GetRockboxVersionRequest {})
    .await?
    .into_inner();
  Ok(response.version)
}
