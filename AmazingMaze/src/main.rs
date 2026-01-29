use crate::Maze::{Branch, Leaf};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
enum Exploration {
    UnExplored,
    Explored,
}

enum Maze {
    Branch { label: String, left: Rc<Maze>, right: Rc<Maze>, status: RefCell<Exploration> },
    Leaf { label: String },
}

impl Maze {
    fn explore(&self, trace: &mut Vec<String>) {
        match self {
            Leaf { label } => trace.push(label.to_string()),
            Branch { label, left, right, status } => {
                trace.push(label.to_string());

                if *status.borrow() == Exploration::UnExplored {
                    status.replace(Exploration::Explored);
                    left.explore(trace);
                    right.explore(trace);
                }
            }
        }
    }
}

fn leaf(label: &str) -> Rc<Maze> {
    Rc::new(Leaf { label: label.to_string() })
}

fn branch(label: &str, left: Rc<Maze>, right: Rc<Maze>) -> Rc<Maze> {
    Rc::new(Branch { label: label.to_string(), left, right, status: RefCell::new(Exploration::UnExplored) })
}

fn maze() -> Rc<Maze> {
    let leaf5 = leaf("5");
    let branch3 = branch("3", leaf("4"), leaf5.clone());

    branch("0",
           branch("1",
                  leaf("2"),
                  branch3.clone()),
           branch("6",
                  branch3.clone(),
                  branch("7",
                         leaf5.clone(),
                         leaf("8"),
                  ),
           ),
    )
}

fn main() {
    let my_maze = maze();

    let mut trace: Vec<String> = vec![];
    my_maze.explore(&mut trace);

    println!("{:?}", trace);

    assert_eq!(
            trace,
            vec!["0", "1", "2", "3", "4", "5", "6", "3", "7", "5", "8"],
        );
}
