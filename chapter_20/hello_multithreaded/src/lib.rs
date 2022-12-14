use std::fmt::{Display, Formatter};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Sending terminate message to all workers.");

    for _ in &self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }
    println!("Shutting down all workers.");

    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);

      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
  NewJob(Job),
  Terminate,
}

#[derive(Clone)]
pub struct PoolCreationError;

impl Display for PoolCreationError {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "Pool size must be greater than 0")
  }
}

impl std::fmt::Debug for PoolCreationError {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "Pool size must be greater than 0")
  }
}

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
    if size == 0 {
      return Err(PoolCreationError);
    }

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    Ok(ThreadPool { workers, sender })
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);

    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

// type SomeType = Arc<Mutex<mpsc::Receiver<Job>>>;

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      // let job = receiver.lock().unwrap().recv().unwrap();
      let message = receiver.lock().unwrap().recv().unwrap();

      match message {
        Message::NewJob(job) => {
          println!("Worker {} got a job; executing.", id);
          job();
        }
        Message::Terminate => {
          println!("Worker {} was told to terminate.", id);

          break;
        }
      }
    });

    Worker {
      id,
      thread: Some(thread),
    }
  }
}
