// https://www.codewars.com/kata/5583090cbe83f4fd8c000051

// fn digitize(n: u64) -> Vec<u8> {
//     let s = format!("{}", n);
//     let s_iter = s.bytes().rev();
//     let mut s = Vec::<u8>::new();
//     for i in s_iter{
//         s.push((i - 48) as u8);
//     }
//     s
// }

fn digitize(n: u64) -> Vec<u8> {
    format!("{}", n)
        .bytes()
        .rev()
        .map(|x|((x-48) as u8))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(35231), vec![1,3,2,5,3]);
        assert_eq!(digitize(0), vec![0]);
    }
}