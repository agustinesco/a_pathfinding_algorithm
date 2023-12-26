
use std::collections::{HashMap, HashSet};
use std::vec;

use nalgebra::DMatrix;
use crate::position::Position;
use crate::Node;

#[derive(Debug)]
pub struct Board {
    pub width: usize,
    pub heigth: usize,
    pub nodes: DMatrix<Node>,
    pub obstacles: HashSet<Position>,
    pub start_point: Node,
    pub end_point: Node,
    pub path: Vec<Position>
}

impl Board {
    pub fn new(heigth: usize, width: usize) -> Board {
        let mut nodes: Vec<Node> = vec![];
        for x in 0..width {
            for y in 0.. heigth{
                nodes.push(Node::new(None, Position::new(y, x), None, None));
            }
        }

        let end_point = Node::new(None, Position::new(width - 1, heigth - 1), None, None);
        let mut start_point = Node::new(None, Position::new(0, 0), Some(0), None);
        start_point.h_cost = Some(start_point.pos.distance_to_other_position(end_point.pos));

        Board {
            nodes: DMatrix::from_vec(heigth, width, nodes),
            heigth,
            width,
            obstacles: HashSet::new(),
            end_point,
            start_point,
            path: vec![],
        }
    }

    pub fn generate_obstacles(&mut self, amount: i32){
        for _ in 0..amount{
            self.obstacles.insert(Position::generate_random_position(self.heigth, self.width));
        }
    }

    pub fn switch_obstacle(&mut self, node: &Node) -> &mut Board {
        if self.obstacles.contains(&node.pos) {
            self.obstacles.remove(&node.pos);
        } else {
            self.obstacles.insert(node.pos.clone());
        }
        self
    }

    pub fn a_start_find(&mut self) {
        self.reset_path();
        let mut to_search_values: HashMap<Position, Node> = HashMap::new();
        let mut processed_targets: HashSet<Position> = HashSet::new();
    
        to_search_values.insert(self.start_point.pos, self.start_point.clone());
    
        while !to_search_values.is_empty() {
            let (best_candidate, node) = to_search_values
                .clone()
                .into_iter()
                .min_by(|(_, node_1), (_, node_2)| node_1.f_cost.cmp(&node_2.f_cost))
                .unwrap();
    
            if best_candidate.eq(&self.end_point.pos) {
                self.add_path(node.clone());
                return;
            }
    
            let new_neightbours = self.get_candidates(&best_candidate, self.end_point.clone(), &processed_targets, &node);
            new_neightbours.iter().for_each(|n: &Node|  {
                to_search_values.insert(n.pos, n.clone());
            });
    
            to_search_values.remove(&best_candidate);
            processed_targets.insert(best_candidate);
        }
        self.path = vec![];
    }

    fn get_candidates(
        &mut self,
        position: &Position,
        target: Node,
        processed_targets: &HashSet<Position>,
        node: &Node
    ) -> Vec<Node> {
        let mut candidates = vec![];
    
        for x in position.x.saturating_sub(1) .. position.x + 2{
            for y in position.y.saturating_sub(1) .. position.y + 2{
                if let Some(n) = self.nodes.get_mut((x, y)){
                    if n.walkable(&self.obstacles) && (n.pos.x != position.x || n.pos.y != position.y) {
                        let node_processed = processed_targets.contains(&n.pos);
                        let cost_to_neighbour;
                        match  node.g_cost{
                            None => panic!("Entré a nodo vacio"),
                            Some(g_cost) => cost_to_neighbour = g_cost + 1,
                        } 
                        if let Some(neightbout_g_cost) = n.g_cost{
                            if !node_processed || cost_to_neighbour < neightbout_g_cost {
                                n.g_cost = Some(node.g_cost.unwrap() + 1); 
                                n.comes_from = Some(Box::new(node.clone()));
                                if !node_processed {
                                    let distance_to_target = n.pos.distance_to_other_position(target.pos) as i32;
                                    n.h_cost = Some(distance_to_target);
                                    n.f_cost = Some(distance_to_target + node.g_cost.unwrap() + 1);
                                    candidates.push(n.clone());                
                                }
                            }
                        } else {
                            if !node_processed {
                                n.g_cost = Some(node.g_cost.unwrap() + 1); 
                                n.comes_from = Some(Box::new(node.clone()));
                                let distance_to_target = n.pos.distance_to_other_position(target.pos) as i32;
                                n.h_cost = Some(distance_to_target);
                                n.f_cost = Some(distance_to_target + node.g_cost.unwrap() + 1);
                                candidates.push(n.clone());                
                            }
                        }
                    }
                }
            }
        }
        candidates
    }

    fn add_path(&mut self, node: Node) {
        self.path = vec![];
        let mut current: Node = node;
        while !current.comes_from.is_none() {
            self.path.push(current.pos);
            current = *current.comes_from.unwrap();
        }
        println!("aber path {:?}", self.path);
    }

    pub fn clean_obstacles(&mut self) {
        self.obstacles = HashSet::new()
    }
    pub fn reset_path(&mut self) {
        self.path = vec![];
        for x in 0..self.width {
            for y in 0.. self.heigth{
                if let Some(node) = self.nodes.get_mut((x, y)){
                    node.g_cost = None;
                    node.comes_from = None;
                    node.h_cost = None;
                    node.f_cost = None;
                }
                
            }
        }
        
    }
    
}

