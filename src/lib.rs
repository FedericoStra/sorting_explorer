mod bubblesort;
mod insertionsort;

pub trait SortingAlgorithm {
    fn sort_with_stats<T, S>(&self, slice: &mut [T], stats: S) -> S
    where
        T: Ord,
        S: SortingStatsTrait;

    fn sort<'s, T: Ord>(&self, slice: &'s mut [T]) -> &'s mut [T] {
        self.sort_with_stats(slice, EmptySortingStats);
        slice
    }
}

pub trait SortingStatsTrait {
    fn comparisons(&mut self, n: i64);
    fn swaps(&mut self, n: i64);

    #[inline]
    fn comparison(&mut self) {
        self.comparisons(1)
    }

    #[inline]
    fn swap(&mut self) {
        self.swaps(1)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EmptySortingStats;
impl SortingStatsTrait for EmptySortingStats {
    #[inline]
    fn comparisons(&mut self, _n: i64) {}

    #[inline]
    fn swaps(&mut self, _n: i64) {}
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SortingStats {
    comparisons: i64,
    swaps: i64,
}

impl SortingStatsTrait for SortingStats {
    #[inline]
    fn comparisons(&mut self, n: i64) {
        self.comparisons += n
    }

    #[inline]
    fn swaps(&mut self, n: i64) {
        self.swaps += n
    }
}

struct NoopSort;

impl SortingAlgorithm for NoopSort {
    fn sort_with_stats<T, S>(&self, _slice: &mut [T], stats: S) -> S
    where
        T: Ord,
        S: SortingStatsTrait,
    {
        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noopsort() {
        let mut v = [3, 2, 1, 7, 6];
        assert_eq!(
            NoopSort.sort_with_stats(&mut v[..], EmptySortingStats),
            EmptySortingStats
        );
        assert_eq!(v, [3, 2, 1, 7, 6]);
    }
}
