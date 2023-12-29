mod complex;
use complex::Complex;

use std::io::stdin;

fn input() -> f64{
  let mut a = String::new();
  stdin().read_line(&mut a).unwrap();
  return a.trim().parse::<f64>().unwrap();
}

fn main() {
  let (xre, xim, yre, yim);
  xre = input();
  xim = input();
  yre = input();
  yim = input();
  let x = Complex {re: xre, im: xim};
  let y = Complex {re: yre, im: yim};

  println!("x+y: {}", x+y);
  println!("x-y: {}", x-y);
  println!("xy: {}", x*y);
  println!("x/y: {}", x/y);
  println!("|x|: {}", x.abs());
  println!("conj(x): {}", x.conj());
}
