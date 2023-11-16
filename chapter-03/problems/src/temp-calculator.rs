fn converter(d: f64, to: char) -> f64 {
  return if to == 'c' || to == 'C' { (d - 32) * (5 / 9) } else { (d * 9 / 5) + 32 };
}