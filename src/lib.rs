use core::num;
use std::sync::mpsc::{channel};
use std::sync::Mutex;
use std::sync::Arc;
use std::vec;

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>
}

impl ThreadPool {

    pub fn new(num_threads: u8) -> Self {
        let (sender, reciever) = channel::<Box<dyn Fn() + Send>>();
        let reciever = Arc::new(Mutex::new(reciever));
        let mut _handles = vec![];

        for _ in 0..num_threads {
            let clone = reciever.clone();
            let handle = std::thread::spawn(move || {
                loop {
                    let work = clone.lock().unwrap().recv().unwrap();
                    work();
                }
            });

            _handles.push(handle);

        }
        Self{
            _handles
        }
    }

    pub fn execute<T: Fn()>(&self, work: T) {}
    
}

#[cfg(test)]
mod tests {
    use crate::ThreadPool;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new();
        pool.execute(|| println!("Hello from thread 1"));
        pool.execute(|| println!("Hello from thread 2"));
    }
}
