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
use crate::game::game_data::map::Map;
use crate::game::GameRef;
use tcod::input::KeyCode;

pub struct Camera {
    x: f64,
    y: f64,
}

impl Camera {
    const SMALL_MOVEMENT: i32 = 1;
    const LARGE_MOVEMENT: i32 = 10;

    pub fn new() -> Self {
        // TODO: When map sizes are no longer hard coded, find a reasonable default for this
        Self { x: 180., y: 180. }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn center_on(&mut self, target: Coordinate) {
        self.x = f64::from(target.x) + 0.5;
        self.y = f64::from(target.y) + 0.5;
    }

    pub fn move_cam(&mut self, x: f64, y: f64, map: &Map) {
        self.x = (x + self.x).max(0.).min(f64::from(map.extent.width) + 1.0);
        self.y = (y + self.y).max(0.).min(f64::from(map.extent.height) + 1.0);
    }

    /*
    TODO: If we ever use a renderer setup, respect the scroll rate
    void Game::MoveCam(float x, float y) {
        camX = std::min(std::max(x * renderer->ScrollRate() + camX, 0.0f), Map::Inst()->Width() + 1.0f);
        camY = std::min(std::max(y * renderer->ScrollRate() + camY, 0.0f), Map::Inst()->Height() + 1.0f);
    }*/

    pub fn update(&mut self, game_ref: &mut GameRef) {
        let addition = if game_ref.input.press_key_event.raw.shift {
            Self::LARGE_MOVEMENT
        } else {
            Self::SMALL_MOVEMENT
        };
        let mut diff_x = 0;
        let mut diff_y = 0;
        let code = game_ref.input.press_key_event.raw.code;
        if code == KeyCode::Up {
            diff_y -= addition;
        } else if code == KeyCode::Down {
            diff_y += addition;
        } else if code == KeyCode::Left {
            diff_x -= addition;
        } else if code == KeyCode::Right {
            diff_x += addition;
        }

        self.move_cam(f64::from(diff_x), f64::from(diff_y), &game_ref.game_data.map);
    }
}
