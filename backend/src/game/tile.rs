use std::collections::HashMap;

use rocket::serde::Serialize;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Tile {
    StartingTile,
    Monastery,
    MonasteryWithRoad,
    CityCapWithCrossroad,
    TriangleWithRoad,
    TriangleWithRoadWithCOA,
    Straight,
    CityCap,
    Separator,
    TripleRoad,
    Invalid,
    Curve,
    QuadrupleRoad,
    Connector,
    ConnectorWithCOA,
    Left,
    Right,
    TripleCity,
    TripleCityWithCOA,
    VerticalSeparator,
    TripleCityWithRoad,
    TripleCityWithRoadWithCOA,
    Triangle,
    TriangleWithCOA,
    QuadrupleCityWithCOA,
}

impl Tile {
    pub fn to_id(self) -> i32 {
        match self {
            Tile::StartingTile => 0,
            Tile::Monastery => 1,
            Tile::MonasteryWithRoad => 2,
            Tile::CityCapWithCrossroad => 3,
            Tile::TriangleWithRoad => 4,
            Tile::TriangleWithRoadWithCOA => 5,
            Tile::Straight => 6,
            Tile::CityCap => 7,
            Tile::Separator => 8,
            Tile::TripleRoad => 9,
            Tile::Curve => 10,
            Tile::QuadrupleRoad => 11,
            Tile::Connector => 12,
            Tile::ConnectorWithCOA => 13,
            Tile::Left => 14,
            Tile::Right => 15,
            Tile::TripleCity => 16,
            Tile::TripleCityWithCOA => 17,
            Tile::VerticalSeparator => 18,
            Tile::TripleCityWithRoad => 19,
            Tile::TripleCityWithRoadWithCOA => 20,
            Tile::Triangle => 21,
            Tile::TriangleWithCOA => 22,
            Tile::QuadrupleCityWithCOA => 23,
            Tile::Invalid => -1,
        }
    }
}

pub fn to_tile(id: i32) -> Tile {
    match id {
        0 => Tile::StartingTile,
        1 => Tile::Monastery,
        2 => Tile::MonasteryWithRoad,
        3 => Tile::CityCapWithCrossroad,
        4 => Tile::TriangleWithRoad,
        5 => Tile::TriangleWithRoadWithCOA,
        6 => Tile::Straight,
        7 => Tile::CityCap,
        8 => Tile::Separator,
        9 => Tile::TripleRoad,
        10 => Tile::Curve,
        11 => Tile::QuadrupleRoad,
        12 => Tile::Connector,
        13 => Tile::ConnectorWithCOA,
        14 => Tile::Left,
        15 => Tile::Right,
        16 => Tile::TripleCity,
        17 => Tile::TripleCityWithCOA,
        18 => Tile::VerticalSeparator,
        19 => Tile::TripleCityWithRoad,
        20 => Tile::TripleCityWithRoadWithCOA,
        21 => Tile::Triangle,
        22 => Tile::TriangleWithCOA,
        23 => Tile::QuadrupleCityWithCOA,
        _ => Tile::Invalid,
    }
}

#[allow(dead_code)]
pub fn discard_test_tiles() -> Vec<Tile> {
    let mut tiles = vec![];
    for _ in 0..15 {
        tiles.push(Tile::Monastery);
    }
    for _ in 0..2 {
        tiles.push(Tile::QuadrupleRoad);
    }
    tiles
}
#[allow(dead_code)]
pub fn discard_test_remaining_tiles(out_tiles: Vec<Tile>) -> Vec<Tile> {
    let mut map = HashMap::new();
    for al in &tiles() {
        map.entry(*al).or_insert(0);
    }
    map.entry(Tile::StartingTile).or_insert(0);
    for ot in &out_tiles {
        map.entry(*ot).and_modify(|v| *v += 1);
    }
    let mut tiles = vec![];
    for _ in 0..1 - *map.get(&Tile::StartingTile).unwrap() {
        tiles.push(Tile::StartingTile);
    }
    for _ in 0..15 - *map.get(&Tile::Monastery).unwrap() {
        tiles.push(Tile::Monastery);
    }
    for _ in 0..2 - *map.get(&Tile::QuadrupleRoad).unwrap() {
        tiles.push(Tile::QuadrupleRoad);
    }
    tiles
}

