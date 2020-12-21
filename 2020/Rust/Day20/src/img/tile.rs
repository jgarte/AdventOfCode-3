// MIT License
//
// Copyright (c) 2020 Pedro Rodrigues
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// https://adventofcode.com/2020/day/20

#![allow(non_snake_case)]

use std::fmt;
use std::str::FromStr;


type Edge = String;
type Matrix = Vec<String>;

#[derive(Clone,Copy)]
pub enum TileEdge {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub struct Tile {
    pub id: i32,
    matrix: Matrix,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = vec![
            format!("Id: {}", self.id),
            self.matrix.join("\n"),
        ];

        write!(f, "{}", t.join("\n"))
    }
}

impl Tile {
    pub fn new(lines: &Vec<String>) -> Self {
        Tile {
            id: i32::from_str(&lines[0].split(':').collect::<Vec<&str>>()[0][5..]).unwrap(),
            matrix: lines[1..].to_vec(),
        }
    }

    pub fn size(&self) -> usize {
        self.matrix.len()
    }

    pub fn left_edge(&self) -> Edge {
        (0..self.matrix.len())
            .map(|row| self.matrix[row].chars().nth(0).unwrap())
            .collect()
    }

    pub fn right_edge(&self) -> Edge {
        let l = self.matrix[0].len();

        (0..self.matrix.len())
            .map(|row| self.matrix[row].chars().nth(l-1).unwrap())
            .collect()
    }

    pub fn top_edge(&self) -> Edge {
        self.matrix[0].to_string()
    }

    pub fn bottom_edge(&self) -> Edge {
        self.matrix.iter().last().unwrap().to_string()
    }

    pub fn edge(&self, e: TileEdge) -> Edge {
        match e {
            TileEdge::Top => self.top_edge(),
            TileEdge::Left => self.left_edge(),
            TileEdge::Bottom => self.bottom_edge(),
            TileEdge::Right => self.right_edge(),
        }
    }

    pub fn edges(&self) -> Vec<Edge> {
        vec![
            self.top_edge(),
            self.bottom_edge(),
            self.left_edge(),
            self.right_edge(),
        ]
    }

    pub fn all_edges(&self) -> Vec<Edge> {
        let mut edges = self.edges();
        edges.append(&mut self.edges()
            .iter()
            .map(|e| e.chars().rev().collect())
            .collect()
        );

        edges
    }

    fn strip_border(&self) -> Tile {
        let height = self.matrix.len();
        let width = height;
        let new_matrix = self.matrix[1..=height-1].to_vec();

        Tile {
            id: self.id,
            matrix: new_matrix
                .iter()
                .map(|row| row[1..=width-1].to_string())
                .collect()
        }
    }

    fn flip(&self) -> Tile {
        Tile {
            id: self.id,
            matrix: self.matrix
                .iter()
                .map(|row| row.chars().rev().collect())
                .clone()
                .collect()
        }
    }

    pub fn rotate(&self) -> Tile {
        let m = (0..self.matrix.len())
            .map(|row| {
                self.matrix
                    .iter()
                    .map(|r| r.chars().nth(row).unwrap())
                    .clone()
                    .rev()
                    .collect::<String>()
            })
            .collect();

        Tile {
            id: self.id,
            matrix: m,
        }
    }

    pub fn transformations(&self) -> Transformation {
        Transformation::new(self.clone())
    }
}

pub struct Transformation {
    curr: Tile,
    counter: usize,
}

impl Transformation {
    fn new(t: Tile) -> Self {
        Transformation {
            curr: t,
            counter: 0,
        }
    }
}

impl Iterator for Transformation {
    type Item = Tile;

    fn next(&mut self) -> Option<Tile> {
        self.counter += 1;

        match self.counter % 3 {
            0 => self.curr = self.curr.rotate(),
            2 => self.curr = self.curr.flip(),
            _ => (),
        }
        
        Some(self.curr.clone())
    }
}
