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
use tcod::heightmap::HeightMap;

mod tile;
mod weather;

use crate::coordinate::{Coordinate, Direction};
use crate::data::base::{Position, Rectangle, Size};
use crate::data::random::Generator;
use crate::game::game_data::camera::Camera;
use crate::game::game_data::construction::Construction;
use crate::game::game_data::filth_node::FilthNode;
use crate::game::game_data::water_node::WaterNode;
use crate::util::extras::Array2DCoordinateAccessor;
use crate::util::tcod::Chars;
use crate::util::{compare_and_pick, dual_map, Array2D, SafeConsole};
use itertools::iproduct;
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;
use tcod::console::Offscreen;
use tcod::{colors, Color};
pub use tile::*;
pub use weather::*;

const HARDCODED_WIDTH: usize = 500;
const HARDCODED_HEIGHT: usize = 500;

pub struct Map {
    pub height_map: HeightMap,
    tile_map: Array2D<Tile>,
    cached_tile_map: Array2D<CacheTile>,
    pub extent: Size,
    pub water_level: f32,
    overlay_flags: i32,
    // TODO: Use enum/bitflags?
    map_markers: Vec<(i32, MapMarker)>,
    marker_ids: i32,
    changed_tiles: HashSet<Coordinate>,
    weather: Weather,

    // Moved from Game
    water_list: Vec<Rc<RefCell<WaterNode>>>,
    filth_list: Vec<FilthNode>,
    static_construction_list: HashMap<i32, Construction>,
    dynamic_construction_list: HashMap<i32, Construction>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            height_map: HeightMap::new(HARDCODED_WIDTH as i32, HARDCODED_HEIGHT as i32),
            tile_map: Array2D::new(HARDCODED_WIDTH, HARDCODED_HEIGHT),
            cached_tile_map: Array2D::new_with(HARDCODED_WIDTH, HARDCODED_HEIGHT, |i, e| {
                let mut cache_tile = CacheTile::default();
                cache_tile.x = i as i32;
                cache_tile.y = e as i32;

                cache_tile
            }),
            extent: Size::new(HARDCODED_WIDTH as i32, HARDCODED_HEIGHT as i32),
            water_level: -0.8,
            overlay_flags: 0,
            map_markers: vec![],
            marker_ids: 0,
            changed_tiles: HashSet::new(),
            weather: Weather::new(),

