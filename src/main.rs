use bitcoin_prac::ecc::{S256Point, S256Field, N, G, Signiture, hash256, PrivateKey};
use bigint::uint::{U256, U512};

fn main() {

    // let z = U256::from_big_endian(&[0xbc, 0x62, 0xd4, 0xb8, 0x0d, 0x9e, 0x36, 0xda, 0x29, 0xc1, 0x6c, 0x5d, 0x4d, 0x9f, 0x11, 0x73, 0x1f, 0x36, 0x05, 0x2c, 0x72, 0x40, 0x1a, 0x76, 0xc2, 0x3c, 0x0f, 0xb5, 0xa9, 0xb7, 0x44, 0x23]);
    // let r = U256::from_big_endian(&[0x37, 0x20, 0x6a, 0x06, 0x10, 0x99, 0x5c, 0x58, 0x07, 0x49, 0x99, 0xcb, 0x97, 0x67, 0xb8, 0x7a, 0xf4, 0xc4, 0x97, 0x8d, 0xb6, 0x8c, 0x06, 0xe8, 0xe6, 0xe8, 0x1d, 0x28, 0x20, 0x47, 0xa7, 0xc6]);
    // let s = U256::from_big_endian(&[0x8c, 0xa6, 0x37, 0x59, 0xc1, 0x15, 0x7e, 0xbe, 0xae, 0xc0, 0xd0, 0x3c, 0xec, 0xca, 0x11, 0x9f, 0xc9, 0xa7, 0x5b, 0xf8, 0xe6, 0xd0, 0xfa, 0x65, 0xc8, 0x41, 0xc8, 0xe2, 0x73, 0x8c, 0xda, 0xec]);
    // let px = U256::from_big_endian(&[0x04, 0x51, 0x9f, 0xac, 0x3d, 0x91, 0x0c, 0xa7, 0xe7, 0x13, 0x8f, 0x70, 0x13, 0x70, 0x6f, 0x61, 0x9f, 0xa8, 0xf0, 0x33, 0xe6, 0xec, 0x6e, 0x09, 0x37, 0x0e, 0xa3, 0x8c, 0xee, 0x6a, 0x75, 0x74]);
    // let py = U256::from_big_endian(&[0x82, 0xb5, 0x1e, 0xab, 0x8c, 0x27, 0xc6, 0x6e, 0x26, 0xc8, 0x58, 0xa0, 0x79, 0xbc, 0xdf, 0x4f, 0x1a, 0xda, 0x34, 0xce, 0xc4, 0x20, 0xca, 0xfc, 0x7e, 0xac, 0x1a, 0x42, 0x21, 0x6f, 0xb6, 0xc4]);

    // let p = S256Point::new(S256Field::new(px), S256Field::new(py));
    // let s_inv = _u512_to_u256(_pow_with_mod(_u256_to_u512(s), _u256_to_u512(N()-U256::from_dec_str("2").unwrap()), _u256_to_u512(N())));
    // let u = _u512_to_u256(_u256_to_u512(z) * _u256_to_u512(s_inv) % _u256_to_u512(N()));
    // let v = _u512_to_u256(_u256_to_u512(r) * _u256_to_u512(s_inv) % _u256_to_u512(N()));
    // println!("{}", (&G().scalar_mul(u)+&p.scalar_mul(v)).x().num==r);
    // let a = U256::from_big_endian(&[3]);
    // println!("{}", a);

    // println!("{}", "Hello world");
    // let mut hasher = Sha256::new();
    // hasher.update(b"Hello world");
    // let res = &hasher.finalize()[..];
    // println!("{:?}", res);
    // let mut ss = String::from("");
    // for i in res {
    //     ss = ss + &u8_to_hex(*i);
    // }
    // println!("{}", ss);
    // let mut hasher = Sha256::new();
    // hasher.update(ss);
    // let res = &hasher.finalize()[..];
    // println!("{:?}", res);
    
    let e = hash256(String::from("genius_jaeyeop"));
    let z = hash256(String::from("gangcheon"));
    // let k = U256::from_dec_str("123456789").unwrap();
    // let r = _u256_to_u512(G().scalar_mul(k).x().num);
    // let k_inv = _pow_with_mod(_u256_to_u512(k), _u256_to_u512(N()-U256::from_big_endian(&[2])), _u256_to_u512(N()));
    // let s = ((z% _u256_to_u512(N())+(r*e)% _u256_to_u512(N()))% _u256_to_u512(N()) * k_inv % _u256_to_u512(N())) % _u256_to_u512(N());
    // let point = G().scalar_mul(_u512_to_u256(e));
    // println!("{:?}", point);
    // println!("r: {:?}", r);
    // println!("s: {:?}", s);
    let p = PrivateKey::new(e);
    println!("{:?}", p.sign(z));
    
}

fn _u512_to_u256(num: U512) -> U256 {
    let mut num_endian: [u8; 64] = [0; 64];
    num.to_big_endian(&mut num_endian);
    U256::from_big_endian(&num_endian[32..64])
}

fn _u256_to_u512(num: U256) -> U512 {
    let mut num_endian: [u8; 32] = [0; 32];
    num.to_big_endian(&mut num_endian);
    U512::from_big_endian(&num_endian)
}

fn _pow_with_mod(num: U512, exp: U512, mod_num: U512) -> U512 {
    let one = U512::from_dec_str("1").unwrap();
    let two = U512::from_dec_str("2").unwrap();

    if exp == one {
        return num % mod_num;
    }
    let temp = _pow_with_mod(num, exp/two, mod_num);
    if exp % two == one {
        return (((temp*temp)%mod_num)*num)%mod_num;
    }
    return (temp*temp)%mod_num;
}
