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
    free_list: VecDeque<Range>,
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

        Self { files, free_list }
    }

    fn compact_frag(&mut self) {
        for file in self.files.iter_mut().rev() {
            let Some(mut cur) = file.pop() else {
                continue;
            };

            while !cur.is_empty() {
                let Some(free) = self.free_list.pop_front() else {
                    file.push(cur);
                    return;
                };

                if free.start > cur.end {
                    file.push(cur);
                    self.free_list.push_front(free);
                    return;
                }

                let (new, rest_free) = free.split_front(cur.length());
                let (new_cur, _) = cur.split_back(new.length());
                cur = new_cur;
                file.push(new);

                if !rest_free.is_empty() {
                    self.free_list.push_front(rest_free);
                }
            }
        }
    }

    fn compact(&mut self) {
        for file in self.files.iter_mut().rev() {
            let Some(cur) = file.last_mut() else {
                continue;
            };

            let Some(free) = self
                .free_list
                .iter_mut()
                .find(|x| x.length() >= cur.length())
            else {
                continue;
            };

            if free.start < cur.start {
                let (new, rest_free) = free.split_front(cur.length());
                *cur = new;
                *free = rest_free;
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
