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

use crate::game::game_state::{GameState, GameStateChange, GameStateResult, GameStateUpdateResult};
use crate::game::GameRef;
use snafu::{ResultExt, Snafu};
use std::borrow::Cow;
use tcod::{colors, console, BackgroundFlag, Console, TextAlignment};

use crate::data::paths::PathProvider;
use crate::data::settings::Settings;
use crate::data::tile_sets::tile_set::TilesetMetadata;
use crate::data::tile_sets::TileSets;
use crate::util::tcod::Chars;
use std::path::Path;
use tcod::console::Offscreen;
use tcod::input::KeyCode;

#[derive(Debug, Snafu)]
pub enum Error {
    SettingsSave {
        source: crate::data::settings::Error,
    },
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Default)]
pub struct TileSetsDialog {
    tile_sets_metadata: Option<Vec<Box<dyn TilesetMetadata>>>,
    off_screen: Option<Offscreen>,
    original_tile_set: Option<String>,
    selected_tile_set: usize,
    screen_width: i32,
    screen_height: i32,
    list_width: i32,
    scroll: i32,
    sub_h: i32,
}

impl TileSetsDialog {
    pub fn game_state_change(_: &mut GameRef) -> GameStateChange {
        GameStateChange::Push(Self::game_state())
    }

    pub fn game_state() -> Box<dyn GameState> {
        Box::new(Self::default())
    }

    fn update_offscreen(&mut self) {
        let sub = self.off_screen.as_mut().unwrap();
        sub.set_default_background(colors::BLACK);
        sub.set_alignment(TextAlignment::Left);
        for (current_y, tile_set_metadata) in
            self.tile_sets_metadata.as_ref().unwrap().iter().enumerate()
        {
            if self.selected_tile_set == current_y {
                sub.set_default_foreground(colors::GREEN);
            } else {
                sub.set_default_background(colors::DARK_GREY);
            }
            sub.print(0, current_y as i32, tile_set_metadata.name());
        }
    }

    pub fn save_settings<P: AsRef<Path>>(&mut self, settings: &Settings, path: P) -> Result {
        settings.save(path).context(SettingsSave)?;

        Ok(())
    }
}

