// https://www.codewars.com/kata/5949481f86420f59480000e7
fn odd_or_even(numbers: Vec<i32>) -> String {
    let mut flag = 0;
    for i in numbers.iter(){
        if i % 2 != 0{
            flag += 1;
        }
    }
    match flag % 2 {
        0 => "even".to_string(),
        _ => "odd".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        assert_eq!(odd_or_even(vec![]), "even");
    }
    #[test]
    fn test_single_even() {
        assert_eq!(odd_or_even(vec![0]), "even");
    }
    #[test]
    fn test_single_odd() {
        assert_eq!(odd_or_even(vec![1]), "odd");
    }
    #[test]
    fn test_even() {
        assert_eq!(odd_or_even(vec![0, 1, 5]), "even");
    }
    #[test]
    fn test_odd() {
        assert_eq!(odd_or_even(vec![0, 1, 4]), "odd");
    }
    #[test]
    fn test_negative_even() {
        assert_eq!(odd_or_even(vec![0, -1, -5]), "even");
    }
    #[test]
    fn test_negative_odd() {
        assert_eq!(odd_or_even(vec![0, -1, 2]), "odd");
    }
}