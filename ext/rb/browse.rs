use deno_core::{error::AnyError, op2};

use crate::{
  api::rockbox::v1alpha1::{
    browse_service_client::BrowseServiceClient, TreeGetEntriesRequest,
  },
  build_url,
};

use crate::types::Entry;

#[op2(async)]
#[serde]
pub async fn op_tree_get_entries(
  #[string] path: Option<String>,
) -> Result<Vec<Entry>, AnyError> {
  let url = build_url();
  let mut client = BrowseServiceClient::connect(url).await?;
  let response = client
    .tree_get_entries(TreeGetEntriesRequest { path })
    .await?
    .into_inner();
  Ok(response.entries.into_iter().map(Entry::from).collect())
}
