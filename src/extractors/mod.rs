use futures::Future;
use mysk_lib::prelude::*;
use std::pin::Pin;

pub(crate) mod logged_in;

pub type ExtractorFuture<SelfT> = Pin<Box<dyn Future<Output = Result<SelfT>>>>;
