use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct Trie<'a> {
    root: TrieNode<'a>,
}

impl<'a> Trie<'a> {
    pub fn insert(&mut self, word: &'a str) {
        let mut node = &mut self.root;
        for c in word.bytes() {
            node = node.get_mut_or_default(c);
        }
        node.word = Some(word);
    }

    pub fn prefix_of<'b>(&'a self, word: &'b str) -> PrefixOfIter<'a, 'b> {
        PrefixOfIter {
            cur_node: Some(&self.root),
            remaining_word: word.as_bytes(),
        }
    }
}

impl<'a> FromIterator<&'a str> for Trie<'a> {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut trie = Trie::default();
        for word in iter {
            trie.insert(word);
        }
        trie
    }
}

pub struct PrefixOfIter<'a, 'b> {
    cur_node: Option<&'a TrieNode<'a>>,
    remaining_word: &'b [u8],
}

impl<'a, 'b> PrefixOfIter<'a, 'b> {
    fn advance(&mut self) -> bool {
        let Some(node) = self.cur_node else {
            return false;
        };

        let Some(&c) = self.remaining_word.first() else {
            self.cur_node = None;
            return false;
        };

        self.cur_node = node.get(c);
        self.remaining_word = &self.remaining_word[1..];

        true
    }
}

impl<'a, 'b> Iterator for PrefixOfIter<'a, 'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(word) = self.cur_node?.word {
                self.advance();
                return Some(word);
            }

            if !self.advance() {
                return None;
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
struct TrieNode<'a> {
    children: BTreeMap<u8, TrieNode<'a>>,
    word: Option<&'a str>,
}

impl<'a> TrieNode<'a> {
    fn get(&self, c: u8) -> Option<&TrieNode<'a>> {
        self.children.get(&c)
    }

    fn get_mut_or_default(&mut self, c: u8) -> &mut TrieNode<'a> {
        self.children.entry(c).or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_of_test() {
        let trie = Trie::from_iter(["ab", "abb", "bc", "abbc"]);
        let res: Vec<_> = trie.prefix_of("abbd").collect();
        assert_eq!(&res, &["ab", "abb"]);
    }
}
