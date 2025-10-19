use std::net::TcpStream;
use std::time::Duration;
use std::thread;
fn tcp_client(){
            if let Ok(client) = TcpStream::connect("127.0.0.1:8080"){
                println!("Connected to Server from a seperate thread!!!");}
            else {
                println!("Connection Failed :[");
                thread::sleep(Duration::from_secs(3));}
}

fn main(){
    let client = thread::spawn(||
        {
            tcp_client();
        }
        );
    
    client.join().unwrap();


}
