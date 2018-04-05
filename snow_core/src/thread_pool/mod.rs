use std::{panic, sync::{Arc, Mutex, mpsc::{channel, Receiver, Sender}},
          thread::{self, JoinHandle}, vec::Vec};

/// Thread pool that sends work to Workers using a multiple receivers - single sender model
pub struct ThreadPool<T: FnOnce() + Send + 'static> {
    sender: Sender<Work<T>>,
    receiver: Arc<Mutex<Receiver<Work<T>>>>,
    workers: Vec<Worker>,
}

impl<T: FnOnce() + Send + 'static> ThreadPool<T> {
    /// Create a new thread pool
    /// # Arguments
    /// * `worker_num` Number of workers to spawn
    pub fn new(worker_num: usize) -> Self {
        let (tx, rx) = channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::new();

        for i in 0..worker_num {
            workers.push(Worker::spawn(Arc::clone(&rx)));
        }

        Self {
            sender: tx,
            receiver: rx,
            workers,
        }
    }

    /// Do work by sending it to a worker thread
    /// # Arguments
    /// * `work_unit` work unit that will be sent to a worker
    pub fn do_work(&self, work_unit: T) {
        self.sender.send(Work::Unit(work_unit));
    }

    /// Consume the thread pool and makes sure all the workers finish
    pub fn dispose(self) {
        for i in 0..self.workers.len() {
            self.sender.send(Work::End);
        }

        self.workers.into_iter().for_each(|worker| worker.join());
    }
}

/// A worker does work by using accepting work units.
/// It also catches panics
pub struct Worker {
    handle: JoinHandle<()>,
}

impl Worker {
    /// Create a new worker, this will also start the worker thread, causing it to accept work
    /// # Arguments
    /// * `receiver` A channel receiver that will be sued to receive work units
    pub fn spawn<T: FnOnce() + Send + 'static>(receiver: Arc<Mutex<Receiver<Work<T>>>>) -> Self {
        Self {
            handle: thread::spawn(move || {
                panic::set_hook(Box::new(|info| {
                    println!("Worker panic caught: {:?}", info);
                }));

                'work_loop: while let Ok(work) = receiver.lock().unwrap().recv() {
                    match work {
                        Work::Unit(work_unit) => {
                            let unwind_safe_work_unit = panic::AssertUnwindSafe(work_unit);
                            let _ = panic::catch_unwind(move || unwind_safe_work_unit());
                        }
                        Work::End => {
                            break 'work_loop;
                        }
                    }
                }
            }),
        }
    }

    /// Consume worker and wait for the worker to join
    /// Note that this will not do anything to tell that worker thread that it's time to end
    /// To do that, the caller must first make sure the worker receives a Work::End message
    pub fn join(self) {
        self.handle.join();
    }
}

/// Work that is sent to workers
pub enum Work<T: FnOnce() + Send + 'static> {
    Unit(T),
    End,
}
