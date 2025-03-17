use std::future::Future;

pub trait SafeState: Send + Sync + 'static {}
impl<S: Send + Sync + 'static> SafeState for S {}

pub trait FutureResponse: Future<Output = Result<(), anyhow::Error>> + Send + 'static {}
impl<Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static> FutureResponse for Fut {}
