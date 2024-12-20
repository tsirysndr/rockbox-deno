use deno_core::op2;

#[op2(async)]
pub async fn op_get_global_status() {}

#[op2(async)]
pub async fn op_get_rockbox_version() {}
