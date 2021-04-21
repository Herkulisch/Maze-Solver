extern crate image as img;
use crate::graph::Graph;
use img::{Rgb, RgbImage};
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as fmtResult};
pub struct Maze {
    size: [u32; 2],
    maze: Vec<bool>,
    graph: Graph<([u32; 2], bool)>,
    entry: [u32; 2],
}

impl Maze {
    pub fn new(path: String, entry_x: u32, entry_y: u32) -> Result<Maze, MazeError> {
        let image: RgbImage = img::open(&path).unwrap().to_rgb8();
        let dimensions = image.dimensions();
        let mut maze = vec![false; (dimensions.0 * dimensions.1) as usize];

        for column in 0..(dimensions.0) {
            for row in 0..(dimensions.1) {
                let pixel = image.get_pixel(column, row);
                if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 {
                    maze[(row * dimensions.0 + column) as usize] = true;
                }
            }
        }
        if entry_x < dimensions.0 && entry_y < dimensions.1 {
            let mut maze = Maze {
                size: [dimensions.0, dimensions.1],
                maze: maze,
                graph: Graph::new(),
                entry: [entry_x, entry_y],
            };
            maze.scan();
            return Ok(maze);
        }
        Err(MazeError::OutOfBounds)
    }

    pub fn get_graph(&self) -> &Graph<([u32; 2], bool)> {
        &self.graph
    }

    fn get_tile(&self, x: u32, y: u32) -> Result<bool, MazeError> {
        if x < self.size[0] && y < self.size[1] {
            return Ok(self.maze[(y * self.size[0] + x) as usize]);
        }
        Err(MazeError::OutOfBounds)
    }

    fn set_tile(&mut self, x: u32, y: u32, value: bool) -> () {
        self.maze[(y * self.size[0] + x) as usize] = value;
    }

    /**
     * # Returns
     * The indices of the neighbors as a tuple of 4 u32 if they exist, if one
     * of them does not exist it gets replaced with a None
     * they are ordered in the following way: (top, right, bottom, left)
     */
    fn get_neighbors(&self, x: u32, y: u32) -> Result<[Option<bool>; 4], MazeError> {
        if self.get_tile(x, y).is_ok() {
            let mut neighbors: [Option<bool>; 4] = [None, None, None, None];
            if x != 0 {
                neighbors[3] = self.get_tile(x - 1, y).ok();
            } else {
                neighbors[3] = None;
            }
            if y != 0 {
                neighbors[0] = self.get_tile(x, y - 1).ok();
            } else {
                neighbors[0] = None;
            }
            neighbors[1] = self.get_tile(x + 1, y).ok();
            neighbors[2] = self.get_tile(x, y + 1).ok();
            return Ok(neighbors);
        }
        Err(MazeError::OutOfBounds)
    }

