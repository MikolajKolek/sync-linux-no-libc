extern crate std;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::{Arc};
use std::thread;
use std::time::Duration;

use crate::sync::{Condvar, Mutex};

#[test]
fn smoke() {
    let c = Condvar::new();
    c.notify_one();
    c.notify_all();
}

#[test]
#[cfg_attr(any(target_os = "emscripten", target_os = "wasi"), ignore)] // no threads
fn notify_one() {
    let m = Arc::new(Mutex::new(()));
    let m2 = m.clone();
    let c = Arc::new(Condvar::new());
    let c2 = c.clone();

    let g = m.lock();
    let _t = thread::spawn(move || {
        let _g = m2.lock();
        c2.notify_one();
    });
    let g = c.wait(g);
    drop(g);
}

#[test]
#[cfg_attr(any(target_os = "emscripten", target_os = "wasi"), ignore)] // no threads
fn notify_all() {
    const N: usize = 10;

    let data = Arc::new((Mutex::new(0), Condvar::new()));
    let (tx, rx) = channel();
    for _ in 0..N {
        let data = data.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            let &(ref lock, ref cond) = &*data;
            let mut cnt = lock.lock();
            *cnt += 1;
            if *cnt == N {
                tx.send(()).unwrap();
            }
            while *cnt != 0 {
                cnt = cond.wait(cnt);
            }
            tx.send(()).unwrap();
        });
    }
    drop(tx);

    let &(ref lock, ref cond) = &*data;
    rx.recv().unwrap();
    let mut cnt = lock.lock();
    *cnt = 0;
    cond.notify_all();
    drop(cnt);

    for _ in 0..N {
        rx.recv().unwrap();
    }
}

#[test]
#[cfg_attr(any(target_os = "emscripten", target_os = "wasi"), ignore)] // no threads
fn wait_while() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // Inside of our lock, spawn a new thread, and then wait for it to start.
    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });

    // Wait for the thread to start up.
    let &(ref lock, ref cvar) = &*pair;
    let guard = cvar.wait_while(lock.lock(), |started| !*started);
    assert!(*guard);
}

#[test]
#[cfg_attr(any(target_os = "emscripten", target_os = "wasi"), ignore)] // condvar wait not supported
fn wait_timeout_wait() {
    let m = Arc::new(Mutex::new(()));
    let c = Arc::new(Condvar::new());

    loop {
        let g = m.lock();
        let (_g, no_timeout) = c.wait_timeout(g, Duration::from_millis(1));
        // spurious wakeups mean this isn't necessarily true
        // so execute test again, if not timeout
        if !no_timeout.timed_out() {
            continue;
        }

        break;
    }
}

#[test]
#[cfg_attr(any(target_os = "emscripten", target_os = "wasi"), ignore)] // no threads
fn wait_timeout_wake() {
    let m = Arc::new(Mutex::new(()));
    let c = Arc::new(Condvar::new());

    loop {
        let g = m.lock();

        let c2 = c.clone();
        let m2 = m.clone();

        let notified = Arc::new(AtomicBool::new(false));
        let notified_copy = notified.clone();

        let t = thread::spawn(move || {
            let _g = m2.lock();
            thread::sleep(Duration::from_millis(1));
            notified_copy.store(true, Ordering::Relaxed);
            c2.notify_one();
        });
        let (g, timeout_res) = c.wait_timeout(g, Duration::from_millis(u64::MAX));
        assert!(!timeout_res.timed_out());
        // spurious wakeups mean this isn't necessarily true
        // so execute test again, if not notified
        if !notified.load(Ordering::Relaxed) {
            t.join().unwrap();
            continue;
        }
        drop(g);

        t.join().unwrap();

        break;
    }
}
