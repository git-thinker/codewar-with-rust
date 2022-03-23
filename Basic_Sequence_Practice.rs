// https://www.codewars.com/kata/5436f26c4e3d6c40e5000282

fn sum_of_n(n: i32) -> Vec<i32> {
    let mut v = vec![];
    for i in 0..=n.abs(){
        v.push((1..=i).sum());
    }
    if n < 0{
        v = v.into_iter().map(|x:i32|{-x}).collect();
    }
    v
}

#[test]
fn normal_tests() {
    assert_eq!(sum_of_n(3), vec![0, 1, 3, 6]);
    assert_eq!(sum_of_n(-4), vec![0, -1, -3, -6, -10]);
    assert_eq!(sum_of_n(1), vec![0, 1]);
    assert_eq!(sum_of_n(0), vec![0]);
    assert_eq!(sum_of_n(10), vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
}