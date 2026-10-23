use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use foundation::{
    Activity, ActivityOptions, FoundationError, NotificationCenter, NotificationObserver,
    ProcessInfo,
};

const NAME: &str = "rs.doom-fish.foundation.test-notification";

#[test]
fn private_center_delivers_a_post_synchronously() {
    let center = NotificationCenter::new();
    let count = Arc::new(AtomicUsize::new(0));
    let observed_count = Arc::clone(&count);
    let _observer = center
        .add_observer(NAME, move || {
            observed_count.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();

    center.post(NAME).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 1);
}

#[test]
fn private_center_delivers_to_both_observers() {
    let center = NotificationCenter::new();
    let first_count = Arc::new(AtomicUsize::new(0));
    let second_count = Arc::new(AtomicUsize::new(0));
    let first = Arc::clone(&first_count);
    let second = Arc::clone(&second_count);
    let _first = center
        .add_observer(NAME, move || {
            first.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();
    let _second = center
        .add_observer(NAME, move || {
            second.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();

    center.post(NAME).unwrap();
    assert_eq!(first_count.load(Ordering::SeqCst), 1);
    assert_eq!(second_count.load(Ordering::SeqCst), 1);
}

#[test]
fn dropped_observer_does_not_receive_new_posts() {
    let center = NotificationCenter::new();
    let count = Arc::new(AtomicUsize::new(0));
    let observed_count = Arc::clone(&count);
    let observer = center
        .add_observer(NAME, move || {
            observed_count.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();

    center.post(NAME).unwrap();
    drop(observer);
    center.post(NAME).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 1);
}

#[test]
fn callback_can_drop_its_own_observer() {
    let center = NotificationCenter::new();
    let registration = Arc::new(Mutex::new(None::<NotificationObserver>));
    let callback_registration = Arc::clone(&registration);
    let count = Arc::new(AtomicUsize::new(0));
    let observed_count = Arc::clone(&count);
    let observer = center
        .add_observer(NAME, move || {
            let removed = callback_registration.lock().unwrap().take();
            drop(removed);
            observed_count.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();
    *registration.lock().unwrap() = Some(observer);

    center.post(NAME).unwrap();
    center.post(NAME).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 1);
}

#[test]
fn callback_runs_on_the_posting_thread_after_center_clone() {
    let center = NotificationCenter::new();
    let observed_thread = Arc::new(Mutex::new(None));
    let observed = Arc::clone(&observed_thread);
    let _observer = center
        .add_observer(NAME, move || {
            *observed.lock().unwrap() = Some(thread::current().id());
        })
        .unwrap();

    let posting_center = center.clone();
    drop(center);
    let posting_thread = thread::spawn(move || {
        let thread_id = thread::current().id();
        posting_center.post(NAME).unwrap();
        thread_id
    })
    .join()
    .unwrap();

    assert_eq!(*observed_thread.lock().unwrap(), Some(posting_thread));
}

#[test]
fn in_flight_callback_survives_observer_drop() {
    let center = NotificationCenter::new();
    let count = Arc::new(AtomicUsize::new(0));
    let observed_count = Arc::clone(&count);
    let (started_tx, started_rx) = mpsc::channel();
    let (resume_tx, resume_rx) = mpsc::channel();
    let resume_rx = Mutex::new(resume_rx);
    let observer = center
        .add_observer(NAME, move || {
            started_tx.send(()).unwrap();
            resume_rx.lock().unwrap().recv().unwrap();
            observed_count.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();

    let posting_center = center.clone();
    let posting = thread::spawn(move || posting_center.post(NAME).unwrap());
    started_rx.recv().unwrap();

    let (dropped_tx, dropped_rx) = mpsc::channel();
    let dropping = thread::spawn(move || {
        drop(observer);
        dropped_tx.send(()).unwrap();
    });
    let remaining_refs = dropped_rx
        .recv_timeout(Duration::from_secs(1))
        .ok()
        .map(|()| Arc::strong_count(&count));

    resume_tx.send(()).unwrap();
    posting.join().unwrap();
    dropping.join().unwrap();
    if let Some(refs) = remaining_refs {
        assert_eq!(refs, 2);
    }
    assert_eq!(count.load(Ordering::SeqCst), 1);

    center.post(NAME).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 1);
}

#[test]
fn different_private_centers_do_not_share_observers() {
    let first = NotificationCenter::new();
    let second = NotificationCenter::new();
    let count = Arc::new(AtomicUsize::new(0));
    let observed_count = Arc::clone(&count);
    let _observer = first
        .add_observer(NAME, move || {
            observed_count.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();

    second.post(NAME).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 0);
}

#[test]
fn rejects_names_and_reasons_with_nul_bytes_and_empty_reasons() {
    let center = NotificationCenter::new();
    assert!(matches!(
        center.add_observer("invalid\0name", || {}),
        Err(FoundationError::InvalidArgument(_))
    ));
    assert!(matches!(
        center.post("invalid\0name"),
        Err(FoundationError::InvalidArgument(_))
    ));
    assert!(matches!(
        ProcessInfo::begin_activity(ActivityOptions::BACKGROUND, ""),
        Err(FoundationError::InvalidArgument(_))
    ));
    assert!(matches!(
        ProcessInfo::begin_activity(ActivityOptions::BACKGROUND, "invalid\0reason"),
        Err(FoundationError::InvalidArgument(_))
    ));
}

#[test]
fn process_info_notification_names_match_foundation() {
    assert_eq!(
        ProcessInfo::THERMAL_STATE_DID_CHANGE_NOTIFICATION,
        "NSProcessInfoThermalStateDidChangeNotification"
    );
    assert_eq!(
        ProcessInfo::POWER_STATE_DID_CHANGE_NOTIFICATION,
        "NSProcessInfoPowerStateDidChangeNotification"
    );
}

#[test]
fn activity_options_match_the_sdk_and_preserve_raw_bits() {
    assert_eq!(ActivityOptions::IDLE_DISPLAY_SLEEP_DISABLED.bits(), 1 << 40);
    assert_eq!(ActivityOptions::IDLE_SYSTEM_SLEEP_DISABLED.bits(), 1 << 20);
    assert_eq!(ActivityOptions::SUDDEN_TERMINATION_DISABLED.bits(), 1 << 14);
    assert_eq!(
        ActivityOptions::AUTOMATIC_TERMINATION_DISABLED.bits(),
        1 << 15
    );
    assert_eq!(ActivityOptions::ANIMATION_TRACKING_ENABLED.bits(), 1 << 45);
    assert_eq!(ActivityOptions::TRACKING_ENABLED.bits(), 1 << 46);
    assert_eq!(
        ActivityOptions::USER_INITIATED.bits(),
        0x00ff_ffff | (1 << 20)
    );
    assert_eq!(
        ActivityOptions::USER_INITIATED_ALLOWING_IDLE_SYSTEM_SLEEP.bits(),
        0x00ff_ffff & !(1 << 20)
    );
    assert_eq!(ActivityOptions::BACKGROUND.bits(), 0xff);
    assert_eq!(ActivityOptions::LATENCY_CRITICAL.bits(), 0xff_0000_0000);
    assert_eq!(
        ActivityOptions::USER_INTERACTIVE.bits(),
        (0x00ff_ffff | (1 << 20)) | 0xff_0000_0000
    );

    let mut options = ActivityOptions::BACKGROUND | ActivityOptions::IDLE_SYSTEM_SLEEP_DISABLED;
    options |= ActivityOptions::SUDDEN_TERMINATION_DISABLED;
    assert_eq!(
        ActivityOptions::from_bits(options.bits()),
        ActivityOptions::from_bits(0xff | (1 << 20) | (1 << 14))
    );
    assert_eq!(ActivityOptions::from_bits(u64::MAX).bits(), u64::MAX);
}

#[test]
fn begins_and_ends_a_process_activity() {
    let activity =
        ProcessInfo::begin_activity(ActivityOptions::BACKGROUND, "Test a process activity")
            .unwrap();
    drop(activity);
}

#[test]
fn notification_and_activity_handles_have_thread_safe_traits() {
    fn assert_send<T: Send>() {}
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<NotificationCenter>();
    assert_send::<NotificationObserver>();
    assert_send_sync::<Activity>();
}
