extern crate core;
extern crate regex;

pub mod connection;
pub mod pages;

use pages::*;

use std::{io::prelude::*, net::TcpStream, sync::Arc};

use connection::HttpRequest;

struct Callback {
    pattern: regex::Regex,
    page: Arc<Box<HttpPage>>,
}

pub struct HttpServer {
    callbacks: Vec<Callback>,
}

impl HttpServer {
    pub fn new() -> Self {
        HttpServer {
            callbacks: Vec::new(),
        }
    }

    pub fn callback<T: AsRef<str>>(
        &mut self,
        pattern: T,
        page: Arc<Box<HttpPage>>,
    ) -> Result<(), regex::Error> {
        let pattern = regex::Regex::new(pattern.as_ref())?;
        self.callbacks.push(Callback { pattern, page });

        Ok(())
    }

    pub fn handle_connection(&self, stream: &mut TcpStream) {
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer).unwrap();

        match connection::HttpRequest::parse_from(String::from_utf8_lossy(&buffer[..bytes_read])) {
            Ok(http_request) => for callback in &self.callbacks {
                if let Some(_captures) = callback.pattern.captures(http_request.path()) {
                    callback.page.process(&http_request, stream);
                    return;
                }
            },
            Err(err) => {
                println!("Connection error: {}", err);
            }
        }
    }
}