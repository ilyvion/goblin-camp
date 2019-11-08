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

mod construction;
mod filth_node;
mod map;
mod water_node;

use crate::coordinate::{Coordinate, Direction};
use crate::data::random::Generator;
use crate::data::settings::Settings;
use crate::game::game_data::filth_node::FilthNode;
use crate::game::game_data::map::{Map, MapExtentHelper, TileType};
use crate::game::game_data::water_node::WaterNode;
use crate::util::SafeConsole;
pub use map::MapRenderData;
use tcod::line::Line;

pub struct GameData {
    pub running: bool,
    map: Map,
    /*
    int screenWidth, screenHeight;
    Season season;
    int time;
    int age;
    int orcCount, goblinCount;
    unsigned int peacefulFaunaCount;
    bool paused;
    int charWidth, charHeight;
    bool toMainMenu, running;
    int safeMonths;
    bool refreshStockpiles;
    static bool devMode;
    Coordinate marks[12];

    boost::shared_ptr<Events> events;

    std::list<std::pair<int, boost::function<void()> > > delays;

    boost::shared_ptr<MapRenderer> renderer;
    bool gameOver;

    std::map<int, boost::shared_ptr<Construction> > staticConstructionList;
    std::map<int, boost::shared_ptr<Construction> > dynamicConstructionList;
    std::map<int, boost::shared_ptr<NPC> > npcList;

    static bool initializedOnce;

    float camX, camY;

    TCODConsole* buffer;

    std::map<std::string, boost::shared_ptr<Squad> > squadList;
    std::list<boost::shared_ptr<Squad> > hostileSquadList;

    std::map<int,boost::shared_ptr<Item> > itemList;

    std::map<int, boost::shared_ptr<NatureObject> > natureList;
    std::list<boost::weak_ptr<WaterNode> > waterList;

    std::list<boost::weak_ptr<FilthNode> > filthList;

    std::list<boost::weak_ptr<BloodNode> > bloodList;

    std::list<boost::weak_ptr<FireNode> > fireList;

    std::list<boost::shared_ptr<Spell> > spellList;
    */
}

impl GameData {
    const RIVER_DIRECTIONS: [Direction; 4] = [
        Direction::West,
        Direction::East,
        Direction::North,
        Direction::South,
    ];

    pub fn new() -> Self {
        Self {
            running: false,
            map: Map::new(),
        }
    }

    pub fn reset(&mut self) {
        self.map = Map::new();
        // TODO: Finish!
        /*
        instance->npcList.clear();
        instance->natureList.clear(); //Ice decays into ice objects and water, so clear this before items and water
        instance->itemList.clear(); //Destroy current items, that way ~Construction() won't have items to try and stockpile
        while (!instance->staticConstructionList.empty()) {
            instance->staticConstructionList.erase(instance->staticConstructionList.begin());
        }
        while (!instance->dynamicConstructionList.empty()) {
            instance->dynamicConstructionList.erase(instance->dynamicConstructionList.begin());
        }

        Map::Reset();
        JobManager::Reset();
        StockManager::Reset();
        Announce::Reset();
        Camp::Reset();
        for (size_t i = 0; i < Faction::factions.size(); ++i) {
            Faction::factions[i]->Reset();
        }
        Stats::Reset();

        delete StockManagerDialog::stocksDialog;
        StockManagerDialog::stocksDialog = 0;

        delete Menu::mainMenu;
        Menu::mainMenu = 0;

        delete Menu::territoryMenu;
        Menu::territoryMenu = 0;

        UI::Reset();

        delete instance;
        instance = 0;
        */
        //unimplemented!()
    }

    pub fn generate_map(
        &mut self,
        generator: &mut dyn Generator,
        settings: &Settings,
        state: Option<&mut MapGenerationState>,
    ) -> Option<MapGenerationState> {
        if state.is_none() {
            return Some(MapGenerationState::new());
        }
        let state = state.unwrap();

        use MapGenerationStage::*;
        match state.stage {
            HeightMapClear => {
                self.map.height_map.clear();
            }
            GenerateRiver => {
                self.generate_river(&mut state.px, &mut state.py, generator, settings);
            }
            GenerateHills => {
                self.generate_hills(generator);
            }
            GenerateSmoothing => {
                self.generate_smoothing(generator);
            }
            GenerateTiles => {
                self.generate_tiles(generator);
            }
            Naturify => {
                for x in 0..self.map.extent.width {
                    for y in 0..self.map.extent.height {
                        self.map.naturify(Coordinate::new(x, y));
                    }
                }
            }
            GenerateBog => {
                self.generate_bog(generator);
            }
            RandomizeWind => {
                self.map.randomize_wind(generator);
            }
            CalculateFlow => {
                self.map.calculate_flow(state.px, state.py, generator);
            }
            UpdateCache => {
                self.map.update_cache();
            }

            Done => (),
        }

        state.stage.next();
        None
    }

