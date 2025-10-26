use std::net::{SocketAddr, TcpStream, TcpListener};
use std::io::{BufReader, Write};
use std::thread;
use std::time::Duration;
use std::fs::{File, read};

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
    let filedata =  openFileAsByte().expect("Couldn't read file");
    let ReadData:&[u8] = &filedata.as_slice(); 
    stream.write(ReadData).expect("Failed to respond!");}


fn openFileAsByte()->Result<Vec<u8>, Box<dyn std::error::Error>>{

    let mut fileData = read("pp.JPG").expect("Couldnt read file");
    println!("Read Data {:?}", &fileData);
    Ok(fileData)
}





fn main() {

    println!("Opening file cpp");
    let listen = thread::spawn(|| {
        tcp_server();
    });

    listen.join().unwrap();
    
    
}   


