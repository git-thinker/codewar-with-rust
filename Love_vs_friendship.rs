// https://www.codewars.com/kata/59706036f6e5d1e22d000016

fn words_to_marks(s: &str) -> u32 {
    s.to_string()
    .bytes()
    .map(|x|(x as u32 -'a' as u32 + 1) as u32)
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(words_to_marks("attitude"), 100);
        assert_eq!(words_to_marks("friends"), 75);
        assert_eq!(words_to_marks("family"), 66);
        assert_eq!(words_to_marks("selfness"), 99);
        assert_eq!(words_to_marks("knowledge"), 96);
    }   
}