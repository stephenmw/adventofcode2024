// Contains a range [start, end)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Range {
        Self { start, end }
    }

    pub fn length(&self) -> u64 {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    // Split into two non-overlapping ranges. If length >= self.length, the
    // first range will be self and the second range will be empty.
    pub fn split_front(&self, length: u64) -> (Range, Range) {
        let a = Range {
            start: self.start,
            end: (self.start + length).min(self.end),
        };
        let b = Range {
            start: (self.start + length).min(self.end),
            end: self.end,
        };

        (a, b)
    }

    // Split into two non-overlapping ranges. If length >= self.length, the
    // first range will be empty and the second range will be self.
    pub fn split_back(&self, length: u64) -> (Range, Range) {
        let a = Range {
            start: self.start,
            end: self.end.saturating_sub(length).max(self.start),
        };
        let b = Range {
            start: self.end.saturating_sub(length).max(self.start),
            end: self.end,
        };

        (a, b)
    }

    pub fn iter(&self) -> impl Iterator<Item = u64> {
        self.start..self.end
    }

    pub fn sum(&self) -> u64 {
        self.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_is_empty_test() {
        assert!(Range::new(0, 0).is_empty());
        // non-canonical range
        assert!(Range::new(20, 1).is_empty());

        // not zero
        assert!(!Range::new(0, 1).is_empty());
    }

    #[test]
    fn split_front_test() {
        let r = Range::new(0, 10);
        assert_eq!(r.split_front(4), (Range::new(0, 4), Range::new(4, 10)));
        assert_eq!(r.split_front(10), (r, Range::new(10, 10)));
        assert_eq!(r.split_front(11), (r, Range::new(10, 10)));
    }

    #[test]
    fn split_back_test() {
        let r = Range::new(0, 10);
        assert_eq!(r.split_back(4), (Range::new(0, 6), Range::new(6, 10)));
        assert_eq!(r.split_back(10), (Range::new(0, 0), r));
        assert_eq!(r.split_back(11), (Range::new(0, 0), r));
    }
}
