fn main() {
    println!("Hello, world!");
    for i in 1..101 {
        let result = fizzbuzz(& i);
        println!("{}:{}",i, result);
    }
}


pub fn fizzbuzz(value:&i32) -> String{
    let dividable_by_3 = value % 3 == 0;
    let dividable_by_5 = value % 5 == 0;

    match (dividable_by_3, dividable_by_5) {
        (false, false) => value.to_string(),
        (true, false) => "fizz".to_string(),
        (false, true) => "buzz".to_string(),
        (true, true) => "fizzbuzz".to_string()
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    fn fizzbuzz_test(value:i32, expected:&str){
        let result = fizzbuzz(& value);
        assert_eq!(result, expected);
    }
    
    #[test]
    fn it_should_return_fizz_for_3() {
        fizzbuzz_test(3, "fizz");
    }    
    
    #[test]
    fn it_should_return_fizz_for_6() {
        fizzbuzz_test(6, "fizz");
    }
    
    #[test]
    fn it_should_return_buzz_for_5() {
        fizzbuzz_test(5, "buzz");
}    
    #[test]
    fn it_should_return_buzz_for_25() {
        fizzbuzz_test(25, "buzz");
    }

    #[test]
    fn it_should_return_fizzbuzz_for_15() {
        fizzbuzz_test(15, "fizzbuzz");
    }   
    
    #[test]
    fn it_should_return_fizzbuzz_for_60() {
        fizzbuzz_test(60, "fizzbuzz");
    }

    #[test]
    fn it_should_return_1_for_1() {
        fizzbuzz_test(1, "1");
    }

    #[test]
    fn it_should_return_2_for_2() {
        fizzbuzz_test(2, "2");
    }

    #[test]
    fn it_should_return_13_for_13() {
        fizzbuzz_test(13, "13");
    }
}
