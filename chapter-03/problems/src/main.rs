fn converter(d: f64, to: char) -> f64 {
  return if to == 'c' || to == 'C' { (d - 32.0) * (5.0 / 9.0) } else { (d * 9.0 / 5.0) + 32.0 };
}

fn fib(nth: i8) -> i128 {
  let mut prev: i128 = 0;
  let mut curr: i128 = 1;

  if nth <= 0 {
    panic!("Uh oh!");
  }

  if nth == 1 {
    return 0;
  } else if nth == 2 {
    return 1;
  }

  for _number in 3..(nth + 1) {
    let temp = prev + curr;
    prev = curr;
    curr = temp;
  }

  return curr;
}

fn main() {
  println!("{}", fib(8));
}