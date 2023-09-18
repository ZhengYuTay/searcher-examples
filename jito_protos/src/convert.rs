use std::cmp::min;

use bincode::serialize;
use solana_perf::packet::{Packet, PACKET_DATA_SIZE};
use solana_sdk::transaction::VersionedTransaction;

use crate::packet::{Meta as ProtoMeta, Packet as ProtoPacket};

/// Converts a protobuf packet to a VersionedTransaction
pub fn versioned_tx_from_packet(p: &ProtoPacket) -> Option<VersionedTransaction> {
    let mut data = [0; PACKET_DATA_SIZE];
    let copy_len = min(data.len(), p.data.len());
    data[..copy_len].copy_from_slice(&p.data[..copy_len]);
    let mut packet = Packet::new(data, Default::default());
    if let Some(meta) = &p.meta {
        packet.meta_mut().size = meta.size as usize;
    }
    packet.deserialize_slice(..).ok()
}

/// Converts a VersionedTransaction to a protobuf packet
pub fn proto_packet_from_versioned_tx(tx: &VersionedTransaction) -> ProtoPacket {
    let data = serialize(tx).expect("serializes");
    let size = data.len() as u64;
    ProtoPacket {
        data,
        meta: Some(ProtoMeta {
            size,
            addr: "".to_string(),
            port: 0,
            flags: None,
            sender_stake: 0,
        }),
    }
}

#[cfg(test)]
mod tests {
    use solana_perf::test_tx::test_tx;
    use solana_sdk::transaction::VersionedTransaction;

    use crate::convert::{proto_packet_from_versioned_tx, versioned_tx_from_packet};

    #[test]
    fn test_proto_to_packet() {
        let tx_before = VersionedTransaction::from(test_tx());
        let tx_after = versioned_tx_from_packet(&proto_packet_from_versioned_tx(&tx_before))
            .expect("tx_after");

        assert_eq!(tx_before, tx_after);
    }
}
