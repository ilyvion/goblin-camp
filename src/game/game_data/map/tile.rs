/*
    Copyright 2010-2011 Ilkka Halila
    Copyright 2019 Alexander Krivács Schrøder

    This file is part of Goblin Camp Revival.

    Goblin Camp Revival is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Goblin Camp Revival is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Goblin Camp Revival.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::coordinate::Direction;
use crate::data::random::Generator;
use crate::game::game_data::construction::{Construction, ConstructionTag};
use crate::game::game_data::filth_node::FilthNode;
use crate::game::game_data::map::MapDrawable;
use crate::game::game_data::water_node::WaterNode;
use std::cell::RefCell;
use std::rc::Rc;
use tcod::Color;

#[derive(Default, Clone)]
pub struct Tile {
    tile_type: TileType,
    vis: bool,
    walkable: bool,
    buildable: bool,
    move_cost: i32,
    pub construction: i32,
    low: bool,
    blocks_water: bool,
    pub water: Option<Rc<RefCell<WaterNode>>>,
    graphic: char,
    fore_color: Color,
    original_fore_color: Color,
    back_color: Color,
    pub nature_object: i32,
    //std::set<int> npcList; //Set of NPC uid's
    //std::set<int> itemList; //Set of Item uid's
    pub(crate) filth: Option<FilthNode>,
    //boost::shared_ptr<FilthNode> filth;
    //boost::shared_ptr<BloodNode> blood;
    //boost::shared_ptr<FireNode> fire;
    marked: bool,
    pub walked_over: i32,
    pub corruption: i32,
    territory: bool,
    pub burnt: i32,
    pub flow: Direction,
}

impl Tile {
    pub fn reset_type_and_height(
        &mut self,
        tile_type: TileType,
        tile_height: f32,
        generator: &mut dyn Generator,
    ) {
        // TODO: Do some magic number extractions up in here

        self.tile_type = tile_type;
        self.vis = true;
        self.walkable = true;
        self.buildable = true;
        self.low = false;

        match self.tile_type {
            TileType::Grass => {
                self.original_fore_color = Color::new(generator.generate_up_to_u8(49), 127, 0);
                if generator.generate_integer_up_to(9) < 9 {
                    if tile_height < -0.01 {
                        self.original_fore_color =
                            Color::new(generator.generate_u8(100, 192), 127, 0);
                    } else if tile_height < 0.0 {
                        self.original_fore_color =
                            Color::new(generator.generate_u8(20, 170), 127, 0);
                    } else if tile_height > 4.0 {
                        self.original_fore_color =
                            Color::new(90, generator.generate_u8(120, 150), 90);
                    }
                }
                self.back_color = Color::new(0, 0, 0);
                self.graphic = match generator.generate_integer_up_to(9) {
                    0..=3 => '.',
                    4..=7 => ',',
                    8 => ':',
                    _ => '\'',
                }
            }
            TileType::Ditch | TileType::Riverbed => {
                self.low = true;
                self.graphic = '_';
                self.original_fore_color = Color::new(125, 50, 0);
                self.move_cost = generator.generate_integer(3, 5);
                self.flow = Direction::NoDirection; // Reset flow
            }
            TileType::Bog => {
                self.graphic = match generator.generate_integer_up_to(9) {
                    0..=3 => '~',
                    4..=7 => ',',
                    8 => ':',
                    _ => '\'',
                };
                self.original_fore_color = Color::new(generator.generate_up_to_u8(184), 127, 70);
                self.back_color = Color::new(60, 30, 20);
                self.move_cost = generator.generate_integer(6, 10);
            }
            TileType::Rock => {
                self.graphic = if generator.generate_bool() { ',' } else { '.' };
                self.original_fore_color = Color::new(
                    generator.generate_u8(182, 201),
                    generator.generate_u8(182, 201),
                    generator.generate_u8(182, 201),
                );
                self.back_color = Color::new(0, 0, 0);
            }
            TileType::Mud => {
                self.low = true;
                self.graphic = if generator.generate_bool() { '#' } else { '~' };
                self.original_fore_color = Color::new(
                    generator.generate_u8(120, 130),
                    generator.generate_u8(80, 90),
                    0,
                );
                self.move_cost = 5;
            }
            TileType::Snow => {
                let color_num = generator.generate_integer(195, 250);
                self.original_fore_color = Color::new(
                    (color_num + generator.generate_integer(-5, 5)) as u8,
                    (color_num + generator.generate_integer(-5, 5)) as u8,
                    (color_num + generator.generate_integer(-5, 5)) as u8,
                );
                self.back_color = Color::new(0, 0, 0);
                self.graphic = match generator.generate_integer_up_to(9) {
                    0..=3 => '.',
                    4..=7 => ',',
                    8 => ':',
                    _ => '\'',
                };
            }
            TileType::None => {
                self.vis = false;
                self.walkable = false;
                self.buildable = false;
            }
        }
        self.fore_color = self.original_fore_color;
    }

    pub fn burn(&mut self, magnitude: i32) {
        if self.tile_type == TileType::Grass {
            self.burnt = 10.min(self.burnt + magnitude).max(0);
            if self.burnt == 0 {
                self.corrupt(0); /*Corruption changes the color, and by corrupting by 0 we just return to what color the tile
                                 would be without any burning */
                return;
            }

            if self.tile_type == TileType::Grass {
                if self.burnt < 5 {
                    self.fore_color.r = (130 + (5 - self.burnt) * 10) as u8;
                    self.fore_color.g = (80 + (5 - self.burnt) * 5) as u8;
                    self.fore_color.b = 0;
                } else {
                    self.fore_color.r = (50 + (10 - self.burnt) * 12) as u8;
                    self.fore_color.g = (50 + (10 - self.burnt) * 6) as u8;
                    self.fore_color.b = ((self.burnt - 5) * 10) as u8;
                }
            }
        }
    }

    pub fn corrupt(&mut self, magnitude: i32) {
        self.corruption += magnitude;
        if self.corruption < 0 {
            self.corruption = 0;
        }
        if self.tile_type == TileType::Grass {
            self.fore_color = self.original_fore_color
                + Color::new(self.walked_over.min(255) as u8, 0, 0)
                - Color::new(0, self.corruption.min(255) as u8, 0);
            if self.burnt > 0 {
                self.burn(0); // To re-do the color
            }
        }
    }

    pub fn has_water(&self) -> bool {
        self.water.is_some()
    }
}

