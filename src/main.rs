mod reader_writer_rf;
mod reader_writer_wf;

mod runner;
mod semaphore;

pub trait ReaderWriter {
    /// read from shared memory with delay(secs)
    fn read_for(&self, delay: usize) -> String;
    /// write to shared memory with delay(secs)
    fn write_for(&self, s: String, delay: usize);
}

fn main() {
    println!("hello");
}
