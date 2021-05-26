use crate::field_element::{FieldElement, Prime};
use crate::point::{Curve, CurvePoint};
use anyhow::Result;
use num_bigint::{BigInt, Sign};
use num_traits::{ToPrimitive, Pow};
use once_cell::sync::Lazy;
use rand::seq::IteratorRandom;
use std::ops;

static A: Lazy<BigInt> = Lazy::new(|| BigInt::from(0));
static B: Lazy<BigInt> = Lazy::new(|| BigInt::from(7));
static P: Lazy<Prime> =
    Lazy::new(|| Prime::new(BigInt::from(2).pow(256_u16) - BigInt::from(2).pow(32_u8) - 977));
static C: Lazy<Curve<S256Field>> = Lazy::new(|| {
    Curve::new(
        P.field_element(A.clone()).into(),
        P.field_element(B.clone()).into(),
    )
});
static N: Lazy<BigInt> = Lazy::new(|| {
    BigInt::parse_bytes(
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap()
});
static G: Lazy<S256Point> = Lazy::new(|| {
    S256Point::new(
        BigInt::parse_bytes(
            b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            16,
        )
        .unwrap(),
        BigInt::parse_bytes(
            b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
            16,
        )
        .unwrap(),
    )
    .unwrap()
});

static BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

fn encode_base58(bytes: &[u8]) -> String {
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
        result = format!("{}{}", BASE58_ALPHABET.chars().nth(mod_num.into()).unwrap(), result);
    }
    format!("{}{}", prefix, result)
}

#[derive(Debug, PartialEq, Clone)]
pub struct S256Field<'a> {
    pub inner: FieldElement<'a>,
}

impl<'a> S256Field<'a> {
    fn new<N: Into<BigInt>>(n: N) -> Self {
        P.field_element(n).into()
    }

    fn sqrt(&self) -> S256Field<'a> {
        self.pow(&((&(*P).0 + 1) / 4)).into()
    }
}

impl<'a> ops::Deref for S256Field<'a> {
    type Target = FieldElement<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> AsRef<FieldElement<'a>> for S256Field<'a> {
    fn as_ref(&self) -> &FieldElement<'a> {
        &self.inner
    }
}

impl<'a> From<FieldElement<'a>> for S256Field<'a> {
    fn from(inner: FieldElement<'a>) -> S256Field<'a> {
        Self { inner }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct S256Point<'a> {
    pub cp: CurvePoint<'a, S256Field<'a>>,
}

impl<'a> S256Point<'a> {
    pub fn parse(sec_bin: &[u8]) -> Result<S256Point<'a>> {
        if sec_bin[0] == 4 {
            let x = BigInt::from_bytes_be(Sign::Plus, &sec_bin[1..33]);
            let y = BigInt::from_bytes_be(Sign::Plus, &sec_bin[33..65]);
            return S256Point::new(x, y);
        }
        let is_even = sec_bin[0] == 2;
        let x = S256Field::new(BigInt::from_bytes_be(Sign::Plus, &sec_bin[1..]));
        let alpha: S256Field =
            (&S256Field::from(x.pow(&3.into())).inner + &S256Field::new(B.clone()).inner)?.into();
        let beta = alpha.sqrt();
        let (even_beta, odd_beta) = if &beta.num % 2 == BigInt::from(0) {
            (beta.clone(), S256Field::new(&(*P).0 - &beta.num))
        } else {
            (S256Field::new(&(*P).0 - &beta.num), beta)
        };

        if is_even {
            return S256Point::new(x.inner.num, even_beta.inner.num);
        }
        S256Point::new(x.inner.num, odd_beta.inner.num)
    }

    pub fn new<A, B>(x: A, y: B) -> Result<S256Point<'a>>
    where
        A: Into<BigInt>,
        B: Into<BigInt>,
    {
        C.point(x, y).map(|cp| cp.into())
    }

    pub fn inf() -> S256Point<'a> {
        C.inf().into()
    }

    pub fn verify(&self, z: BigInt, sig: Signature) -> bool {
        let s_inv = sig.s.modpow(&(&*N - 2), &*N);
        let u = z * &s_inv % &*N;
        let v = &sig.r * s_inv % &*N;
        let total = (&*(u * G.clone()).unwrap() + &*(v * self.clone()).unwrap()).unwrap();
        let r = &total.p.as_actual().x.num;
        r == &sig.r
    }

    pub fn sec(&self, compressed: bool) -> Vec<u8> {
        if compressed {
            if &self.cp.p.as_actual().y.num % 2 == BigInt::from(0) {
                let mut result = vec![0x02];
                result.append(&mut self.cp.p.as_actual().x.num.to_bytes_be().1);
                return result;
            }
            let mut result = vec![0x03];
            result.append(&mut self.cp.p.as_actual().x.num.to_bytes_be().1);
            return result;
        }

        let mut result = vec![0x04];
        result.append(&mut self.cp.p.as_actual().x.num.to_bytes_be().1);
        result.append(&mut self.cp.p.as_actual().y.num.to_bytes_be().1);

        result
    }
}

impl<'a> ops::Deref for S256Point<'a> {
    type Target = CurvePoint<'a, S256Field<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.cp
    }
}

impl<'a> From<CurvePoint<'a, S256Field<'a>>> for S256Point<'a> {
    fn from(cp: CurvePoint<'a, S256Field<'a>>) -> S256Point<'a> {
        Self { cp }
    }
}

impl<'a> ops::Mul<S256Point<'a>> for BigInt {
    type Output = Result<S256Point<'a>>;

