use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::time::Duration;
use std::thread;
use std::fs::File;
fn tcp_client(){
        let mut getStream = match TcpStream::connect("127.0.0.1:8080") {
        
            Ok(mut getStream) => {
                println!("Recieving file..........");
                let mut fileNameLengthBuf = [0;1];
                let mut fileSizeBuf = [0u8; 8];
                 

                getStream.read_exact(&mut fileNameLengthBuf);
                println!("{:?}", fileNameLengthBuf);
                let fileNameLength = u8::from_be_bytes(fileNameLengthBuf);
                println!("filename read from be to u64, filenamelength is {}", fileNameLength);
                let lengthForBuffer = convert_u64_to_usize(fileNameLength);
                println!("filename converted from ");
                println!("{}", lengthForBuffer);
                let mut fileNameBuf = vec![0; lengthForBuffer];

                getStream.read_exact(&mut fileNameBuf);
                if let Ok(filename) = String::from_utf8(fileNameBuf){
                    println!("Recieving File: {:?}",&filename);
                }
                getStream.read_exact(&mut fileSizeBuf);
                
                
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

fn convert_u64_to_usize(integer: u8)->usize{
    let int_8: u8 = integer;
    let int_usize: usize = int_8 as usize;
    return int_usize
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
