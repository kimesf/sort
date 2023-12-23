use crate::algorithms::Algorithm;

pub struct Quick {}

impl Algorithm for Quick {
    fn sort(list: &mut Vec<i32>) {
        // TODO
        list.swap(0,1)
    }
}
