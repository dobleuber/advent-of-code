use std::collections::BTreeMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut pages: Vec<(i32, i32)> = vec![];
    let mut manuals: Vec<Vec<i32>> = vec![];
    let mut page_order: BTreeMap<i32, Vec<i32>> = BTreeMap::new();

    input.lines().for_each(|line| {
        if line.contains("|") {
            let mut parts = line.split("|");
            let page = parts.next().unwrap().parse::<i32>().unwrap();
            let manual = parts.next().unwrap().parse::<i32>().unwrap();
            pages.push((page, manual));
        } else if line.contains(",") {
            let page: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            manuals.push(page);
        }
    });

    pages.iter().for_each(|(l, h)| {
        page_order.entry(*h).or_default().push(*l);
    });

    let valid_manuals: Vec<_> = manuals.iter().filter(|manual| {
        for (i, p) in manual.iter().enumerate() {
            let invalid_pages: Vec<i32> = page_order.get(p).unwrap_or(&vec![]).to_vec();
            let next_pages = &manual[i + 1..];
            let valid_manual = !next_pages.iter().any(|n| {
                invalid_pages.contains(n)
            });
            if !valid_manual {
                return false;
            }
        }
        true
    }).collect();
    
    let middles_sum: i32 = valid_manuals.iter().map(|manual| manual[manual.len() / 2]).sum();

    Ok(middles_sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