            water_list: vec![],
            filth_list: vec![],
            static_construction_list: HashMap::new(),
            dynamic_construction_list: HashMap::new(),
        }
    }

    // ResetType in original
    pub fn set_tile_type(
        &mut self,
        p: Coordinate,
        tile_type: TileType,
        generator: &mut dyn Generator,
    ) {
        self.set_tile_type_and_height(p, tile_type, 0., generator)
    }

    // ResetType in original
    pub fn set_tile_type_and_height(
        &mut self,
        p: Coordinate,
        tile_type: TileType,
        tile_height: f32,
        generator: &mut dyn Generator,
    ) {
        if self.extent.is_inside(p) {
            self.tile_map.by_coordinate_mut(p).reset_type_and_height(
                tile_type,
                tile_height,
                generator,
            );
            self.changed_tiles.insert(p);
        }
    }

    // TODO: Rename to something better. Reduces effects like walking, burning and corrupting.
    pub fn naturify(&mut self, p: Coordinate) {
        if self.extent.is_inside(p) {
            let tile = self.tile_map.by_coordinate_mut(p);
            if tile.walked_over > 0 {
                tile.walked_over -= 1;
            }
            if tile.burnt > 0 {
                tile.burn(-1);
            }
            if tile.walked_over == 0 && tile.nature_object < 0 && tile.construction < 0 {
                // TODO: Extract into own method?
                let mut nature_objects = 0;
                let begin = self.extent.shrink(p - 2);
                let end = self.extent.shrink(p + 2);
                for ix in begin.x..=end.x {
                    for iy in begin.y..=end.y {
                        if self.tile_map[ix as usize][iy as usize].nature_object >= 0 {
                            nature_objects += 1;
                        }
                    }
                }
                let tile = self.tile_map.by_coordinate_mut(p);

                //Corrupted areas have less flora
                let nature_objects_target = if tile.corruption < 100 { 6 } else { 1 };
                if nature_objects < nature_objects_target {
                    // TODO: Create nature object
                    // Game::Inst()->CreateNatureObject(p, natureObjects);
                    unimplemented!()
                }
            }
        }
    }

    pub fn randomize_wind(&mut self, generator: &mut dyn Generator) {
        self.weather.randomize_wind(generator);
    }

    pub fn calculate_flow(&mut self, px: [i32; 4], py: [i32; 4], generator: &mut dyn Generator) {
        self.set_river_flow(px, py, generator);
        self.set_ground_flow(generator);
    }

    pub fn add_to_cache(&mut self, p: Coordinate) {
        self.changed_tiles.insert(p);
    }

    pub fn update_cache(&mut self) {
        for tile_coord in self.changed_tiles.drain() {
            let construction = self.tile_map.by_coordinate(tile_coord).construction;
            self.cached_tile_map
                .by_coordinate_mut(tile_coord)
                .update_from(
                    self.tile_map.by_coordinate(tile_coord),
                    [
                        &self.static_construction_list,
                        &self.dynamic_construction_list,
                    ]
                    .construction(construction),
                );
        }
    }

    pub fn filth(&self, p: Coordinate) -> Option<&FilthNode> {
        self.tile_map.by_coordinate(p).filth.as_ref()
    }

    pub fn remove_filth(&mut self, p: Coordinate) {
        unimplemented!()
    }

    pub fn water(&self, p: Coordinate) -> Option<Ref<WaterNode>> {
        self.tile_map
            .by_coordinate(p)
            .water
            .as_ref()
            .map(|w| w.borrow())
    }

    pub fn water_mut(&mut self, p: Coordinate) -> Option<RefMut<WaterNode>> {
        self.tile_map
            .by_coordinate_mut(p)
            .water
            .as_mut()
            .map(|w| w.borrow_mut())
    }

    pub fn add_water(&mut self, p: Coordinate, water: WaterNode) {
        let water_rc = Rc::new(RefCell::new(water));
        self.water_list.push(Rc::clone(&water_rc));
        self.tile_map.by_coordinate_mut(p).water = Some(water_rc);
    }

    pub fn render_map(&mut self, mut render_data: MapRenderData) {
        let (char_x, char_y) = tcod::system::get_char_size();

        render_data.viewport.position.x /= char_x;
        render_data.viewport.position.y /= char_y;
        render_data.viewport.size.width /= char_x;
        render_data.viewport.size.height /= char_y;

        let up_left = Coordinate::new(
            render_data.camera.x() as i32 - (render_data.viewport.size.width / 2),
            render_data.camera.y() as i32 - (render_data.viewport.size.height / 2),
        );

        //let (screen_delta_x, screen_delta_y) = up_left.into();

        let minimap = self.render_minimap(&render_data, up_left);

        // fn blit<C: Console + SafeConsole, S:Console>(&mut self, source: &S, source_rect: Rectangle, dest_rect: Rectangle,foreground_alpha: f32, background_alpha: f32) {
        render_data.console.blit::<tcod::console::Root, _>(
            &minimap,
            render_data.viewport,
            render_data.viewport.position,
            1.,
            1.,
        );

        /*
        (float focusX, float focusY, int viewportX, int viewportY, int viewportW, int viewportH)

            if (!(map->GetOverlayFlags() & TERRAIN_OVERLAY)) {
                InternalDrawMapItems("static constructions",  Game::Inst()->staticConstructionList, upleft, &minimap);
                InternalDrawMapItems("dynamic constructions", Game::Inst()->dynamicConstructionList, upleft, &minimap);
                //TODO: Make this consistent
                for (std::map<int,boost::shared_ptr<Item> >::iterator itemi = Game::Inst()->itemList.begin(); itemi != Game::Inst()->itemList.end();) {
                    if (!itemi->second) {
                        std::map<int,boost::shared_ptr<Item> >::iterator tmp = itemi;
                        ++itemi;
                        Game::Inst()->itemList.erase(tmp);
                        continue;
                    } else if (!itemi->second->ContainedIn().lock()) {
                        itemi->second->Draw(upleft, &minimap);
                    }
                    ++itemi;
                }
            }

            for (Map::MarkerIterator markeri = map->MarkerBegin(); markeri != map->MarkerEnd(); ++markeri) {
                int markerX = markeri->second.X();
                int markerY = markeri->second.Y();
                if (markerX >= upleft.X() && markerX < upleft.X() + viewportW
                    && markerY >= upleft.Y() && markerY < upleft.Y() + viewportH) {
                        minimap.putCharEx(markerX - upleft.X(), markerY - upleft.Y(), markeri->second.Graphic(), markeri->second.Color(), TCODColor::black);
                }
            }


            InternalDrawMapItems("NPCs",                  Game::Inst()->npcList, upleft, &minimap);
            for (std::list<boost::weak_ptr<FireNode> >::iterator firei = Game::Inst()->fireList.begin(); firei != Game::Inst()->fireList.end(); ++firei) {
                if (firei->lock()) firei->lock()->Draw(upleft, &minimap);
            }
            for (std::list<boost::shared_ptr<Spell> >::iterator spelli = Game::Inst()->spellList.begin(); spelli != Game::Inst()->spellList.end(); ++spelli) {
                (*spelli)->Draw(upleft, &minimap);
            }

            TCODConsole::blit(&minimap, 0, 0, viewportW, viewportH, console, viewportX, viewportY);
        */
    }

    #[allow(clippy::nonminimal_bool)]
    fn set_river_flow(&mut self, px: [i32; 4], py: [i32; 4], generator: &mut dyn Generator) {
        let mut x_directions = compare_and_pick(&px, Direction::West, Direction::East, || {
            generator.generate_bool()
        });
        let mut y_directions = compare_and_pick(&py, Direction::North, Direction::South, || {
            generator.generate_bool()
        });
        let coordinates = Coordinate::from_slices(&px, &py);
        let rectilinear_distances = dual_map(&coordinates, |c1, c2| c1.rectilinear_distance_to(c2));
        let xy_distances = dual_map(&coordinates, |c1, c2| c1.xy_difference(c2));

        // Reverse?
        if generator.generate_bool() {
            x_directions.iter_mut().for_each(|x| *x = x.reverse());
            y_directions.iter_mut().for_each(|y| *y = y.reverse());
        }

        #[derive(Copy, Clone, Eq, PartialEq)]
        struct Unfinished(i32, Coordinate);
        impl Ord for Unfinished {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }
        impl PartialOrd for Unfinished {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        let mut unfinished = BinaryHeap::new();

        unfinished.push(Unfinished(0, coordinates[0]));

        let mut favor = [false, false];

        if xy_distances[0] > 15 {
            favor[0] = true;
        } else if xy_distances[0] < 15 {
            favor[1] = true;
        }

        let mut stage = 0;
        let mut touched = HashSet::new();
        while !unfinished.is_empty() {
            let current = unfinished.pop().unwrap().1;

            let mut result = [
                generator.generate_integer_up_to(if favor[0] { 3 } else { 1 }),
                generator.generate_integer_up_to(if favor[1] { 3 } else { 1 }),
            ];

            if result[0] == result[1] {
                if generator.generate_bool() {
                    result[0] += 1;
                } else {
                    result[1] += 1;
                }
            }
            if result[0] > result[1] {
                self.tile_map.by_coordinate_mut(current).flow = x_directions[stage];
            } else {
                self.tile_map.by_coordinate_mut(current).flow = y_directions[stage];
            }

            for y in current.y - 1..=current.y + 1 {
                for x in current.x - 1..=current.x + 1 {
                    let pos = Coordinate::new(x, y);
                    if self.extent.is_inside(pos)
                        && !touched.contains(&pos)
                        && self.tile_map.by_coordinate(pos).has_water()
                    {
                        touched.insert(pos);
                        unfinished.push(Unfinished(
                            std::i32::MAX - pos.rectilinear_distance_to(coordinates[0]),
                            pos,
                        ));

                        if stage < 2
                            && update_stage_favor(
                                &mut stage,
                                &mut favor,
                                pos,
                                &coordinates,
                                &rectilinear_distances,
                                &xy_distances,
                            )
                            && stage < 2
                        {
                            update_stage_favor(
                                &mut stage,
                                &mut favor,
                                pos,
                                &coordinates,
                                &rectilinear_distances,
                                &xy_distances,
                            );
                        }
                    }
                }
            }
        }
    }

    /// Calculate flow for all ground tiles
    ///
    /// 'flow' is used for propagation of filth over time, and
    /// displacement of objects in the river.
    ///
    /// Flow is determined by the heightmap: each tile flows to its
    /// lowest neighbor. When all neighbors have the same height,
    /// we choose to flow towards the river, by picking a random
    /// water tile and flowing toward it.
    fn set_ground_flow(&mut self, generator: &mut dyn Generator) {
        for (y, x) in iproduct!(0..self.extent.height, 0..self.extent.width) {
            let pos = Coordinate::new(x, y);
            let tile = self.tile_map.by_coordinate_mut(pos);
            if tile.flow == Direction::NoDirection {
                let mut lowest = Coordinate::new(x, y);
                for (iy, ix) in iproduct!(y - 1..=y + 1, x - 1..=x + 1) {
                    let candidate = Coordinate::new(ix, iy);
                    if self.extent.is_inside(candidate)
                        && self.height_map.get_value(ix, iy)
                            < self.height_map.get_value(lowest.x, lowest.y)
                    {
                        lowest = candidate;
                    }
                }

                tile.flow = pos.direction_to(lowest);

                if tile.flow == Direction::NoDirection && !self.water_list.is_empty() {
                    // No slope here, so approximate towards river
                    let random_water = generator.select_by_ref(&self.water_list[..]);
                    let random_water = &*random_water.borrow();
                    let coord = random_water.position;
                    tile.flow = pos.direction_to(coord);
                }
            }
        }
    }

    fn render_minimap(&mut self, render_data: &MapRenderData, up_left: Coordinate) -> Offscreen {
        let mut minimap = Offscreen::new(
            render_data.viewport.size.width,
            render_data.viewport.size.height,
        );
        for (y, x) in iproduct!(
            up_left.y..up_left.y + minimap.height(),
            up_left.x..up_left.x + minimap.width()
        ) {
            let xy = Coordinate::new(x, y);
            if self.extent.is_inside(xy) {
                let tile = self.tile_map.by_coordinate(xy);
                tile.draw(&mut minimap, xy - up_left);

                // TODO: Support overlays (map->GetOverlayFlags() & TERRITORY_OVERLAY)
                //       Then do: minimap.setCharBackground(x-screenDeltaX,y-screenDeltaY, map->IsTerritory(xy) ? TCODColor(45,85,0) : TCODColor(80,0,0));
                if let Some(water) = self.water(xy) {
                    if water.depth() > 0 {
                        water.draw(&mut minimap, xy - up_left);
                    }
                }
                if let Some(filth) = self.filth(xy) {
                    if filth.depth() > 0 {
                        filth.draw(&mut minimap, xy - up_left);
                    }
                }
            /*
                int natNum = map->GetNatureObject(xy);
                if (natNum >= 0) {
                    Game::Inst()->natureList[natNum]->Draw(upleft,&minimap);
                }
            */
            } else {
                minimap.put_char_ex(
                    (xy - up_left).into(),
                    Chars::Block3.into(),
                    colors::BLACK,
                    colors::WHITE,
                );
            }
        }

        minimap
    }
}

