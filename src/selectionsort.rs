use super::{SortingAlgorithm, SortingStatsTrait};

#[derive(Debug, Default)]
pub struct SelectionSort;

impl SortingAlgorithm for SelectionSort {
    fn sort_with_stats<T, S>(&self, slice: &mut [T], mut stats: S) -> S
    where
        T: Ord,
        S: SortingStatsTrait,
    {
        if slice.len() <= 1 {
            return stats;
        }

        for i in 0..slice.len() - 1 {
            let mut imin = i;
            for j in i + 1..slice.len() {
                stats.comparison();
                if slice[j] < slice[imin] {
                    imin = j;
                }
            }
            if imin > i {
                slice.swap(i, imin);
                stats.swap();
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
    fn selectionsort() {
        let mut v = [3, 2, 4, 1, 5, 7, 6];
        let stats = SelectionSort.sort_with_stats(&mut v[..], SortingStats::default());
        assert_eq!(v, [1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(
            stats,
            SortingStats {
                comparisons: 21,
                swaps: 3
            }
        );
    }
}
