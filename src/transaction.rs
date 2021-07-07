use crate::helper::{hash256, read_variant};
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

    fn serialize(&self) -> Vec<u8> {
        unimplemented!();
    }

    fn hash(&self) -> Vec<u8> {
        hash256(&self.serialize())
    }
}

#[derive(Debug, Clone)]
struct TxIn {
    prev_tx: BigInt,
    prev_index: u32,
    script_sig: Script,
    sequence: u32,
}

impl TxIn {
    pub fn new(
        prev_tx: BigInt,
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
            BigInt::from_bytes_le(Sign::Plus, &prev_tx),
            u32::from_le_bytes(prev_index),
            script_sig,
            u32::from_le_bytes(sequence),
        )
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
}
