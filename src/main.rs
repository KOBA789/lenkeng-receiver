use std::io::Write;
use std::net::UdpSocket;
use std::net::Ipv4Addr;

const PACKET_SIZE: usize = 1024;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:2068").expect("failed to bind to address");
    let membership: Ipv4Addr = "226.2.2.2".parse().unwrap();
    let ifaddr: Ipv4Addr = "192.168.168.123".parse().unwrap();
    socket.join_multicast_v4(&membership, &ifaddr).expect("failed to join to multicast group");
    let mut buf: Vec<u8> = vec![0; PACKET_SIZE];
    let mut is_open = false;
    println!("--myboundary\nContent-Type: image/jpeg\n");
    loop {
        socket.recv(&mut buf).expect("failed to read from socket");
        //let frame_n = (buf[0] as u16) * 0xFF + buf[1] as u16;
        let part_n = (buf[2] as u16) * 0xFF + buf[3] as u16;
        if part_n == 0 {
            println!("\n--myboundary\nContent-Type: image/jpeg\n");
            is_open = true;
        }
        if is_open {
            std::io::stdout().write_all(&buf[4..]).expect("could not write out buffer");
        }
    }
}
