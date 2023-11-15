use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, fs::File};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000");
    match listener {
        Ok(listener) => {
            println!("监听成功！");
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                handle_stream(stream);
            }
        },
        Err(e) => panic!("监听失败：{}", e),  
    }

}

fn handle_stream(mut stream:TcpStream){
    println!("进入handle——stream");
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).unwrap();
    let s = String::from_utf8_lossy(&buffer);
    let first_char = s.chars().next().unwrap();
    println!("{}",first_char);
    let str:String = s.chars().skip(1).collect();

    if first_char == '1' {

        match File::open(&str) {
            Ok(mut file) => {
                let mut contents = Vec::new();
                file.read_to_end(&mut contents).unwrap();
                stream.write_all(&contents).unwrap();
            }
            Err(_) => {
                stream.write_all(b"N").unwrap();
            }
        }
    }else{
        let uppercase = str.to_uppercase();
        stream.write_all(uppercase.as_bytes()).unwrap();
    }

}