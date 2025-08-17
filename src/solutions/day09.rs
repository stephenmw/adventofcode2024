use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    let mut fs = FileSystem::new(&data);
    fs.defrag();
    let checksum = fs.checksum();
    Ok(checksum.to_string())
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    bail!("not yet implemented")
}

#[derive(Debug, Clone, Copy)]
struct FileID(u16);

impl FileID {
    const FREE: Self = Self(u16::MAX);

    const fn new(id: u16) -> Option<Self> {
        if id == u16::MAX { None } else { Some(Self(id)) }
    }

    fn is_free(&self) -> bool {
        self.0 == u16::MAX
    }

    fn get(&self) -> Option<u16> {
        if self.0 == u16::MAX {
            None
        } else {
            Some(self.0)
        }
    }
}

struct FileSystem {
    sectors: Vec<FileID>,
}

impl FileSystem {
    fn new(blocks: &[u8]) -> Self {
        let disk_size: usize = blocks.iter().map(|&x| x as usize).sum();
        let mut sectors = Vec::with_capacity(disk_size);

        for (i, &block_size) in blocks.iter().enumerate() {
            match i % 2 == 0 {
                true => {
                    let id = FileID::new((i / 2) as u16).unwrap();
                    for _ in 0..block_size {
                        sectors.push(id);
                    }
                }
                false => {
                    for _ in 0..block_size {
                        sectors.push(FileID::FREE);
                    }
                }
            }
        }

        Self { sectors }
    }

    fn defrag(&mut self) {
        let Some(mut free_cur) = self.find_next_free(0) else {
            return;
        };
        let Some(mut filled_cur) = self.find_last_filled(self.sectors.len() - 1) else {
            return;
        };

        while free_cur < filled_cur {
            self.sectors.swap(free_cur, filled_cur);

            free_cur = match self.find_next_free(free_cur) {
                Some(next) => next,
                None => break,
            };
            filled_cur = match self.find_last_filled(filled_cur) {
                Some(next) => next,
                None => break,
            };
        }
    }

    fn find_next_free(&self, start: usize) -> Option<usize> {
        self.sectors
            .iter()
            .skip(start)
            .position(|x| x.is_free())
            .map(|p| p + start)
    }

    fn find_last_filled(&self, end: usize) -> Option<usize> {
        self.sectors[..end + 1]
            .iter()
            .rev()
            .position(|x| !x.is_free())
            .map(|p| end - p)
    }

    fn checksum(&self) -> u64 {
        self.sectors
            .iter()
            .enumerate()
            .filter_map(|(i, x)| Some(i as u64 * x.get()? as u64))
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
        //assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "")
    }
}
