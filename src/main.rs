use std::{
    env::current_dir,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use web_server::ThreadPool;
const NUMBER_THREADS: usize = 5;

fn main() {
    let addr: String = String::from("127.0.0.1:3000");
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);
    let pool = ThreadPool::new(NUMBER_THREADS);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let current_dir = current_dir().unwrap();
    let buf_read = BufReader::new(&mut stream);
    let head = buf_read.lines().next().unwrap().unwrap_or_else(move |e| {
        println!("There was an error: {:?}", e);
        String::from("")
    });

    let hello = fs::read_to_string(current_dir.join("hello.html")).unwrap();
    let not_found = fs::read_to_string(current_dir.join("404.html")).unwrap();

    let (status, contents) = match &head[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", hello),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", fs::read_to_string("hello.html").unwrap())
        }
        _ => ("HTTP/1.1 404 NOT FOUND", not_found),
    };

    let length = contents.len();
    let res = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(res.as_bytes()).unwrap();
}
