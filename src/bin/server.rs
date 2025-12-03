use std::net::{SocketAddr, TcpStream, TcpListener};
use std::io::{BufReader, Write};
use std::fs::{File, read};
use std::path::{Path, PathBuf};
fn tcp_server(filePath: String, filename: String){
    let mut count:i32=0;
    let addrs =[
        SocketAddr::from(([127,0,0,1], 8080)),
    ];
    if let Ok(listener) = TcpListener::bind(&addrs[..])  {
        
        println!("Bound to the address 8080");
        
        for stream in listener.incoming(){
            let stream = stream.unwrap();
            send_file_size(&stream,filePath.clone());
            send_file_name(&stream, filename.clone());
            transmit_file(&stream, filePath.clone());

        }
    }
    else {
        println!("Couldn't bind to address");
    }

}


fn send_file_size(mut stream: &TcpStream, filePath: String){
    let buffer = BufReader::new(stream);
    
    let filedata =  openFileAsByte(filePath).expect("Couldn't read file");
    let ReadData:&[u8] = &filedata.as_slice(); 
    let size = getMetaData(ReadData) as u64; 
    
    println!("Transmitting Filesize\n");
    stream.write(&size.to_be_bytes()); //to view filesize, i. use netcat to save file as a binary,
     //ii hexdump -C recieved.bin | head

}

fn send_file_name(mut stream: &TcpStream, filename: String){
    stream.write(&filename.as_bytes());

}


fn transmit_file(mut stream: &TcpStream, filePath: String){
    let buffer = BufReader::new(stream);
    let file = openFileAsByte(filePath).expect("Error reading file");
    let fileContents: &[u8] = &file.as_slice();
    println!("\nStreaming whole file");
    let bytesWritten = stream.write(fileContents).expect("Failed to respond!");
    println!("Written bytes {:?}", bytesWritten);
}


fn openFileAsByte(fileName: String)->Result<Vec<u8>, Box<dyn std::error::Error>>{

    let mut fileData = read(fileName).expect("Couldnt read file"); //filename is essentially
                                                                   //file path
    println!("Read Data {:?}", &fileData);
    println!("Size of file: {}", fileData.len());//this is returning the size just fine    
    Ok(fileData)
}

fn getMetaData(ReadData: &[u8])->usize{
    let dataLength = ReadData.len();
    
    return dataLength
}

fn getFilename()->(String, String){
    loop{
    println!("Please Enter the File's path that you need to host.");
    let mut file_string = String::new();
    let input_path = std::io::stdin().read_line(&mut file_string).unwrap();//saving the value to file_string
    let trimmed_file_string = file_string.trim();
    let file_path = PathBuf::from(&trimmed_file_string);
    
    if file_path.exists(){
        //let fileOsString = file_path.as_os_str();
        if let Some(file) = file_path.file_name(){
            if let Some(validName) = file.to_str(){
                return (validName.to_owned(), trimmed_file_string.to_owned());
            }
        }
        
    }
    else{
        println!("File not found, try again!");
    }}  
}


   
fn main() {
        println!("Welcome to pherrous");
        let filename: String;
        let filepath:String;
        (filename,filepath) = getFilename();
        println!("Serving file {:?} at 127.0.0.1:8080", filename);
        tcp_server(filepath, filename);
      


}

