use super::{SortingAlgorithm, SortingStatsTrait};

#[derive(Debug, Default)]
pub struct InsertionSort {
    binary_search: bool,
}

impl InsertionSort {
    pub fn binary_search() -> InsertionSort {
        InsertionSort {
            binary_search: true,
        }
    }
}

fn binary_search<T, S>(slice: &[T], x: &T, stats: &mut S) -> usize
where
    T: Ord,
    S: SortingStatsTrait,
{
    use std::cmp::Ordering::*;

    let mut size = slice.len();
    if size == 0 {
        return 0;
    }
    let mut base = 0;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        stats.comparison();
        // SAFETY: the call is made safe by the following inconstants:
        // - `mid >= 0`: by definition
        // - `mid < size`: `mid = size / 2 + size / 4 + size / 8 ...`
        let cmp = unsafe { slice.get_unchecked(mid) }.cmp(x);
        base = if cmp == Greater { base } else { mid };
        size -= half;
    }
    // SAFETY: base is always in [0, size) because base <= mid.
    stats.comparison();
    let cmp = unsafe { slice.get_unchecked(base) }.cmp(x);
    if cmp == Equal {
        base
    } else {
        base + (cmp == Less) as usize
    }
}

impl SortingAlgorithm for InsertionSort {
    fn sort_with_stats<T, S>(&self, slice: &mut [T], mut stats: S) -> S
    where
        T: Ord,
        S: SortingStatsTrait,
    {
        if self.binary_search {
            for i in 1..slice.len() {
                let j = binary_search(&slice[0..i], &slice[i], &mut stats);
                slice[j..=i].rotate_right(1);
                stats.swaps((i - j) as i64);
            }
        } else {
            // for i in 1..slice.len() {
            //     for j in (0..i).rev() {
            //         stats.comparison();
            //         if slice[j] > slice[j + 1] {
            //             slice.swap(j, j + 1);
            //             stats.swap();
            //         } else {
            //             break;
            //         }
            //     }
            // }

            // for i in 1..slice.len() {
            //     for j in (1..=i).rev() {
            //         stats.comparison();
            //         if slice[j - 1] > slice[j] {
            //             slice.swap(j - 1, j);
            //             stats.swap();
            //         } else {
            //             break;
            //         }
            //     }
            // }

            for i in 1..slice.len() {
                let mut j = i;
                while j > 0 && {
                    stats.comparison();
                    slice[j - 1] > slice[j]
                } {
                    slice.swap(j - 1, j);
                    stats.swap();
                    j -= 1;
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
    fn insertionsort() {
        let mut v = [3, 2, 4, 1, 5, 7, 6];
        let stats = InsertionSort::default().sort_with_stats(&mut v[..], SortingStats::default());
        assert_eq!(v, [1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(
            stats,
            SortingStats {
                comparisons: 9,
                swaps: 5
            }
        );
    }

    #[test]
    fn insertionsort_binary_search() {
        let mut v = [3, 2, 4, 1, 5, 7, 6];
        let stats =
            InsertionSort::binary_search().sort_with_stats(&mut v[..], SortingStats::default());
        assert_eq!(v, [1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(
            stats,
            SortingStats {
                comparisons: 17,
                swaps: 5
            }
        );
    }
}
