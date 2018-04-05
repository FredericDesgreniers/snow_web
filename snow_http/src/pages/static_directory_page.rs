use pages::HttpPage;
use std::fs::File;
use std::fs;
use connection::HttpRequest;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::Result;
use std::collections::HashMap;

/// Creates a static directory page and registers it in an http server
#[macro_export]
macro_rules! static_directory_page(
	($server: expr, $path: expr, $name: expr ) => {
	if let Ok(dir_page) = StaticDirectoryPage::new($path, $name) {
		let index_page = Arc::new(Box::new(dir_page) as Box<HttpPage>);
			$server.add_callback(format!("^{}",$path), Arc::clone(&index_page)).unwrap();
		}
	}
);

/// A static directory page
/// Keeps in memory and serves every file in the directory
/// non-recursive
pub struct StaticDirectoryPage {
    url_path: String,
    header_bytes: Vec<u8>,
    file_map: HashMap<String, Vec<u8>>,
}

impl StaticDirectoryPage {
    pub fn new<T: AsRef<str>>(url_path: T, path: T) -> Result<Self> {
        let mut file_map = HashMap::new();

        let paths = fs::read_dir(path.as_ref())?;
        paths
            .into_iter()
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            return Some(entry);
                        }
                    }
                }

                return None;
            })
            .for_each(|entry| {
                let mut buffer = Vec::new();

                if let Ok(mut file) = File::open(entry.path()) {
                    if let Ok(_) = file.read_to_end(&mut buffer) {
                        file_map.insert(entry.file_name().to_str().unwrap().to_string(), buffer);
                    }
                }
            });

        Ok(Self {
            header_bytes: "HTTP/1.1 200 OK\r\n\r\n".as_bytes().to_vec(),
            file_map,
            url_path: url_path.as_ref().to_string(),
        })
    }
}

impl HttpPage for StaticDirectoryPage {
    fn process(&self, request: &HttpRequest, connection: &mut TcpStream) {
        connection.write(&self.header_bytes).unwrap();

        if let Some(file_index) = request.path().find(self.url_path.as_str()) {
            let file_name = &request.path()[file_index + self.url_path.len()..];
            if let Some(contents) = self.file_map.get(&String::from(file_name)) {
                connection.write(contents).unwrap();
            }
        }

        connection.flush().unwrap();
    }
}
