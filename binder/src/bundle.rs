use crate::sys;

use std::ptr::{self, NonNull};

/// Container for a Bundle that can be sent through Binder.
///
/// This type represents a bundle that is owned by Rust code.
#[repr(transparent)]
pub struct Bundle {
    ptr: NonNull<sys::APersistableBundle>,
}

/// Safety: This type guarantees that it owns the APersistableBundle and that all access to
/// the APersistableBundle happens through the Bundle, so it is ok to send across threads.
///
/// It would not be okay to implement Sync, because that would allow you to call
/// the reading methods from several threads in parallel, which would be a data
/// race on the contents inside the APersistableBundle.
unsafe impl Send for Bundle {}

impl Bundle {
    fn new() -> Self {
        // Safety: If `APersistableBundle_new` succeeds, it always returns
        // a valid pointer. If it fails, the process will crash.
        let ptr = unsafe { sys::APersistableBundle_new() };
        Self { ptr: NonNull::new(ptr).expect("APersistableBundle_new returned null pointer") }
    }
}

#[test]
fn test_bundle_new() {
    let bundle = Bundle::new();
}