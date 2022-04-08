use std::net::{TcpListener, TcpStream};
use std::io::{Error, Read, Write};

fn main(){

    println!("Start server ......");

    let listener = match TcpListener::bind("127.0.0.1:3000")  {
        Ok(e) => e,
        Err(err) => {
            println!("Start server fail error={}", &err);
            panic!("Start server fail...");
        }
    }; 

    for stream in listener.incoming() {
        let c:TcpStream = stream.unwrap();
        let data = match handle_read(c){
            Ok(e) => e,
            Err(err)=> {
                println!("Read data fail error {}", &err);
                vec![]
            }
        };
        println!("Read data={}", String::from_utf8(data).unwrap());
    }

    println!("Shutdown server ......");
}


fn handle_read(mut stream: TcpStream) -> Result<Vec<u8>, Error> {
    let mut buf = [0; 512];
    let mut buffer = vec![];
    loop {
        let size = stream.read(&mut buf)?;
        if size == 0  {
            return Ok(buffer);
        }
        for i in 0..size {
            buffer.push(buf[i]);
        }
        stream.write(&buf[..size])?;
    }
}