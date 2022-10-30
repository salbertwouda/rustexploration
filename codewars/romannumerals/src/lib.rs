use core::num;


/// Converts a number to a string representating roman numeral/// 
pub fn num_as_roman(num: i32) -> String {

  const M: (i32, char) = (1000,'M');
  const D: (i32, char)= (500,'D'); 
  const C: (i32, char) = (100,'C');
  const L: (i32, char) = (50,'L');
  const X: (i32, char) = (10,'X');
  const V: (i32, char) = (5, 'V');
  const I: (i32, char) = (1,'I');

  const ROMAN_NUMERALS_CHARS: [(i32, char); 7] = [
    M,D,C,L,X,V,I
  ];

  fn get_possible_prefixer (c:char) -> Option<(i32, char)>{
    return match c {
      'M' => Some(C),
      'D' => Some(C),
      'C' => Some(X),
      'L' => Some(X),
      'X' => Some(I),
      'V' => Some(I),
      _ => None
    }
  }
  let mut remainder = num;
  let mut result:Vec<char> = Vec::<char>::new();

  for (i,roman_numeral) in ROMAN_NUMERALS_CHARS.into_iter().enumerate() {
    
    let number_of_divisions = remainder / roman_numeral.0;

    match number_of_divisions {
      0 => { },
      1|2|3 => {
        // apply normal case
        remainder = remainder % roman_numeral.0;
        for _n in 0..number_of_divisions{
          result.push(roman_numeral.1)
        }
      }
      4 => {
        // apply postfix
        remainder = remainder % roman_numeral.0;
        let previous = ROMAN_NUMERALS_CHARS[i-1];
        result.push(roman_numeral.1);
        result.push(previous.1);
      }
      _ => {
        panic!("should not happen");
      }
    }

    // apply prefix
    let next_exists = i+1 < ROMAN_NUMERALS_CHARS.len();
    if next_exists {
      match get_possible_prefixer(roman_numeral.1) {
        Some (prefix) => {
            let prefix_range_lower_boundary = roman_numeral.0 - prefix.0;
            if remainder >= prefix_range_lower_boundary {
              result.push(prefix.1);
              result.push(roman_numeral.1);
              remainder -= roman_numeral.0 - prefix.0;
          }
        }
        None =>{}
      }
    }
  }
  
  let result_string: String = result.iter().collect();
  return result_string;
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