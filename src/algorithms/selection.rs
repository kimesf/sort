use crate::algorithms::Algorithm;

pub struct Selection {}

impl Algorithm for Selection {
    fn sort(list: &mut Vec<i32>) {
        let length = list.len();
        let mut lower: usize;

        for i in 0..length {
            lower = i;

            for j in i + 1..length {
                if list[j] < list[lower] {
                    lower = j;
                }
            }

            list.swap(i, lower);
        }
    }
}
