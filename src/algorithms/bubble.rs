use crate::algorithms::Algorithm;

pub struct Bubble {}

impl Algorithm for Bubble {
    fn sort(list: &mut Vec<i32>) {
        let length = list.len();
        let mut swapped: bool;

        for i in 0..length {
            swapped = false;

            for j in 0..length - i - 1 {
                if list[j] > list[j + 1] {
                    list.swap(j, j + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }
        }
    }
}
