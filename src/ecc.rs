use std::ops::{Add, Sub, Mul, Div};
use bigint::uint::{U256, U512};
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use rand::Rng;

pub fn N() -> U256 {
    U256::from_big_endian(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x41])
}

pub fn G() -> S256Point {
    let gx = U256::from_big_endian(&[0x79, 0xBE, 0x66, 0x7E, 0xF9, 0xDC, 0xBB, 0xAC, 0x55, 0xA0, 0x62, 0x95, 0xCE, 0x87, 0x0B, 0x07, 0x02, 0x9B, 0xFC, 0xDB, 0x2D, 0xCE, 0x28, 0xD9, 0x59, 0xF2, 0x81, 0x5B, 0x16, 0xF8, 0x17, 0x98]);
    let gy = U256::from_big_endian(&[0x48, 0x3A, 0xDA, 0x77, 0x26, 0xA3, 0xC4, 0x65, 0x5D, 0xA4, 0xFB, 0xFC, 0x0E, 0x11, 0x08, 0xA8, 0xFD, 0x17, 0xB4, 0x48, 0xA6, 0x85, 0x54, 0x19, 0x9C, 0x47, 0xD0, 0x8F, 0xFB, 0x10, 0xD4, 0xB8]);
    S256Point::new(S256Field::new(gx), S256Field::new(gy))
}

#[derive(PartialEq, Debug)]
pub struct S256Field {
    pub num: U256,
    pub prime: U256
}

fn secp256k1_p() -> U256 {
    U256::from_big_endian(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0xff, 0xff, 0xfc, 0x2f]) 
}

impl S256Field {
    pub fn new(num: U256) -> Self {
        let one = U256::one();
        let prime = secp256k1_p();
        if num >= prime {
            panic!("Num {} not in  field range to {}", num, prime-one)
        }

        S256Field {
            num: num,
            prime: prime
        }
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }

    pub fn ne(&self, other: &Self) -> bool {
        self.num != other.num || self.prime != other.prime
    }

    pub fn pow(&self, exp: isize) -> Self {
        let one = U256::from_dec_str("1").unwrap();
        if exp >= 0 { 
            let exp_temp = U256::from_dec_str(&(exp).to_string()).unwrap();
            let num = _u512_to_u256(_pow_with_mod(_u256_to_u512(self.num), _u256_to_u512(exp_temp), _u256_to_u512(self.prime)));

            return Self {  
                num: num,
                prime: self.prime
            };
        } else {
            let mut exp_temp = U256::from_dec_str(&(-1*exp).to_string()).unwrap();
            exp_temp = self.prime-one-(exp_temp%(self.prime-one));

            let num = _u512_to_u256(_pow_with_mod(_u256_to_u512(self.num), _u256_to_u512(exp_temp), _u256_to_u512(self.prime)));

            return Self {  
                num: num,
                prime: self.prime
            };
        }
    }
}

impl<'a, 'b> Add<&'b S256Field> for &'a S256Field {
    type Output = S256Field;

    fn add(self, other: &'b S256Field) -> S256Field {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num_temp = _u256_to_u512(self.num);
        let onum_temp = _u256_to_u512(other.num);
        let num = (num_temp + onum_temp) % _u256_to_u512(self.prime);
        S256Field {
            num: _u512_to_u256(num),
            prime: self.prime
        }
    }
}

impl<'a, 'b> Sub<&'b S256Field> for &'a S256Field {
    type Output = S256Field;

    fn sub(self, other: &'b S256Field) -> S256Field {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        if self.num >= other.num {
            let num = (self.num - other.num) % self.prime;

            S256Field {
                num: num,
                prime: self.prime
            }
        } else {
            let num = self.prime - ((other.num - self.num) % self.prime);

            S256Field {
                num: num,
                prime: self.prime
            }
        }
        
    }
}

impl<'a, 'b> Mul<&'b S256Field> for &'a S256Field {
    type Output = S256Field;

    fn mul(self, other: &'b S256Field) -> S256Field {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num_temp = _u256_to_u512(self.num);
        let onum_temp = _u256_to_u512(other.num);
        let num = _u512_to_u256((num_temp * onum_temp) % _u256_to_u512(self.prime));

        S256Field {
            num: num,
            prime: self.prime
        }
    }
}


