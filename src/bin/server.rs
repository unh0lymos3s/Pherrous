use std::net::{SocketAddr, TcpStream, TcpListener};
use std::io::{BufReader, Write};
use std::thread;
use std::time::Duration;
fn tcp_server(){
    let mut count:i32=0;
    let addrs =[
        SocketAddr::from(([127,0,0,1], 8080)),
    ];
    if let Ok(listener) = TcpListener::bind(&addrs[..])  {
        
        println!("Bound to the address 8080");
        /*match listener.accept() {
            Ok((_socket, addr)) => println!("new client {addr:?}"),
            Err(e) => println!("Error Receieved {e:?}"),
        }*/
        for stream in listener.incoming(){
            count +=1;            
            thread::spawn(||{
            let stream = stream.unwrap();
            handle_connection(stream);
            });
            println!("Thread {}", count);
        }
    }
    else {
        println!("Couldn't bind to address");
    }

}

fn handle_connection(mut stream: TcpStream){
    let buffer = BufReader::new(&stream);
    println!("request successful!!!!!!! 200");
    let response = "Hello from server!".as_bytes();    
    stream.write(response).expect("Failed to respond!");

}
fn main() 

    {
    let listen = thread::spawn(|| {
        tcp_server();
    });
/*    let streamer = thread::spawn(|| {
        tcp_client();
        //
        //
    });*/
//    streamer.join().unwrap();
    listen.join().unwrap();
    
    
}   


