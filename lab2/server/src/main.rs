use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_stream(stream);
    }

}

fn handle_stream(mut stream:TcpStream){
    
}