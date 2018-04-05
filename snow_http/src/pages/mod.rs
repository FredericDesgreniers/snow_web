pub mod static_page;
pub mod static_directory_page;

pub use static_page::*;
pub use static_directory_page::*;

use super::HttpRequest;

use std::net::TcpStream;

/// An http page, serves binary content
pub trait HttpPage: Sync + Send {
    fn process(&self, request: &HttpRequest, connection: &mut TcpStream);
}
