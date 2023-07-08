use std::thread::JoinHandle;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub struct ThreadPool
{
	workers: Vec<Worker>,
	sender: Sender<Job>,
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

		return ThreadPool { workers, sender };
	}

	pub fn execute<F>(&self, f: F)
	where
		F: FnOnce() + Send + 'static,
	{
		let job = Box::new(f);

		self.sender.send(job).unwrap();
	}
}

struct Worker
{
	id: usize,
	thread: JoinHandle<()>,
}

impl Worker
{
	fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker
	{
		let thread = std::thread::spawn(move || loop {
			let job = receiver.lock().unwrap().recv().unwrap();

			println!("Worker {id} got a job; executing.");

            job();
		});

		return Worker { id, thread };
	}
}

type Job = Box<dyn FnOnce() + Send + 'static>;
