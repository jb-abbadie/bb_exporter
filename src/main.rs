use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde::Deserialize;
use toml;

use prometheus::{Opts, Counter, Registry, Encoder, TextEncoder};

#[derive(Debug, Deserialize)]
struct Config {
    server: ServerConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    port: u16,
}


fn main() {
    let data = load_file("test.toml".to_string());
    let decoded: Config = toml::from_str(&data).unwrap();

    let a = setup_prom();
    let mut buffer = Vec::new();

    let r = Registry::new();
    r.register(Box::new(a.clone())).unwrap();


    let encoder = TextEncoder::new();
    let metric_families = r.gather();

    encoder.encode(&metric_families, &mut buffer).unwrap();
    let output = String::from_utf8(buffer.clone()).unwrap();



    println!("{}", output);
    println!("Port: {}", decoded.server.port);
}

fn load_file(file_name: String) -> String {
    let path = Path::new(&file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldnt open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s
}

fn setup_prom() -> Counter {
    let counter_opts = Opts::new("test_counter", "test counter help");
    let counter = Counter::with_opts(counter_opts).unwrap();

    counter.inc();

    counter
}
