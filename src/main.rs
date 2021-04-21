#![allow(dead_code)]

extern crate image as img;
mod graph;
mod maze;
use maze::Maze;
use std::env;
use std::time::Instant;

fn main() {
    //index_300x300 = s(177,0)
    //index_16x9 = s(6,0)
    let mut path = String::new();
    let mut entry_x: u32 = 0;
    let mut entry_y: u32 = 0;
    let mut node_image_path = String::new();

    for (i, arg) in env::args().enumerate() {
        if i == 1 {
            path = String::from(&arg);
        }
        if i == 2 {
            entry_x = arg.trim_end().parse().unwrap();
        }
        if i == 3 {
            entry_y = arg.trim_end().parse().unwrap();
        }
        if i > 3 {
            if arg.contains("-o") {
                node_image_path = String::from("./node_image.png");
            }
        }
    }

    let now = Instant::now();
    let mut start = now.elapsed().as_secs();
    let maze = Maze::new(path, entry_x, entry_y).unwrap();
    let mut end = now.elapsed().as_secs();
    let build_graph_time = end - start;
    start = now.elapsed().as_secs();
    let coords = Maze::solve_maze(&maze).unwrap();
    end = now.elapsed().as_secs();
    if node_image_path.len() > 0 {
        maze.export_graph_png(node_image_path);
    }
    println!("Das Berechnen des Graphen für das Labyrinth dauert {} Sekunden, dabei wurden {} Knoten gesetzt und {} Kanten gesetzt.",build_graph_time,maze.get_graph().get_node_amount(),maze.get_graph().get_edge_amount());
    println!("Das Finden des Ausgangs durch BFS dauert {} Sekunden und es befindet sich an den Koordinaten ({},{}).",end-start,coords[0],coords[1]);
}
