// https://www.codewars.com/kata/57ebaa8f7b45ef590c00000c

fn switcher(numbers: Vec<&str>) -> String {
    numbers.into_iter()
    .map(|x|{
        match x.parse::<u8>().unwrap(){
            27 => '!',
            28 => '?',
            29 => ' ',
            n => (26 - n + 'a' as u8) as char 
        }
    }).collect::<String>()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::switcher;

    #[test]
    fn example_tests() {
        assert_eq!(switcher(vec!["24", "12", "23", "22", "4", "26", "9", "8"]), "codewars");
        assert_eq!(switcher(vec!["25","7","8","4","14","23","8","25","23","29","16","16","4"]), "btswmdsbd kkw"); 
        assert_eq!(switcher(vec!["4", "24"]), "wc");
    }
}
