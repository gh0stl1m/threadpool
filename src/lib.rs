use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::sync::Arc;
use std::vec;

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
    sender: Sender<Box<dyn Fn() + Send>>
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
                    let work = clone.lock().unwrap().recv();
                    match work {
                       Ok(rx) => {
                           println!("Got a job! executing");
                           rx();
                           println!("Job finished");
                       }
                       Err(_) => {
                           println!("Worker signing off");
                           break;
                       }
                    }
                }
            });

            _handles.push(handle);

        }
        Self{
            _handles,
            sender
        }
    }

    pub fn execute<T: Fn() + Send + 'static>(&self, work: T) {

        self.sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::ThreadPool;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(10);
        let foo = || std::thread::sleep(std::time::Duration::from_secs(1));
        pool.execute(foo.clone());
        pool.execute(foo);
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
