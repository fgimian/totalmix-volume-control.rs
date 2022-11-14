use anyhow::Result;
use rosc::OscPacket;
use std::{
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    time::Duration,
};

pub trait Sender {
    fn send(&self, packet: &OscPacket) -> Result<()>;
}

pub trait Receiver {
    fn receive(&self) -> Result<OscPacket>;
}

pub(crate) struct UdpSender {
    socket: UdpSocket,
    addr: SocketAddrV4,
}

impl UdpSender {
    pub fn new(addr: SocketAddrV4) -> Result<Self> {
        let bind_addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0);
        Ok(UdpSender {
            socket: UdpSocket::bind(bind_addr)?,
            addr,
        })
    }
}

impl Sender for UdpSender {
    fn send(&self, packet: &OscPacket) -> Result<()> {
        let packet = rosc::encoder::encode(packet)?;
        self.socket.send_to(&packet, &self.addr)?;
        Ok(())
    }
}

pub(crate) struct UdpReceiver {
    socket: UdpSocket,
}

impl UdpReceiver {
    pub fn bind(addr: SocketAddrV4) -> Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        socket.set_read_timeout(Some(Duration::from_secs(5)))?;
        Ok(UdpReceiver { socket })
    }
}

impl Receiver for UdpReceiver {
    fn receive(&self) -> Result<OscPacket> {
        let mut buf = [0u8; rosc::decoder::MTU];
        let (size, _) = self.socket.recv_from(&mut buf)?;
        let (_, packet) = rosc::decoder::decode_udp(&buf[..size])?;
        Ok(packet)
    }
}