impl GameState for TileSetsDialog {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("Tile sets dialog")
    }

    fn activate(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        self.screen_width = game_ref.root.width();
        self.screen_height = game_ref.root.height();

        self.list_width = self.screen_width / 3;

        let tile_sets_metadata =
            TileSets::load_tile_set_metadata(&game_ref.data.paths, game_ref.logger.clone())?;
        self.sub_h = tile_sets_metadata.len() as i32;
        self.original_tile_set = game_ref.data.settings.tile_set.clone();
        let tile_set = self
            .original_tile_set
            .as_ref()
            .map(|s| s.as_ref())
            .unwrap_or("default");
        self.selected_tile_set = if let Some((current_selection, _)) = tile_sets_metadata
            .iter()
            .enumerate()
            .find(|&(_, t)| t.dir_name() == tile_set)
        {
            current_selection
        } else {
            0
        };
        self.tile_sets_metadata = Some(tile_sets_metadata);

        self.off_screen = Some(Offscreen::new(self.list_width - 2, self.sub_h.max(1)));
        self.update_offscreen();

        self.scroll = 0;

        Ok(())
    }

    fn update(&mut self, game_ref: &mut GameRef) -> GameStateUpdateResult {
        if game_ref.input.key_event.raw.code == KeyCode::Escape {
            game_ref.data.settings.tile_set = self.original_tile_set.take();
            return Ok(GameStateChange::Pop);
        }

        let mouse_event = game_ref.input.mouse_event;
        if mouse_event.clicked {
            let position = mouse_event.character_position;
            let button_distance = (self.screen_width - self.list_width) / 3;
            if position.x == self.list_width - 2 {
                if position.y == 1 {
                    self.scroll = 0.max(self.scroll - 1);
                } else if position.y == self.screen_height - 2 {
                    self.scroll = 0.max((self.sub_h - self.screen_height + 3).min(self.scroll + 1));
                }
            } else if position.x > 1
                && position.x < self.list_width - 2
                && position.y > 1
                && position.y < self.screen_height - 2
                && position.y - 2 + self.scroll
                    < self.tile_sets_metadata.as_ref().unwrap().len() as i32
            {
                self.selected_tile_set = (self.scroll + position.y - 2) as usize;
            } else if position.y >= self.screen_height - 6 && position.y < self.screen_height - 3 {
                if position.x >= self.list_width + button_distance - 4
                    && position.x < self.list_width + button_distance + 4
                {
                    // OK button
                    let selected_tile_set =
                        &self.tile_sets_metadata.as_ref().unwrap()[self.selected_tile_set];
                    game_ref.data.settings.tile_set =
                        Some(selected_tile_set.dir_name().to_string());

                    self.save_settings(
                        &game_ref.data.settings,
                        game_ref.data.paths.settings_file(),
                    )?;
                    return Ok(GameStateChange::Pop);
                } else if position.x >= self.list_width + 2 * button_distance - 4
                    && position.x < self.list_width + 2 * button_distance + 4
                {
                    // Cancel button
                    game_ref.data.settings.tile_set = self.original_tile_set.take();
                    return Ok(GameStateChange::Pop);
                }
            }
        }

        Ok(GameStateChange::None)
    }

    fn draw(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        game_ref.root.set_alignment(TextAlignment::Left);
        game_ref.root.set_default_foreground(colors::WHITE);
        game_ref.root.set_default_background(colors::BLACK);

        // Left frame
        game_ref.root.print_frame(
            0,
            0,
            self.list_width,
            self.screen_height,
            true,
            BackgroundFlag::Set,
            Some("Tile sets"),
        );
        console::blit(
            self.off_screen.as_ref().unwrap(),
            (0, self.scroll),
            (self.list_width - 2, self.screen_height - 4),
            game_ref.root,
            (1, 2),
            1.,
            1.,
        );

        if self.scroll > 0 {
            game_ref.root.put_char(
                self.list_width - 2,
                1,
                Chars::ArrowN.into_char(),
                BackgroundFlag::Set,
            );
        }
        if self.scroll < self.sub_h - self.screen_height + 3 {
            game_ref.root.put_char(
                self.list_width - 2,
                self.screen_height - 2,
                Chars::ArrowS.into_char(),
                BackgroundFlag::Set,
            );
        }

        // Right frame
        game_ref.root.print_frame(
            self.list_width,
            0,
            self.screen_width - self.list_width,
            self.screen_height,
            true,
            BackgroundFlag::Set,
            Some("Details"),
        );
        let tile_sets_metadata = self.tile_sets_metadata.as_ref().unwrap();
        if self.selected_tile_set < tile_sets_metadata.len() {
            let selected_tile_set = &tile_sets_metadata[self.selected_tile_set];
            game_ref.root.print(self.list_width + 3, 2, "Name:");
            game_ref
                .root
                .print(self.list_width + 12, 2, selected_tile_set.name());
            game_ref.root.print(self.list_width + 3, 4, "Size:");
            game_ref.root.print(
                self.list_width + 12,
                4,
                selected_tile_set.size().to_string(),
            );
            game_ref.root.print(self.list_width + 3, 6, "Author:");
            game_ref
                .root
                .print(self.list_width + 12, 6, selected_tile_set.author());
            game_ref.root.print(self.list_width + 3, 8, "Version:");
            game_ref
                .root
                .print(self.list_width + 12, 8, selected_tile_set.version());
            game_ref.root.print(self.list_width + 3, 10, "Description:");
            game_ref.root.print_rect(
                self.list_width + 3,
                12,
                self.screen_width - self.list_width - 6,
                self.screen_height - 19,
                selected_tile_set.description(),
            )
        }

        // Buttons
        let button_distance = (self.screen_width - self.list_width) / 3;
        game_ref.root.print_frame(
            self.list_width + button_distance - 4,
            self.screen_height - 6,
            8,
            3,
            true,
            BackgroundFlag::Default,
            None::<&str>,
        );
        game_ref.root.print(
            self.list_width + button_distance - 1,
            self.screen_height - 5,
            "Ok",
        );
        game_ref.root.print_frame(
            self.list_width + 2 * button_distance - 4,
            self.screen_height - 6,
            8,
            3,
            true,
            BackgroundFlag::Default,
            None::<&str>,
        );
        game_ref.root.print(
            self.list_width + 2 * button_distance - 3,
            self.screen_height - 5,
            "Cancel",
        );

        Ok(())
    }
}
