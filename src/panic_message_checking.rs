#![allow(clippy::bool_comparison)]

use std::boxed::Box;
use std::hint;
use std::ops::Deref;
use std::panic::{catch_unwind, UnwindSafe};
use std::ptr::NonNull;
use std::sync::{
    atomic::{AtomicBool, AtomicPtr, Ordering},
    Arc,
};

static HOOK_GUARD: AtomicBool = AtomicBool::new(true);

struct GuardReleaser;

impl Drop for GuardReleaser {
    fn drop(&mut self) {
        HOOK_GUARD.store(true, Ordering::Relaxed)
    }
}

/// Assert complex message containment in panic arising from closure.
/// # Summary
/// Use instead of `#[should_panic(expected="xyz)"]` when willing to avoid
/// complex string inlined in attribute declaration.
/// # Caution
/// Downside of hooking-into-panic approach is that all test run
/// in test batch must use this `assert` call instead of `#[should_panic]`
/// expectation.
/// That since panics out of controlled process on parallel thread will
/// be handled by current custom hook with known results (mem-leak, test-fail).
pub fn assert<F: FnOnce() -> () + UnwindSafe>(f: F, exp_msg: &str) {
    let atom_msg = AtomicPtr::new(NonNull::<Option<String>>::dangling().as_ptr());
    let atom_flg = AtomicBool::new(false);

    let arc_info = Arc::new((atom_msg, atom_flg));
    let arc_info2 = arc_info.clone();

    // thread-safety (tests are run in parallel usually)
    // new hook registration must be denied until method will return/panic
    // otherwise assert panic could interact with new hook:
    // a) causing memory leak — String never dropped (read)
    // b) depending on order possibly overwritting expected msg
    while HOOK_GUARD
        .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
        .is_err()
    {
        hint::spin_loop();
    }

    // on unwind as well as on normal return, drop
    // call will release HOOK_GUARD so other
    // threads will be able to take it
    #[allow(unused_variables)]
    let lock = GuardReleaser;

    std::panic::set_hook(Box::new(move |pi| {
        let msg = Box::new(Some(pi.to_string()));

        let info2 = arc_info2.deref();
        info2.0.store(Box::into_raw(msg), Ordering::Relaxed);
        info2.1.store(true, Ordering::Relaxed);
    }));

    let res = catch_unwind(|| {
        f();
    });
    _ = std::panic::take_hook();

    assert!(res.is_err(), "FnOnce provided did not panic at all.");

    let info = arc_info.deref();
    let atom_flg = &info.1;

    while atom_flg.load(Ordering::Relaxed) == false {
        hint::spin_loop();
    }

    let ptr = info.0.load(Ordering::Relaxed);
    let msg = unsafe { ptr.read() }.unwrap();

    assert!(
        msg.contains(exp_msg),
        "\r\nMISMATCH ⸺ expected message not contained\r\nMSG: {}\r\nEXP:{}",
        msg,
        exp_msg
    );
}

#[cfg(test)]
#[cfg(feature = "rh-panic-tests")]
mod test {

    use super::*;

    const REALLY_COMPLEX_MULTILINE_STR: &str = "Character: – U+2013
Name: EN DASH
General Character Properties
Block: General Punctuation
Unicode category: Punctuation, Dash
Various Useful Representations
UTF-8: 0xE2 0x80 0x93
UTF-16: 0x2013
C octal escaped UTF-8: \\342\\200\\223
XML decimal entity: &#8211;";

    #[test]
    fn assert_assertion_satisfied() {
        let f = || panic!("{}", REALLY_COMPLEX_MULTILINE_STR);
        assert(f, REALLY_COMPLEX_MULTILINE_STR);
    }

    #[test]
    #[should_panic(
        expected = "MISMATCH ⸺ expected message not contained\r\nMSG: panicked at src/panic_message_checking.rs:119:20:\nSOMETHING_DIFFERENT"
    )]
    fn assert_different_pnc() {
        let f = || panic!("{}", "SOMETHING_DIFFERENT");
        assert(f, REALLY_COMPLEX_MULTILINE_STR);
    }

    #[test]
    #[should_panic(expected = "FnOnce provided did not panic at all.")]
    fn assert_no_pnc() {
        let f = || {};
        assert(f, REALLY_COMPLEX_MULTILINE_STR);
    }
}

// cargo fmt & cargo test --release --features rh-panic-tests
