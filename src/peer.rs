use crate::packet::Packet;
use async_std::sync::{channel, Receiver, Sender};
use std::collections::HashMap;

pub type PeerGroup = HashMap<u32, Peer>;

pub struct Peer {
    peer_id: u32,
    pub inbound: Receiver<Packet>,
    pub inbound_sender: Sender<Packet>,
    pub outbound: Receiver<Packet>,
    pub outbound_sender: Sender<Packet>,
}

impl Peer {
    pub fn new(peer_id: u32) -> Self {
        let (inbound_sender, inbound) = channel(100);
        let (outbound_sender, outbound) = channel(100);
        Peer {
            peer_id,
            inbound,
            inbound_sender,
            outbound,
            outbound_sender,
        }
    }
}
