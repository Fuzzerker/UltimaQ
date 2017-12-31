use std::net::TcpListener;
use std::io::Write;
use std::io::Read;
use std::str;

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:9123").unwrap();
    println!("listening!");
    let mut x = 0;
    for stream in listener.incoming() {
        println!("going");
        let mut unwrapped_stream = stream.unwrap();
        let mut control_buffer: [u8; 4] = [0;4];
        match unwrapped_stream.read(&mut control_buffer) {
            Ok(i) => {
                println!("control buffer 0 {}", control_buffer[0] );
                println!("control buffer 1 {}", control_buffer[1] );
                println!("control buffer 2 {}", control_buffer[2] );
                println!("control buffer 3 {}", control_buffer[3] );
                let control = str::from_utf8(&control_buffer).unwrap();
                println!("{}", control);
                let vec_size = control.parse::<usize>().unwrap();
                
                let mut body_buffer = vec![0; vec_size];
               
                unwrapped_stream.read_exact(&mut body_buffer);
                let body = match String::from_utf8(body_buffer) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

                println!("result: {}", body);
           
                
                let s = format!("{} {}",
                "Hello world",
                x.to_string()
                
        
                );
                let a = &s;
                println!("{}",a);
                unwrapped_stream.write(s.as_bytes()).unwrap();
                
            }
            Err(err) => {
                println!("error");
                println!("{}", err);
                }
        }
       


        x = x + 1;
    }
}