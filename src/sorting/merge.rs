pub fn sort(array: &mut [isize]) {
    sort_in_range(array, 0, array.len() - 1);
}

fn sort_in_range(arr: &mut [isize], low: usize, high: usize) {
    if low < high {
        let mid = low + (high - low) / 2;

        sort_in_range(arr, low, mid);
        sort_in_range(arr, mid + 1, high);

        merge(arr, low, mid, high);
    }
}

fn merge(arr: &mut [isize], low: usize, mid: usize, high: usize) {
    let n1 = mid - low + 1;
    let n2 = high - mid;

    let mut left = vec![0; n1];
    let mut right = vec![0; n2];

    for i in 0..n1 {
        left[i] = arr[low +i];
    }

    for i in 0..n2 {
        right[i] = arr[mid + 1 + i];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = low;

    while i < n1 && j < n2 {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < n1 {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < n2 {
        arr[k] = right[j];
        j += 1;
        k += 1;
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
