// https://www.codewars.com/kata/5a91a7c5fd8c061367000002

fn minimum_steps(nums: &[i32], value: i32) -> usize {
    let mut nums = nums.to_owned();
    nums.sort();
    let mut value = value;
    for (i, item) in nums.iter().enumerate(){
        value -= item;
        if value <= 0{
            return i;
        }
    };
    0usize
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(minimum_steps(&[4, 6, 3], 7), 1);
        assert_eq!(minimum_steps(&[10, 9, 9, 8], 17), 1);
        assert_eq!(minimum_steps(&[8, 9, 10, 4, 2], 23), 3);
        assert_eq!(minimum_steps(&[19, 98, 69, 28, 75, 45, 17, 98, 67], 464), 8);
        assert_eq!(minimum_steps(&[4, 6, 3], 2), 0);
    }
}