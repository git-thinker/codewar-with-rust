// https://www.codewars.com/kata/56a5d994ac971f1ac500003e

use std::collections::VecDeque;

fn longest_consec(strarr: Vec<&str>, k: usize) -> String { 
    if k > strarr.len(){
        return String::new();
    }
    let mut qsize = VecDeque::<usize>::new();
    let mut maxsize:usize;
    let mut maxpot = 0usize;
    for i in 0..k{
        qsize.push_back(strarr[i].len());
    }
    maxsize = qsize.iter().sum();
    for i in 0..strarr.len()-k{
        qsize.pop_front();
        qsize.push_back(strarr[i+k].len());
        if maxsize < qsize.iter().sum(){
           maxpot = i + 1;
           maxsize = qsize.iter().sum();
        }
    }
    strarr[maxpot..maxpot+k].join(&"")
}

fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
    testing(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1, 
        "oocccffuucccjjjkkkjyyyeehh");
    testing(vec![], 3, "");
    testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
    testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}