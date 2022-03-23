// https://www.codewars.com/kata/5259b20d6021e9e14c0010d4

fn reverse_words(str: &str) -> String {
    let mut v = Vec::new();
    for item in str.split(" "){
        v.push(item.chars().rev().collect::<String>());
    }
    v.join(" ")
}

// Rust tests
#[test]
fn sample_test() {
  assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
  assert_eq!(reverse_words("apple"), "elppa");
  assert_eq!(reverse_words("a b c d"),"a b c d");
  assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow");
}