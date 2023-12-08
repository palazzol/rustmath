use std::ops::{AddAssign, Add, SubAssign, Sub, MulAssign, Mul, DivAssign, Div};

// This is an Ordered Field

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Field<T> {
    x: T,
}

// Implementation for f64

impl Field<f64> {
    // New member 
    pub fn new(x: f64) -> Field<f64> {
        return Field::<f64>{x};
    }
}
    
/// TBD - These functions should be bundled in a trait?
impl Field<f64> {
    // Zero member
    pub fn zero() -> Field<f64> {
        Field::<f64> {x: 0.0_f64}
    } 
    // One member
    pub fn one() -> Field<f64> {
        Field::<f64> {x: 1.0_f64}
    }
    // Additive inverse
    pub fn complement(&mut self) {
        *self = &Field::<f64>::zero() - self;
    }
    // Multiplicative inverse
    pub fn invert(&mut self) {
        *self = &Field::<f64>::one() / self;
    }
}

impl AddAssign for Field<f64> {
    fn add_assign(&mut self, f1: Field<f64>) {
        println!("Potentially unwanted copy of arg");
        self.x = self.x+f1.x;
    }
}

impl AddAssign<&Field<f64>> for Field<f64> {
    fn add_assign(&mut self, f1: &Field<f64>) {
        self.x = self.x+f1.x;
    }
}

impl SubAssign for Field<f64> {
    fn sub_assign(&mut self, f1: Field<f64>) {
        println!("Potentially unwanted copy of arg");
        self.x = self.x-f1.x;
    }
}

impl SubAssign<&Field<f64>> for Field<f64> {
    fn sub_assign(&mut self, f1: &Field<f64>) {
        self.x = self.x-f1.x;
    }
}

impl MulAssign for Field<f64> {
    fn mul_assign(&mut self, f1: Field<f64>) {
        println!("Potentially unwanted copy of arg");
        self.x = self.x*f1.x;
    }
}

impl MulAssign<&Field<f64>> for Field<f64> {
    fn mul_assign(&mut self, f1: &Field<f64>) {
        self.x = self.x*f1.x;
    }
}

impl DivAssign for Field<f64> {
    fn div_assign(&mut self, f1: Field<f64>) {
        println!("Potentially unwanted copy of arg");
        self.x = self.x/f1.x;
    }
}

impl DivAssign<&Field<f64>> for Field<f64> {
    fn div_assign(&mut self, f1: &Field<f64>) {
        self.x = self.x/f1.x;
    }
}

impl Add for Field<f64> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        println!("Potentially unwanted copies of 2 args");
        Self {x: self.x + other.x}
    }
}

impl Add<&Field<f64>> for &Field<f64> {
    type Output = Field<f64>;
    //fn add(self, other: Self) -> Field::<f64> {
    fn add(self, other: &Field<f64>) -> Field::<f64> {
        Field::<f64> {x: self.x + other.x}
    }
}

impl Sub for Field<f64> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        println!("Potentially unwanted copies of 2 args");
        Self {x: self.x - other.x}
    }
}

impl Sub<&Field<f64>> for &Field<f64> {
    type Output = Field<f64>;
    //fn sub(self, other: Self) -> Field::<f64> {
    fn sub(self, other: &Field<f64>) -> Field::<f64> {
        Field::<f64> {x: self.x - other.x}
    }
}

impl Mul for Field<f64> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        println!("Potentially unwanted copies of 2 args");
        Self {x: self.x * other.x}
    }
}

impl Mul<&Field<f64>> for &Field<f64> {
    type Output = Field<f64>;
    //fn mul(self, other: Self) -> Field::<f64> {
    fn mul(self, other: &Field<f64>) -> Field::<f64> {
        Field::<f64> {x: self.x * other.x}
    }
}

impl Div for Field<f64> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        println!("Potentially unwanted copies of 2 args");
        Self {x: self.x / other.x}
    }
}

impl Div<&Field<f64>> for &Field<f64> {
    type Output = Field<f64>;
    //fn div(self, other: Self) -> Field::<f64> {
    fn div(self, other: &Field<f64>) -> Field::<f64> {
        Field::<f64> {x: self.x / other.x}
    }
}

