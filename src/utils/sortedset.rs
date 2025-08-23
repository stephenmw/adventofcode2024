#[derive(Clone, Debug, Default)]
pub struct SortedSet<T> {
    data: Vec<T>,
}

impl<T: Ord> SortedSet<T> {
    pub fn insert(&mut self, val: T) -> bool {
        match self.data.binary_search(&val) {
            Ok(_) => false,
            Err(idx) => {
                self.data.insert(idx, val);
                true
            }
        }
    }

    pub fn remove(&mut self, val: &T) -> bool {
        let Ok(idx) = self.data.binary_search(&val) else {
            return false;
        };
        self.data.remove(idx);
        true
    }

    pub fn next_gt(&self, val: &T) -> Option<&T> {
        let idx = match self.data.binary_search(&val) {
            Ok(x) => x.checked_add(1)?,
            Err(x) => x,
        };

        self.data.get(idx)
    }

    pub fn next_lt(&self, val: &T) -> Option<&T> {
        let idx = self.data.binary_search(&val).unwrap_or_else(|x| x);
        self.data.get(idx.checked_sub(1)?)
    }
}

impl<T: Ord + Eq> From<Vec<T>> for SortedSet<T> {
    fn from(mut data: Vec<T>) -> Self {
        data.sort_unstable();
        data.dedup();
        Self { data }
    }
}
