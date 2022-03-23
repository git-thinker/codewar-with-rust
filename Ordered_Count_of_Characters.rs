// https://www.codewars.com/kata/57a6633153ba33189e000074

fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    let mut ret: Vec<(char, i32)> = Vec::new();
    for i in sip.chars(){
        let mut flag = true;
        for item in ret.iter_mut(){
            if item.0 == i{
                item.1 += 1;
                flag = false;
            }
        }
        if flag{
            ret.push((i, 1));
        }

    }
    ret
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abradacadabra() {
        assert_eq!(
            ordered_count("abracadabra"),
            vec![('a', 5), ('b', 2), ('r', 2), ('c', 1), ('d', 1)]
        );
    }
    #[test]
    fn test_banana() {
        assert_eq!(ordered_count("banana"), vec![('b', 1), ('a', 3), ('n', 2)]);
    }
    #[test]
    fn test_master_solver() {
        assert_eq!(
            ordered_count("i am a master kata solver"),
            vec![
                ('i', 1),
                (' ', 5),
                ('a', 5),
                ('m', 2),
                ('s', 2),
                ('t', 2),
                ('e', 2),
                ('r', 2),
                ('k', 1),
                ('o', 1),
                ('l', 1),
                ('v', 1)
            ]
        );
    }
}
