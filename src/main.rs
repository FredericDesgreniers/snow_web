#![feature(nll)]

extern crate rayon;
extern crate snow_core;
#[macro_use]
extern crate snow_http;
#[macro_use]
extern crate lazy_static;

use std::sync::Arc;

use snow_core::tcp;
use snow_http::{HttpServer, pages::*};

lazy_static! {
    static ref HTTP_SERVER: HttpServer = {
        let mut server = HttpServer::new();

        static_directory_page!(server, "/res/", "res/");
        static_page!(server, "/", "index.html");

        server
    };
}

fn main() {
    http_server_start!(HTTP_SERVER, 80, 10).unwrap();
}
