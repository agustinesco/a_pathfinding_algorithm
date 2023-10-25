use nalgebra::DMatrix;
use std::{collections::HashMap, thread, time, vec};

use rand::{self, Rng};

#[derive(Debug)]

pub struct Board {
    width: usize,
    heigth: usize,
    positions: DMatrix<Option<Node>>,
    obstacles: Vec<Position>,
}

impl Board {
    fn new(heigth: usize, width: usize) -> Board {
        let mut positions: Vec<Option<Node>> = vec![];
        for _ in 0..heigth {
            for _ in 0..width {
                positions.push(None);
            }
        }

        Board {
            positions: DMatrix::from_vec(heigth, width, positions),
            heigth: heigth,
            width: width,
            obstacles: vec![],
        }
    }

    fn generate_obstacles(mut self, amount: i32) -> (Vec<Position>, Board){
        let mut obstacles = vec![];
        for _ in 0..amount{
            let obstacle = Position::generate_random_position(self.heigth, self.width);
            obstacles.push(obstacle);
            self.positions[(obstacle.x, obstacle.y)] = Some(Node::new(None, obstacle, 0, 0));
        }

        self.obstacles = obstacles.clone();

        (obstacles, self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x: x, y: y }
    }

    fn generate_random_position(heigth: usize, width: usize) -> Position {
        let mut rng = rand::thread_rng();
        let random_start_x = rng.gen_range(0..width);
        let random_start_y = rng.gen_range(0..heigth);
        Position::new(random_start_x, random_start_y)
    }

    fn distance_to_other_position(self, other_point: Position) -> i32 {
        let x_distance = i32::abs(other_point.x as i32 - self.x as i32);
        let y_distance = i32::abs(other_point.y as i32 - self.y as i32);

        let remaining = i32::abs(x_distance - y_distance);

        return 14 * i32::min(x_distance, y_distance) + 10 * remaining;
    }

    fn get_neigthbours(self, board: &Board) -> Vec<Position> {
        let mut positions = vec![];
        if self.x > 0 {
            positions.push(Position {
                x: self.x - 1,
                y: self.y,
            }); // left
            if self.y > 0 {
                positions.push(Position {
                    x: self.x - 1,
                    y: self.y - 1,
                }); // left down
            }
            if self.y < board.heigth{
                positions.push(Position {
                    x: self.x - 1,
                    y: self.y + 1,
                }); // left up
            }
        }
        if self.y > 0 {
            positions.push(Position {
                x: self.x,
                y: self.y - 1,
            }); //down
        }
        if self.y < board.heigth{
            positions.push(Position {
                x: self.x,
                y: self.y + 1,
            }); // up
        }
        if self.x < board.width{
            positions.push(Position {
                x: self.x + 1,
                y: self.y,
            }); // right
            if self.y > 0 {
                positions.push(Position {
                    x: self.x + 1,
                    y: self.y - 1,
                }); // right down
            }
            if self.y < board.heigth{
                positions.push(Position {
                    x: self.x + 1,
                    y: self.y + 1,
                }); // right up
            }
        }

        positions
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    comes_from: Option<Box<Node>>,
    pos: Position,
    // g_cost determine the distance from the node to the begining
    g_cost: i32,
    // h_cost determine the distance to the objective node
    h_cost: i32,
    // f_cost determines the best_candidate, it's the g_cost + the h_cost
    f_cost: i32,
}

impl Node {
    fn new(comes_from: Option<Box<Node>>, position: Position, g_cost: i32, h_cost: i32) -> Node {
        Node {
            comes_from: comes_from,
            pos: position,
            g_cost: g_cost,
            h_cost: h_cost,
            f_cost: g_cost + h_cost,
        }
    }
}

fn rebuild_path(node: Node) -> Vec<Position> {
    let mut current: Node = node;
    let mut result: Vec<Position> = vec![];
    while !current.comes_from.is_none() {
        result.push(current.pos);
        current = *current.comes_from.unwrap();
    }
    result
}

fn a_start_find(start_point: Position, end_point: Position, mut board: Board, obstacles: Vec<Position>) -> Vec<Position> {
    let h_cost = start_point.distance_to_other_position(end_point) as i32;
    let start_node: Node = Node {
        comes_from: None,
        pos: start_point,
        g_cost: 0,
        h_cost: h_cost,
        f_cost: 0,
    };
    let mut to_search_values: HashMap<Position, Node> = HashMap::new();
    let mut processed_targets: HashMap<Position, i32> = HashMap::new();

    for obstacle in obstacles{
        processed_targets.insert(obstacle, 1);
    }

    to_search_values.insert(start_node.pos, start_node.clone());

    while !to_search_values.is_empty() {
        let (best_candidate, node) = to_search_values
            .clone()
            .into_iter()
            .min_by(|(_, node_1), (_, node_2)| node_1.f_cost.cmp(&node_2.f_cost))
            .unwrap();

         thread::sleep(time::Duration::from_millis(300));
         print!("\x1B[2J");
         print_board(&board, &start_point, &end_point, &vec![]);
        if best_candidate.eq(&end_point) {
            let path_to_point = rebuild_path(node.clone());
            print_board(&board, &start_point, &end_point, &path_to_point);
            return path_to_point;
        }

        let neighbours = get_neigthbours(&best_candidate, processed_targets.clone(), &board);

        to_search_values.remove(&best_candidate);
        processed_targets.insert(best_candidate, 1);
        for n in neighbours {
            let h_cost = n.distance_to_other_position(end_point) as i32;
            let g_cost = node.g_cost + 1;
            let new_node = Node::new(Some(Box::new(node.clone())), n, g_cost, h_cost);
            to_search_values.insert(n, new_node.clone());
            board.positions[(n.x, n.y)] = Some(new_node);
        }
    }
    return vec![];
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let (obstacles, board) = Board::new(10, 10).generate_obstacles(30);

    a_start_find(
        Position::new(0, 0),
        Position::new(9, 9),
        board,
        obstacles
    );
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed)
}

fn get_neigthbours(
    position: &Position,
    processed_values: HashMap<Position, i32>,
    board: &Board,
) -> Vec<Position> {
    position
        .get_neigthbours(&board)
        .into_iter()
        .filter(|n| !processed_values.contains_key(&n) && n.x < board.width && n.y < board.heigth && !board.obstacles.contains(&n))  
        .collect()
}

fn print_board(
    board: &Board,
    start_point: &Position,
    end_point: &Position,
    path_to_point: &Vec<Position>,
) {
    for i in 0..board.heigth {
        for j in 0..board.width {
            if i == start_point.y && j == start_point.x {
                print!("{:^8}|", "start");
            } else if i == end_point.y && j == end_point.x {
                print!("{:^8}|", "goal");
            } else if path_to_point.contains(&Position { x: j, y: i }) {
                print!("{:^8}|", "->");
            } else {
                let elem = &board.positions[(j, i)];
                match elem {
                    None => print!("{:^8}|", "*"),
                    Some(node) => {
                        if node.f_cost == 0{
                            print!("{:^8}|", "obs")
                        }
                        else{
                            print!("{:^8}|", format!("g{},h{}", node.g_cost, node.h_cost))
                        }
                    }
                }
            }
        }
        println!();
    }
}

