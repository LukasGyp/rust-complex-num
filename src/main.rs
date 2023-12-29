use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

#[derive(Copy, Clone)]
struct Complex {
  re: f64,
  im: f64,
}

impl Complex {
  fn abs(&self) -> f64 {
    (self.re * self.re + self.im * self.im).sqrt()
  }

  fn conj(&self) -> Complex {
    Complex {
      re: self.re,
      im: -self.im
    }
  }
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.im == 0.0 {
      write!(f, "{}", self.re)
    } else if self.re == 0.0 {
      write!(f, "{}i", self.im)
    } else {
      write!(f, "{}{:+}i", self.re, self.im)
    }
  }
}

impl Add<Complex> for Complex {
  type Output = Complex;

  fn add(self, other: Complex) -> Complex {
    Complex {
      re: self.re + other.re,
      im: self.im + other.im,
    }
  }
}

impl Sub for Complex {
  type Output = Complex;

  fn sub(self, other: Complex) -> Complex {
    Complex {
      re: self.re - other.re,
      im: self.im - other.im,
    }
  }
}

impl Mul for Complex {
  type Output = Complex;

  fn mul(self, other: Complex) -> Complex {
    Complex {
      re: self.re * other.re - self.im * other.im,
      im: self.re * other.im + self.im * other.re,
    }
  }
}

impl Div for Complex {
  type Output = Complex;

  fn div(self, other: Complex) -> Complex {
    Complex {
      re: (self.re * other.re + self.im * other.im) / (other.re * other.re + other.im * other.im),
      im: (self.re * other.im - self.im * other.re) / (other.re * other.re + other.im * other.im),
    }
  }
}

fn main() {
  let x = Complex {re: 6., im:2.};
  let y = Complex {re: 3., im:1.};

  println!("x+y: {}", x+y);
  println!("x-y: {}", x-y);
  println!("xy: {}", x*y);
  println!("x/y: {}", x/y);
  println!("|x|: {}", x.abs());
  println!("conj(x): {}", x.conj());
}
