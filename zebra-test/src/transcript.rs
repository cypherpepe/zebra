//! A [`Service`](tower::Service) implementation based on a fixed transcript.

use color_eyre::eyre::{eyre, Report};
use futures::future::{ready, Ready};
use std::{
    fmt::Debug,
    task::{Context, Poll},
};
use tower::Service;

pub struct Transcript<R, S, I>
where
    I: Iterator<Item = (R, S)>,
{
    messages: I,
}

impl<R, S, I> From<I> for Transcript<R, S, I>
where
    I: Iterator<Item = (R, S)>,
{
    fn from(messages: I) -> Self {
        Self { messages }
    }
}

impl<R, S, I> Service<R> for Transcript<R, S, I>
where
    R: Debug + Eq,
    I: Iterator<Item = (R, S)>,
{
    type Response = S;
    type Error = Report;
    type Future = Ready<Result<S, Report>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: R) -> Self::Future {
        if let Some((expected_request, response)) = self.messages.next() {
            if request == expected_request {
                ready(Ok(response))
            } else {
                ready(Err(eyre!(
                    "Expected {:?}, got {:?}",
                    expected_request,
                    request
                )))
            }
        } else {
            ready(Err(eyre!("Got request after transcript ended")))
        }
    }
}
