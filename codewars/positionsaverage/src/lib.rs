use core::num;
use std::ops::Mul;

fn pos_average(s: &str) -> f64 {
    let numbers:Vec<&str> = s.split(",")
        .map(|x| x.trim())
        .collect();

    let total_and_equal = &numbers[..]
        .into_iter()
        .enumerate()
        .map(|(i, number)| {

            match try_get_tail(&numbers[..], i+1) {
                Some (tail) => {
                    let equal_positions:i32 = tail
                        .into_iter()
                        .map(|number2| count_equal_positions(number, number2))
                        .sum();
                    let result = (number.len().mul(tail.len()) as i32, equal_positions, number);
                    println!("map #{:?} {:?}, result= {:?}, {:?}", i, number, result, tail);
                    return result;
                }
                None => {
                    return (0 ,0, number)
                }
            }
           
        })
        .fold((0,0), |r, x| {
            return (r.0+x.0, r.1+x.1)
        });

    let result = round_10d(((total_and_equal.1 as f64) / (total_and_equal.0 as f64)) * 100_f64);
    println!("aggregate result {:?} = {:?}", total_and_equal, result);

    
    return result;
}

fn try_get_tail<'a>(target:&'a [&str], tail_start:usize) -> Option<&'a[&'a str]> {
    return if tail_start < target.len() {
        Some (&target[tail_start..])
    }else{
        None
    }
}

fn round_10d(target:f64) -> f64{
    let d = 10_f64.powf(10_f64);
    return (target * d).round() / d;
}

fn count_equal_positions(s1:&str, s2:&str) -> i32 {
    let cs2:Vec<char> = s2.chars().collect();
    let result:i32 = s1.chars().enumerate().map(|(i,c)| {
        let co2 = cs2.get(i);
        match co2 {
            Some(c2) =>  if c2 == &c {1} else {0}
            _ => 0
        }
    }
   ).sum();
   return result;
}


#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::float_eq;
    
    fn assert_float_equals(actual: f64, expected: f64) {
        let merr = 1.0e-12;
        let res = float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
        assert!(res, "Expected value must be near: {:e} but was:{:e}", expected, actual);
    }
    
    #[test]
    fn basic_tests() {
        assert_float_equals(pos_average("466960, 069060, 494940, 060069, 060090, 640009, 496464, 606900, 004000, 944096"), 26.6666666667);
        // assert_float_equals(pos_average("444996, 699990, 666690, 096904, 600644, 640646, 606469, 409694, 666094, 606490"), 29.2592592593);
    }
    #[test]
    fn basic_tests2() {
        // assert_float_equals(pos_average("466960, 069060, 494940, 060069, 060090, 640009, 496464, 606900, 004000, 944096"), 26.6666666667);
        assert_float_equals(pos_average("444996, 699990, 666690, 096904, 600644, 640646, 606469, 409694, 666094, 606490"), 29.2592592593);
    }

    #[test]
    fn description_test() {
        assert_float_equals(pos_average("6900690040, 4690606946, 9990494604"), 26.6666666667);
    }

    #[test]
    fn count_equal_positions_test() {
        assert_eq!(count_equal_positions("11", "01"), 1);
        assert_eq!(count_equal_positions("312", "012"), 2);
        assert_eq!(count_equal_positions("012345", "067345"), 4);
    }
}