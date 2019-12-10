#![allow(dead_code)]

use crate::*;
use id_tree::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Orbits(String, String);
pub struct Data(Vec<Orbits>);

impl FromStr for Orbits {
    type Err = AocErr;

    fn from_str(s: &str) -> AocResult<Orbits> {
        let ix = s
            .find(')')
            .ok_or_else(|| custom_err("Orbit relation invalid"))?;
        let (s1, s2) = s.split_at(ix);

        Ok(Orbits(s1.to_string(), s2[1..].to_string()))
    }
}

impl FromStr for Data {
    type Err = AocErr;

    fn from_str(s: &str) -> AocResult<Self> {
        let v: ParseLineVec<Orbits> = s.parse()?;
        Ok(Data(v.0))
    }
}

fn build_tree(data: Data) -> Tree<(String, usize)> {
    use id_tree::InsertBehavior::*;

    let mut map = HashMap::<String, HashSet<String>>::new();

    for o in data.0.iter() {
        map.entry(o.0.clone()).or_default().insert(o.1.clone());
    }

    let mut tree = TreeBuilder::new()
        .with_node_capacity(data.0.len() / 2)
        .build();

    let root_id: NodeId = tree
        .insert(Node::new(("COM".to_string(), 0)), AsRoot)
        .unwrap();

    let mut q = VecDeque::new();
    q.push_back(("COM".to_string(), 0, root_id));

    while let Some((key, level, parent)) = q.pop_front() {
        let childs = match map.get(&key) {
            Some(childs) => childs,
            None => continue,
        };

        for child in childs {
            let level = level + 1;
            let child_id: NodeId = tree
                .insert(Node::new((child.clone(), level)), UnderNode(&parent))
                .unwrap();
            q.push_back((child.clone(), level, child_id))
        }
    }

    tree
}

fn calc_checksum(data: Data) -> AocResult<usize> {
    let tree = build_tree(data);

    let root_id = tree.root_node_id().unwrap();
    let sum = tree
        .traverse_pre_order(&root_id)
        .unwrap()
        .map(|n| n.data().1)
        .sum();

    Ok(sum)
}

fn calc_orbit_moves(data: Data) -> AocResult<usize> {
    let tree = build_tree(data);
    let root_id = tree.root_node_id().unwrap();

    let find_node = |s: String| {
        tree.traverse_pre_order(&root_id)
            .unwrap()
            .find(|n| n.data().0 == s)
            .unwrap()
    };

    let get_parent = |n: &Node<(String, usize)>| tree.get(n.parent().unwrap()).unwrap();

    let mut left = find_node("YOU".to_string());
    let mut right = find_node("SAN".to_string());
    let mut steps = 0;

    //Find least common ancestor
    while left != right {
        let left_level = left.data().1;
        let right_level = right.data().1;

        use std::cmp::Ordering::*;
        match left_level.cmp(&right_level) {
            Less => {
                right = get_parent(right);
                steps += 1;
            }
            Greater => {
                left = get_parent(left);
                steps += 1;
            }
            Equal => {
                right = get_parent(right);
                left = get_parent(left);
                steps += 2;
            }
        }
    }

    Ok(steps - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> AocResult<()> {
        let data: Data = parse_file(FileType::Example, 6, 1)?;
        assert_eq!(data.0[0], Orbits("COM".to_string(), "B".to_string()));

        Ok(())
    }

    #[test]
    fn part1() -> AocResult<()> {
        let data: Data = parse_file(FileType::Example, 6, 1)?;
        assert_eq!(calc_checksum(data)?, 42);

        let data: Data = parse_file(FileType::Input, 6, 1)?;
        assert_eq!(calc_checksum(data)?, 139_597);

        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        let data: Data = parse_file(FileType::Example, 6, 2)?;
        assert_eq!(calc_orbit_moves(data)?, 4);

        let data: Data = parse_file(FileType::Input, 6, 1)?;
        assert_eq!(calc_orbit_moves(data)?, 286);

        Ok(())
    }
}
