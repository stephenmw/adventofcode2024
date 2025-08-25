use std::cmp;

pub struct RevHeapElem<K, V> {
    pub key: K,
    pub value: V,
}

impl<K: PartialEq, V> PartialEq for RevHeapElem<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Eq, V> Eq for RevHeapElem<K, V> {}

impl<K: Ord, V> PartialOrd for RevHeapElem<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Ord for RevHeapElem<K, V> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.key.cmp(&other.key).reverse()
    }
}
