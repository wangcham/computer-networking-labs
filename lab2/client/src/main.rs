use std::{io::{self, Write, stdout, Read}, net::{TcpListener, TcpStream}, fs::File};

fn main(){
    println!("请输入选项");
    println!("输入1为获取大写字符串");
    println!("输入2为发送文件");
    println!("输入quit退出程序");
    
    loop{
        print!(" >>> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input.trim() {
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

fn str_method(){
    println!("请输入字符串,按回车输入完成");
    io::stdout().flush().unwrap();
    let mut strinput = String::new();
    io::stdin().read_line(&mut strinput).unwrap();
    let input = strinput.trim();
    let flag = 1;
    let info = send(& input,flag);
    match info {
        Some(value) => println!("{}",value),
        None => println!("连接失败"),
    }
}

fn file_method(){
    println!("请输入文件名字:");
    io::stdout().flush().unwrap();
    let mut strinput = String::new();
    io::stdin().read_line(&mut strinput).unwrap();
    let input = strinput.trim();
    let flag = 2;
    let info = send(&input,flag);
    print!("{}", flag);
    match info {
        Some(value) => println!("{}",value),
        None => println!("连接失败"),
    }
}

fn send(str:&str,flag:i32) -> Option<String>{
    println!("请输入服务器ip地址和端口");
    println!("比如127.0.0.1:5000");
    io::stdout().flush().unwrap();
    let mut strinput = String::new();
    io::stdin().read_line(&mut strinput).unwrap();
    let addr = strinput.trim();
    let addr = addr.trim();
    
    //判断是否连接成功
    let mut stream =match TcpStream::connect(addr){
        Ok(stream) => stream,
        Err(_) => return None,
    };

    if flag == 1 {
        let filename = str.trim();
        stream.write(filename.as_bytes()).unwrap();

        stream.flush().unwrap();
        let mut buffer = Vec::new();

        stream.read_to_end(&mut buffer).unwrap();
        //判断文件是否存在，依据是翻译的流是否为N
        if String::from_utf8_lossy(&mut buffer) == "N" {
            return Some("未查询到文件".to_string());
        }
        //接受文件
        let mut file = File::create("received_file.txt").unwrap();
        file.write_all(&buffer).unwrap();
        return Some("接收到文件，名字是received_file".to_string());

    }else{
        stream.write(str.as_bytes()).unwrap();
        stream.flush().unwrap();
        let buffer = Vec::new();
        let received_string = String::from_utf8_lossy(&buffer);
        
        return Some(received_string.to_string());
    }
}