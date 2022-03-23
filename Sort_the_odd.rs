// https://www.codewars.com/kata/578aa45ee9fd15ff4600090d

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut arr = arr.to_vec();
    let mut mask = Vec::new();
    for i in 0..arr.len(){
        if arr[i] % 2 == 1{
            mask.push(i);
        }
    }
    for i in 0..mask.len(){
        for j in 0..(mask.len()-i-1){
            if arr[mask[j]] > arr[mask[j+1]]{
                let tmp = arr[mask[j]];
                arr[mask[j]] = arr[mask[j+1]];
                arr[mask[j+1]] = tmp;
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array(&[]), []);
    }
}