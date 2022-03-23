// https://www.codewars.com/kata/59a96d71dbe3b06c0200009c

fn generate_shape(n: i32) -> String {
    let mut s = String::new();
    for _ in 0..n{
        for _ in 0..n{
            s.push('+');
        }
        s.push('\n');
    }
    if s.len() != 0{
        s.pop();
    }
    s
}

#[test]
fn sample_test() {
  assert_eq!(generate_shape(3), "+++\n+++\n+++");
}