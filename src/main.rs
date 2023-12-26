#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod position;
mod node;
mod board;

use egui::Color32;
use eframe::{run_native, App, NativeOptions, };

use node::Node;
use board::Board;

struct Pathfinder {
    board: Board
}

impl Default for Pathfinder {
    fn default() -> Self {
        Self {
            board: Board::new(5, 5),
        }
    }
}

impl App for Pathfinder{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Panel top").show(ctx, |ui| {
            let button = egui::Button::new("Correr algoritmo").fill(Color32::WHITE);

            if ui.add(button).clicked() {
                self.board.a_start_find();
            }
            let button = egui::Button::new("Limpiar obstaculos").fill(Color32::WHITE);

            if ui.add(button).clicked() {
                self.board.clean_obstacles();
            }
            let button = egui::Button::new("generar obstaculos").fill(Color32::WHITE);

            if ui.add(button).clicked() {
                self.board.generate_obstacles(5);
            }
            let button = egui::Button::new("reiniciar").fill(Color32::WHITE);

            if ui.add(button).clicked() {
                self.board.reset_path();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("Casillas").show(ui, |ui| {
                let row_width = self.board.width;
                for (i, node) in  self.board.nodes.clone().iter_mut().enumerate() {
                    
                    if node.pos.clone() == self.board.start_point.pos {
                        let button = egui::Button::new("inicio").fill(Color32::LIGHT_BLUE);
                        ui.add(button);
                    } else if node.pos.clone() == self.board.end_point.pos{
                        let button = egui::Button::new("fin").fill(Color32::GREEN);
                        ui.add(button);
                    } else if self.board.path.contains(&node.pos) {
                        let button = egui::Button::new(format!("camino {};{} vengo de {};{}",node.pos.x, node.pos.y,node.comes_from.clone().unwrap().pos.x, node.comes_from.clone().unwrap().pos.y)).fill(Color32::LIGHT_GREEN);
                        ui.add(button);
                    } else {
                        let color;
                        if self.board.obstacles.contains(&node.pos.clone()){
                            color = Color32::RED;
                        } else {
                            color = Color32::BLUE
                        }
                        let button = egui::Button::new(format!("agregar obs en \n {};{}", node.pos.x, node.pos.y)).fill(color);
                        if ui.add(button).clicked() {
                            self.board.switch_obstacle(node);
                        }
                    }
                    if (i + 1) % row_width == 0 && i != 0 {
                        ui.end_row();
                    }
                }
            })
        });
    }
} 

fn main() -> eframe::Result<()> {
    use std::time::Instant;
    let now = Instant::now();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let win_options = NativeOptions::default();
    run_native("Parhfinder", win_options , Box::new(|_cc| Box::<Pathfinder>::default()))
}

