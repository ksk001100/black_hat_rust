use std::env;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let url = match args.len() {
        2 => &args[1],
        _ => panic!("Argument error... (ex: tcp_port_scanner scanme.nmap.org)"),
    };

    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for i in 1..=1024 {
        let addr = format!("{}:{}", url, i)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();

        let open_ports = open_ports.clone();
        handles.push(thread::spawn(move || {
            match TcpStream::connect_timeout(&addr, Duration::from_secs(3)) {
                Ok(_) => open_ports.lock().unwrap().push(i),
                Err(_) => (),
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let open_ports = Arc::try_unwrap(open_ports).unwrap();
    open_ports.lock().unwrap().sort();
    for port in open_ports.lock().unwrap().iter() {
        println!("{} open", port);
    }
}
