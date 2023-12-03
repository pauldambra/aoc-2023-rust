use std::{fmt::Display};
use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Clone, PartialEq)]
pub struct Coordinate {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Debug)]
pub enum CoordinateError {
    WhatAmI
}

#[derive(Clone, Eq, Hash)]
pub struct MultiXCoord {
    pub(crate) xs: Vec<i32>,
    pub(crate) y: i32,
    pub(crate) val: String,
}

impl MultiXCoord {
    pub fn new(xs: Vec<i32>, y: i32, val: String) -> Result<Self, CoordinateError> {
        Ok(Self { xs, y, val })
    }

    pub fn neighbours(&self) -> Vec<Coordinate> {
        let mut nbrs = vec![];

        for x in &self.xs {
            let crd = Coordinate::new(*x, self.y);
            nbrs.push(crd.unwrap().neighbours());
        }

        return nbrs.concat();
    }
}


impl Coordinate {
    pub fn new(x: i32, y: i32) -> Result<Self, CoordinateError> {
        Ok(Self { x, y })
    }

    pub fn neighbours(&self) -> Vec<Coordinate> {
        let mut xs = vec![];

        xs.push(Coordinate::new(self.x - 1, self.y - 1).unwrap());
        xs.push(Coordinate::new(self.x, self.y - 1).unwrap());
        xs.push(Coordinate::new(self.x + 1, self.y - 1).unwrap());
        xs.push(Coordinate::new(self.x - 1, self.y).unwrap());
        xs.push(Coordinate::new(self.x + 1, self.y).unwrap());
        xs.push(Coordinate::new(self.x - 1, self.y + 1).unwrap());
        xs.push(Coordinate::new(self.x, self.y + 1).unwrap());
        xs.push(Coordinate::new(self.x + 1, self.y + 1).unwrap());

        return xs;
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", &self.x, &self.y)
    }
}

impl Display for MultiXCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "xs: {:?}, y: {} holds {}", &self.xs, &self.y, &self.val)
    }
}

impl PartialEq<Self> for MultiXCoord {
    fn eq(&self, other: &Self) -> bool {
        return self.xs == other.xs && self.y == other.y && self.val == other.val
    }
}

impl PartialOrd<Self> for MultiXCoord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MultiXCoord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.xs.first().cmp(&other.xs.first())
    }
}