pub fn tiles() -> Vec<Tile> {
    let mut tiles = vec![];
    for _ in 0..3 {
        tiles.push(Tile::StartingTile);
    }
    for _ in 0..4 {
        tiles.push(Tile::Monastery);
    }
    for _ in 0..2 {
        tiles.push(Tile::MonasteryWithRoad);
    }
    for _ in 0..3 {
        tiles.push(Tile::CityCapWithCrossroad);
    }
    for _ in 0..3 {
        tiles.push(Tile::TriangleWithRoad);
    }
    for _ in 0..2 {
        tiles.push(Tile::TriangleWithRoadWithCOA);
    }
    for _ in 0..8 {
        tiles.push(Tile::Straight);
    }
    for _ in 0..5 {
        tiles.push(Tile::CityCap);
    }
    for _ in 0..2 {
        tiles.push(Tile::Separator);
    }
    for _ in 0..4 {
        tiles.push(Tile::TripleRoad);
    }
    for _ in 0..9 {
        tiles.push(Tile::Curve);
    }
    for _ in 0..1 {
        tiles.push(Tile::QuadrupleRoad);
    }
    for _ in 0..1 {
        tiles.push(Tile::Connector);
    }
    for _ in 0..2 {
        tiles.push(Tile::ConnectorWithCOA);
    }
    for _ in 0..3 {
        tiles.push(Tile::Left);
    }
    for _ in 0..3 {
        tiles.push(Tile::Right);
    }
    for _ in 0..3 {
        tiles.push(Tile::TripleCity);
    }
    for _ in 0..1 {
        tiles.push(Tile::TripleCityWithCOA);
    }
    for _ in 0..3 {
        tiles.push(Tile::VerticalSeparator);
    }
    for _ in 0..1 {
        tiles.push(Tile::TripleCityWithRoad);
    }
    for _ in 0..2 {
        tiles.push(Tile::TripleCityWithRoadWithCOA);
    }
    for _ in 0..3 {
        tiles.push(Tile::Triangle);
    }
    for _ in 0..2 {
        tiles.push(Tile::TriangleWithCOA);
    }
    for _ in 0..1 {
        tiles.push(Tile::QuadrupleCityWithCOA);
    }
    assert_eq!(tiles.len(), 71);
    tiles
}

