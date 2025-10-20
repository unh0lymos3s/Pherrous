use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::Duration;
use std::thread;
fn tcp_client(){
        let mut client = match TcpStream::connect("127.0.0.1:8080") {

            Ok(client) => {println!("Connected to Server from a seperate thread!!!");
                let message = "Hello!".as_bytes();
                client.write(&message);}
            Err(e)=> {
                println!("Connection Failed :[");}};
        thread::sleep(Duration::from_secs(3));
}

fn main(){
    let client = thread::spawn(||
        {
            tcp_client();
        }
        );
    
    client.join().unwrap();


}
