pub mod heapelem;
pub use heapelem::RevHeapElem;

pub mod trie;
pub use trie::Trie;

use ahash::AHashMap;

pub fn freq_table<T>(values: impl IntoIterator<Item = T>) -> AHashMap<T, u64>
where
    T: Eq + std::hash::Hash,
{
    let mut ret = AHashMap::new();
    for n in values.into_iter() {
        *ret.entry(n).or_insert(0) += 1;
    }
    ret
}
