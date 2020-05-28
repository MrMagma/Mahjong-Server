use crate::game::tile::{Tile, Wind, TileDragon, TileSeason, TileFlower};

pub const DECK: [Tile; 144] = [
    Tile::Dot(1),
    Tile::Dot(2),
    Tile::Dot(3),
    Tile::Dot(4),
    Tile::Dot(5),
    Tile::Dot(6),
    Tile::Dot(7),
    Tile::Dot(8),
    Tile::Dot(9),
    Tile::Bamboo(1),
    Tile::Bamboo(2),
    Tile::Bamboo(3),
    Tile::Bamboo(4),
    Tile::Bamboo(5),
    Tile::Bamboo(6),
    Tile::Bamboo(7),
    Tile::Bamboo(8),
    Tile::Bamboo(9),
    Tile::Character(1),
    Tile::Character(2),
    Tile::Character(3),
    Tile::Character(4),
    Tile::Character(5),
    Tile::Character(6),
    Tile::Character(7),
    Tile::Character(8),
    Tile::Character(9),
    Tile::Dot(1),
    Tile::Dot(2),
    Tile::Dot(3),
    Tile::Dot(4),
    Tile::Dot(5),
    Tile::Dot(6),
    Tile::Dot(7),
    Tile::Dot(8),
    Tile::Dot(9),
    Tile::Bamboo(1),
    Tile::Bamboo(2),
    Tile::Bamboo(3),
    Tile::Bamboo(4),
    Tile::Bamboo(5),
    Tile::Bamboo(6),
    Tile::Bamboo(7),
    Tile::Bamboo(8),
    Tile::Bamboo(9),
    Tile::Character(1),
    Tile::Character(2),
    Tile::Character(3),
    Tile::Character(4),
    Tile::Character(5),
    Tile::Character(6),
    Tile::Character(7),
    Tile::Character(8),
    Tile::Character(9),
    Tile::Dot(1),
    Tile::Dot(2),
    Tile::Dot(3),
    Tile::Dot(4),
    Tile::Dot(5),
    Tile::Dot(6),
    Tile::Dot(7),
    Tile::Dot(8),
    Tile::Dot(9),
    Tile::Bamboo(1),
    Tile::Bamboo(2),
    Tile::Bamboo(3),
    Tile::Bamboo(4),
    Tile::Bamboo(5),
    Tile::Bamboo(6),
    Tile::Bamboo(7),
    Tile::Bamboo(8),
    Tile::Bamboo(9),
    Tile::Character(1),
    Tile::Character(2),
    Tile::Character(3),
    Tile::Character(4),
    Tile::Character(5),
    Tile::Character(6),
    Tile::Character(7),
    Tile::Character(8),
    Tile::Character(9),
    Tile::Dot(1),
    Tile::Dot(2),
    Tile::Dot(3),
    Tile::Dot(4),
    Tile::Dot(5),
    Tile::Dot(6),
    Tile::Dot(7),
    Tile::Dot(8),
    Tile::Dot(9),
    Tile::Bamboo(1),
    Tile::Bamboo(2),
    Tile::Bamboo(3),
    Tile::Bamboo(4),
    Tile::Bamboo(5),
    Tile::Bamboo(6),
    Tile::Bamboo(7),
    Tile::Bamboo(8),
    Tile::Bamboo(9),
    Tile::Character(1),
    Tile::Character(2),
    Tile::Character(3),
    Tile::Character(4),
    Tile::Character(5),
    Tile::Character(6),
    Tile::Character(7),
    Tile::Character(8),
    Tile::Character(9),
    Tile::Wind(Wind::East),
    Tile::Wind(Wind::South),
    Tile::Wind(Wind::West),
    Tile::Wind(Wind::North),
    Tile::Wind(Wind::East),
    Tile::Wind(Wind::South),
    Tile::Wind(Wind::West),
    Tile::Wind(Wind::North),
    Tile::Wind(Wind::East),
    Tile::Wind(Wind::South),
    Tile::Wind(Wind::West),
    Tile::Wind(Wind::North),
    Tile::Wind(Wind::East),
    Tile::Wind(Wind::South),
    Tile::Wind(Wind::West),
    Tile::Wind(Wind::North),
    Tile::Dragon(TileDragon::Red),
    Tile::Dragon(TileDragon::Green),
    Tile::Dragon(TileDragon::White),
    Tile::Dragon(TileDragon::Red),
    Tile::Dragon(TileDragon::Green),
    Tile::Dragon(TileDragon::White),
    Tile::Dragon(TileDragon::Red),
    Tile::Dragon(TileDragon::Green),
    Tile::Dragon(TileDragon::White),
    Tile::Dragon(TileDragon::Red),
    Tile::Dragon(TileDragon::Green),
    Tile::Dragon(TileDragon::White),
    Tile::Flower(TileFlower::PlumBlossom),
    Tile::Flower(TileFlower::Orchid),
    Tile::Flower(TileFlower::Chrysanthemum),
    Tile::Flower(TileFlower::Bamboo),
    Tile::Season(TileSeason::Spring),
    Tile::Season(TileSeason::Summer),
    Tile::Season(TileSeason::Autumn),
    Tile::Season(TileSeason::Winter)
];