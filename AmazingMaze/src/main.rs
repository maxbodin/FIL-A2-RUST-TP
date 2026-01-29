use crate::Maze::{Branch, Leaf};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone, Copy)]
enum Exploration {
    UnExplored,
    PartiallyExplored,
    Explored,
}

enum Maze {
    Branch { label: String, left: Rc<Maze>, right: Rc<Maze>, status: RefCell<Exploration> },
    Leaf { label: String },
}

impl Maze {
    fn leaf(label: &str) -> Rc<Self> {
        Rc::new(Leaf { label: label.to_string() })
    }

    fn branch(label: &str, left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Rc::new(Branch { label: label.to_string(), left, right, status: RefCell::new(Exploration::UnExplored) })
    }

    fn maze() -> Rc<Self> {
        let leaf5 = Self::leaf("5");
        let branch3 = Self::branch("3", Self::leaf("4"), leaf5.clone());

        Self::branch("0",
                     Self::branch("1",
                                  Self::leaf("2"),
                                  branch3.clone()),
                     Self::branch("6",
                                  branch3.clone(),
                                  Self::branch("7",
                                               leaf5.clone(),
                                               Self::leaf("8"),
                                  ),
                     ),
        )
    }

    fn explore(&self, trace: &mut Vec<String>) {
        match self {
            Leaf { label } => trace.push(label.clone()),
            Branch { label, left, right, status } => {
                trace.push(label.clone());

                if *status.borrow() == Exploration::UnExplored {
                    status.replace(Exploration::Explored);
                    left.explore(trace);
                    right.explore(trace);
                }
            }
        }
    }

    fn reset(&self) {
        if let Branch { label: _, left, right, status } = self {
            if *status.borrow() == Exploration::Explored {
                status.replace(Exploration::UnExplored);
                left.reset();
                right.reset();
            }
        }
    }

    fn explore_concurrent(&self, node: Rc<Maze>, work: &mut Vec<Rc<Maze>>, trace: &mut Vec<String>) {
        match self {
            Leaf { label } => trace.push(label.clone()),
            Branch { label, left, right, status } => {
                let current_status = *status.borrow();

                match current_status {
                    Exploration::UnExplored => {
                        trace.push(label.clone());
                        status.replace(Exploration::PartiallyExplored);
                        work.push(node.clone());
                        work.push(left.clone());
                    }
                    Exploration::PartiallyExplored => {
                        status.replace(Exploration::Explored);
                        work.push(right.clone());
                    }
                    Exploration::Explored => {
                        trace.push(label.clone());
                    }
                }
            }
        }
    }
}

fn main() {
    let my_maze = Maze::maze();

    println!("--- Test de la version simple de explore ---");
    let mut trace: Vec<String> = vec![];
    my_maze.explore(&mut trace);

    println!("{:?}", trace);

    assert_eq!(
            trace,
            vec!["0", "1", "2", "3", "4", "5", "6", "3", "7", "5", "8"],
        );

    my_maze.reset();

    println!("--- Test de la version concurrente de explore ---");
    let mut work = vec![my_maze];
    // In a concurrent version, each worker would do the following.
    let mut trace = vec![];
    while work.len() != 0 {
        let node = work.pop().expect("Work stack should not be empty!!");
        node.explore_concurrent(node.clone(), &mut work, &mut trace);
        println!("trace so far: {:?}", trace);
    }

    assert_eq!(
            trace,
            vec!["0", "1", "2", "3", "4", "5", "6", "3", "7", "5", "8"],
        );
}
