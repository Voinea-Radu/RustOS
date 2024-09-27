use spin::{Mutex, MutexGuard};

pub struct Locked<T> {
    inner: Mutex<T>,
}

impl<T> Locked<T> {
    pub const fn new(data: T) -> Self {
        Self {
            inner: Mutex::new(data),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.inner.lock()
    }
}
