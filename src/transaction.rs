use crate::helper::{hash256, read_variant, encode_variant};
use num_bigint::{BigInt, Sign};
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Tx {
    version: u32,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
    locktime: u32,
    testnet: bool,
}

impl Tx {
    fn parse<R: Read>(reader: &mut R, testnet: bool) -> Self
    where
        R: Read,
    {
        let mut version = [0u8; 4];
        reader.read_exact(&mut version).unwrap();

        let input_len = read_variant(reader);
        let mut tx_ins = Vec::new();
        for _ in 0..input_len {
            tx_ins.push(TxIn::parse(reader));
        }

        let output_len = read_variant(reader);
        let mut tx_outs = Vec::new();
        for _ in 0..output_len {
            tx_outs.push(TxOut::parse(reader));
        }

        let mut locktime = [0u8; 4];
        reader.read_exact(&mut locktime).unwrap();
        Self {
            version: u32::from_le_bytes(version),
            tx_ins,
            tx_outs,
            locktime: u32::from_le_bytes(locktime),
            testnet,
        }
    }

    fn id(&self) -> String {
        self.hash()
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = self.version.to_le_bytes().to_vec();

        result.append(&mut encode_variant(self.tx_ins.len() as u64));
        for tx_in in &self.tx_ins {
            result.append(&mut tx_in.serialize());
        }

        result.append(&mut encode_variant(self.tx_outs.len() as u64));
        for tx_out in &self.tx_outs {
            result.append(&mut tx_out.serialize());
        }

        result.append(&mut self.locktime.to_le_bytes().to_vec());
        result
    }

    fn hash(&self) -> Vec<u8> {
        hash256(&self.serialize())
    }
}

#[derive(Debug, Clone)]
struct TxIn {
    prev_tx: [u8; 32],
    prev_index: u32,
    script_sig: Script,
    sequence: u32,
}

impl TxIn {
    pub fn new(
        prev_tx: [u8; 32],
        prev_index: u32,
        script_sig: Option<Script>,
        sequence: u32,
    ) -> Self {
        Self {
            prev_tx,
            prev_index,
            script_sig: script_sig.unwrap_or(Script::new()),
            sequence,
        }
    }

    pub fn parse<R>(reader: &mut R) -> Self
    where
        R: Read,
    {
        let mut prev_tx = [0u8; 32];
        reader.read_exact(&mut prev_tx).unwrap();

        let mut prev_index = [0u8; 4];
        reader.read_exact(&mut prev_index).unwrap();

        let script_sig = Some(Script::parse(reader));

        let mut sequence = [0u8; 4];
        reader.read_exact(&mut sequence).unwrap();
        Self::new(
            prev_tx,
            u32::from_le_bytes(prev_index),
            script_sig,
            u32::from_le_bytes(sequence),
        )
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = self.prev_tx.to_vec();
        result.append(&mut self.prev_index.to_le_bytes().to_vec());
        result.append(&mut self.script_sig.serialize());
        result.append(&mut self.sequence.to_le_bytes().to_vec());
        result
    }
}

#[derive(Debug, Clone)]
struct TxOut {
    amount: u32,
    script_pubkey: ScriptPubKey,
}

impl TxOut {
    pub fn new(amount: u32, script_pubkey: ScriptPubKey) -> Self {
        Self {
            amount,
            script_pubkey,
        }
    }

    pub fn parse<R>(reader: &mut R) -> Self
    where
        R: Read,
    {
        let mut amount = [0u8; 4];
        reader.read_exact(&mut amount).unwrap();
        let script_pubkey = ScriptPubKey::parse(reader);
        Self::new(u32::from_le_bytes(amount), script_pubkey)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = self.amount.to_le_bytes().to_vec();
        result.append(&mut self.script_pubkey.serialize());
        result
    }
}

#[derive(Debug, Clone)]
struct Script;

impl Script {
    pub fn new() -> Self {
        Self
    }

    pub fn parse<R>(reader: &mut R) -> Self
    where
        R: Read,
    {
        Script::new()
    }

    pub fn serialize(&self) -> Vec<u8> {
        vec![]
    }
}

#[derive(Debug, Clone)]
struct ScriptPubKey;

impl ScriptPubKey {
    pub fn new() -> Self {
        Self
    }

    pub fn parse<R>(reader: &mut R) -> Self
    where
        R: Read,
    {
        ScriptPubKey::new()
    }

    pub fn serialize(&self) -> Vec<u8> {
        vec![]
    }
}
