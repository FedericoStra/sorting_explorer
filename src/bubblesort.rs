use super::{SortingAlgorithm, SortingStatsTrait};

pub struct BubbleSort;

impl SortingAlgorithm for BubbleSort {
    fn sort_with_stats<T, S>(&self, slice: &mut [T], mut stats: S) -> S
    where
        T: Ord,
        S: SortingStatsTrait,
    {
        let mut something_got_swapped = true;
        while something_got_swapped {
            something_got_swapped = false;
            for i in 1..slice.len() {
                stats.comparison();
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    stats.swap();
                    something_got_swapped = true;
                }
            }
        }
        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SortingStats;

    #[test]
    fn bubblesort() {
        let mut v = [3, 2, 1, 7, 6];
        assert_eq!(
            BubbleSort.sort_with_stats(&mut v[..], SortingStats::default()),
            SortingStats {
                comparisons: 12,
                swaps: 4
            }
        );
        assert_eq!(v, [1, 2, 3, 6, 7]);
    }
}