    pub fn render_map(&mut self, render_data: MapRenderData) {
        self.map.render_map(render_data);
    }

    fn generate_river(
        &mut self,
        px: &mut [i32; 4],
        py: &mut [i32; 4],
        generator: &mut dyn Generator,
        settings: &Settings,
    ) {
        let river_start_left = generator.generate_bool();
        let river_end_right = generator.generate_bool();

        while {
            if river_start_left {
                px[0] = 0;
                py[0] = generator.generate_integer_up_to(self.map.extent.height - 1);
            } else {
                px[0] = generator.generate_integer_up_to(self.map.extent.width - 1);
            }

            px[1] = 10 + generator.generate_integer_up_to(self.map.extent.width - 20);
            py[1] = 10 + generator.generate_integer_up_to(self.map.extent.height - 20);
            px[2] = 10 + generator.generate_integer_up_to(self.map.extent.width - 20);
            py[2] = 10 + generator.generate_integer_up_to(self.map.extent.height - 20);

            if river_end_right {
                px[3] = self.map.extent.width - 1;
                py[3] = generator.generate_integer_up_to(self.map.extent.height - 1);
            } else {
                px[3] = generator.generate_integer_up_to(self.map.extent.width - 1);
                py[3] = self.map.extent.height - 1;
            }

            // This conditional ensures that the river's beginning and end are at least 100 units apart
            f64::from((px[0] - px[3]).pow(2) + (py[0] - py[3]).pow(2)).sqrt() < 100.
        } {}

        let depth = settings.river_depth as i32;
        let width = settings.river_width;
        self.map.height_map.dig_bezier(
            *px,
            *py,
            width as f32,
            -depth as f32,
            width as f32,
            -depth as f32,
        );
    }

    fn generate_hills(&mut self, generator: &mut dyn Generator) {
        let mut hills = 0;
        let mut infinity_check = 0;

        // infinity_check is just there to make sure our while loop doesn't become an infinite one
        // in case no suitable hill sites are found
        while hills < self.map.extent.width / 66 && infinity_check < 1000 {
            let candidate =
                generator.generate_coordinate_within_origin_extent(self.map.extent.into());
            if self.find_river_distance(candidate) > 35 {
                let centers = [
                    candidate,
                    generator.generate_coordinate_within_distance(candidate, 7),
                    generator.generate_coordinate_within_distance(candidate, 7),
                ];
                let heights = [35, 25, 25];
                for (&c, &h) in centers.iter().zip(heights.iter()) {
                    let height = generator.generate_integer(15, h);
                    let radius = generator.generate_integer(1, 3);
                    self.map.height_map.add_hill(
                        c.x as f32,
                        c.y as f32,
                        radius as f32,
                        height as f32,
                    );
                }

                hills += 1;
            }

            infinity_check += 1;
        }
    }

    fn generate_smoothing(&mut self, generator: &mut dyn Generator) {
        self.map.height_map.rain_erosion(
            self.map.extent.area() * 5,
            0.005,
            0.3,
            generator.get_tcod_rng(),
        );

        // This is a simple kernel transformation that does some horizontal smoothing
        // (lifted straight from the libtcod docs)
        const DX: [i32; 3] = [-1, 1, 0];
        const DY: [i32; 3] = [0; 3];
        const WEIGHT: [f32; 3] = [0.33; 3];
        self.map
            .height_map
            .kernel_transform(&DX, &DY, &WEIGHT, 0., 1.);
    }

    fn generate_tiles(&mut self, generator: &mut dyn Generator) {
        // Translate heightmap values into tiles
        for x in 0..self.map.extent.width {
            for y in 0..self.map.extent.height {
                let p = Coordinate::new(x, y);
                let height = self.map.height_map.get_value(x, y);
                if height < self.map.water_level {
                    let mut tile_chosen = false;
                    for ix in (x - 3)..=(x + 3) {
                        let ip = Coordinate::new(ix, y);
                        if self.map.extent.is_inside(ip)
                            && self.map.height_map.get_value(ix, y) >= self.map.water_level
                        {
                            self.map.set_tile_type(p, TileType::Ditch, generator);
                            tile_chosen = true;
                            break;
                        }
                    }
                    if !tile_chosen {
                        for iy in (y - 3)..=(y + 3) {
                            let ip = Coordinate::new(x, iy);
                            if self.map.extent.is_inside(ip)
                                && self.map.height_map.get_value(x, iy) >= self.map.water_level
                            {
                                self.map.set_tile_type(p, TileType::Ditch, generator);
                                tile_chosen = true;
                                break;
                            }
                        }
                    }
                    if !tile_chosen {
                        self.map.set_tile_type(p, TileType::Riverbed, generator);
                        self.create_water(p, WaterNode::RIVER_DEPTH, 0, generator);
                    }
                } else if height < 4.5 {
                    self.map.set_tile_type(p, TileType::Grass, generator);
                } else {
                    self.map.set_tile_type(p, TileType::Rock, generator);
                }
            }
        }
    }