    fn scan(&mut self) {
        let mut upper_neighbor_nodes: Vec<Option<usize>> = vec![None; self.size[0] as usize];
        let mut left_neighbor: Option<usize> = None;
        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                if self.get_tile(x, y).unwrap() != true {
                    let [top_option, right_option, bottom_option, left_option] =
                        self.get_neighbors(x, y).unwrap();
                    if top_option.is_none()
                        || right_option.is_none()
                        || bottom_option.is_none()
                        || left_option.is_none()
                    {
                        if top_option.is_some() && top_option.unwrap() == false {
                            let node_index = self.graph.add_node(([x, y], true));
                            self.graph.set_bi_edge(
                                upper_neighbor_nodes[x as usize].unwrap(),
                                node_index,
                                0,
                            );
                        } else if right_option.is_some() && right_option.unwrap() == false {
                            let node_index = self.graph.add_node(([x, y], true));
                            left_neighbor = Some(node_index);
                        } else if bottom_option.is_some() && bottom_option.unwrap() == false {
                            let node_index = self.graph.add_node(([x, y], true));
                            upper_neighbor_nodes[x as usize] = Some(node_index);
                        } else if left_option.is_some() && left_option.unwrap() == false {
                            let node_index = self.graph.add_node(([x, y], true));
                            self.graph
                                .set_bi_edge(left_neighbor.unwrap(), node_index, 0);
                        }
                    } else if top_option.is_some()
                        && right_option.is_some()
                        && bottom_option.is_some()
                        && left_option.is_some()
                    {
                        let [top, right, bottom, left] = [
                            top_option.unwrap(),
                            right_option.unwrap(),
                            bottom_option.unwrap(),
                            left_option.unwrap(),
                        ];
                        if top == false && right == false && bottom == false && left == false
                        //┼
                        {
                            let node_index = self.graph.add_node(([x, y], false));
                            self.graph
                                .set_bi_edge(left_neighbor.unwrap(), node_index, 0);
                            self.graph.set_bi_edge(
                                upper_neighbor_nodes[x as usize].unwrap(),
                                node_index,
                                0,
                            );
                            left_neighbor = Some(node_index);
                            upper_neighbor_nodes[x as usize] = Some(node_index);
                        } else if top == false && right == false && bottom == true && left == false
                        //┴
                        {
                            let node_index = self.graph.add_node(([x, y], false));
                            self.graph
                                .set_bi_edge(left_neighbor.unwrap(), node_index, 0);
                            self.graph.set_bi_edge(
                                upper_neighbor_nodes[x as usize].unwrap(),
                                node_index,
                                0,
                            );
                            left_neighbor = Some(node_index);
                            upper_neighbor_nodes[x as usize] = None;
                        } else if top == true && right == false && bottom == false && left == false
                        //┬
                        {
                            let node_index = self.graph.add_node(([x, y], false));
                            self.graph
                                .set_bi_edge(left_neighbor.unwrap(), node_index, 0);
                            left_neighbor = Some(node_index);
                            upper_neighbor_nodes[x as usize] = Some(node_index);
                        } else if top == false && right == false && bottom == false && left == true
                        //├
                        {
                            let node_index = self.graph.add_node(([x, y], false));
                            self.graph.set_bi_edge(
                                upper_neighbor_nodes[x as usize].unwrap(),
                                node_index,
                                0,
                            );
                            left_neighbor = Some(node_index);
                            upper_neighbor_nodes[x as usize] = Some(node_index);
                        } else if top == false && right == true && bottom == false && left == false
                        //¬├
                        {
                            let node_index = self.graph.add_node(([x, y], false));
                            self.graph
                                .set_bi_edge(left_neighbor.unwrap(), node_index, 0);
                            self.graph.set_bi_edge(
                                upper_neighbor_nodes[x as usize].unwrap(),
                                node_index,
                                0,
                            );
                            upper_neighbor_nodes[x as usize] = Some(node_index);
                            left_neighbor = None;
                        } else if top == true && right == false && bottom == false && left == true
                        //┌
                        {
                            let node_index = self.graph.add_node(([x, y], false));
                            upper_neighbor_nodes[x as usize] = Some(node_index);
                            left_neighbor = Some(node_index);
                        } else if top == true && right == true && bottom == false && left == false
                        //┐
                        {
                            let node_index = self.graph.add_node(([x, y], false));
                            self.graph
                                .set_bi_edge(left_neighbor.unwrap(), node_index, 0);
                            upper_neighbor_nodes[x as usize] = Some(node_index);
                            left_neighbor = None;
                        } else if top == false && right == true && bottom == true && left == false
                        //┘
                        {
                            let node_index = self.graph.add_node(([x, y], false));
                            self.graph
                                .set_bi_edge(left_neighbor.unwrap(), node_index, 0);
                            self.graph.set_bi_edge(
                                upper_neighbor_nodes[x as usize].unwrap(),
                                node_index,
                                0,
                            );
                            left_neighbor = None;
                            upper_neighbor_nodes[x as usize] = None;
                        } else if top == false && right == false && bottom == true && left == true
                        //└
                        {
                            let node_index = self.graph.add_node(([x, y], false));
                            self.graph.set_bi_edge(
                                upper_neighbor_nodes[x as usize].unwrap(),
                                node_index,
                                0,
                            );
                            left_neighbor = Some(node_index);
                            upper_neighbor_nodes[x as usize] = None;
                        } else if top == false && right == true && bottom == true && left == true
                        //↓
                        {
                            let node_index = self.graph.add_node(([x, y], false));
                            self.graph.set_bi_edge(
                                upper_neighbor_nodes[x as usize].unwrap(),
                                node_index,
                                0,
                            );
                            upper_neighbor_nodes[x as usize] = None;
                        } else if top == true && right == false && bottom == true && left == true
                        //←
                        {
                            let node_index = self.graph.add_node(([x, y], false));
                            left_neighbor = Some(node_index);
                        } else if top == true && right == true && bottom == false && left == true
                        //↑
                        {
                            let node_index = self.graph.add_node(([x, y], false));
                            upper_neighbor_nodes[x as usize] = Some(node_index);
                        } else if top == true && right == true && bottom == true && left == false
                        //→
                        {
                            let node_index = self.graph.add_node(([x, y], false));
                            self.graph
                                .set_bi_edge(left_neighbor.unwrap(), node_index, 0);
                            left_neighbor = None;
                        }
                    }
                }
            }
        }
    }

    pub fn solve_maze(maze: &Maze) -> Option<[u32; 2]> {
        Self::bfs(maze.get_graph(), 0)
    }

    fn bfs(graph: &Graph<([u32; 2], bool)>, start_node_index: usize) -> Option<[u32; 2]> {
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut visited: Vec<usize> = Vec::new();
        let mut exit_node: Option<[u32; 2]> = None;
        queue.push_back(start_node_index);
        visited.push(start_node_index);
        let mut skipped_first = false;
        while !queue.is_empty() {
            let node_index = queue.pop_front().unwrap();
            if graph.get_node(node_index).element.1 == true {
                if skipped_first {
                    exit_node = Some(graph.get_node(node_index).element.0);
                    break;
                } else {
                    skipped_first = true;
                }
            }
            for child_index in graph.get_neighbors(node_index) {
                if !visited.contains(&child_index) {
                    queue.push_back(child_index);
                    visited.push(child_index);
                }
            }
        }
        return exit_node;
    }

    fn dfs(graph: &Graph<([u32; 2], bool)>, start_node_index: usize) -> Option<[u32; 2]> {
        let node_length = graph.get_node_amount();
        let mut color: Vec<char> = vec!['w'; node_length];
        let mut pi: Vec<Option<usize>> = vec![None; node_length];
        unimplemented!();
    }

    pub fn export_graph_png(&self, path: String) {
        let mut image = img::RgbImage::new(self.size[0], self.size[1]);
        for x in 0..self.size[0] {
            for y in 0..self.size[1] {
                if self.get_tile(x, y).unwrap() == true {
                    image.put_pixel(x, y, Rgb([0, 0, 0]));
                } else {
                    image.put_pixel(x, y, Rgb([255, 255, 255]));
                }
            }
        }
        for node_index in 0..self.graph.get_node_amount() {
            let (tile, _) = self.graph.get_node(node_index).element;
            image.put_pixel(tile[0], tile[1], Rgb([255, 160, 122]));
        }
        image.save(&path).unwrap();
        println!("Bild wurde in {} gespeichert.", path);
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        let mut maze_string = String::new();
        for row in 0..self.size[1] {
            for column in 0..self.size[0] {
                match self.get_tile(column, row).unwrap() {
                    true => maze_string.push('⬛'),
                    false => maze_string.push('⬜'),
                }
            }
            maze_string.push_str("\n");
        }
        write!(f, "{}", maze_string)
    }
}

