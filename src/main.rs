use std::{collections::HashMap, vec};

use rand::{self, Rng};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Position{
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position{
        Position{x: x, y: y}
    }

    fn generate_random_position() -> Position{
        let mut rng = rand::thread_rng();
        let random_start_x = rng.gen_range(0..200);
        let random_start_y = rng.gen_range(0..200);
        Position::new(random_start_x, random_start_y)
    }

    fn distance_to_other_position(self, other_point: Position) -> f64 {
        let number: i32 = i32::pow(other_point.x - self.x, 2) + i32::pow(other_point.y - self.y, 2);

        f64::sqrt(number as f64)
    }

    fn get_neigthbours(self) -> Vec<Position> {
        let left: Position = Position { x: self.x -1, y: self.y }; 
        let right: Position = Position { x: self.x + 1, y: self.y };
        let up: Position = Position { x: self.x, y: self.y + 1 };
        let down: Position = Position { x: self.x, y: self.y - 1 };

        vec![left, right, up, down]
    }
}

#[derive(Clone, Debug)]
pub struct Node{
    comes_from: Option<Box<Node>>,
    pos: Position,
    // g_cost determine the distance from the node to the begining
    g_cost: i32,
    // f_cost determines the best_candidate, it's the g_cost + the h_cost
    f_cost: i32,
}

impl Node {
    fn new(comes_from: Option<Box<Node>>, position: Position, g_cost: i32, f_cost: i32) -> Node {
        Node{
            comes_from: comes_from,
            pos: position,
            g_cost: g_cost,
            f_cost: f_cost,
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

fn a_start_find(start_point: Position, end_point: Position) -> Vec<Position>{
    let start_node: Node = Node { comes_from: None, pos: start_point, g_cost: 0, f_cost: 0};
    let mut to_search_values: HashMap<Position, Node> = HashMap::new();
    let mut processed_targets: Vec<Position> = vec![];
    to_search_values.insert(start_node.pos, start_node.clone());

    while !to_search_values.is_empty() {
        let (best_candidate, node) = 
            to_search_values
            .clone()
            .into_iter()
            .min_by(|(_, node_1), (_, node_2)| {
                node_1.f_cost.cmp(&node_2.f_cost)
            })
            .unwrap();   

        if best_candidate.eq(&end_point) {
            return rebuild_path(node.clone()); 
        }

        let neighbours = get_neigthbours(&best_candidate, processed_targets.clone());

        to_search_values.remove(&best_candidate);
        processed_targets.push(best_candidate);
        for n in neighbours{
            let h_cost = n.distance_to_other_position(end_point) as i32;
            let g_cost = node.g_cost + 1; 
            let new_node = Node::new(Some(Box::new(node.clone())), best_candidate, g_cost, g_cost + h_cost);
            to_search_values.insert(n, new_node);
        }
    }
    return vec![];
}


fn main() {
    use std::time::Instant;
    let now = Instant::now();
    a_start_find(Position::generate_random_position(), Position::generate_random_position());
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed)
}

/* fn draw_board(height: i32, width: i32, points: Vec<Position>, start_point: Position, end_point: Position){
    println!();
    for i in 0..height{
        for j in 0..width{
                if i == start_point.y && j == start_point.x{
                    print!("start|");
                } else if  i == end_point.y && j == end_point.x{
                    print!("goal |");
                }
                else if points.contains(&Position { x: j, y: i }) {
                    print!("  p  |");
                } else {
                    print!("{} , {}|", j, i)
                }
        }
        println!()
    }
}
*/
fn get_neigthbours(position: &Position, processed_values: Vec<Position>) -> Vec<Position> {
    position.get_neigthbours().into_iter().filter(|n| !processed_values.contains(&n) && n.x >= 0 && n.x <= 200 && n.y >= 0 && n.y <= 200).collect()
}