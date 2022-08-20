fn main() {
    println!("Hello, world!");
    for i in 1..101 {
        let result = fizzbuzz(& i);
        println!("{}:{}",i, result);
    }
}

fn fizzbuzz(value:&i32) -> String{
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
