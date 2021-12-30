use crate::arc_sync_cell::ArcSyncCell;
use crate::sync_cell::ISyncCell;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    _id: usize,
    _handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: ArcSyncCell<Receiver<Job>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let res = receiver.borrow_mut().recv();
            match res {
                Ok(job) => {
                    println!("Worker {} got job", id);
                    job()
                }
                Err(_) => {
                    println!("Worker {} signing off", id);
                    break;
                }
            }
        });
        Worker {
            _id: id,
            _handle: handle,
        }
    }
}

pub struct TestPool {
    _workers: Vec<Worker>,
    sender_: mpsc::Sender<Job>,
}

impl TestPool {
    pub fn new(worker_count: usize) -> Self {
        let (send, receive) = mpsc::channel();
        let receiver = ArcSyncCell::new(receive);
        let mut workers = Vec::with_capacity(worker_count);

        for n in 0..worker_count {
            workers.push(Worker::new(n, receiver.clone()))
        }

        TestPool {
            _workers: workers,
            sender_: send,
        }
    }

    pub fn execute<F>(&self, closure: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender_
            .send(Box::new(closure))
            .expect("Thread shut down too early");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    //noinspection ALL
    #[test]
    fn test_pool() {
        let pool = TestPool::new(4);
        let count = Arc::new(AtomicUsize::new(0));

        let count1 = count.clone();
        let count2 = count.clone();
        let count3 = count.clone();
        let count4 = count.clone();

        pool.execute(move || {
            let prev = count1.fetch_add(1, Ordering::AcqRel);
            thread::sleep(Duration::from_millis(32));
            println!("prev val: {}", prev);
        });
        pool.execute(move || {
            let prev = count2.fetch_add(1, Ordering::AcqRel);
            thread::sleep(Duration::from_millis(32));
            println!("prev val: {}", prev);
        });
        pool.execute(move || {
            let prev = count3.fetch_add(1, Ordering::AcqRel);
            thread::sleep(Duration::from_millis(32));
            println!("prev val: {}", prev);
        });
        pool.execute(move || {
            let prev = count4.fetch_add(1, Ordering::AcqRel);
            thread::sleep(Duration::from_millis(32));
            println!("prev val: {}", prev);
        });

        thread::sleep(Duration::from_secs(2));
        let count = count.load(Ordering::Acquire);
        assert_eq!(count, 4);
    }
}
