use crate::algorithms::Algorithm;

pub struct Insertion {}

impl Algorithm for Insertion {
    fn sort(list: &mut Vec<i32>) {
        let mut j: usize;

        for i in 0..list.len() - 1 {
            j = i + 1;

            while j > 0 {
                if list[j - 1] <= list[j] {
                    break;
                }

                list.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}
