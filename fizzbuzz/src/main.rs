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
    if dividable_by_3 && dividable_by_5 {
        return "fizzbuzz".to_string();
    }else if dividable_by_3 {
        return "fizz".to_string();
    }else if dividable_by_5 {
        return "buzz".to_string();
    }
    return value.to_string();

}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_should_return_fizz_for_3() {
        let value = 3;
        let result = fizzbuzz(& value);
        assert_eq!(result, "fizz");
    }

    #[test]
    fn it_should_return_buzz_for_5() {
        let value = 5;
        let result = fizzbuzz(& value);
        assert_eq!(result, "buzz");
    }

    #[test]
    fn it_should_return_fizz_for_15() {
        let value = 15;
        let result = fizzbuzz(& value);
        assert_eq!(result, "fizzbuzz");
    }

    #[test]
    fn it_should_return_1_for_1() {
        let value = 1;
        let result = fizzbuzz(& value);
        assert_eq!(result, "1");
    }
}
