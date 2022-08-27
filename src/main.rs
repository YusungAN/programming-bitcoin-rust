use bitcoin_prac::ecc::{G, hash256, PrivateKey};
use bigint::uint::{U256, U512};

fn main() {
    let e = hash256(String::from("wowowow")); // 비밀키, 실제로는 이렇게 만들만 안됩니다.
    let z = hash256(String::from("Igeo neomu himdeureo")); // 서명 해시, 그냥 담고 싶은 메세지.

    let p = PrivateKey::new(e);
    let point = G().scalar_mul(e);
    let sig = p.sign(z);
    println!("{:?}", sig);
    println!("{:?}", point.verify(z, &sig));
    
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
