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

pub struct Weather {
    wind_direction: Direction,
    prevailing_wind_direction: Direction,
    current_weather: WeatherType,
    tile_change: bool,
    change_all: bool,
    tile_change_rate: i32,
    change_position: i32,
    current_temperature: i32,
    current_season: i32,
}

impl Weather {
    pub fn new() -> Self {
        Self {
            wind_direction: Direction::North,
            prevailing_wind_direction: Direction::North,
            current_weather: WeatherType::Normal,
            tile_change: false,
            change_all: false,
            tile_change_rate: 0,
            change_position: 0,
            current_season: -1,
            current_temperature: 0,
        }
    }

    pub fn randomize_wind(&mut self, generator: &mut dyn Generator) {
        self.prevailing_wind_direction = generator.auto_select();
        self.wind_direction = self.prevailing_wind_direction;
    }
}

pub enum WeatherType {
    Normal,
    Rain,
}
