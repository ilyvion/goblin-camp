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

pub struct Entity {
    pos: Coordinate,
    uid: i32,
    zone: i32,
    reserved: bool,
    name: String,
    faction: i32,
    velocity: i32,
    next_velocity_move: i32,
    velocity_target: Coordinate,
    // std::list<FlightPath> flightPath;
    bulk: i32,
    strobe: f32,
    // static int uids;
}
