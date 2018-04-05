use pages::HttpPage;
use std::fs::File;
use connection::HttpRequest;
use std::net::TcpStream;
use std::io::{Read, Write};

/// Creates a static page and registers it in an http server
#[macro_export]
macro_rules! static_page(
	($server: expr, $path: expr, $name: expr ) => {
		let index_page = Arc::new(Box::new(StaticPage::new($name)) as Box<HttpPage>);
        $server.callback(format!("^{}",$path), Arc::clone(&index_page)).unwrap();
	}
);

/// An in-memory static page
pub struct StaticPage {
    header_bytes: Vec<u8>,
    page_bytes: Vec<u8>,
}

impl StaticPage {
    pub fn new<T: AsRef<str>>(path: T) -> Self {
        let mut buffer: Vec<u8> = Vec::with_capacity(512);

        let mut file = File::open(path.as_ref()).unwrap();
        let _ = file.read_to_end(&mut buffer).unwrap();

        Self {
            header_bytes: "HTTP/1.1 200 OK\r\n\r\n".as_bytes().to_vec(),
            page_bytes: buffer,
        }
    }
}

impl HttpPage for StaticPage {
    fn process(&self, _request: &HttpRequest, connection: &mut TcpStream) {
        connection.write(&self.header_bytes).unwrap();
        connection.write(&self.page_bytes).unwrap();
        connection.flush().unwrap();
    }
}
