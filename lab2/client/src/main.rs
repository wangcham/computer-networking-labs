use std::{io::{self, Write, Read}, net::TcpStream, fs::File};

fn main() {
    println!("请输入选项");
    println!("输入1为获取大写字符串");
    println!("输入2为发送文件");
    println!("输入quit退出程序");

    loop {
        print!(" >>> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "quit" => break,
            "1" => str_method(),
            "2" => file_method(),
            _ => {
                println!("请输入正确的指令！");
                continue;
            }
        }
    }
}

fn str_method() {
    println!("请输入字符串,按回车输入完成");
    io::stdout().flush().unwrap();
    let mut strinput = String::new();
    io::stdin().read_line(&mut strinput).unwrap();
    let input = strinput.trim();
    let flag = 1;
    let info = send(input, flag);
    match info {
        Some(value) => println!("{}", value),
        None => println!("连接失败"),
    }
}

fn file_method() {
    println!("请输入文件地址:");
    io::stdout().flush().unwrap();
    let mut strinput = String::new();
    io::stdin().read_line(&mut strinput).unwrap();
    let input = strinput.trim();
    let flag = 2;
    let info = send(input, flag);
    print!("{}", flag);
    match info {
        Some(value) => println!("{}", value),
        None => println!("连接失败"),
    }
}

fn send(str: &str, flag: i32) -> Option<String> {
    println!("设置地址为本地5000端口");
    let mut addr = "127.0.0.1:5000";

    let mut stream = match TcpStream::connect(addr) {
        Ok(stream) => {
            println!("连接成功");
            stream
        }
        Err(_) => return None,
    };
    if flag == 1{
        // 使用 Vec<u8> 作为缓冲区
        let mut buffer = Vec::new();
        let data = flag.to_string()+ &str + "9";
        println!("{}",&str);
        buffer.extend_from_slice(data.as_bytes());
        stream.write_all(&buffer).unwrap();

        let mut sx = String::new();

        // 循环读取数据，直到遇到结束符 "9"
        loop {
            let mut temp_buffer = [0; 1]; // 临时缓冲区，用于读取一个字符
            match stream.read(&mut temp_buffer) {
                Ok(n) => {
                    if n == 0 { // 如果读取到的数据长度为 0，表示连接已经关闭
                        break;
                    }
                    let temp_s = String::from_utf8_lossy(&temp_buffer);
                    if temp_s == "9" { // 如果读取到的字符是结束符 "9"
                        break;
                    } else {
                        sx.push_str(&temp_s);
                    }
                }
                Err(_) => {
                    println!("读取数据时发生错误");
                    break;
                }
            }
        }
        let real  = &sx[1..];
        return Some(real.to_string());
    
    }else{
        let mut buffer = 

    }
}
