fn quick_sort<T: PartialOrd>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    let pivot = partition(data);
    quick_sort(&mut data[..pivot]);
    quick_sort(&mut data[pivot + 1..]);
}

fn partition<T: PartialOrd>(data: &mut [T]) -> usize {
    let len = data.len();
    let pivot = len / 2;

    data.swap(pivot, len - 1);

    let mut i = 0;
    for j in 0..len - 1 {
        if data[j] < data[len - 1] {
            data.swap(i, j);
            i += 1;
        }
    }

    data.swap(i, len - 1);

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut data = vec![6, 3, 4, 1, 2, 9, 0, 8, 7];

        quick_sort(&mut data);

        assert!(data == vec![0, 1, 2, 3, 4, 6, 7, 8, 9]);
    }
}
