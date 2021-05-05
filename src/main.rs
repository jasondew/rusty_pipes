use etherparse;
use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::without_packet_info("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];

    loop {
        let bytes_read = nic.recv(&mut buf[..]).unwrap();
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[..bytes_read]) {
            Err(value) => continue,
            Ok(ip_header) => {
                if ip_header.protocol() == 0x06 {
                    match etherparse::TcpHeaderSlice::from_slice(
                        &buf[ip_header.slice().len()..bytes_read],
                    ) {
                        Err(value) => println!("Err {:?}", value),
                        Ok(tcp_header) => {
                            // println!("IP header: {:?} TCP header: {:?}", ip_header, tcp_header);
                            let data_offset = (tcp_header.data_offset() * 4) as usize;
                            println!(
                                "{:}:{:} -> {:}:{:} #=> {:?}",
                                ip_header.source_addr(),
                                tcp_header.source_port(),
                                ip_header.destination_addr(),
                                tcp_header.destination_port(),
                                &buf[data_offset..bytes_read]
                            );
                        }
                    }
                } else {
                    println!("unknown protocol: {:?}", ip_header.protocol());
                }
            }
        }
    }

    Ok(())
}
