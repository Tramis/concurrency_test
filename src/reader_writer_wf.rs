//! reader-writer problem
//!
//! reader first, which means that when there is a reading reader, other later coming reader will always be able to read, no matter whether there is waiting writer
//!
//! but at the moment of lock being released, which competitor would accquire the lock is unknown
use std::{
    sync::{Condvar, Mutex, RwLock},
    time::Duration,
};

use crate::{semaphore::Semaphore, ReaderWriter};

/// reader-writer problem
///
/// reader first, which means that when the lock is released, reader will accquire the lock before writers
struct ReaderWriterWf {
    content: RwLock<String>,

    write_sem: Semaphore,
    write_wait_cnt: Mutex<usize>,
    cond_var: Condvar,
}

impl Default for ReaderWriterWf {
    fn default() -> Self {
        Self {
            content: String::new().into(),

            write_sem: Semaphore::new(1, 1),
            write_wait_cnt: 0.into(),
            cond_var: Condvar::new(),
        }
    }
}

impl ReaderWriter for ReaderWriterWf {
    fn read_for(&self, delay: usize) -> String {
        {
            let mut write_wait_cnt = self.write_wait_cnt.lock().unwrap();
            while *write_wait_cnt > 0 {
                write_wait_cnt = self.cond_var.wait(write_wait_cnt).unwrap();
            }
        }

        std::thread::sleep(Duration::from_secs(delay as u64));
        self.content.read().unwrap().clone()
    }

    fn write_for(&self, s: String, delay: usize) {
        {
            let mut write_wait_cnt = self.write_wait_cnt.lock().unwrap();
            *write_wait_cnt += 1;
        }
        self.write_sem.accquire();

        std::thread::sleep(Duration::from_secs(delay as u64));
        *self.content.write().unwrap() += &s;

        self.write_sem.release();
        {
            let mut write_wait_cnt = self.write_wait_cnt.lock().unwrap();
            *write_wait_cnt -= 1;
            self.cond_var.notify_all();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::runner::reader_writer_runner::Runner;

    use super::ReaderWriterWf;

    #[test]
    fn basic() {
        ReaderWriterWf::run_one(
            "
        [w0] s 0 w 2
        [w1] s 1 w 2
        [w2] s 2 w 2
        [w3] s 3 w 2
        [w4] s 12 w 2

        [r0] s 0 r 0
        [r1] s 2 r 1
        [r2] s 14 r 1
        "
            .into(),
        );
    }
}
