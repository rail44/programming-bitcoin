use digest::Digest;
use num_bigint::{BigInt, Sign};
use num_traits::ToPrimitive;
use ripemd160::Ripemd160;
use sha2::Sha256;

static BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn encode_base58(bytes: &[u8]) -> String {
    let mut count = 0;
    for b in bytes {
        if b == &0 {
            count += 1;
            continue;
        }
        break;
    }

    let mut num = BigInt::from_bytes_be(Sign::Plus, bytes);
    let prefix = "1".repeat(count);
    let mut result = String::new();
    while num > 0.into() {
        let mod_num = (num.clone() % BigInt::from(58)).to_u8().unwrap();
        num = num / 58;
        result = format!(
            "{}{}",
            BASE58_ALPHABET.chars().nth(mod_num.into()).unwrap(),
            result
        );
    }
    format!("{}{}", prefix, result)
}

fn hash256(b: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(b);
    let once = hasher.finalize();
    let mut hasher = Sha256::new();
    hasher.update(once);
    hasher.finalize().to_vec()
}

pub fn encode_base58_checksum(b: &[u8]) -> String {
    encode_base58(&[b, &hash256(b)[..4]].concat())
}

pub fn hash160(s: &[u8]) -> Vec<u8> {
    let mut sha_hasher = Sha256::new();
    let mut ripemd_hasher = Ripemd160::new();
    sha_hasher.update(s);
    ripemd_hasher.update(sha_hasher.finalize());
    ripemd_hasher.finalize().to_vec()
}

#[test]
fn test_exam_4_4() {
    let b = BigInt::parse_bytes(
        b"7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
        16,
    )
    .unwrap()
    .to_bytes_be()
    .1;

    let base58 = encode_base58(&b);

    assert_eq!(
        base58,
        "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6".to_string()
    );
}
