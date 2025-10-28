use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::time::Duration;
use std::thread;
use std::fs::File;
fn tcp_client(){
        let mut getStream = match TcpStream::connect("127.0.0.1:8080") {

            Ok(getStream) => {
                println!("Recieving file..........");    
                acceptData(getStream);
            },
                
        
            Err(e) => {
                println!("Connection Failed :[");
            }
        };  
}

fn acceptData(mut stream: TcpStream)->Result<(), Box<dyn std::error::Error>>{
    let mut recievedFile = File::create("cpp.cpp")?;
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;
    recievedFile.write_all(&mut buffer);

    Ok(())
    
    
}

fn main(){
    let client = thread::spawn(||
        {
            tcp_client();
        }
        );
    
    client.join().unwrap();


}
