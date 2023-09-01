

use std::{sync::Arc, time::Duration};

use crate::ReaderWriter;

#[derive(Copy, Clone)]
enum Command{
    Sleep(usize),
    Read(usize),
    Write(usize),
}

pub fn run_one<T: ReaderWriter + Default + Send + Sync + 'static>(work_set: String) {
    let a = Arc::new(T::default());

    let commands = work_set
        .split('\n')
        .filter(|s|!s.is_empty())
        .map(|s| s.trim())
        .map(|s| s.split(' ').map(|s| s.into()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

        println!("{commands:?}");

/*
        [build] r 3 w 3
        [r0] s 1 r r r
        [r1] 
        [r2] 
        [w0] s 1 w w w s 2 w w
        [w1] 
        [w2] 
*/

    let build_command = &commands[0];

    let r_thread_num = build_command[2].parse::<usize>().unwrap();
    let w_thread_num = build_command[4].parse::<usize>().unwrap();
    
    let run_commands = &commands[1..].iter().map(|commands|{
        let mut res = vec![];
        for (now, next) in commands.iter().zip(&commands[1..]){
            match (now.chars().next().unwrap(), next.chars().next().unwrap()) {
                ('[', _) => (),
                ('r', _) => res.push(Command::Read(next.parse::<usize>().unwrap())),
            ('w', _) => res.push(Command::Write(next.parse::<usize>().unwrap())),
                ('s', _) => res.push(Command::Sleep(next.parse::<usize>().unwrap())),
                _ => ()
            }
        }
        res
    }).collect::<Vec<Vec<Command>>>();

    let thread_names = &commands[1..].iter().map(|v|v[0].clone()).collect::<Vec<String>>();

    let mut threads_handler = vec![];

    for (i, run_command) in run_commands.iter().enumerate(){
        let tmp_a = a.clone();
        let commands = run_command.clone();
        let t_name = thread_names[i].clone();

        threads_handler.push(std::thread::spawn(move ||{
            for command in commands{
                match command{
                    Command::Sleep(delay) => std::thread::sleep(Duration::from_secs(delay as u64)),
                    Command::Read(delay) => {
                        println!("[{t_name}] read: {}", tmp_a.read_for(delay));
                    },
                    Command::Write(delay) => {
                        let mut tmp = vec![];
                        for _ in 0..4{
                            let offset = rand::random::<u8>() % 26;
                            let ch = ('a' as u8 + offset) as char;
                            tmp.push(ch.to_string())
                        }
                        tmp_a.write_for(tmp.join(""),delay);
                        println!("[{t_name}] wrote: {}", tmp.join(""));
                    }
                }
            }
        }));
    }

    for t in threads_handler{
        t.join();
    }
}

#[test]
fn test_zip(){
    let a = vec![1, 2, 3, 4];
    
    for x in a.iter().zip(&a[1..]){
        println!("{x:?}")
    }
}