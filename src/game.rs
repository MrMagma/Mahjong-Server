mod tile;
mod deck;

use tile::{Tile, TileSeason, TileFlower, TileDragon, Wind};
use deck::{DECK};

pub struct Player {
    pub wind: Wind,
    pub hand: Vec<Tile>
}

impl Player {
    pub fn create_player(wind: Wind) -> Player {
        Player {
            wind,
            hand: vec![]
        }
    }
}

pub struct Game {
    pub players: [Player; 4],
    wall: Vec<Tile>,
    discard: Vec<Tile>
}

pub enum GrabType {
    Pong,
    Kong,
    Chow
}

impl Game {
    pub fn get_player(&mut self, player_wind: Wind) -> &mut Player {
        let mut index = 0;

        for i in 0..self.players.len() {
            if player_wind == self.players[i].wind {
                index = i;
                break;
            }
        }

        return &mut self.players[index];
    }
    pub fn draw_tile(&mut self, player: &mut Player) -> Option<Tile> {
        let draw = self.wall.pop();

        match draw {
            Some(tile) => {
                player.hand.push(tile);
                player.hand.sort_unstable();

                Some(tile)
            }
            None => None
        }
    }
    pub fn check_discard(&self) -> Option<&Tile> {
        self.discard.last()
    }
    pub fn grab_discard(&mut self, player: &mut Player, grab_type: GrabType) -> Option<Tile> {
        if self.discard.len() == 0 {
            return None;
        }

        let last_discard = *self.discard.last().unwrap();

        match grab_type {
            GrabType::Pong | GrabType::Kong => {
                let mut count = 0;

                for tile in &player.hand {
                    if *tile == last_discard {
                        count += 1;
                    }
                }

                return if let GrabType::Pong = grab_type {
                    if count >= 2 {
                        self.discard.pop()
                    } else {
                        None
                    }
                } else {
                    if count >= 3 {
                        self.discard.pop()
                    } else {
                        None
                    }
                }
            },
            GrabType::Chow => {
                match self.discard.last().unwrap() {
                    Tile::Bamboo(v1) | Tile::Character(v1) | Tile::Dot(v1) => {
                        let mut flags: (Option<Tile>, Option<Tile>, Option<Tile>, Option<Tile>) = (None, None, None, None);
                        let last_discard = self.discard.last().unwrap();

                        for tile in &player.hand {
                            match (last_discard, tile) {
                                (Tile::Bamboo(_), Tile::Bamboo(v2)) |
                                (Tile::Character(_), Tile::Character(v2)) |
                                (Tile::Dot(_), Tile::Dot(v2)) => {
                                    if *v1 > 1 && *v2 == v1 - 2 {
                                        flags.0 = Some(*tile);
                                    } else if *v1 > 0 && *v2 == v1 - 1 {
                                        flags.1 = Some(*tile);
                                    } else if *v1 < 9 && *v2 == v1 + 1 {
                                        flags.2 = Some(*tile);
                                    } else if *v1 < 8 && *v2 == v1 + 2 {
                                        flags.3 = Some(*tile);
                                    }
                                },
                                _ => {}
                            }
                        }

                        return if flags.0 != None && flags.1 != None ||
                            flags.1 != None && flags.2 != None ||
                            flags.2 != None && flags.3 != None {
                            self.discard.pop()
                        } else {
                            None
                        }
                    },
                    _ => None
                }
            }
        }
    }
    pub fn discard_tile(&mut self, player: &mut Player, index: usize) {
        if index >= player.hand.len() {
            return
        }

        self.discard.push(player.hand.remove(index));
    }
    pub fn create_game() -> Game {
        Game {
            players: [Player::create_player(Wind::North),
                        Player::create_player(Wind::East),
                        Player::create_player(Wind::South),
                        Player::create_player(Wind::West)],
            wall: DECK.to_vec(),
            discard: Vec::new()
        }
    }
}