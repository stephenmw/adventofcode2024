use std::collections::VecDeque;

use crate::range::Range;
use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    let mut fs = FileSystem::new(&data);
    fs.compact_frag();
    let checksum = fs.checksum();
    Ok(checksum.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    let mut fs = FileSystem::new(&data);
    fs.compact();
    let checksum = fs.checksum();
    Ok(checksum.to_string())
}

struct FileSystem {
    files: Vec<Vec<Range>>,
    free_list: FreeList,
}

impl FileSystem {
    fn new(blocks: &[u8]) -> Self {
        let mut files = Vec::new();
        let mut free_list = VecDeque::new();

        let mut cur = 0u64;

        for (i, &size) in blocks.iter().enumerate() {
            let end = cur + size as u64;
            let r = Range::new(cur, end);
            cur = end;

            match i % 2 == 0 {
                true => files.push(vec![r]),
                false => free_list.push_back(r),
            }
        }

        Self {
            files,
            free_list: FreeList::new(free_list.iter()),
        }
    }

    fn compact_frag(&mut self) {
        for file in self.files.iter_mut().rev() {
            let Some(mut cur) = file.pop() else {
                continue;
            };

            while !cur.is_empty() {
                let Some(free) = self.free_list.pop_first(1) else {
                    file.push(cur);
                    return;
                };

                if free.start > cur.end {
                    file.push(cur);
                    self.free_list.add(free);
                    return;
                }

                let (new, rest_free) = free.split_front(cur.length());
                let (new_cur, _) = cur.split_back(new.length());
                cur = new_cur;
                file.push(new);

                if !rest_free.is_empty() {
                    self.free_list.add(rest_free);
                }
            }
        }
    }

    fn compact(&mut self) {
        for file in self.files.iter_mut().rev() {
            let Some(cur) = file.last_mut() else {
                continue;
            };

            let Some(free) = self.free_list.pop_first(cur.length()) else {
                continue;
            };

            if free.start < cur.start {
                let (new, rest_free) = free.split_front(cur.length());
                *cur = new;
                self.free_list.add(rest_free);
            }
        }
    }

    fn checksum(&self) -> u64 {
        self.files
            .iter()
            .enumerate()
            .map(|(i, ranges)| i as u64 * ranges.iter().map(|x| x.sum()).sum::<u64>())
            .sum()
    }
}

#[derive(Clone, Debug, Default)]
struct FreeList {
    free_list: [VecDeque<Range>; 10],
}

impl FreeList {
    fn new<'a, I: Iterator<Item = &'a Range>>(ranges: I) -> Self {
        let mut ret = Self::default();
        for &r in ranges {
            ret.free_list[r.length() as usize].push_back(r);
        }
        for queue in &mut ret.free_list {
            queue.make_contiguous().sort();
        }
        ret
    }
    fn add(&mut self, r: Range) {
        let queue = &mut self.free_list[r.length() as usize];
        let index = match queue.binary_search(&r) {
            Ok(index) => index,
            Err(index) => index,
        };
        queue.insert(index, r);
    }

    // pop the first free space of at least length
    fn pop_first(&mut self, length: u64) -> Option<Range> {
        let queue = self.free_list[length as usize..]
            .iter_mut()
            .filter_map(|q| {
                let r = q.front()?.clone();
                Some((q, r))
            })
            .min_by_key(|(_, r)| *r)
            .map(|(q, _)| q)?;
        queue.pop_front()
    }
}

mod parser {
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<u8>> {
        let block = one_of("0123456789").map(|x| x.to_digit(10).unwrap() as u8);
        ws_all_consuming(many1(block)).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "1928")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "2858")
    }
}
