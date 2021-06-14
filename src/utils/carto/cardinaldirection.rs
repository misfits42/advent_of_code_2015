/// Represents one of the four cardinal directions.
#[derive(Copy, Clone)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West
}

impl CardinalDirection {
    /// Determines new direction resulting from single 90-degree rotation to left (CCW).
    pub fn rotate_left(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => return CardinalDirection::West,
            CardinalDirection::East => return CardinalDirection::North,
            CardinalDirection::South => return CardinalDirection::East,
            CardinalDirection::West => return CardinalDirection::South,
        }
    }

    /// Determines new direction resulting from single 90-degree rotation to right (CW).
    pub fn rotate_right(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => return CardinalDirection::East,
            CardinalDirection::East => return CardinalDirection::South,
            CardinalDirection::South => return CardinalDirection::West,
            CardinalDirection::West => return CardinalDirection::North,
        }
    }

    /// Gets the unit vector (length 1) corresponding to the cardinal direction.
    pub fn get_unit_vector(&self) -> (i64, i64) {
        match self {
            CardinalDirection::North => (0, -1),
            CardinalDirection::East => (1, 0),
            CardinalDirection::South => (0, 1),
            CardinalDirection::West => (-1, 0),
        }
    }
}
