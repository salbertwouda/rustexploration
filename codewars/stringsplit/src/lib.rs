use std::vec::Vec;
use itertools::Itertools;

fn solution(s: &str) -> Vec<String> {
    s.chars()
      .chunks(2)
      .into_iter()
      .map(
        |c| c.pad_using(2, |_| '_')
             .collect()
      )
      .collect()
}

pub fn solution_basic(s:&str) -> Vec<String> {

    let mut current = String::new();
    let mut result = Vec::new(); 
    for (i, c) in s.chars().enumerate() {
        current.push(c);
        if (i+1) % 2 == 0 {
            result.push(current);
            current = String::new();
        }
    }

    if current.len() > 0 {
        if current.len() == 1 {
            current.push('_');
        }
        result.push(current);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn basic() {
            assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
            assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
            assert_eq!(solution(""), [] as [&str; 0]);
        }
        
    }
}
