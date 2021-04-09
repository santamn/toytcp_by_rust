use crate::packet::TCPPacket;
use crate::tcpflags;
use crate::tcpflags;
use anyhow::{Context, Result};
use pnet::packet::{ip::IpNextHeaderProtocols, tcp::TcpPacket, Packet};
use pnet::packet::{ip::IpNextHeaderProtocols, Packet};
use pnet::transport::{self, TransportChannelType, TransportProtocol, TransportSender};
use pnet::util;
use pnet::util;
use std::collections::VecDeque;
use std::fmt::{self, Debug};
use std::fmt::{self, Display};
use std::net::Ipv4Addr;
use std::net::{IpAddr, Ipv4Addr};
use std::time::SystemTime;

const SOCKET_BUFFER_SIZE: usize = 4380;
const TCP_HEADER_SIZE: usize = 20;

#[derive(Clone)]
pub struct TCPPacket {
    buffer: Vec<u8>,
}

impl TCPPacket {
    pub fn new(payload_len: usize) -> Self {
        Self {
            buffer: vec![0; TCP_HEADER_SIZE + payload_len],
        }
    }

    pub fn set_src(&mut self, port: u16) {
        self.buffer[0..2].copy_from_slice(&port.to_be_bytes()) // ポート番号の数値をビッグエンディアンに直している
    }

    pub fn set_dest(&mut self, port: u16) {
        self.buffer[2..4].copy_from_slice(&port.to_be_bytes())
    }

    pub fn set_flag(&mut self, flag: u8) {
        self.buffer[13] = flag;
    }
}
