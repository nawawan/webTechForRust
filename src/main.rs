use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use tera::{Tera, Context};

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();
    let mut items = Vec::<String>::new();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &mut items);
    }
}


fn handle_connection(mut stream: TcpStream, items: &mut Vec<String>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();


    let req = String::from_utf8_lossy(&buffer[..]).to_string();
    let contents = req.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();


    match (&contents[0] as &str , &contents[1] as &str ){
        ("POST", "/add") => {
            println!("POST");
            items.push(contents.last().unwrap().split("=").last().unwrap().to_string());
            let response = format!("HTTP/1.1 200 OK\r\n\r\n {}", response_html(items));
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
        ("GET", "/todo") => {
            println!("GET");
            let response = format!("HTTP/1.1 200 OK\r\n\r\n {}", response_html(items));
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
        ("GET", "/favicon.ico") => {
            println!("Not Found: favicon.ico");
        },
        ("GET", "/static/todo.css") => {
            println!("Not Found: /static/todo.css");
        },
        _ => {
            println!("Invalid request");
        }
    }

}

fn response_html(items: &mut Vec<String>) -> String {
    let mut context = Context::new();
    context.insert("todos", &items);
    let tera: Tera = Tera::new("templates/**/*").unwrap();
    let rendered: String = tera.render("todo.html", &context).unwrap();
    rendered
}