fn _u256_to_u512(num: U256) -> U512 {
    let mut num_endian: [u8; 32] = [0; 32];
    num.to_big_endian(&mut num_endian);
    U512::from_big_endian(&num_endian)
}

fn _u512_to_u256(num: U512) -> U256 {
    let mut num_endian: [u8; 64] = [0; 64];
    num.to_big_endian(&mut num_endian);
    U256::from_big_endian(&num_endian[32..64])
}

fn _pow_with_mod(num: U512, exp: U512, mod_num: U512) -> U512 {
    let one = U512::one();
    let two = U512::from_big_endian(&[2]);

    if exp == one {
        return num % mod_num;
    }
    let temp = _pow_with_mod(num, exp/two, mod_num);
    if exp % two == one {
        return (((temp*temp)%mod_num)*num)%mod_num;
    }
    return (temp*temp)%mod_num;
}

fn u8_to_hex(n: u8) -> String {
    let mut table = HashMap::new();
    let mut nn = n;
    table.insert(0, "0");
    table.insert(1, "1");
    table.insert(2, "2");
    table.insert(3, "3");
    table.insert(4, "4");
    table.insert(5, "5");
    table.insert(6, "6");
    table.insert(7, "7");
    table.insert(8, "8");
    table.insert(9, "9");
    table.insert(10, "A");
    table.insert(11, "B");
    table.insert(12, "C");
    table.insert(13, "D");
    table.insert(14, "E");
    table.insert(15, "F");

    let mut s = String::from("");
    while nn > 0 {
        s = s + table.get(&(nn%16)).unwrap();
        nn = nn / 16;
    }

    return s.chars().rev().collect::<String>();
}

pub fn hash256(s: String) -> U256 {
    let mut hasher = Sha256::new();
    hasher.update(s);
    let res = &hasher.finalize()[..];
    let mut ss = String::from("");
    for i in res {
        ss = ss + &u8_to_hex(*i);
    }
    let mut hasher = Sha256::new();
    hasher.update(ss);
    let res = &hasher.finalize()[..];
    U256::from_big_endian(res)
}

impl<'a, 'b> Div<&'b S256Field> for &'a S256Field {
    type Output = S256Field;

