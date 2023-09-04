//! the workset format is like:
//!
//! ```
//! [r0] s 1 r 2    // [thread_name] sleep for 1 sec, and then read for 2 secs
//! [r1] s 2 r 2
//! [r3] s 3 r 2
//! [r5] s 4 r 2
//! [r6] s 5 r 2
//! [r7] s 6 r 2
//! [r8] s 7 r 2
//! [r9] s 8 r 2
//! [r2] s 11 r 1
//! [w0] s 0 w 0    // [thread_name] sleep for 0 sec, and then writer for 0 sec
//! [w1] s 2 w 1
//! [w2] s 10 w 1
//! ```
//!

use std::{sync::Arc, thread::JoinHandle, time::Duration};

use crate::ReaderWriter;

#[derive(Debug, Copy, Clone)]
enum Command {
    Sleep(usize),
    Read(usize),
    Write(usize),
}

pub trait Runner {
    fn run_one(work_set: String);
}

impl<T> Runner for T
where
    T: ReaderWriter + Default + Send + Sync + 'static,
{
    fn run_one(work_set: String) {
        let a = Arc::new(T::default());

        let commands = work_set
            .split('\n')
            .map(|s| s.split(' ').filter(|s| !s.is_empty()).collect())
            .filter(|s: &Vec<&str>| !s.iter().all(|s| s.is_empty()))
            .collect::<Vec<Vec<&str>>>();

        println!("{commands:?}");

        let run_commands = commands
            .iter()
            .map(|command| {
                command
                    .windows(2)
                    .filter(|v| ["r", "w", "s"].contains(&v[0]))
                    .map(|v| match (v[0], v[1]) {
                        ("r", delay) => Command::Read(delay.parse::<usize>().unwrap()),
                        ("w", delay) => Command::Write(delay.parse::<usize>().unwrap()),
                        ("s", delay) => Command::Sleep(delay.parse::<usize>().unwrap()),
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Command>>>();

        let threads_handler = run_commands
            .into_iter()
            .zip(commands.iter().map(|v| v[0].to_string()))
            .map(|(run_command, t_name)| {
                let tmp_a = a.clone();
                // println!("build thread: {t_name}");
                std::thread::spawn(move || {
                    for &command in run_command.iter() {
                        match command {
                            Command::Sleep(delay) => {
                                std::thread::sleep(Duration::from_secs(delay as u64));
                                // println!("[{t_name}] will sleep: {}", delay);
                            }
                            Command::Read(delay) => {
                                println!("[{t_name}] read: {}", tmp_a.read_for(delay));
                            }
                            Command::Write(delay) => {
                                let tmp = (0..4)
                                    .map(|_| {
                                        (('a' as u8 + (rand::random::<u8>() % 26)) as char)
                                            .to_string()
                                    })
                                    .collect::<Vec<String>>()
                                    .join("");
                                tmp_a.write_for(tmp.clone(), delay);
                                println!("[{t_name}] wrote: {}", tmp);
                            }
                        }
                    }
                })
            })
            .collect::<Vec<JoinHandle<_>>>();

        for t in threads_handler {
            t.join().unwrap();
        }
    }
}

#[test]
fn test_zip() {
    let a = vec![1, 2, 3, 4];

    for x in a.iter().zip(&a[1..]) {
        println!("{x:?}")
    }
}
