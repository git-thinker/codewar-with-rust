// https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3

fn abbrev_name(name: &str) -> String {
    let name = name.to_string();
    let s: Vec<_> = name.split_whitespace().collect();
    let mut ret = String::new();
    for item in s{
        let c = item.as_bytes()[0] as char;
        ret.push(c.to_uppercase().next().unwrap());
        ret.push('.');
    }
    // if ret.len() != 0 {
        ret.pop();
    // }
    ret
}

// Rust test example:
#[test]
fn sample_tests() {
  assert_eq!(abbrev_name("Sam Harris"), "S.H");
  assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
  assert_eq!(abbrev_name("Evan Cole"), "E.C");
  assert_eq!(abbrev_name("P Favuzzi"), "P.F");
  assert_eq!(abbrev_name("David Mendieta"), "D.M");
}