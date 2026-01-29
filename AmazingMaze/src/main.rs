enum Exploration {
    UnExplored,
    Explored,
}

enum Maze<'a> {
    Branch { label: String, left: &'a Maze<'a>, right: &'a Maze<'a> },
    Leaf { label: String },
}

fn main() {
    let maze5 = Maze::Leaf{label: "5".to_string()};

    let maze3 = Maze::Branch{
        label: "3".to_string(),
        left: &Maze::Leaf { label: "4".to_string() },
        right: &maze5
    };

    let my_maze = Maze::Branch{
        label: "0".to_string(),
        left: &Maze::Branch {
            label: "1".to_string(),
            left: &Maze::Leaf {
                label: "2".to_string()
            },
            right: &maze3 },
        right: &Maze::Branch {
            label: "6".to_string(),
            left: &maze3,
            right: &Maze::Branch {
                label: "7".to_string(),
                left: &maze5,
                right: &Maze::Leaf {
                    label: "8".to_string()
                },
            },
        }
    };
}
