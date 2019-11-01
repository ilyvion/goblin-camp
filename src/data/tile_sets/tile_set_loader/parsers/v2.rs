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

use crate::data::tile_sets::tile_set::TilesetMetadata as TilesetMetadataTrait;
use crate::data::tile_sets::tile_set_loader::parsers::{Error, Io, Parser, TileSetParser};
use crate::ui::Size;
use serde_derive::Deserialize;
use serde_tcod_config_parser::de::Deserializer;
use snafu::ResultExt;
use std::path::{Path, PathBuf};

pub struct TileSetParserV2 {
    path: PathBuf,
}

impl TileSetParserV2 {
    pub const FILE_NAME: &'static str = "tilesetV2.dat";

    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl TileSetParser for TileSetParserV2 {
    fn parse_metadata(&mut self) -> Result<Box<dyn TilesetMetadataTrait>, Error> {
        let file = std::fs::read_to_string(&self.path).context(Io)?;
        let mut metadata: TilesetMetadata = Deserializer::from_str(&file).context(Parser)?;
        metadata.dir = self
            .path
            .parent()
            .ok_or(Error::PathParentError {
                child: self.path.clone(),
            })?
            .to_path_buf();
        Ok(Box::new(metadata))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename = "creature_sprite")]
#[serde(deny_unknown_fields)]
struct CreatureSprite {
    instance_name: String,

    sprites: Vec<i32>,
    #[serde(rename = "weaponOverlays", default)]
    weapon_overlays: Vec<i32>,
    fps: Option<i32>,
    #[serde(rename = "equipmentMap", default)]
    equipment_map: bool,
    #[serde(rename = "paperdoll", default)]
    paper_doll: bool,
    #[serde(rename = "weaponTypes", default)]
    weapon_types: Vec<String>,
    #[serde(rename = "armorTypes", default)]
    armor_types: Vec<String>,
    #[serde(rename = "armourTypes", default)]
    armour_types: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "plant_sprite")]
#[serde(deny_unknown_fields)]
struct PlantSprite {
    instance_name: String,

    sprites: Vec<i32>,
    fps: Option<i32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "item_sprite")]
#[serde(deny_unknown_fields)]
struct ItemSprite {
    instance_name: String,

    sprites: Vec<i32>,
    #[serde(rename = "drawWhenWielded", default)]
    draw_when_wielded: bool,
    fps: Option<i32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "spell_sprite")]
#[serde(deny_unknown_fields)]
struct SpellSprite {
    instance_name: String,

    sprites: Vec<i32>,
    fps: Option<i32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "construction_sprite")]
#[serde(deny_unknown_fields)]
struct ConstructionSprite {
    instance_name: String,

    sprites: Vec<i32>,
    #[serde(rename = "underconstructionSprites", default)]
    under_construction_sprites: Vec<i32>,
    #[serde(rename = "openSprite")]
    open_sprite: Option<i32>,
    #[serde(rename = "unreadyTrapSprites", default)]
    unready_trap_sprites: Vec<i32>,
    width: Option<i32>,
    #[serde(rename = "connectionMap", default)]
    connection_map: bool,
    #[serde(rename = "frameCount")]
    frame_count: Option<i32>,
    fps: Option<i32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "status_effect_sprite")]
#[serde(deny_unknown_fields)]
struct StatusEffectSprite {
    instance_name: String,

    sprites: Vec<i32>,
    #[serde(rename = "flashRate")]
    flash_rate: Option<i32>,
    fps: Option<i32>,
    #[serde(rename = "alwaysOn", default)]
    always_on: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "terrain_sprite")]
#[serde(deny_unknown_fields)]
struct TerrainSprite {
    instance_name: String,

    #[serde(rename = "wangTileset", default)]
    wang_tileset: bool,
    #[serde(rename = "snowWangTileset", default)]
    snow_wang_tileset: bool,
    sprites: Vec<i32>,
    #[serde(rename = "snowSprites", default)]
    snow_sprites: Vec<i32>,
    #[serde(rename = "heightSplits", default)]
    height_splits: Vec<f32>,
    #[serde(rename = "edgeSprites", default)]
    edge_sprites: Vec<i32>,
    #[serde(rename = "snowEdgeSprites", default)]
    snow_edge_sprites: Vec<i32>,
    #[serde(default)]
    details: Vec<i32>,
    #[serde(rename = "burntDetails", default)]
    burnt_details: Vec<i32>,
    #[serde(rename = "snowedDetails", default)]
    snowed_details: Vec<i32>,
    #[serde(rename = "corruptDetails", default)]
    corrupt_details: Vec<i32>,
    #[serde(rename = "detailsChance")]
    details_chance: Option<f32>,
    #[serde(default)]
    corruption: Vec<i32>,
    #[serde(rename = "corruptionOverlay", default)]
    corruption_overlay: Vec<i32>,
    #[serde(rename = "burntOverlay", default)]
    burnt_overlay: Vec<i32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "tileset")]
