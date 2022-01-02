pub fn sort(arr: &mut [isize]) {
    for i in 0..arr.len() {
        for j in (0..i).rev() {
            if arr[j] >= arr[j + 1] {
                arr.swap(j, j + 1);
            } else {
                break
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::sort;

    #[test]
    fn basics() {
        let mut nums = [7, 2, 4, 6];
        sort(&mut nums);
        assert_eq!(nums, [2, 4, 6, 7]);
    }
}
