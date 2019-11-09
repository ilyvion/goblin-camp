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

use crate::coordinate::Coordinate;
use crate::data::random::Generator;
use crate::game::game_data::filth_node::FilthNode;
use crate::game::game_data::map::MapGraphicDrawable;
use crate::util::tcod::Chars;
use tcod::Color;

#[derive(Debug, Copy, Clone, Default)]
pub struct WaterNode {
    pub position: Coordinate,
    depth: i32,
    time_from_river_bed: i32,
    color: Color,
    graphic: char,
    inert_counter: i32,
    inert: bool,
    filth: i32,
    coastal: bool,
}

impl WaterNode {
    pub const RIVER_DEPTH: i32 = 5000;

    pub fn new(
        position: Coordinate,
        depth: i32,
        time_from_river_bed: i32,
        generator: &mut dyn Generator,
    ) -> Self {
        let mut water = Self {
            position,
            depth,
            time_from_river_bed,
            graphic: '?',
            color: Color::new(0, 128, 255),
            inert_counter: 0,
            inert: false,
            filth: 0,
            coastal: false,
        };
        water.update_graphic(generator);

        water
    }

    // AddFilth in original
    pub fn set_filth(&mut self, filth_depth: i32) {
        self.filth = filth_depth;
    }

    pub fn depth(&self) -> i32 {
        self.depth
    }

    // If this returns true,
    pub fn set_depth(
        &mut self,
        new_depth: i32,
        generator: &mut dyn Generator,
    ) -> Option<Coordinate> {
        let result = if self.depth <= 20 && new_depth <= 20 && self.depth != new_depth {
            Some(self.position)
        } else {
            None
        };
        self.depth = new_depth;
        self.update_graphic(generator);

        result
    }

    fn update_graphic(&mut self, generator: &mut dyn Generator) {
        let graphic = match self.depth {
            0 => ' ',
            2 => Chars::Block3.into(),
            1 => '.',
            _ => 219 as char,
        };

        let col = 140.max(255 - (self.depth / 25));
        if i32::from(self.color.b) < 0.max(col - (self.filth * 20)) {
            self.color.b += 1
        };
        if i32::from(self.color.b) > 0.max(col - (self.filth * 20)) {
            self.color.b -= 1
        };

        if i32::from(self.color.g) < (col / 4).max(150.min(self.filth * 10)) {
            self.color.g += 1
        }
        if i32::from(self.color.g) > (col / 4).max(150.min(self.filth * 10)) {
            self.color.g -= 1
        }

        if i32::from(self.color.r) < 190.min(self.filth * 10) {
            self.color.r += 10
        }
        if i32::from(self.color.r) > 190.min(self.filth * 10) {
            self.color.r -= 10
        }

        if self.color.b < 200 && generator.generate_integer_up_to(39) == 0 {
            self.color.b += 20
        }
        if self.color.g < 225 && generator.generate_integer_up_to(9999) == 0 {
            self.color.g += generator.generate_up_to_u8(24)
        }
    }
}

impl MapGraphicDrawable for WaterNode {
    fn graphic(&self) -> char {
        self.graphic
    }

    fn fore_color(&self) -> Color {
        self.color
    }
}
