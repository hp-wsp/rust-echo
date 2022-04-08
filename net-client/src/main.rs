use std::net::TcpStream;
use std::io::{Write, BufReader, BufRead};

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:3000"){
        Ok(c)  => c
        ,Err(err) => {
            println!("Connect host 127.0.0.1:3000 fail error {}", &err);
            panic!("Connect host 127.0.0.1:3000 fail");
        }
    };

    let buf = "hello\n".as_bytes();
    stream.write(buf).unwrap();

    let mut buffer = vec![];
    let mut reader = BufReader::new(&stream);

    let size = reader.read_until(b'\n', &mut buffer).unwrap();
    let str = String::from_utf8(buffer).unwrap();
    println!("Receive server data size={},data={}", size, str);
}
