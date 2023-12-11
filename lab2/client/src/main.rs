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
    println!("请输入内容，按回车输入完成");
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
    println!("filepath is {}",input);
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
        let mut buffer = Vec::new();
        let data = flag.to_string()+&str+"9";
        //仍然以9作为结尾，标志流的末尾
        buffer.extend_from_slice(data.as_bytes());
        stream.write_all(&buffer).unwrap();
        print!("flag is {}",flag);
        
    let mut buffer = [0; 512]; // 创建一个缓冲区来存储接收到的数据
    let mut file = File::create("received_file").unwrap(); // 创建一个文件来保存接收到的数据

    loop {
        let nbytes = stream.read(&mut buffer).unwrap(); // 从流中读取数据
        if nbytes == 0 {
            break; // 如果没有读取到数据，那么就跳出循环
        }
        file.write_all(&buffer[..nbytes]).unwrap(); // 将数据写入文件
    }

    println!("文件已接收并保存到本地");

    return Some(String::from("文件传输成功"));



    }
}
