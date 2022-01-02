pub fn sort(array: &mut [isize]) {
    sort_in_range(array, 0, array.len() - 1);
}

fn sort_in_range(array: &mut [isize], first: usize, last: usize) {
    if first < last {
        let midpoint = partition(array, first, last);
        sort_in_range(array, first, midpoint - 1);
        sort_in_range(array, midpoint + 1, last);
    }
}

fn partition(array: &mut [isize], first: usize, last: usize) -> usize {
    let pivot = array[last];

    let mut i = first;
    let mut j = first;

    while j < last {
        if array[j] < pivot {
            array.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    array.swap(i, last);
    i
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
