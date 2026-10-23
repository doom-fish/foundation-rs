use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use foundation::prelude::*;

fn main() -> Result<(), FoundationError> {
    let center = NotificationCenter::new();
    let notifications = Arc::new(AtomicUsize::new(0));
    let count = Arc::clone(&notifications);
    let observer = center.add_observer("rs.doom-fish.foundation.example", move || {
        count.fetch_add(1, Ordering::SeqCst);
    })?;

    center.post("rs.doom-fish.foundation.example")?;
    println!("notifications: {}", notifications.load(Ordering::SeqCst));

    drop(observer);
    center.post("rs.doom-fish.foundation.example")?;
    println!(
        "notifications after drop: {}",
        notifications.load(Ordering::SeqCst)
    );

    {
        let _activity =
            ProcessInfo::begin_activity(ActivityOptions::BACKGROUND, "Run the Foundation example")?;
        println!("activity began");
    }
    println!("activity ended");
    Ok(())
}
