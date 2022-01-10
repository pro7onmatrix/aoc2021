use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;

#[derive(Copy, Clone, Debug, PartialEq)]
enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Debug, PartialEq)]
struct Amphipod {
    typ: AmphipodType,
    visited: Vec<usize>,
}

impl Amphipod {
    fn new(typ: char) -> Self {
        let typ = match typ {
            'A' => AmphipodType::Amber,
            'B' => AmphipodType::Bronze,
            'C' => AmphipodType::Copper,
            'D' => AmphipodType::Desert,
            _ => panic!("Invalid type!"),
        };

        Self { typ, visited: Vec::new() }
    }
}

impl std::fmt::Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.typ {
            AmphipodType::Amber  => write!(f, "A"),
            AmphipodType::Bronze => write!(f, "B"),
            AmphipodType::Copper => write!(f, "C"),
            AmphipodType::Desert => write!(f, "D"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    value: Option<Amphipod>,
    index: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: Option<Amphipod>, index: usize) -> Self {
        Self { value, index, left: None, right: None }
    }

    fn add_left(&mut self, value: Option<Amphipod>) {
        let mut offset = 1;
        let mut current = &*self;

        while let Some(node) = &current.right {
            offset += 1;
            current = node.as_ref();
        }

        self.left = Some(Box::new(Node::new(value, self.index + offset)));
    }

    fn add_right(&mut self, value: Option<Amphipod>) {
        self.right = Some(Box::new(Node::new(value, self.index + 1)));
    }

    fn get_left(&self) -> &Node {
        self.left.as_ref().unwrap().as_ref()
    }

    fn get_right(&self) -> &Node {
        self.right.as_ref().unwrap().as_ref()
    }

    fn find_first_incorrect(&mut self) -> Option<&mut Node> {
        if !self.is_correct() {
            return Some(self);
        }

        if let Some(node) = &mut self.right {
            if let Some(found) = node.find_first_incorrect() {
                return Some(found);
            }
        }

        if let Some(node) = &mut self.left {
            if let Some(found) = node.find_first_incorrect() {
                return Some(found);
            }
        }

        None
    }

    fn find_incorrect(&self) -> Vec<&Node> {
        let mut incorrect = Vec::new();

        if !self.is_correct() {
            incorrect.push(self);
        }

        if let Some(node) = &self.right {
            incorrect.append(&mut node.find_incorrect());
        }

        if let Some(node) = &self.left {
            incorrect.append(&mut node.find_incorrect());
        }

        incorrect
    }

    fn is_correct(&self) -> bool {
        if let Some(amphipod) = &self.value {
            match amphipod.typ {
                AmphipodType::Amber => {
                    if self.index == 4 {
                        return true;
                    } else if self.index == 3 {
                        if let Some(node) = &self.right {
                            if let Some(other) = &node.value {
                                match other.typ {
                                    AmphipodType::Amber => return true,
                                    _ => return false,
                                }
                            }
                        }
                    }
                    return false;
                },
                AmphipodType::Bronze => {
                    if self.index == 8 {
                        return true;
                    } else if self.index == 7 {
                        if let Some(node) = &self.right {
                            if let Some(other) = &node.value {
                                match other.typ {
                                    AmphipodType::Bronze => return true,
                                    _ => return false,
                                }
                            }
                        }
                    }
                    return false;
                },
                AmphipodType::Copper => {
                    if self.index == 12 {
                        return true;
                    } else if self.index == 11 {
                        if let Some(node) = &self.right {
                            if let Some(other) = &node.value {
                                match other.typ {
                                    AmphipodType::Copper => return true,
                                    _ => return false,
                                }
                            }
                        }
                    }
                    return false;
                },
                AmphipodType::Desert => {
                    if self.index == 16 {
                        return true;
                    } else if self.index == 15 {
                        if let Some(node) = &self.right {
                            if let Some(other) = &node.value {
                                match other.typ {
                                    AmphipodType::Desert => return true,
                                    _ => return false,
                                }
                            }
                        }
                    }
                    return false;
                },
            }
        }

        true
    }

    fn is_allowed(&mut self, active: &Node) -> bool {
        let amphipod = active.value.as_ref().unwrap();

        if self.value.is_some() {
            return false;
        }

        if amphipod.visited.iter().find(|&&i| i == self.index).is_some() {
            return false;
        }

        match self.index {
            2 | 6 | 10 | 14 => false,
            3 | 4 => amphipod.typ == AmphipodType::Amber,
            7 | 8 => amphipod.typ == AmphipodType::Bronze,
            11 | 12 => amphipod.typ == AmphipodType::Copper,
            15 | 16 => amphipod.typ == AmphipodType::Desert,
            _ => true,
        }
    }

    fn find_path(&self, target: &Node) -> Option<Vec<&Node>> {
        let mut nodes = vec![self];

        if self == target {
            return Some(nodes);
        }

        if let Some(node) = &self.right {
            if let Some(mut path) = node.find_path(target) {
                nodes.append(&mut path);
                return Some(nodes);
            }
        }

        if let Some(node) = &self.left {
            if let Some(mut path) = node.find_path(target) {
                nodes.append(&mut path);
                return Some(nodes);
            }
        }

        None
    }
}

impl std::default::Default for Node {
    fn default() -> Self {
        Self { value: None, index: 0, left: None, right: None }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(value) = &self.value {
            write!(f, "{}: {}, ", self.index, value)?;
        } else {
            write!(f, "{}: ., ", self.index)?;
        }

        if let Some(node) = &self.right {
            write!(f, "{}", node)?;
        }

        if let Some(node) = &self.left {
            writeln!(f, "")?;
            write!(f, "{}", node)?;
        }

        Ok(())
    }
}

