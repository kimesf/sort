use crate::algorithms::Algorithm;
pub struct Merge {}

impl Algorithm for Merge {
    fn sort(list: &mut Vec<i32>) {
        let length = list.len();

        Self::do_sort(list, 0, length - 1);
    }
}

impl Merge {
    fn do_sort(list: &mut Vec<i32>, from: usize, to: usize) {
        let sorted = from >= to;
        if sorted {
            return;
        }

        let mid = from + (to - from) / 2;

        Self::do_sort(list, from, mid);
        Self::do_sort(list, mid + 1, to);

        Self::merge(list, from, mid, to);
    }

    fn merge(list: &mut Vec<i32>, from: usize, mid: usize, to: usize) {
        let left_size = mid - from + 1;
        let right_size = to - mid;

        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();

        for i in 0..left_size {
            left.push(list[from + i]);
        }

        for i in 0..right_size {
            right.push(list[mid + 1 + i]);
        }

        let mut l = 0;
        let mut r = 0;
        let mut current_position = from;

        while l < left_size && r < right_size {
            if left[l] < right[r] {
                list[current_position] = left[l];
                l += 1;
            } else {
                list[current_position] = right[r];
                r += 1;
            }

            current_position += 1;
        }

        while l < left_size {
            list[current_position] = left[l];
            l += 1;
            current_position += 1;
        }

        while r < right_size {
            list[current_position] = right[r];
            r += 1;
            current_position += 1;
        }
    }
}
