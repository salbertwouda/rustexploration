use itertools::Itertools;

pub fn order(sentence: &str) -> String {
    let mut ws: Vec<_> = sentence.split_whitespace().map(String::from).collect();
    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    ws.join(" ")
}

pub fn order_first_impl(sentence: &str) -> String {
    return match sentence {
        "" => String::new(),
        _ => sentence
                .split_whitespace()
                .map(|word| (
                    String::from(word), 
                    word.chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10)).next().unwrap().unwrap()
                ))
                .into_iter()
                .sorted_by(|a,b| a.1.to_owned().cmp(&b.1.to_owned()))
                .map(|(word,_)|word)
                .collect::<Vec<String>>()
                .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }


}