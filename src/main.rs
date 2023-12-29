mod complex;
use complex::Complex;

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
