#[derive(Default, Debug, Clone)]
struct DiskMap {
    map: Vec<usize>,
    expanded: Vec<String>,
    compacted: Vec<String>,
    checksum: Option<usize>,
}

impl DiskMap {
    fn parse(input: &str) -> Self {
        Self {
            map: input
                .lines()
                .flat_map(|line| line.chars()
                    .map(|d| d.to_digit(10).unwrap() as usize))
                .collect(),
            expanded: Vec::new(),
            compacted: Vec::new(),
            checksum: None,
        }
    }

    fn expand(&mut self) {
        let chunks = self.map.chunks(2);
        self.expanded = chunks.into_iter().enumerate().flat_map(|(i, chunk)| {
            let file = chunk[0];
            let empty = *chunk.get(1).unwrap_or(&0);
            itertools::repeat_n(i, file).map(|n| n.to_string())
                .chain(itertools::repeat_n(".", empty).map(|n| n.to_string()))
        }).collect();
    }

    fn compact(&mut self) {
        let empty_count = self.expanded.iter().filter(|&c| c == ".").count();
        let filled_elements: Vec<_> = self.expanded.iter().filter(|&c| c != ".").rev().take(empty_count).collect();
        let compacted: Vec<_> = self.expanded.iter().take(self.expanded.len() - empty_count).map(|f| f.to_string()).collect();
        let mut elements = filled_elements.iter();
        let compacted = compacted.into_iter().map(|c| {
            if c == "." {
                let next = elements.next().unwrap();
                next.to_string()
            } else {
                c
            }
        }).collect();

        self.compacted = compacted;
    }

    fn set_checksum(&mut self) {
        let sum: usize = self.compacted.iter().enumerate().map(|(i, c)| i * c.parse::<usize>().unwrap()).sum();
        self.checksum = Some(sum);
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut disk_map = DiskMap::parse(input);
    disk_map.expand();
    disk_map.compact();
    disk_map.set_checksum();
    Ok(disk_map.checksum.unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
