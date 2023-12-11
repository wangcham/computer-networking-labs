use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, fs::{File, self}, num::ParseIntError};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").expect("监听失败");

    println!("监听成功！");

    for stream in listener.incoming() {
        let stream = stream.expect("获取客户端连接时发生错误");
        handle_stream(stream);
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut s = String::new();
    let mut buffer = [0; 1]; // Use a single-element buffer

loop {
    match stream.read_exact(&mut buffer) {
        Ok(_) => {
            let temps = String::from_utf8_lossy(&buffer);
            if temps == "9" {
                break;
            } else {
                s.push_str(&temps);
            }
        }
        Err(_) => {
            println!("读取数据失败");
            break;
        }
    }
}

    let first_char = s.chars().next().expect("解析字符串失败");
    println!("first char is {}", first_char);

    let str_content: String = s.chars().skip(1).collect();

    // 处理接收到的数据
    if first_char == '2' {
        println!("进入2方法");
        send_file(&str_content,&mut stream);
    } else {
        println!("获取字符串操作");
        let one = "1";
        let uppercase = format!("{}{}{}", one, str_content.to_uppercase(), "9");
        let response = uppercase.as_bytes();

        // 发送数据
        stream.write_all(response).unwrap();
    }
}

fn send_file(file_path: &str, stream: &mut TcpStream) {
    println!("filepath is {}", file_path);
    match File::open(file_path) {
        Ok(mut file) => {
            let contents = fs::read(file_path).expect("无法读取文件");
            stream.write_all(&contents).expect("发送失败");
            println!("发送成功");
        }
        Err(_) => {
            // 发送错误标志
            stream.write_all(b"N").expect("发送错误标志失败");
        }
    }
}