    fn div(self, other: &'b S256Field) -> S256Field {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let two = U256::from_big_endian(&[2]);
        //let num = self.num * (other.num.pow((self.prime-2) as u32) % self.prime) % self.prime;
        let num = (_u256_to_u512(self.num) * _pow_with_mod(_u256_to_u512(other.num), _u256_to_u512(self.prime-two), _u256_to_u512(self.prime))) % _u256_to_u512(self.prime);

        S256Field {
            num: _u512_to_u256(num),
            prime: self.prime
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct S256Point {
    x: Option<S256Field>,
    y: Option<S256Field>,
    pub a: S256Field,
    pub b: S256Field
}

impl S256Point {
    pub fn new(x: S256Field, y: S256Field) -> Self {  
        let a = S256Field::new(U256::zero());
        let b = S256Field::new(U256::from_big_endian(&[7]));  
        if y.pow(2) != &(&x.pow(3) + &(&a*&x)) + &b {
            panic!("({:?}, {:?}) is not on the curve", x, y);
        }
        Self { a: a, b: b, x: Some(x), y: Some(y) }
    }

    pub fn new_inf(a: S256Field, b: S256Field) -> Self {
        Self { a: a, b: b, x: None, y: None }
    }

    pub fn x(&self) -> &S256Field {
        self.x.as_ref().unwrap()
    }

    pub fn y(&self) -> &S256Field {
        self.y.as_ref().unwrap()
    }

    pub fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }

    pub fn ne(&self, other: &Self) -> bool {
        return self.x != other.x || self.y != other.y || self.a != other.a || self.b != other.b
    }

    
    pub fn scalar_mul(&self, coefficient: U256) -> Self {
        println!("calculating scalar multiplication...");
        let n = N();
        let mut coef = coefficient % n;
        let a = S256Field::new(self.a.num);
        let b = S256Field::new(self.b.num);
        let x = S256Field::new(self.x().num);
        let y = S256Field::new(self.y().num);
        let mut currrent = S256Point { x: Some(x), y: Some(y), a: a, b: b };

        let a = S256Field::new(self.a.num);
        let b = S256Field::new(self.b.num);
        let mut result = S256Point::new_inf(a, b);
        while coef > U256::zero() {
            if coef & U256::one() == U256::one() {
                result = &result + &currrent;
            }
            currrent = &currrent + &currrent;
            coef = coef >> 1;
        }

        return result;
    }

    pub fn verify(&self, z: U256, sig: &Signiture) -> bool {
        let s_inv = _u512_to_u256(_pow_with_mod(_u256_to_u512(sig.s), _u256_to_u512(N()-U256::from_dec_str("2").unwrap()), _u256_to_u512(N())));
        let u = _u512_to_u256(_u256_to_u512(z) * _u256_to_u512(s_inv) % _u256_to_u512(N()));
        let v = _u512_to_u256(_u256_to_u512(sig.r) * _u256_to_u512(s_inv) % _u256_to_u512(N()));
        let self_obj = S256Point::new(S256Field::new(self.x().num), S256Field::new(self.y().num));
        let total = &G().scalar_mul(u)+&self_obj.scalar_mul(v);
        total.x().num == sig.r
    }
}

impl<'a, 'b> Add<&'b S256Point> for &'a S256Point {
    type Output = S256Point;

    fn add(self, other: &'b S256Point) -> S256Point {
        if self.a != other.a || self.b != other.b {
            panic!("Points are not in same curve.");
        }

        if self.x.is_none() {
            return S256Point {
                x: Some(S256Field::new(other.x().num)),
                y: Some(S256Field::new(other.y().num)),
                a: S256Field::new(other.a.num),
                b: S256Field::new(other.b.num)
            };
        }

        if other.x.is_none() {
            return S256Point {
                x: Some(S256Field::new(self.x().num)),
                y: Some(S256Field::new(self.y().num)),
                a: S256Field::new(self.a.num),
                b: S256Field::new(self.b.num)
            };
        }

        if self.x() == other.x() && self.y() != other.y() {
            return S256Point {
                x: None,
                y: None,
                a: S256Field::new(self.a.num),
                b: S256Field::new(self.b.num)
            };
        }

        if self.x() != other.x() {
            let s = &(other.y() - self.y()) / &(other.x() - self.x());
            let x3 = &(&(s.pow(2))-self.x())-other.x();
            let x3_2 = &(&(s.pow(2))-self.x())-other.x();
            //println!("{:?} {:?}", other.y() - self.y(), other.x() - self.x());
            return S256Point {
                x: Some(x3),
                y: Some(&(&s*&(self.x()-&x3_2))-self.y()),
                a: S256Field::new(self.a.num),
                b: S256Field::new(self.b.num)
            };
        }

        if self == other && other.y() != &(S256Field::new(U256::zero())) {
            let three = S256Field::new(U256::from_big_endian(&[3]));
            let two = S256Field::new(U256::from_big_endian(&[2]));
            let s = &(&(&three*&self.x().pow(2))+&self.a) / &(&two*self.y());
            let x3 = &(s.pow(2))-&(&two*self.x());
            let x3_2 = &(s.pow(2))-&(&two*self.x());
            return S256Point {
                x: Some(x3),
                y: Some(&(&s*&(self.x()-&x3_2))-self.y()),
                a: S256Field::new(self.a.num),
                b: S256Field::new(self.b.num)
            };
        }

        if self == other && self.y() == &(S256Field::new(U256::zero())) {
            return S256Point {
                x: None,
                y: None,
                a: S256Field::new(self.a.num),
                b: S256Field::new(self.b.num)
            };
        }

        // 별 의미 없음
        return S256Point {
            x: None,
            y: None,
            a: S256Field::new(self.a.num),
            b: S256Field::new(self.b.num)
        };

    }
}

#[derive(Debug)]
pub struct Signiture {
    r: U256,
    s: U256
}


impl Signiture {
    pub fn new(r: U256, s: U256) -> Self {
        Signiture { r: r, s: s }
    }

}

pub struct PrivateKey {
    secret: U256,
    point: S256Point
}

impl PrivateKey {
    pub fn new(secret: U256) -> Self {
        Self {
            secret: secret,
            point: G().scalar_mul(secret)
        }
    }

    fn gen_rand_k() -> U256 {
        let arr: Vec<u8> = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x41];
        let mut res: [u8; 32] = [0; 32];
        let mut rng = rand::thread_rng();

        for (idx, el) in arr.iter().enumerate() {
            let tmp = rng.gen_range(0..=*el);
            res[idx] = tmp;
        }

        let mut last = U256::from_big_endian(&res);
        if last >= N() {
            last = N() - U256::one();
        }
        last
    }

    pub fn sign(&self, z: U256) -> Signiture {
        let zz = _u256_to_u512(z);
        let k = Self::gen_rand_k();
        let r = _u256_to_u512(G().scalar_mul(k).x().num);
        let k_inv = _pow_with_mod(_u256_to_u512(k), _u256_to_u512(N()-U256::from_big_endian(&[2])), _u256_to_u512(N()));
        let s = ((zz% _u256_to_u512(N())+(r*_u256_to_u512(self.secret))% _u256_to_u512(N()))% _u256_to_u512(N()) * k_inv % _u256_to_u512(N())) % _u256_to_u512(N());
        let mut s = _u512_to_u256(s);

        if s > N()/U256::from_big_endian(&[2]) {
            s = N() - s;
        }

        Signiture { r: _u512_to_u256(r), s: s }
    }
}

#[derive(PartialEq, Debug)]
pub struct Point {
    x: Option<f64>,
    y: Option<f64>,
    pub a: f64,
    pub b: f64
}

impl Point {
    pub fn new(x: f64, y: f64, a: f64, b: f64) -> Self {     
        if y.powi(2) != x.powi(3) + a*x + b {
            panic!("({}, {}) is not on the curve", x, y);
        }
        Self { a: a, b: b, x: Some(x), y: Some(y) }
    }