impl Debug for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        let mut maze_string = String::new();
        for row in 0..self.size[1] {
            for column in 0..self.size[0] {
                match self.get_tile(column, row).unwrap() {
                    true => maze_string.push('1'),
                    false => maze_string.push('0'),
                }
            }
            maze_string.push_str("\n");
        }
        write!(f, "{}", maze_string)
    }
}

enum Directions {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

impl Display for Directions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(
            f,
            "{}",
            match self {
                Directions::Top => "⬆",
                Directions::Right => "➡",
                Directions::Bottom => "⬇",
                Directions::Left => "⬅",
            }
        )
    }
}

impl Debug for Directions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(
            f,
            "{}",
            match self {
                Directions::Top => "A",
                Directions::Right => "->",
                Directions::Bottom => "V",
                Directions::Left => "<-",
            }
        )
    }
}

pub enum MazeError {
    OutOfBounds,
    NoExitFound,
}

impl Display for MazeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        match self {
            Self::OutOfBounds => write!(f, "The requested Index does not Exist"),
            Self::NoExitFound => write!(f, "Could not find an Exit for the Maze"),
        }
    }
}
impl Debug for MazeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        match self {
            Self::OutOfBounds => write!(f, "The requested Index does not Exist"),
            Self::NoExitFound => write!(f, "Could not find an Exit for the Maze"),
        }
    }
}

impl Error for MazeError {}
