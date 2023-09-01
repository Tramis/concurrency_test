use std::{
    cell::RefCell,
    sync::{Condvar, Mutex},
};

pub struct Semaphore {
    cnt: Mutex<u64>,
    limit: u64,
    cv: Condvar,
}

impl Semaphore {
    pub fn new(cnt: u64, limit: u64) -> Self {
        Self {
            cnt: Mutex::new(cnt.into()),
            limit: limit,
            cv: Condvar::new(),
        }
    }

    pub fn accquire(&self) {
        let mut cnt = self.cnt.lock().unwrap();

        while *cnt == 0 {
            cnt = self.cv.wait(cnt).unwrap();
        }

        *cnt -= 1;
    }

    pub fn release(&self) {
        let mut cnt = self.cnt.lock().unwrap();
        *cnt += if *cnt == self.limit { 0 } else { 1 };
        self.cv.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use rand::random;

    use super::*;
    use std::{sync::Arc, thread, time::Duration};

    #[test]
    fn basic() {
        let sem = Arc::new(Semaphore::new(1, 2));
        let sem1 = sem.clone();
        let sem2 = sem.clone();
        let sem3 = sem.clone();
        let sem4 = sem.clone();

        let t1 = thread::spawn(move || {
            sem1.accquire();
            println!("blocking");
            thread::sleep(Duration::new(3, 0));
            println!("blocking complete");
            sem1.release();
        });

        let t2 = thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            println!("2 try accquire");
            sem2.accquire();
            println!("2 accquired");
            thread::sleep(Duration::new(3, 0));
            sem2.release();
        });

        let t3 = thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            println!("3 try accquire");
            sem3.accquire();
            println!("3 accquired");
            thread::sleep(Duration::new(3, 0));
            sem3.release();
        });

        let t4 = thread::spawn(move || {
            thread::sleep(Duration::new(1, 0));
            println!("4 try accquire");
            sem4.accquire();
            println!("4 accquired");
            thread::sleep(Duration::new(3, 0));
            sem4.release();
        });

        t1.join();
        t2.join();
        t3.join();
        t4.join();
    }

    #[test]
    fn sync(){
        let sem1 = Arc::new(Semaphore::new(0, 10));
        let sem4 = sem1.clone();

        let t1 = thread::spawn(move || {
            sem1.accquire();
            println!("later");
            sem1.release();
        });

        let t4 = thread::spawn(move || {
            println!("former");
            thread::sleep(Duration::new(3, 0));
            sem4.release();
        });

        t1.join();
        t4.join();
    }
}
