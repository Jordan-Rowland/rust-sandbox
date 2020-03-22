fn main() {
    let x = 10;
    let r = &x;
    let rr = &r; // `rr` is a `&&x`

    if is_ten(rr) {
      println!("Same!");
    }
  }

  fn is_ten(val: &i32) -> bool {
    *val == 10
  }