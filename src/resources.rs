//! The resources are the 'passive' stuffs that are interacted with during the game
//! even though they do not strictly belong to the game.
//! 
//! Author:  X. Gillard
//! Date:    March 2023
//! Licence: MIT 

use std::ops::Index;

use bracket_lib::{pathfinding::{Algorithm2D, BaseMap, SmallVec}, terminal::{DistanceAlg, Point}};

use crate::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map{
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>
}

impl Index<Position> for Map {
    type Output = TileType;

    fn index(&self, Position{x, y}: Position) -> &Self::Output {
        &self.tiles[y*self.width + x]
    }
}

impl Index<Point> for Map {
    type Output = TileType;

    fn index(&self, pt: Point) -> &Self::Output {
        &self.tiles[self.point2d_to_index(pt)]
    }
}

impl Map {
    /// Returns true iff the entity is allowed to move on to the next position (x,y)
    pub fn can_enter(&self, dest: Point) -> bool {
        self.in_bounds(dest) && self[dest] == TileType::Floor
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    fn in_bounds(&self, Point { x, y }: Point) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    fn point2d_to_index(&self, Point { x, y }: Point) -> usize {
        (y*self.width as i32 + x) as usize
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        let x = idx % self.width;
        let y = idx / self.width;
        bracket_lib::prelude::Point::new(x, y)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    
    /// Returns the accessible neighboring cells
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut list = SmallVec::new();
        let Point { x, y } = self.index_to_point2d(idx);
        let up = Point::new(x, y-1);
        let down = Point::new(x, y+1);
        let left = Point::new(x-1, y);
        let right = Point::new(x+1, y);

        if self.can_enter(up) {
            list.push((self.point2d_to_index(up), 1.0));
        }
        if self.can_enter(down) {
            list.push((self.point2d_to_index(down), 1.0));
        }
        if self.can_enter(left) {
            list.push((self.point2d_to_index(left), 1.0));
        }
        if self.can_enter(right) {
            list.push((self.point2d_to_index(right), 1.0));
        }

        list
    }
    /// Returns the admissible heuristic for A*
    fn get_pathing_distance(&self, start: usize, end: usize) -> f32 {
        DistanceAlg::PythagorasSquared
            .distance2d(
                self.index_to_point2d(start), 
                self.index_to_point2d(end))
    }
}