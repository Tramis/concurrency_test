mod reader_writer_rf;
mod reader_writer_wf;

mod runner;
mod semaphore;

pub trait ReaderWriter {
    fn read_for(&self, delay: usize) -> String;
    fn write_for(&self, s: String, delay: usize);
}

fn main() {
    println!("hello");
}
