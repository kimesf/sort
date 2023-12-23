// use std::fs;

mod selection;
mod insertion;
mod bubble;
mod merge;
mod quick;

trait Algorithm {
    fn sort(list: &mut Vec<i32>);

    // fn example_unsorted_list() -> Vec<i32> {
    //     let contents = fs::read_to_string("fixtures/unsorted.txt")
    //         .expect("Unable to read default random number list");

    //     contents.split("\n").map(|n| n.parse().unwrap()).collect()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use selection::Selection;
    use insertion::Insertion;
    use bubble::Bubble;
    use merge::Merge;
    use quick::Quick;

    macro_rules! test_sort {
        ($test_name:ident, $sort_algorithm_struct:ty) => {
            #[test]
            fn $test_name() {
                let mut list = Vec::from([7, 1, 0, 9, 1, 5, 5, 3, 2, 6, 8, 4]);
                <$sort_algorithm_struct>::sort(&mut list);
                assert_eq!(list, [0, 1, 1, 2, 3, 4, 5, 5, 6, 7, 8, 9]);
            }
        };
    }

    test_sort!(selection_works, Selection);
    test_sort!(bubble_works, Bubble);
    test_sort!(insertion_works, Insertion);
    test_sort!(merge_works, Merge);
    test_sort!(quick_works, Quick);
}
