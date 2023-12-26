use std::collections::HashSet;

use crate::position::Position;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub comes_from: Option<Box<Node>>,
    pub pos: Position,
    // g_cost determine the distance from the node to the begining
    pub g_cost: Option<i32>,
    // h_cost determine the distance to the objective node
    pub h_cost: Option<i32>,
    // f_cost determines the best_candidate, it's the g_cost + the h_cost
    pub f_cost: Option<i32>,
}

impl Node {
    pub fn new(comes_from: Option<Box<Node>>, position: Position, g_cost: Option<i32>, h_cost: Option<i32>) -> Node {
        Node {
            comes_from,
            pos: position,
            g_cost,
            h_cost,
            f_cost: get_f_cost(g_cost, h_cost),
        }
    }

    pub fn walkable(&mut self, obstacles: &HashSet<Position>) -> bool{
        !obstacles.contains(&self.pos)
    }
}

fn get_f_cost(g_cost: Option<i32>, h_cost: Option<i32>) -> Option<i32>{
    if let Some(g_cost_found) = g_cost {
        if let Some(h_cost_found) = h_cost{
            println!("aber costo f {:?}", g_cost_found + h_cost_found);
            return Some(g_cost_found + h_cost_found);
        }
    }
    None
}