mod limits;
mod handle;
mod respond;
mod thread_pool;

use std::net::TcpListener;

use crate::handle::*;
use crate::limits::*;
use crate::thread_pool::*;

fn main()
{
	let listener = TcpListener::bind("localhost:8080").unwrap();
	let pool = ThreadPool::new(THREAD_POOL_SIZE);

	for result in listener.incoming() {
		if !result.is_err() {
			pool.execute(|| handle(result.unwrap()));
		}
	}
}
