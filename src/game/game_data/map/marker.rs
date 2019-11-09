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
use crate::game::game_data::map::MapGraphicDrawable;
use tcod::Color;

pub enum MarkerType {
    Flashing,
}

pub struct MapMarker {
    marker_type: MarkerType,
    color: Color,
    original_color: Color,
    duration: i32,
    graphic: char,
    pub pos: Coordinate,
    counter: f32,
    /*

    class MapMarker {
        GC_SERIALIZABLE_CLASS

        MarkerType type;
        TCODColor origColor, color;
        int duration;
        int graphic;
        int x, y; //TODO switch to Coordinate
        float counter;
    public:
        MapMarker(MarkerType=FLASHINGMARKER, int graphic='?', Coordinate position=Coordinate(0,0),
            int duration=1, TCODColor color=TCODColor::pink);
        bool Update();
        int X() const;
        int Y() const;
        Coordinate Position() const;
        int Graphic() const;
        TCODColor Color() const;
    };
        */
}

impl MapGraphicDrawable for MapMarker {
    fn graphic(&self) -> char {
        self.graphic
    }

    fn fore_color(&self) -> Color {
        self.color
    }
}
