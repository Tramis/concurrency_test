//! reader-writer problem
//!
//! reader first, which means that when there is a reading reader, other later coming reader will always be able to read, no matter whether there is waiting writer
//!
//! but at the moment of lock being released, which competitor would accquire the lock is unknown
use std::{
    sync::{Mutex, RwLock},
    time::Duration,
};

use crate::{semaphore::Semaphore, ReaderWriter};

/// reader-writer problem
///
/// reader first, which means that when the lock is released, reader will accquire the lock before writers
struct ReaderWriterRf {
    /// RwLock is for compiling, it actually has the same function I implement here
    content: RwLock<String>,
    reader_cnt: Mutex<usize>,

    write_sem: Semaphore,
}

impl Default for ReaderWriterRf {
    fn default() -> Self {
        Self {
            content: String::new().into(),
            reader_cnt: 0.into(),

            write_sem: Semaphore::new(1, 1),
        }
    }
}

impl ReaderWriter for ReaderWriterRf {
    fn read_for(&self, delay: usize) -> String {
        {
            let mut reader_cnt = self.reader_cnt.lock().unwrap();

            if *reader_cnt == 0 {
                self.write_sem.accquire();
            }
            *reader_cnt += 1;
        }

        std::thread::sleep(Duration::from_secs(delay as u64));
        let res = self.content.read().unwrap().clone();

        {
            let mut reader_cnt = self.reader_cnt.lock().unwrap();

            *reader_cnt -= 1;
            if *reader_cnt == 0 {
                self.write_sem.release();
            }
        }

        res
    }

    fn write_for(&self, s: String, delay: usize) {
        self.write_sem.accquire();

        std::thread::sleep(Duration::from_secs(delay as u64));
        *self.content.write().unwrap() += &s;

        self.write_sem.release();
    }
}

#[cfg(test)]
mod tests {
    use crate::runner::reader_writer_runner::Runner;

    use super::ReaderWriterRf;

    #[test]
    fn basic() {
        ReaderWriterRf::run_one(
            "
        [r0] s 1 r 2
        [r1] s 2 r 2
        [r3] s 3 r 2
        [r5] s 4 r 2
        [r6] s 5 r 2
        [r7] s 6 r 2
        [r8] s 7 r 2
        [r9] s 8 r 2
        [r2] s 11 r 1

        [w0] s 0 w 0
        [w1] s 2 w 1
        [w2] s 10 w 1
        "
            .into(),
        );
    }
}
