use crate::Maze::{Branch, Leaf};
use std::cell::RefCell;

enum Exploration {
    UnExplored,
    Explored,
}

enum Maze<'a> {
    Branch { label: String, left: &'a Maze<'a>, right: &'a Maze<'a>, status: RefCell<Exploration> },
    Leaf { label: String },
}

impl<'a> Maze<'a> {
    fn explore(&self, trace: &mut Vec<String>) {
        match self {
            Leaf { label } => trace.push(label.to_string()),
            Branch { label, left, right, status } => {
                trace.push(label.to_string());
                left.explore(trace);
                right.explore(trace);
            }
        }
    }
}

fn leaf(label: &str) -> Maze {
    Leaf { label: label.to_string() }
}

fn branch<'a>(label: &str, left: &'a Maze<'a>, right: &'a Maze<'a>) -> Maze<'a> {
    Branch { label: label.to_string(), left, right, status: RefCell::new(Exploration::UnExplored) }
}

fn main() {
    let leaf5 = leaf("5");
    let leaf4 = leaf("4");
    let branch3 = branch("3", &leaf4, &leaf5);

    let leaf2 = &leaf("2");
    let leaf8 = &leaf("8");

    let branch1 = &branch("1", leaf2, &branch3);

    let branch7 = &branch("7", &leaf5, leaf8);
    let branch6 = &branch("6", &branch3, branch7);

    let my_maze = branch("0", branch1, branch6);

    let mut trace: Vec<String> = vec![];
    my_maze.explore(&mut trace);

    println!("{:?}", trace);

    assert_eq!(
            trace,
            vec!["0", "1", "2", "3", "4", "5", "6", "3", "4", "5", "7", "5", "8"],
    );

    assert_eq!(
            trace,
            vec!["0", "1", "2", "3", "4", "5", "6", "3", "7", "5", "8"],
        );
}