#[serde(deny_unknown_fields)]
struct Tileset {
    instance_name: String,
    description: String,
    author: String,
    version: String,

    #[serde(rename = "tileWidth")]
    tile_width: i32,

    #[serde(rename = "tileHeight")]
    tile_height: i32,

    texture: Vec<Texture>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "tileset")]
struct TilesetMetadata {
    #[serde(skip)]
    dir: PathBuf,

    instance_name: String,
    description: String,
    author: String,
    version: String,

    #[serde(rename = "tileWidth")]
    tile_width: i32,

    #[serde(rename = "tileHeight")]
    tile_height: i32,

    // TODO: If the deserializer can be made to handle ignored_any, we can remove this
    #[serde(rename = "texture")]
    _texture: Vec<Texture>,
}

impl TilesetMetadataTrait for TilesetMetadata {
    fn dir(&self) -> &Path {
        &self.dir
    }

    fn name(&self) -> &str {
        &self.instance_name
    }

    fn author(&self) -> &str {
        &self.author
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn size(&self) -> Size {
        Size::new(self.tile_width, self.tile_height)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename = "texture")]
#[serde(deny_unknown_fields)]
struct Texture {
    instance_name: String,

    // Terrain modifiers
    #[serde(default)]
    water: Vec<i32>,
    #[serde(default)]
    ice: Vec<i32>,
    #[serde(rename = "waterAndIce", default)]
    water_and_ice: Vec<i32>,
    #[serde(rename = "minorFilth")]
    minor_filth: Option<i32>,
    #[serde(default)]
    filth: Vec<i32>,
    #[serde(rename = "majorFilth", default)]
    major_filth: Vec<i32>,
    #[serde(default)]
    marker: Vec<i32>,
    #[serde(rename = "markerFPS")]
    marker_fps: Option<i32>,
    #[serde(default)]
    blood: Vec<i32>,

    // Overlays
    #[serde(rename = "nonTerritory", default)]
    non_territory: Vec<i32>,
    #[serde(default)]
    territory: Vec<i32>,
    #[serde(default)]
    marked: Vec<i32>,
    #[serde(default)]
    corruption: Vec<i32>,
    #[serde(rename = "corruptionOverlay", default)]
    corruption_overlay: Vec<i32>,
    #[serde(rename = "defaultUnderconstruction")]
    default_under_construction: Option<i32>,
    #[serde(default)]
    fire: Vec<i32>,
    #[serde(rename = "fireFPS")]
    fire_fps: Option<i32>,

    // Cursors
    #[serde(rename = "defaultTileHighlight", default)]
    default_tile_highlight: Vec<i32>,
    #[serde(rename = "constructionTileHighlight", default)]
    construction_tile_highlight: Vec<i32>,
    #[serde(rename = "stockpileTileHighlight", default)]
    stockpile_tile_highlight: Vec<i32>,
    #[serde(rename = "treeFellingTileHighlight", default)]
    tree_felling_tile_highlight: Vec<i32>,
    #[serde(rename = "harvestTileHighlight", default)]
    harvest_tile_highlight: Vec<i32>,
    #[serde(rename = "orderTileHighlight", default)]
    order_tile_highlight: Vec<i32>,
    #[serde(rename = "treeTileHighlight", default)]
    tree_tile_highlight: Vec<i32>,
    #[serde(rename = "dismantleTileHighlight", default)]
    dismantle_tile_highlight: Vec<i32>,
    #[serde(rename = "undesignateTileHighlight", default)]
    un_designate_tile_highlight: Vec<i32>,
    #[serde(rename = "bogTileHighlight", default)]
    bog_tile_highlight: Vec<i32>,
    #[serde(rename = "digTileHighlight", default)]
    dig_tile_highlight: Vec<i32>,
    #[serde(rename = "addTerritoryTileHighlight", default)]
    add_territory_tile_highlight: Vec<i32>,
    #[serde(rename = "removeTerritoryTileHighlight", default)]
    remove_territory_tile_highlight: Vec<i32>,
    #[serde(rename = "gatherTileHighlight", default)]
    gather_tile_highlight: Vec<i32>,

    #[serde(default)]
    creature_sprite: Vec<CreatureSprite>,
    #[serde(default)]
    plant_sprite: Vec<PlantSprite>,
    #[serde(default)]
    item_sprite: Vec<ItemSprite>,
    #[serde(default)]
    construction_sprite: Vec<ConstructionSprite>,
    #[serde(default)]
    spell_sprite: Vec<SpellSprite>,
    #[serde(default)]
    status_effect_sprite: Vec<StatusEffectSprite>,
    #[serde(default)]
    terrain_sprite: Vec<TerrainSprite>,
}
