use super::*;

/// Cardinal/intercardinal directions. Limit characters/npcs to these directions.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
