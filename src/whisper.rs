use futures::channel::oneshot::{Receiver, Sender};
use futures::prelude::*;
use futures::task::{Context, Poll};
use std::pin::Pin;

pub(crate) struct Whisper {
    left: Option<Sender<u64>>,
    right: Receiver<u64>,
}

impl Whisper {
    pub(crate) fn new(left: Sender<u64>, right: Receiver<u64>) -> Whisper {
        Whisper {
            left: Some(left),
            right,
        }
    }
}

impl Future for Whisper {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.right.poll_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(value)) => {
                self.left
                    .take()
                    .expect("Sender was already taken")
                    .send(value + 1)
                    .expect("Receiving end was dropped");
                Poll::Ready(())
            }
            Poll::Ready(Err(e)) => {
                eprintln!("Error while polling receiver: {:?}", e);
                Poll::Ready(())
            }
        }
    }
}