    pub fn new_inf(a: f64, b: f64) -> Self {
        Self { a: a, b: b, x: None, y: None }
    }

    pub fn x(&self) -> f64 {
        self.x.unwrap()
    }

    pub fn y(&self) -> f64 {
        self.y.unwrap()
    }

    pub fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }

    pub fn ne(&self, other: &Self) -> bool {
        return self.x != other.x || self.y != other.y || self.a != other.a || self.b != other.b
    }
}

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {
        if self.a != other.a || self.b != other.b {
            panic!("Points are not in same curve.");
        }

        if self.x.is_none() {
            return Point {
                x: other.x,
                y: other.y,
                a: other.a,
                b: other.b
            };
        }

        if other.x.is_none() {
            return Point {
                x: self.x,
                y: self.y,
                a: self.a,
                b: self.b
            };
        }

        if self.x() == other.x() && self.y() != other.y() {
            return Point {
                x: None,
                y: None,
                a: self.a,
                b: self.b
            };
        }

        if self.x() != other.x() {
            let s = (other.y() - self.y()) / (other.x() - self.x());
            let x3 = s.powi(2)-self.x()-other.x();
            return Point {
                x: Some(x3),
                y: Some(s*(self.x()-x3)-self.y()),
                a: self.a,
                b: self.b
            };
        }

        if self == other && other.y() != 0. {
            let s = (3.*self.x().powi(2)+self.a) / (2.*self.y());
            let x3 = s.powi(2)-2.*self.x();
            return Point {
                x: Some(x3),
                y: Some(s*(self.x()-x3)-self.y()),
                a: self.a,
                b: self.b
            };
        }

        if self == other && self.y() == 0. {
            return Point {
                x: None,
                y: None,
                a: self.a,
                b: self.b
            };
        }

        // 별 의미 없음
        Point { a: 0., b: 0., x: Some(0.), y: Some(0.) }

    }
}