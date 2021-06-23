use crate::helper::hash256;
use std::io::Read;

#[derive(Debug, PartialEq, Clone)]
pub struct Tx {
    version: u32,
    tx_ins,
    tx_outs,
    locktime,
    testnet: bool,
}

impl Tx {
    fn parse<R: Read>(reader: R) -> Self where R: Read {
        let mut version: [u8;4];
        reader.read(&mut version);
        Self {
            version: u32::from_le_bytes(version),
            tx_ins,
            tx_outs,
            locktime,
            testnet,
        }
    }

    fn id(&self) -> String {
        self.hash().iter().map(|n| format!("{:02x}", n)).collect::<String>()
    }

    fn serialize(&self) -> Vec<u8> {
        unimplemented!();
        Vec::new()
    }

    fn hash(&self) -> Vec<u8> {
        hash256(&self.serialize())
    }
}

#[derive(Debug, PartialEq, Clone)]
struct TxIn {
    prev_tx,
    prev_index,
    script_sig: Script,
    sequence,
}

impl TxIn {
    pub fn new(prev_tx, prev_index, script_sig: Option<Script>, sequence) -> Self {
        Self {
            prev_tx,
            prev_index,
            script_sig: script_sig.unwrap_or(Script::new()),
            sequence
        }
    }
}

struct Script;


impl Script {
    pub fn new() -> Self {
        Self
    }
}