impl MapDrawable for Tile {
    fn graphic(&self) -> char {
        self.graphic
    }

    fn fore_color(&self) -> Color {
        self.fore_color
    }

    fn back_color(&self) -> Color {
        self.back_color
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    None,
    Grass,
    Ditch,
    Riverbed,
    Bog,
    Rock,
    Mud,
    Snow,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Grass
    }
}

#[derive(Default, Copy, Clone)]
pub struct CacheTile {
    walkable: bool,
    move_cost: i32,
    construction: bool,
    door: bool,
    trap: bool,
    bridge: bool,
    move_speed_modifier: i32,
    water_depth: i32,
    npc_count: i32,
    fire: bool,
    pub x: i32,
    pub y: i32,
}

impl CacheTile {
    pub fn update_from(&mut self, tile: &Tile, construction: Option<&Construction>) {
        self.walkable = tile.walkable;
        self.move_cost = tile.move_cost;
        if let Some(construction) = construction {
            self.construction = true;
            self.door = construction.has_tag(ConstructionTag::Door);
            self.trap = construction.has_tag(ConstructionTag::Trap);
            self.bridge = construction.has_tag(ConstructionTag::Bridge);
            self.move_speed_modifier = construction.get_move_speed_modifier();
        } else {
            self.construction = false;
            self.door = false;
            self.trap = false;
            self.bridge = false;
            self.move_speed_modifier = 0;
        }

        self.water_depth = if let Some(water) = &tile.water {
            water.borrow().depth()
        } else {
            0
        };

        // TODO: This stuff
        //self.npc_count = tile.npc_list.size();
        //self.fire = tile.fire;
    }
}