    fn mul(self, other: S256Point<'a>) -> Result<S256Point<'a>> {
        let coef = self % &*N;
        coef.mul(other.cp).map(|cp| cp.into())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Signature {
    r: BigInt,
    s: BigInt,
}

impl Signature {
    fn new(r: BigInt, s: BigInt) -> Self {
        Self { r, s }
    }

    fn der(&self) -> Vec<u8> {
        let rbin = self.r.to_bytes_be().1;
        let mut rbin: Vec<u8> = rbin.into_iter().skip_while(|b| b == &0x00).collect();
        if rbin[0] & 0x80 > 0 {
            rbin.insert(0, 0x00);
        }

        let mut result = vec![0x02, rbin.len() as u8];
        result.append(&mut rbin);

        let sbin = self.s.to_bytes_be().1;
        let mut sbin: Vec<u8> = sbin.into_iter().skip_while(|b| b == &0x00).collect();
        if sbin[0] & 0x80 > 0 {
            sbin.insert(0, 0x00);
        }

        result.append(&mut vec![0x02, sbin.len() as u8]);
        result.append(&mut sbin);

        let mut b = vec![0x30, result.len() as u8];
        b.append(&mut result);
        b
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrivateKey<'a> {
    pub secret: BigInt,
    pub point: S256Point<'a>,
}

impl<'a> PrivateKey<'a> {
    fn new(secret: BigInt) -> Self {
        let point = (secret.clone() * G.clone()).unwrap();
        Self { secret, point }
    }

    pub fn sign(&self, z: BigInt) -> Signature {
        let mut rng = rand::thread_rng();

        let n = &*N;
        let range = num_iter::range(BigInt::from(0), n.clone());
        let k = range.choose(&mut rng).unwrap();
        let r = (k.clone() * G.clone())
            .unwrap()
            .cp
            .p
            .into_actual()
            .x
            .num
            .clone();
        let k_inv = k.modpow(&(n - 2), &n);
        let mut s = (z + &r * &self.secret) * k_inv % n;
        if s > n / 2 {
            s = n - s;
        }
        Signature::new(r, s)
    }
}

#[test]
fn test_3_9() {
    assert_eq!((N.clone() * G.clone()).unwrap(), C.inf().into());
}

#[test]
fn test_3_11_3() {
    let z = BigInt::parse_bytes(
        b"bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423",
        16,
    )
    .unwrap();
    let r = BigInt::parse_bytes(
        b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
        16,
    )
    .unwrap();
    let s = BigInt::parse_bytes(
        b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
        16,
    )
    .unwrap();
    let px = BigInt::parse_bytes(
        b"04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574",
        16,
    )
    .unwrap();
    let py = BigInt::parse_bytes(
        b"82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4",
        16,
    )
    .unwrap();

    let point = S256Point::new(px, py).unwrap();
    let s_inv = s.modpow(&(&*N - 2), &*N);
    let u = z * &s_inv % &*N;
    let v = &r * s_inv % &*N;
    assert_eq!(
        (&*(u * G.clone()).unwrap() + &*(v * point).unwrap())
            .unwrap()
            .p
            .as_actual()
            .x
            .num,
        r
    );
}

#[test]
fn test_3_11_4() {
    let z = BigInt::parse_bytes(
        b"bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423",
        16,
    )
    .unwrap();
    let r = BigInt::parse_bytes(
        b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
        16,
    )
    .unwrap();
    let s = BigInt::parse_bytes(
        b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
        16,
    )
    .unwrap();
    let px = BigInt::parse_bytes(
        b"04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574",
        16,
    )
    .unwrap();
    let py = BigInt::parse_bytes(
        b"82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4",
        16,
    )
    .unwrap();

    let sig = Signature::new(r, s);
    let point = S256Point::new(px, py).unwrap();
    assert!(point.verify(z, sig));
}

#[test]
fn test_exam_4_1() {
    let key = PrivateKey::new(BigInt::from(5000));
    assert_eq!(
        key.point.sec(false).iter().map(|n| format!("{:02x}", n)).collect::<String>(),
        "04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10".to_string());

    let p = S256Point::parse(
        &BigInt::parse_bytes(
            b"04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10",
            16
        ).unwrap().to_bytes_be().1
    ).unwrap();
    assert_eq!(p, key.point);
}

#[test]
fn test_exam_4_2() {
    let key = PrivateKey::new(BigInt::from(5001));
    assert_eq!(
        key.point
            .sec(true)
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>(),
        "0357a4f368868a8a6d572991e484e664810ff14c05c0fa023275251151fe0e53d1".to_string()
    );

    let p = S256Point::parse(
        &BigInt::parse_bytes(
            b"0357a4f368868a8a6d572991e484e664810ff14c05c0fa023275251151fe0e53d1",
            16
        ).unwrap().to_bytes_be().1
    ).unwrap();
    assert_eq!(p, key.point);
}

#[test]
fn test_exam_4_3() {
    let r = BigInt::parse_bytes(
        b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
        16,
    )
    .unwrap();
    let s = BigInt::parse_bytes(
        b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
        16,
    )
    .unwrap();

    let sig = Signature::new(r, s);
    let der = sig.der();
    let hex = der.iter().map(|n| format!("{:02x}", n)).collect::<String>();
    assert_eq!(hex, "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec".to_string());
}

#[test]
fn test_exam_4_4() {
    let b = BigInt::parse_bytes(
        b"7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
        16,
    )
    .unwrap().to_bytes_be().1;

    let base58 = encode_base58(&b);

    assert_eq!(base58, "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6".to_string());
}
