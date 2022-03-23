// https://www.codewars.com/kata/57c1ab3949324c321600013f

use std::collections::HashMap;

fn to_leet_speak(s: &str) -> String {
    let mut dict = HashMap::<char, char>::new();   
    dict.insert('A', '@');
    dict.insert('B', '8');
    dict.insert('C', '(');
    dict.insert('D', 'D');
    dict.insert('E', '3');
    dict.insert('F', 'F');
    dict.insert('G', '6');
    dict.insert('H', '#');
    dict.insert('I', '!');
    dict.insert('J', 'J');
    dict.insert('K', 'K');
    dict.insert('L', '1');
    dict.insert('M', 'M');
    dict.insert('N', 'N');
    dict.insert('O', '0');
    dict.insert('P', 'P');
    dict.insert('Q', 'Q');
    dict.insert('R', 'R');
    dict.insert('S', '$');
    dict.insert('T', '7');
    dict.insert('U', 'U');
    dict.insert('V', 'V');
    dict.insert('W', 'W');
    dict.insert('X', 'X');
    dict.insert('Y', 'Y');
    dict.insert('Z', '2');
    dict.insert(' ', ' ');
    return s.to_string().chars().map(|x|dict.get(&x).unwrap()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leet() {
        assert_eq!(to_leet_speak("LEET"), "1337");
    }

    #[test]
    fn codewars() {
        assert_eq!(to_leet_speak("CODEWARS"), "(0D3W@R$");
    }

    #[test]
    fn hello_world() {
        assert_eq!(to_leet_speak("HELLO WORLD"), "#3110 W0R1D");
    }

    #[test]
    fn lorem_ipsum() {
        assert_eq!(to_leet_speak("LOREM IPSUM DOLOR SIT AMET"), "10R3M !P$UM D010R $!7 @M37");
    }

    #[test]
    fn quick_brown_fox() {
        assert_eq!(to_leet_speak("THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG"), "7#3 QU!(K 8R0WN F0X JUMP$ 0V3R 7#3 1@2Y D06");
    }
}