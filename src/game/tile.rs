use std::cmp::Ordering;

#[derive(Copy, Clone, Eq)]
pub enum Wind {
    North,
    South,
    East,
    West
}

impl PartialEq for Wind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Wind::North, Wind::North) |
            (Wind::East, Wind::East) |
            (Wind::South, Wind::South) |
            (Wind::West, Wind::West) => true,
            _ => false
        }
    }
}

impl Ord for Wind {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else {
            match self {
                Wind::East => Ordering::Less,
                Wind::South => match other {
                    Wind::East => Ordering::Greater,
                    _ => Ordering::Less
                },
                Wind::West => match other {
                    Wind::North => Ordering::Less,
                    _ => Ordering::Greater
                },
                Wind::North => Ordering::Greater
            }
        }
    }
}

impl PartialOrd for Wind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq)]
pub enum TileDragon {
    Red,
    Green,
    White
}

impl PartialEq for TileDragon {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TileDragon::Red, TileDragon::Red) |
            (TileDragon::Green, TileDragon::Green) |
            (TileDragon::White, TileDragon::White) => true,
            _ => false
        }
    }
}

impl Ord for TileDragon {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal
        } else {
            match self {
                TileDragon::Red => Ordering::Less,
                TileDragon::Green => match other {
                    TileDragon::Red => Ordering::Greater,
                    TileDragon::White => Ordering::Less,
                    _ => Ordering::Equal
                },
                TileDragon::White => Ordering::Greater
            }
        }
    }
}

impl PartialOrd for TileDragon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq)]
pub enum TileFlower {
    PlumBlossom,
    Orchid,
    Chrysanthemum,
    Bamboo
}

impl PartialEq for TileFlower {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TileFlower::PlumBlossom, TileFlower::PlumBlossom) |
            (TileFlower::Orchid, TileFlower::Orchid) |
            (TileFlower::Chrysanthemum, TileFlower::Chrysanthemum) |
            (TileFlower::Bamboo, TileFlower::Bamboo) => true,
            _ => false
        }
    }
}

impl Ord for TileFlower {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else {
            match self {
                TileFlower::PlumBlossom => Ordering::Less,
                TileFlower::Orchid => match other {
                    TileFlower::PlumBlossom => Ordering::Greater,
                    _ => Ordering::Less
                },
                TileFlower::Chrysanthemum => match other {
                    TileFlower::Bamboo => Ordering::Less,
                    _ => Ordering::Greater
                },
                TileFlower::Bamboo => Ordering::Greater
            }
        }
    }
}

impl PartialOrd for TileFlower {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq)]
pub enum TileSeason {
    Spring,
    Summer,
    Autumn,
    Winter
}

impl PartialEq for TileSeason {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TileSeason::Spring, TileSeason::Spring) |
            (TileSeason::Summer, TileSeason::Summer) |
            (TileSeason::Autumn, TileSeason::Autumn) |
            (TileSeason::Winter, TileSeason::Winter) => true,
            _ => false
        }
    }
}

impl Ord for TileSeason {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else {
            match self {
                TileSeason::Spring => Ordering::Less,
                TileSeason::Summer => match other {
                    TileSeason::Spring => Ordering::Greater,
                    _ => Ordering::Less
                },
                TileSeason::Autumn => match other {
                    TileSeason::Winter => Ordering::Less,
                    _ => Ordering::Greater
                },
                TileSeason::Winter => Ordering::Greater
            }
        }
    }
}

impl PartialOrd for TileSeason {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq)]
pub enum Tile {
    Dot(u8),
    Bamboo(u8),
    Character(u8),
    Wind(Wind),
    Dragon(TileDragon),
    Flower(TileFlower),
    Season(TileSeason)
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Tile::Dot(v1), Tile::Dot(v2)) |
            (Tile::Bamboo(v1), Tile::Bamboo(v2)) |
            (Tile::Character(v1), Tile::Character(v2)) => v1 == v2,
            (Tile::Wind(w1), Tile::Wind(w2)) => w1 == w2,
            (Tile::Dragon(d1), Tile::Dragon(d2)) => d1 == d2,
            (Tile::Flower(f1), Tile::Flower(f2)) => f1 == f2,
            (Tile::Season(s1), Tile::Season(s2)) => s1 == s2,
            _ => false
        }
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Tile::Bamboo(v1) => match other {
                Tile::Bamboo(v2) => v1.cmp(v2),
                _ => Ordering::Less
            },
            Tile::Character(v1) => match other {
                Tile::Bamboo(_) => Ordering::Greater,
                Tile::Character(v2) => v1.cmp(v2),
                _ => Ordering::Less
            },
            Tile::Dot(v1) => match other {
                Tile::Bamboo(_) | Tile::Character(_) => Ordering::Greater,
                Tile::Dot(v2) => v1.cmp(v2),
                _ => Ordering::Less
            },
            Tile::Wind(w1) => match other {
                Tile::Bamboo(_) | Tile::Character(_) | Tile::Dot(_) => Ordering::Greater,
                Tile::Wind(w2) => w1.cmp(w2),
                _ => Ordering::Less
            },
            Tile::Dragon(d1) => match other {
                Tile::Dragon(d2) => d1.cmp(d2),
                Tile::Flower(_) | Tile::Season(_) => Ordering::Less,
                _ => Ordering::Greater
            },
            Tile::Flower(f1) => match other {
                Tile::Flower(f2) => f1.cmp(f2),
                Tile::Season(_) => Ordering::Less,
                _ => Ordering::Greater
            },
            Tile::Season(s1) => match other {
                Tile::Season(s2) => s1.cmp(s2),
                _ => Ordering::Greater
            }
        }
    }
}