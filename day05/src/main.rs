use core::panic;
use std::collections::{HashMap, HashSet};

const EXAMPLE: &str = r#"47|53
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
97,13,75,29,47"#;

fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    // did all of this come from Copilot?!
    let mut parts = input.split("\n\n"); // assuming unix line endings
    let pipes = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split("|");
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let groups = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(",").map(|part| part.parse().unwrap()).collect())
        .collect();
    (pipes, groups)
}

fn get_middle(list: &[u32]) -> u32 {
    let middle = list.len() / 2;
    if list.len() % 2 == 0 {
        println!("even length list: {:?}", list);
        panic!("list is even length");
    }
    list[middle]
}

fn main() {
    assert_eq!(3, get_middle(&[1, 2, 3, 4, 5]));
    // assert_eq!(2, get_middle(&[0, 1, 2, 3, 4, 5]));

    let input = EXAMPLE;
    // let input = include_str!("input.txt");
    let (ordering, update_pages) = parse(input);
    // println!("ordering: {:?}", ordering);
    // println!("update_pages: {:?}", update_pages);

    let ordering_before_to_after: HashMap<u32, HashSet<u32>> =
        ordering
            .iter()
            .fold(HashMap::new(), |mut map, (before, after)| {
                map.entry(*before).or_default().insert(*after);
                map
            });

    let is_in_order = |group: &[u32]| -> bool {
        // pages we've already seen in this group
        let mut seen = HashSet::new();
        for cur_page in group {
            seen.insert(cur_page);
            // pages that must come after this page
            let cur_must_be_before_these = ordering_before_to_after.get(cur_page);
            if let Some(cur_must_be_before_these) = cur_must_be_before_these {
                // let's see if this page is after a page that needs to be after it
                for an_after_page in cur_must_be_before_these {
                    if seen.contains(an_after_page) {
                        // we have already seen this page that must be after the current page
                        // that's not allowed
                        return false;
                    }
                }
            }
        }
        true
    };

    let correct_groups = update_pages
        .iter()
        .filter(|group| is_in_order(group))
        .collect::<Vec<_>>();

    let correct_middle_sum = correct_groups.iter().map(|x| get_middle(x)).sum::<u32>();
    dbg!(correct_middle_sum);

    // -- Part 2 ---
    let incorrect_groups = update_pages
        .iter()
        .filter(|group| !is_in_order(group))
        .collect::<Vec<_>>();

    let fix_order = |group: &[u32]| -> Vec<u32> {
        let mut ordered = group.to_vec();
        ordered.sort_by(|a, b| {
            let pages_that_must_come_after_a = ordering_before_to_after.get(a);
            if let Some(pages_that_must_come_after_a) = pages_that_must_come_after_a {
                if pages_that_must_come_after_a.contains(b) {
                    return std::cmp::Ordering::Less;
                } else {
                    return std::cmp::Ordering::Greater;
                }
            }
            std::cmp::Ordering::Equal
        });
        ordered
    };

    let fixed_groups = incorrect_groups
        .iter()
        .map(|group| fix_order(group))
        .collect::<Vec<_>>();
    let fixed_middle_sum = fixed_groups.iter().map(|x| get_middle(x)).sum::<u32>();
    dbg!(fixed_middle_sum);
}
