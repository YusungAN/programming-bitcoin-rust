use std::ops::{Add, Sub, Mul, Div};

#[derive(PartialEq, Debug)]
pub struct FieldElement {
    pub num: usize,
    pub prime: usize
}

impl FieldElement {
    pub fn new(num: usize, prime: usize) -> Self {
        if num >= prime {
            panic!("Num {} not in  field range to {}", num, prime-1)
        }

        FieldElement {
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
        let mut n = exp;
        while n < 0 {
            n += (self.prime-1) as isize
        }
        let num = self.num.pow(n.try_into().unwrap()) % self.prime;

        Self {
            num: num,
            prime: self.prime
        }
    }
}

impl<'a, 'b> Add<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn add(self, other: &'b FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = (self.num + other.num) % self.prime;
        
        FieldElement {
            num: num,
            prime: self.prime
        }
    }
}

impl<'a, 'b> Sub<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn sub(self, other: &'b FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = ((self.num as i64) - (other.num as i64)).rem_euclid(self.prime as i64);

        FieldElement {
            num: num as usize,
            prime: self.prime
        }
    }
}

impl<'a, 'b> Mul<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn mul(self, other: &'b FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = (self.num * other.num) % self.prime;

        FieldElement {
            num: num,
            prime: self.prime
        }
    }
}

impl<'a, 'b> Div<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn div(self, other: &'b FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let num = self.num * (other.num.pow((self.prime-2) as u32) % self.prime) % self.prime;

        FieldElement {
            num: num,
            prime: self.prime
        }
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
            println!("{} {} {}", self.a, self.x(), self.y());
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