use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::time::Duration;
use std::thread;
use std::fs::File;
fn tcp_client(){
        let mut getStream = match TcpStream::connect("127.0.0.1:8080") {
        
            Ok(mut getStream) => {
                println!("Recieving file..........");    
                let mut fileSizeBuf = [0u8; 8];
                let mut fileNameBuf = vec![0; 9];
                getStream.read_exact(&mut fileSizeBuf);
                getStream.read_exact(&mut fileNameBuf);
                if let Ok(filename) = String::from_utf8(fileNameBuf){
                    println!("Recieving File: {:?}",&filename);
                }
                
               // println!("Recieving File Name: {:?}",filename );
                acceptData(getStream);
                //decode filesize
               
                let filesize = u64::from_be_bytes(fileSizeBuf);
                println!("Recieved file size: {}", filesize);
               // acceptData(getStream);
            },
                
        
            Err(e) => {
                println!("Connection Failed :[");
            }
        };  
}

fn acceptData(mut stream: TcpStream)->Result<(), Box<dyn std::error::Error>>{
    let mut recievedFile = File::create("file.txt")?;
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
