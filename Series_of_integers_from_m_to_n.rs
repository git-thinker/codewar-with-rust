// https://www.codewars.com/kata/5841f680c5c9b092950001ae

fn integer_series(m: u32, n: u32) -> Vec<u32> {
    (m..=n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(integer_series(2, 5), vec![2, 3, 4, 5]);
    }
}