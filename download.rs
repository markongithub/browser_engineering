use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    get_url("http://cnn.com");
}

fn split_in_two(string: &str, separator: &str) -> (String, String) {
    let (first, second) = string.split_once(separator).unwrap();
    (first.to_string(), second.to_string())
}

fn parse_url(url: &str) -> (String, String, String) {
    let (scheme, host_path) = split_in_two(url, "://");
    let full_host_path = if !host_path.contains("/") {
        host_path + "/"
    } else {
        host_path
    };
    let (host, path_without_slash) = split_in_two(&full_host_path, "/");
    let path = "/".to_owned() + &path_without_slash;
    (scheme, host, path)
}

fn retrieve(host: &str, path: &str) {
    // They said just to use port 80 to start with.
    let mut tcp_stream = TcpStream::connect(format!("{}:80", host)).unwrap();
    let request_str = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", path, host);
    tcp_stream.write_all(request_str.as_bytes()).unwrap();
    tcp_stream.flush().unwrap();
    let mut buffer = [0; 1024];
    let size = tcp_stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..size]);
    println!("Server says: {}", message);
}

fn get_url(url: &str) {
    let (_scheme, host, path) = parse_url(url);
    retrieve(&host, &path);
}