    fn generate_bog(&mut self, generator: &mut dyn Generator) {
        // Create a bog
        let mut infinity_check = 0;
        while infinity_check < 1000 {
            let candidate = generator.generate_coordinate_within_rectangle(
                Coordinate::ORIGIN + 30,
                Coordinate::from(self.map.extent) - 30,
            );
            if self.find_river_distance(candidate) > 30 {
                let mut low_offset = generator.generate_integer(-5, 5);
                let mut high_offset = generator.generate_integer(-5, 5);
                for x_offset in -25..25 {
                    let range = ((25 * 25 - x_offset * x_offset) as f64).sqrt() as i32;
                    low_offset = (generator.generate_integer(-1, 1) + low_offset)
                        .min(-5)
                        .max(5);
                    high_offset = (generator.generate_integer(-1, 1) + high_offset)
                        .min(-5)
                        .max(5);
                    for y_offset in -range - low_offset..range + high_offset {
                        self.map.set_tile_type(
                            candidate + Coordinate::from((x_offset, y_offset)),
                            TileType::Bog,
                            generator,
                        );
                    }
                }
                break;
            }
            infinity_check += 1;
        }
    }

    fn find_river_distance(&self, candidate: Coordinate) -> i32 {
        let mut river_distance = 70;

        // We draw four lines from our potential site and measure the least distance to a river
        for &direction in Self::RIVER_DIRECTIONS.iter() {
            let mut distance = 70;
            let line = candidate + Coordinate::from(direction) * distance;

            let mut tcod_line = Line::new((line.x, line.y), (candidate.x, candidate.y));
            while tcod_line.step().is_some() {
                if self.map.extent.is_inside(line)
                    && self.map.height_map.get_value(line.x, line.y) < self.map.water_level
                    && distance < river_distance
                {
                    river_distance = distance;
                }
                distance -= 1;
            }
        }

        river_distance
    }

    fn create_water(
        &mut self,
        pos: Coordinate,
        amount: i32,
        time: i32,
        generator: &mut dyn Generator,
    ) {
        //If there is filth here mix it with the water
        let filth = self.map.filth(pos).map(|f| f.depth());

        if self.map.water_mut(pos).is_none() {
            let mut new_water = WaterNode::new(pos, amount, time, generator);
            if let Some(filth) = filth {
                new_water.set_filth(filth);
            }
            self.map.add_water(pos, new_water);
        } else {
            let mut water = self.map.water_mut(pos).unwrap();
            if let Some(filth) = filth {
                water.set_filth(filth);
            }
            let new_depth = water.depth() + amount;
            let coordinate = water.set_depth(new_depth, generator);
            drop(water);
            if let Some(pos) = coordinate {
                self.map.add_to_cache(pos);
            }
        }

        if let Some(_) = filth {
            self.map.remove_filth(pos);
        }
    }

    fn remove_filth(&mut self, filth: &FilthNode) {
        /*
        boost::shared_ptr<FilthNode> filth = Map::Inst()->GetFilth(pos).lock();
        if (filth) {
            for (std::list<boost::weak_ptr<FilthNode> >::iterator filthi = filthList.begin(); filthi != filthList.end(); ++filthi) {
                if (filthi->lock() == filth) {
                    filthList.erase(filthi);
                    break;
                }
            }
            Map::Inst()->SetFilth(pos, boost::shared_ptr<FilthNode>());
        }
        */
        unimplemented!()
    }
}

enum MapGenerationStage {
    HeightMapClear,
    GenerateRiver,
    GenerateHills,
    GenerateSmoothing,
    GenerateTiles,
    Naturify,
    GenerateBog,
    RandomizeWind,
    CalculateFlow,
    UpdateCache,

    Done,
}

impl MapGenerationStage {
    fn next(&mut self) {
        use MapGenerationStage::*;
        *self = match self {
            HeightMapClear => GenerateRiver,
            GenerateRiver => GenerateHills,
            GenerateHills => GenerateSmoothing,
            GenerateSmoothing => GenerateTiles,
            GenerateTiles => Naturify,
            Naturify => GenerateBog,
            GenerateBog => RandomizeWind,
            RandomizeWind => CalculateFlow,
            CalculateFlow => UpdateCache,
            UpdateCache => Done,

            Done => Done,
        }
    }
}

pub struct MapGenerationState {
    stage: MapGenerationStage,
    px: [i32; 4],
    py: [i32; 4],
}

impl MapGenerationState {
    fn new() -> Self {
        Self {
            stage: MapGenerationStage::HeightMapClear,
            px: [0; 4],
            py: [0; 4],
        }
    }

    pub fn is_done(&self) -> bool {
        match self.stage {
            MapGenerationStage::Done => true,
            _ => false,
        }
    }
}
