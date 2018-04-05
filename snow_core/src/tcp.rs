use std::io::Result;
use std::net::TcpListener;
use thread_pool::ThreadPool;
use std::net::TcpStream;

pub struct TcpStreamListener {}

impl TcpStreamListener {
    pub fn listen<T: Fn(TcpStream) + Send + Sync + 'static + Copy>(
        worker_num: usize,
        port: u32,
        callback: T,
    ) -> Result<()> {
        let thread_pool = ThreadPool::new(worker_num);

        let listening = TcpListener::bind(format!("127.0.0.1:{}", port))?;

        println!("Listening on at http://localhost::{}", port);

        for stream in listening.incoming().into_iter() {
            if let Ok(stream) = stream {
                thread_pool.do_work(move || {
                    callback(stream);
                });
            }
        }

        Ok(())
    }
}
