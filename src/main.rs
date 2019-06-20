use futures::channel::oneshot;
use futures::executor::LocalPool;
use futures::task::SpawnExt;
use futures::FutureExt;

mod error;
mod whisper;

fn main() -> Result<(), error::Error> {
    const N: usize = 100000;

    let mut pool = LocalPool::new();
    let mut spawner = pool.spawner();
    let (mut left_tx, leftmost_rx) = oneshot::channel();

    // Spawn a future to print the end result
    spawner.spawn(leftmost_rx.map(|i| println!("{}", i.expect("Sending end was dropped"))))?;

    // Spawn a ton of futures that whisper across the chain
    for _ in 0..N {
        let (right_tx, right_rx) = oneshot::channel();
        spawner.spawn(whisper::Whisper::new(left_tx, right_rx))?;
        left_tx = right_tx;
    }

    // Start the chain of whispers
    left_tx.send(1)?;

    // Run all futures to completion
    Ok(pool.run())
}
