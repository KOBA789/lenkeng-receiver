use std::net::UdpSocket;
use std::net::Ipv4Addr;
use std::thread;
use std::time::Duration;

mod turbojpeg;

const PACKET_SIZE: usize = 1024;
const MAX_CHUNK: usize = 1000;
const MAGIC_PACKET: [u8; 0x17] = [
    0x54, 0x46, 0x36, 0x7a, 0x60, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x03, 0x01, 0x00, 0x26,
    0x00, 0x00, 0x00, 0x00, 0x02, 0x34, 0xc2,
];
const TX_ADDR: &'static str = "192.168.168.55:48689";

fn main() {
    thread::spawn(move || {
        heatbeat();
    });
    let socket = UdpSocket::bind("0.0.0.0:2068").expect("failed to bind to address");
    let membership: Ipv4Addr = "226.2.2.2".parse().unwrap();
    let ifaddr: Ipv4Addr = "192.168.168.123".parse().unwrap();
    socket.join_multicast_v4(&membership, &ifaddr).expect("failed to join to multicast group");

    let mut pixels = Vec::<u8>::new();
    let mut jpeg_buf = Vec::<u8>::with_capacity(PACKET_SIZE * MAX_CHUNK);
    let mut chunk_buf: Vec<u8> = vec![0; PACKET_SIZE];
    let mut dec = turbojpeg::Decompress::new().unwrap();

    loop {
        socket.recv(&mut chunk_buf).expect("failed to read from socket");
        let part_n = (chunk_buf[2] as u16) * 0xFF + chunk_buf[3] as u16;
        if part_n == 0 {
            jpeg_buf.clear();
        }
        jpeg_buf.extend_from_slice(&chunk_buf[4..]);
        let header = dec.decompress_header(&jpeg_buf);
        if header.dst_size() > pixels.len() {
            pixels.resize(header.dst_size(), 0);
        }
        let _dec_ret = dec.decompress(&jpeg_buf, &header, pixels.as_mut_slice());
    }
}

fn heatbeat() {
    let socket = UdpSocket::bind("192.168.168.123:48689").expect("failed to bind to address");
    loop {
        socket.send_to(&MAGIC_PACKET, TX_ADDR).expect("failed to send heart beat");
        thread::sleep(Duration::from_secs(1));
    }
}
