use itertools::Itertools;

#[derive(Default, Debug, Clone)]
struct DiskMap {
    map: Vec<usize>,
    expanded: Vec<String>, // TODO: change this to an enum: enum DiskMapElement { File(usize), Empty(usize) }
    compacted: Vec<String>, // TODO: change this to an enum: enum DiskMapElement { File(usize), Empty(usize) }
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
            vec![
                itertools::repeat_n(i, file).map(|n| n.to_string()).join(""),
                empty.to_string()
            ]
        }).collect();
    }

    fn compact(&mut self) {
        let mut empty_spaces: Vec<_> = self.expanded.iter().enumerate()
            .filter(|(i, _)| i%2 == 1).map(|(i, e)| (i, e.parse::<usize>().unwrap())).collect();
        let filled_elements: Vec<_> = self.expanded.iter().enumerate().filter(|(i, _)| i % 2 == 0).rev().collect();
        let mut compacted: Vec<_> = self.expanded.clone().iter().map(|f| f.to_string()).collect();
        println!("Empty spaces: {:?}", empty_spaces);
        println!("Filled elements: {:?}", filled_elements);
        for (i, f) in filled_elements {
            let room_required = f.len();
            let empty_space = empty_spaces.iter_mut()
                .find(|(_, e)| *e >= room_required);
            if let Some(empty_space) = empty_space {
                let j = empty_space.0;
                compacted[j] = f.clone();
                println!("f: {f}, i: {i}, empty_space: {empty_space:?}, room_required: {room_required}");
                empty_space.1 -= room_required;
                println!("Empty spaces: {:?}", empty_spaces);
                println!("Compacted: {:?}", compacted);
            } 
        }

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
    // disk_map.set_checksum();

    println!("Expanded {:?}", disk_map.expanded);
    println!("Compacted {:?}", disk_map.compacted);

    Ok(disk_map.checksum.unwrap_or_default().to_string())
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
