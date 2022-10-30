use core::num;


/// Converts a number to a string representating roman numeral/// 
pub fn num_as_roman(mut remainder: i32) -> String {
  
  static SYMBOLS: [(i32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
  ];

  let mut result = String::new();
  for (number, letters) in SYMBOLS {
    while remainder >= number {
      result.push_str(letters);
      remainder -= number;
    }
  }

  return result;
}

#[test]
fn returns_1990_expected() {
  assert_eq!(num_as_roman(1990), "MCMXC");
}


#[test]
fn returns_expected() {
  assert_eq!(num_as_roman(182), "CLXXXII");
  assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}

#[test]
fn returns_3_expected() {
  assert_eq!(num_as_roman(3), "III");
}

#[test]
fn returns_4_expected() {
  assert_eq!(num_as_roman(4), "IV");
}

#[test]
fn returns_6_expected() {
  assert_eq!(num_as_roman(6), "VI");
}
#[test]
fn returns_8_expected() {
  assert_eq!(num_as_roman(8), "VIII");
}
#[test]
fn returns_9_expected() {
  assert_eq!(num_as_roman(9), "IX");
}
#[test]
fn returns_11_expected() {
  assert_eq!(num_as_roman(11), "XI");
}
#[test]
fn returns_15_expected() {
  assert_eq!(num_as_roman(15), "XV");
}
#[test]
fn returns_464_expected() {
  assert_eq!(num_as_roman(464), "CDLXIV");
}
#[test]
fn returns_999_expected() {
  assert_eq!(num_as_roman( 999), "CMXCIX");
}
#[test]
fn returns_49_expected() {
  assert_eq!(num_as_roman( 49), "XLIX");
}