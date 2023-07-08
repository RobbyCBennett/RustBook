use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

#[cfg(debug_assertions)]
#[allow(unused_macros)]
macro_rules! debug
{
	( $x:expr ) => { println!($x) };
	( $x:expr, $y:expr ) => { println!($x, $y) };
}

#[cfg(not(debug_assertions))]
#[allow(unused_macros)]
macro_rules! debug
{
	( $x:expr ) => { std::convert::identity($x) };
	( $x:expr, $y:expr ) => { std::convert::identity($x); std::convert::identity($y) };
}

pub struct ThreadPool
{
	workers: Vec<Worker>,
	sender: Option<Sender<Job>>,
}

impl ThreadPool
{
	pub fn new(size: usize) -> ThreadPool
	{
		assert!(size > 0);

		let (sender, receiver) = std::sync::mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		return ThreadPool { workers, sender: Some(sender) };
	}

	pub fn execute<F>(&self, f: F)
	where
		F: FnOnce() + Send + 'static,
	{
		let job = Box::new(f);

		self.sender.as_ref().unwrap().send(job).unwrap();
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self)
	{
		drop(self.sender.take());

		for worker in &mut self.workers {
			debug!("Shutting down worker {}", worker.id);

			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
	}
}

struct Worker
{
	id: usize,
	thread: Option<JoinHandle<()>>,
}

impl Worker
{
	fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker
	{
		let thread = std::thread::spawn(move || loop {
			let receive_result = receiver.lock().unwrap().recv();

			if receive_result.is_err() {
				debug!("Worker {id} disconnected; shutting down.");
				break;
			}

			debug!("Worker {id} got a job; executing.");
			let job = receive_result.unwrap();
			job();
		});

		return Worker { id, thread: Some(thread) };
	}
}

type Job = Box<dyn FnOnce() + Send + 'static>;
