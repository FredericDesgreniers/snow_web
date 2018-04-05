use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest {
    method: String,
    path: String,
    map: HashMap<String, String>,
}

impl HttpRequest {
    pub fn new<T: AsRef<str>>(method: T, path: T) -> Self {
        Self {
            method: method.as_ref().to_string(),
            path: path.as_ref().to_string(),
            map: HashMap::new(),
        }
    }

    pub fn parse_from<T: AsRef<str>>(text: T) -> Result<HttpRequest, String> {
        let mut lines = text.as_ref().lines();

        let request_line = lines.next().ok_or_else(|| "Empty request")?;

        let mut request_line_parts = request_line.split(' ');

        let mut http_request_info = HttpRequest::new(
            request_line_parts
                .next()
                .ok_or_else(|| "No method in request line")?,
            request_line_parts
                .next()
                .ok_or_else(|| "No path in request line")?,
        );

        for line in lines {
            if let Some(sep_index) = line.find(':') {
                let (key, value) = line.split_at(sep_index);
                let value = value[1..].trim();
                http_request_info
                    .map
                    .insert(key.to_string(), value.to_string());
            }
        }

        Ok(http_request_info)
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn value<T: AsRef<str>>(&self, key: T) -> Option<&String> {
        self.map.get(&key.as_ref().to_string())
    }
}
