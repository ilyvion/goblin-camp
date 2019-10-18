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

use crate::data::paths::PathProvider;
use crate::data::settings::{Renderer, Settings};
use crate::game::game_state::{GameState, GameStateChange, GameStateResult, GameStateUpdateResult};
use crate::game::GameRef;
use crate::ui::{MessageBox, Position, Size};
use crate::util::Flip;
use crate::util::SafeConsole;
use snafu::{ResultExt, Snafu};
use std::borrow::Cow;
use std::collections::HashSet;
use std::hash::Hash;
use std::path::Path;
use tcod::input::KeyCode;
use tcod::{colors, BackgroundFlag, TextAlignment};

#[derive(Debug, Snafu)]
pub enum Error {
    SettingsSave {
        source: crate::data::settings::Error,
    },
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Default)]
pub struct KeysDialog {
    fields: Vec<KeyMapField>,
    focused_field: usize,
    original_settings: Option<Settings>,
    position: Position,
    size: Size,
    message_box: bool,
}

impl KeysDialog {
    const MIN_WIDTH: usize = 40;

    pub fn game_state_change(_: &mut GameRef) -> GameStateChange {
        GameStateChange::Push(Self::game_state())
    }

    pub fn game_state() -> Box<dyn GameState> {
        Box::new(Self::default())
    }

    pub fn save_settings<P: AsRef<Path>>(&mut self, settings: &Settings, path: P) -> Result {
        settings.save(path).context(SettingsSave)?;

        Ok(())
    }
}

impl GameState for KeysDialog {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("Keys dialog")
    }

    fn activate(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        if !self.message_box {
            self.original_settings = Some(game_ref.data.settings.clone());
            self.fields = game_ref
                .data
                .settings
                .key_bindings
                .key_map()
                .map(|(label, value)| KeyMapField {
                    label,
                    value,
                    ..Default::default()
                })
                .collect::<Vec<_>>();

            let w = self
                .fields
                .iter()
                .fold(Self::MIN_WIDTH, |acc, field| acc.max(field.label.len()));
            let h = self.fields.len() + 4;
            self.size = Size::new(w as i32, h as i32);

            let x = game_ref.root.width() / 2 - (w as i32 / 2);
            let y = game_ref.root.height() / 2 - (h as i32 / 2);
            self.position = Position::new(x, y);
        } else {
            self.message_box = false;
        }

        Ok(())
    }

    fn update(&mut self, game_ref: &mut GameRef) -> GameStateUpdateResult {
        if game_ref.input.key_event.raw.code == KeyCode::Escape {
            game_ref
                .data
                .settings
                .restore_from(self.original_settings.take().unwrap());
            return Ok(GameStateChange::Pop);
        } else if game_ref.input.key_event.raw.code == KeyCode::Enter {
            if self.fields.iter().any(|f| f.conflict) {
                self.message_box = true;
                return Ok(GameStateChange::Push(MessageBox::game_state(
                    game_ref,
                    "There are conflicting key bindings",
                    "Understood",
                    Box::new(|| GameStateChange::Pop),
                    None,
                    None,
                )));
            }

            self.save_settings(&game_ref.data.settings, game_ref.data.paths.settings_file())?;
            return Ok(GameStateChange::Pop);
        } else {
            let field = &mut self.fields[self.focused_field];

            let key = game_ref.input.key_event.raw.printable;
            if key >= ' ' && key <= '~' {
                field.update_value(key);
                game_ref
                    .data
                    .settings
                    .key_bindings
                    .update_key_map(self.focused_field, key);

                for field in self.fields.iter_mut() {
                    field.conflict = false;
                }
                let conflicts = self.fields.iter().map(|f| f.value).collect::<Vec<_>>();
                for conflict in get_non_unique_elements(conflicts) {
                    for field in self.fields.iter_mut() {
                        if field.value == conflict {
                            field.conflict = true;
                        }
                    }
                }
            }
        }

        let mouse_event = game_ref.input.mouse_event;
        if mouse_event.clicked
            && (self.position + self.size).contains_position(mouse_event.character_position)
        {
            let internal_position = mouse_event.character_position - self.position;

            if let field @ 3..=17 = internal_position.y {
                self.focused_field = field as usize - 3;
            }
        }

        Ok(GameStateChange::None)
    }

    fn draw(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        game_ref.root.set_alignment(TextAlignment::Left);

        game_ref.root.set_default_foreground(colors::WHITE);
        game_ref.root.set_default_background(colors::BLACK);

        game_ref.root.print_frame(
            self.position,
            self.size,
            true,
            BackgroundFlag::Set,
            Some("Keys"),
        );
        game_ref.root.print(
            self.position + (1, 1),
            "ENTER to save changes, ESC to discard.",
        );

        for (i, field) in self.fields.iter_mut().enumerate() {
            if self.focused_field == i {
                game_ref.root.set_default_foreground(colors::GREEN);
            } else if field.conflict {
                game_ref.root.set_default_foreground(colors::RED);
            }
            game_ref
                .root
                .print(self.position + (1, i as i32 + 3), field.label);
            game_ref.root.print(
                self.position + (self.size.width - 6, i as i32 + 3),
                field.display_value().as_ref(),
            );
            game_ref.root.set_default_foreground(colors::WHITE);
        }

        Ok(())
    }
}

#[derive(Default)]
struct KeyMapField {
    label: &'static str,
    value: char,
    display_value: Option<Cow<'static, str>>,
    conflict: bool,
}

impl KeyMapField {
    fn display_value(&mut self) -> &Cow<'static, str> {
        let value = self.value;
        self.display_value.get_or_insert_with(|| {
            if value == ' ' {
                Cow::Borrowed("[SPC]")
            } else {
                Cow::Owned(format!("[ {} ]", value.to_string()))
            }
        })
    }

    fn update_value(&mut self, value: char) {
        self.value = value;
        self.display_value = None;
    }
}

fn get_non_unique_elements<I>(iter: I) -> impl Iterator<Item = I::Item>
where
    I: IntoIterator,
    I::Item: Eq + Hash + Copy,
{
    let iter = iter.into_iter();
    let (_, size_hint) = iter.size_hint();
    let mut unique = HashSet::with_capacity(size_hint.unwrap_or(0));
    iter.filter(move |x| !unique.insert(*x))
}
