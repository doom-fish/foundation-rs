use core::ffi::c_void;

#[link(name = "objc", kind = "dylib")]
extern "C" {
    fn objc_autoreleasePoolPush() -> *mut c_void;
    fn objc_autoreleasePoolPop(pool: *mut c_void);
}

struct Pool(*mut c_void);

impl Drop for Pool {
    fn drop(&mut self) {
        unsafe { objc_autoreleasePoolPop(self.0) };
    }
}

pub fn autoreleasepool<T>(body: impl FnOnce() -> T) -> T {
    let _pool = Pool(unsafe { objc_autoreleasePoolPush() });
    body()
}