pub struct MapRenderData<'m> {
    camera: &'m Camera,
    viewport: Rectangle,
    console: &'m mut dyn SafeConsole,
}

impl<'m> MapRenderData<'m> {
    pub fn new(camera: &'m Camera, viewport: Rectangle, console: &'m mut dyn SafeConsole) -> Self {
        Self {
            camera,
            viewport,
            console,
        }
    }
}

pub trait MapDrawable {
    fn graphic(&self) -> char;
    fn fore_color(&self) -> Color;
    fn back_color(&self) -> Color {
        colors::BLACK
    }

    fn draw<P: Into<Position>>(&self, console: &mut dyn SafeConsole, p: P) {
        console.put_char_ex(
            p.into(),
            self.graphic(),
            self.fore_color(),
            self.back_color(),
        );
    }
}

fn update_stage_favor(
    stage: &mut usize,
    favor: &mut [bool],
    pos: Coordinate,
    coordinates: &[Coordinate],
    rectilinear_distances: &[i32],
    xy_distances: &[i32],
) -> bool {
    if pos.rectilinear_distance_to(coordinates[*stage]) > rectilinear_distances[*stage] {
        *stage += 1;
        favor[0] = false;
        favor[1] = false;
        if xy_distances[*stage] > 15 {
            favor[0] = true;
        } else if xy_distances[*stage] < 15 {
            favor[1] = true;
        }

        true
    } else {
        false
    }
}

struct MapMarker;

pub trait MapExtentHelper {
    fn is_inside(&self, p: Coordinate) -> bool;
    fn shrink(&self, p: Coordinate) -> Coordinate;
}

impl MapExtentHelper for Size {
    fn is_inside(&self, p: Coordinate) -> bool {
        p.inside_extent(Coordinate::ORIGIN, Coordinate::from(*self))
    }

    fn shrink(&self, p: Coordinate) -> Coordinate {
        p.shrink_extent(Coordinate::ORIGIN, Coordinate::from(*self))
    }
}

pub trait ConstructionHelper<'a> {
    fn construction(self, id: i32) -> Option<&'a Construction>;
}

impl<'a> ConstructionHelper<'a> for &'a [&HashMap<i32, Construction>] {
    fn construction(self, id: i32) -> Option<&'a Construction> {
        self.iter().filter_map(|m| m.get(&id)).next()
    }
}