pub fn remaining_tiles(out_tiles: Vec<Tile>) -> Vec<Tile> {
    let mut map = HashMap::new();
    for al in &tiles() {
        map.entry(*al).or_insert(0);
    }
    for ot in &out_tiles {
        map.entry(*ot).and_modify(|v| *v += 1);
    }
    let mut tiles = vec![];
    for _ in 0..4 - *map.get(&Tile::StartingTile).unwrap() {
        tiles.push(Tile::StartingTile);
    }
    for _ in 0..4 - *map.get(&Tile::Monastery).unwrap() {
        tiles.push(Tile::Monastery);
    }
    for _ in 0..2 - *map.get(&Tile::MonasteryWithRoad).unwrap() {
        tiles.push(Tile::MonasteryWithRoad);
    }
    for _ in 0..3 - *map.get(&Tile::CityCapWithCrossroad).unwrap() {
        tiles.push(Tile::CityCapWithCrossroad);
    }
    for _ in 0..3 - *map.get(&Tile::TriangleWithRoad).unwrap() {
        tiles.push(Tile::TriangleWithRoad);
    }
    for _ in 0..2 - *map.get(&Tile::TriangleWithRoadWithCOA).unwrap() {
        tiles.push(Tile::TriangleWithRoadWithCOA);
    }
    for _ in 0..8 - *map.get(&Tile::Straight).unwrap() {
        tiles.push(Tile::Straight);
    }
    for _ in 0..5 - *map.get(&Tile::CityCap).unwrap() {
        tiles.push(Tile::CityCap);
    }
    for _ in 0..2 - *map.get(&Tile::Separator).unwrap() {
        tiles.push(Tile::Separator);
    }
    for _ in 0..4 - *map.get(&Tile::TripleRoad).unwrap() {
        tiles.push(Tile::TripleRoad);
    }
    for _ in 0..9 - *map.get(&Tile::Curve).unwrap() {
        tiles.push(Tile::Curve);
    }
    for _ in 0..1 - *map.get(&Tile::QuadrupleRoad).unwrap() {
        tiles.push(Tile::QuadrupleRoad);
    }
    for _ in 0..1 - *map.get(&Tile::Connector).unwrap() {
        tiles.push(Tile::Connector);
    }
    for _ in 0..2 - *map.get(&Tile::ConnectorWithCOA).unwrap() {
        tiles.push(Tile::ConnectorWithCOA);
    }
    for _ in 0..3 - *map.get(&Tile::Left).unwrap() {
        tiles.push(Tile::Left);
    }
    for _ in 0..3 - *map.get(&Tile::Right).unwrap() {
        tiles.push(Tile::Right);
    }
    for _ in 0..3 - *map.get(&Tile::TripleCity).unwrap() {
        tiles.push(Tile::TripleCity);
    }
    for _ in 0..1 - *map.get(&Tile::TripleCityWithCOA).unwrap() {
        tiles.push(Tile::TripleCityWithCOA);
    }
    for _ in 0..3 - *map.get(&Tile::VerticalSeparator).unwrap() {
        tiles.push(Tile::VerticalSeparator);
    }
    for _ in 0..1 - *map.get(&Tile::TripleCityWithRoad).unwrap() {
        tiles.push(Tile::TripleCityWithRoad);
    }
    for _ in 0..2 - *map.get(&Tile::TripleCityWithRoadWithCOA).unwrap() {
        tiles.push(Tile::TripleCityWithRoadWithCOA);
    }
    for _ in 0..3 - *map.get(&Tile::Triangle).unwrap() {
        tiles.push(Tile::Triangle);
    }
    for _ in 0..2 - *map.get(&Tile::TriangleWithCOA).unwrap() {
        tiles.push(Tile::TriangleWithCOA);
    }
    for _ in 0..1 - *map.get(&Tile::QuadrupleCityWithCOA).unwrap() {
        tiles.push(Tile::QuadrupleCityWithCOA);
    }
    tiles
}

#[cfg(test)]
mod tests {
    use super::{remaining_tiles, Tile::*};
    #[test]
    fn test_remaining_tiles() {
        let out_tiles = Vec::from([
            StartingTile,
            CityCapWithCrossroad,
            CityCapWithCrossroad,
            Monastery,
            MonasteryWithRoad,
            MonasteryWithRoad,
            TriangleWithRoad,
            TriangleWithRoadWithCOA,
            TriangleWithRoadWithCOA,
            Straight,
            Straight,
            Straight,
            Straight,
            Straight,
            CityCap,
            Separator,
            Separator,
            TripleRoad,
            Curve,
            Curve,
            Curve,
            Connector,
            ConnectorWithCOA,
            Left,
            Right,
            Right,
            Right,
            TripleCity,
            TripleCity,
            VerticalSeparator,
            VerticalSeparator,
            TripleCityWithRoad,
            TripleCityWithRoad,
            TripleCityWithRoadWithCOA,
            Triangle,
            Triangle,
            Triangle,
            TriangleWithCOA,
            TriangleWithCOA,
            QuadrupleCityWithCOA,
        ]);
        let exp_remaining_tiles = Vec::from([
            StartingTile,
            StartingTile,
            StartingTile,
            Monastery,
            Monastery,
            Monastery,
            CityCapWithCrossroad,
            TriangleWithRoad,
            TriangleWithRoad,
            Straight,
            Straight,
            Straight,
            CityCap,
            CityCap,
            CityCap,
            CityCap,
            TripleRoad,
            TripleRoad,
            TripleRoad,
            Curve,
            Curve,
            Curve,
            Curve,
            Curve,
            Curve,
            QuadrupleRoad,
            ConnectorWithCOA,
            Left,
            Left,
            TripleCity,
            TripleCityWithCOA,
            VerticalSeparator,
            TripleCityWithRoadWithCOA,
        ]);
        let remaining_tiles = remaining_tiles(out_tiles);
        assert_eq!(remaining_tiles, exp_remaining_tiles);
    }
}
