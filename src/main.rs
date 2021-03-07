#![allow(dead_code)]
#![allow(unused)]

extern crate image as img;
mod graph;
mod maze;
use maze::Maze;
use std::time::{Duration, Instant};

fn main() {
    let mut maze = Maze::new("./assets/input.png".to_owned(), 6, 0).unwrap();    
    println!("{}{}", maze,maze.get_graph());
}