fn find_available<'a>(graph: &'a mut Node, root: &'a Node, active: &'a Node) -> Option<(&'a mut Node, usize)> {
    if graph.is_allowed(active) {
        let (ancestor, path1, path2) = find_common_ancestor(root, active, graph);

        if is_path_free(&ancestor, &path1, &path2) {
            return Some((graph, path1.len() + path2.len()));
        }
    }

    if let Some(node) = &mut graph.right {
        if let Some(found) = find_available(node, root, active) {
            return Some(found);
        }
    }

    if let Some(node) = &mut graph.left {
        if let Some(found) = find_available(node, root, active) {
            return Some(found);
        }
    }

    None
}

fn find_common_ancestor<'a>(root: &'a Node,
                            node1: &'a Node,
                            node2: &'a mut Node) -> (&'a Node, Vec<&'a Node>, Vec<&'a Node>)
{
    let mut path1 = root.find_path(node1).unwrap().into_iter();
    let mut path2 = root.find_path(node2).unwrap().into_iter();

    let mut a = root;
    let mut b = root;
    let mut prev;

    loop {
        prev = a;

        a = match path1.next() {
            Some(node) => node,
            None => break,
        };

        b = match path2.next() {
            Some(node) => node,
            None => break,
        };

        if a != b {
            break;
        }
    }

    let mut remainder1 = vec![a];
    let mut remainder2 = vec![b];

    remainder1.append(&mut path1.collect());
    remainder2.append(&mut path2.collect());

    (prev, remainder1, remainder2)
}

fn is_path_free(start: &Node, path1: &[&Node], path2: &[&Node]) -> bool {
    if path1.is_empty() {
        return path2.iter().all(|node| node.value.is_none());
    }

    if path2.is_empty() {
        return start.value.is_none() && path1[..path1.len()-1].iter().all(|node| node.value.is_none());
    }

    start.value.is_none()
        && path1[..path1.len()-1].iter().all(|node| node.value.is_none())
        && path2.iter().all(|node| node.value.is_none())
}

fn solve(graph: &mut Node) -> usize {
    let mut score = usize::MAX;

    let incorrect_nodes = graph.find_incorrect();

    for node in incorrect_nodes.iter() {
        let mut local_score = 0;

        while let Some((available, length)) = find_available(graph, graph, node) {
            available.value = node.value;
            node.value = None;

            available.value.unwrap().visited.push(available.index);

            local_score = length * match available.value.unwrap().typ {
                AmphipodType::Amber => 1,
                AmphipodType::Bronze => 10,
                AmphipodType::Copper => 100,
                AmphipodType::Desert => 1000,
            };

            local_score += solve(graph);

            node.value = available.value;
            available.value = None;
        }

        score = score.min(local_score);
    }


    score
}

fn read_input(fname: &str) -> io::Result<Node> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let mut graph = Node::default();
    let mut amphipods = Vec::new();

    for line in reader.lines().skip(2) {
        let chars: Vec<char> = line?.chars().collect();

        for i in (3..10).step_by(2) {
            amphipods.push(chars[i]);
        }
    }

    let mut current = &mut graph;
    for i in 0..10 {
        if i == 2 {
            current.add_right(Some(Amphipod::new(amphipods[0])));
            current.right.as_mut().unwrap().as_mut().add_right(Some(Amphipod::new(amphipods[4])));
        } else if i == 4 {
            current.add_right(Some(Amphipod::new(amphipods[1])));
            current.right.as_mut().unwrap().as_mut().add_right(Some(Amphipod::new(amphipods[5])));
        } else if i == 6 {
            current.add_right(Some(Amphipod::new(amphipods[2])));
            current.right.as_mut().unwrap().as_mut().add_right(Some(Amphipod::new(amphipods[6])));
        } else if i == 8 {
            current.add_right(Some(Amphipod::new(amphipods[3])));
            current.right.as_mut().unwrap().as_mut().add_right(Some(Amphipod::new(amphipods[7])));
        }

        current.add_left(None);
        current = current.left.as_mut().unwrap().as_mut();
    }

    Ok(graph)
}

// fn traverse(graph: &Node) {
//     if graph.is_correct() {
//         println!("Value {} at index {}", graph.value.as_ref().unwrap(), graph.index);
//     }

//     if let Some(node) = graph.right.as_ref() {
//         traverse(node);
//     }

//     if let Some(node) = graph.left.as_ref() {
//         traverse(node);
//     }

// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let graph = read_input(&args[1]).unwrap();

    println!("{}", graph);

    // let first = graph.left.as_ref().unwrap().as_ref()
    //                  .left.as_ref().unwrap().as_ref()
    //                  .right.as_ref().unwrap().as_ref();

    // let second = graph.left.as_ref().unwrap().as_ref()
    //                   .left.as_ref().unwrap().as_ref()
    //                   .left.as_ref().unwrap().as_ref()
    //                   .left.as_ref().unwrap().as_ref();

    // let (common, path1, path2) = find_common_ancestor(&graph, first, second);

    // println!("Common ancestor of indices {} and {} is index {}",
    //          first.index, second.index, common.index);

    // println!("{:?}", path1.iter().map(|node| node.index).collect::<Vec<usize>>());
    // println!("{:?}", path2.iter().map(|node| node.index).collect::<Vec<usize>>());

    // let first = graph.get_left()
    //                  .get_left()
    //                  .get_right();

    // let second = graph.get_left().get_left();

    // let (start, path1, path2) = find_common_ancestor(&graph, first, second);
    // println!("Does a path exist between {} and {}? {}", first.index, second.index,
    //          is_path_free(&start, &path1, &path2));
}
