// https://www.codewars.com/kata/53af2b8861023f1d88000832

fn are_you_playing_banjo(name: &str) -> String {
    let mut name = name.to_string();
    let saying = match name.chars().next().unwrap(){
        'R'|'r' => String::from(" plays banjo"),
        _ => String::from(" does not play banjo"),
    };
    name.push_str(&saying);
    return name;
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_you_playing_banjo() {
        assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
        assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
        assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
    }
}
