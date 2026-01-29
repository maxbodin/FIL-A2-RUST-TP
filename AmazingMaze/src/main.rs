enum Exploration {
    UnExplored,
    Explored,
}

enum Maze<'a> {
    Branch { label: String, left: &'a Maze<'a>, right: &'a Maze<'a> },
    Leaf { label: String },
}

fn leaf(label: &str) -> Maze {
    Maze::Leaf { label: label.to_string() }
}

fn branch<'a>(label: &str, left: &'a Maze<'a>, right: &'a Maze<'a>) -> Maze<'a> {
    Maze::Branch { label: label.to_string(), left, right }
}

fn main() {
    let leaf4 = leaf("4");
    let leaf5 = leaf("5");
    let maze3 = branch("3", &leaf4, &leaf5);


    let my_maze = branch("0",
                         &branch("1",
                                 &leaf("2"),
                                 &maze3),
                         &branch("6",
                                 &maze3,
                                 &branch("7",
                                         &leaf5,
                                         &leaf("8"),
                                 ),
                         ),
    );
}
