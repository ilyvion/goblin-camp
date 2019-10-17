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

use super::{Position, Size, UiContainer};
use crate::game::game_state::{GameState, GameStateChange, GameStateResult, GameStateUpdateResult};
use crate::game::GameRef;
use crate::ui::drawable::Drawable;
use crate::ui::{Button, Dialog, Label};
use crate::util::ConsoleWrapper;
use std::borrow::Cow;
use tcod::TextAlignment;

pub enum MessageBoxResult {
    First,
    Second,
}

pub struct MessageBox {
    dialog: Dialog<UiContainer>,
    first_button_text: String,
    second_button_text: Option<String>,
    first_result: Option<Box<dyn FnOnce() -> GameStateChange>>,
    second_result: Option<Box<dyn FnOnce() -> GameStateChange>>,
}

impl MessageBox {
    const MAX_TEXT_LINE_LENGTH: usize = 50;
    const BUTTON_WIDTH: i32 = 15;

    pub fn game_state<S: AsRef<str>>(
        game_ref: &mut GameRef,
        text: S,
        first_button_text: S,
        first_result: Box<dyn FnOnce() -> GameStateChange>,
        second_button_text: Option<S>,
        second_result: Option<Box<dyn FnOnce() -> GameStateChange>>,
    ) -> Box<dyn GameState> {
        Box::new(Self::create_message_box(
            game_ref,
            text,
            first_button_text,
            first_result,
            second_button_text,
            second_result,
        ))
    }

    pub fn create_message_box<S: AsRef<str>>(
        game_ref: &mut GameRef,
        text: S,
        first_button_text: S,
        first_result: Box<dyn FnOnce() -> GameStateChange>,
        second_button_text: Option<S>,
        second_result: Option<Box<dyn FnOnce() -> GameStateChange>>,
    ) -> Self {
        let text = text.as_ref();
        let size = Size::new(54, (text.len() / Self::MAX_TEXT_LINE_LENGTH + 8) as i32);

        let mut contents = UiContainer::new(Position::ORIGIN, size);

        // Divide the text into separate lines (might panic if not ASCII)
        let mut i = 0;
        loop {
            let j = (i + Self::MAX_TEXT_LINE_LENGTH).min(text.len());
            contents.add_component(Label::new_with_alignment(
                &text[i..j],
                Position::new(27, (2 + (i / Self::MAX_TEXT_LINE_LENGTH)) as i32),
                TextAlignment::Center,
            ));

            i += Self::MAX_TEXT_LINE_LENGTH;
            if i > text.len() {
                break;
            }
        }

        let first_button_text = first_button_text.as_ref().to_string();
        let first_button_shortcut = first_button_text.to_lowercase().chars().next();
        let second_button_text = second_button_text.map(|s| s.as_ref().to_string());
        match &second_button_text {
            None => {
                contents.add_component(Button::new(
                    first_button_text.clone(),
                    Position::new(22, (i / Self::MAX_TEXT_LINE_LENGTH + 3) as i32),
                    MessageBox::BUTTON_WIDTH,
                    first_button_shortcut,
                    true,
                ));
            }
            Some(second_button_text) => {
                contents.add_component(Button::new(
                    first_button_text.clone(),
                    Position::new(8, (i / Self::MAX_TEXT_LINE_LENGTH + 3) as i32),
                    MessageBox::BUTTON_WIDTH,
                    first_button_shortcut,
                    true,
                ));

                let shortcut = second_button_text.to_lowercase().chars().next();
                contents.add_component(Button::new(
                    second_button_text.clone(),
                    Position::new(31, (i / Self::MAX_TEXT_LINE_LENGTH + 3) as i32),
                    MessageBox::BUTTON_WIDTH,
                    shortcut,
                    true,
                ));
            }
        }

        Self {
            dialog: Dialog::new(game_ref, contents, None, size),
            first_button_text,
            second_button_text,
            first_result: Some(first_result),
            second_result,
        }
    }
}

impl GameState for MessageBox {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("Message box")
    }

    fn update(&mut self, game_ref: &mut GameRef) -> GameStateUpdateResult {
        let result = self
            .dialog
            .update(Position::default(), game_ref.input.clone());
        if result.kind.is_some() {
            let button = result.data.unwrap().downcast::<String>().unwrap();
            println!("BUTTON: {}", button);
            if button.as_str() == self.first_button_text {
                let result = (self.first_result.take().unwrap())();
                if let GameStateChange::None = &result {
                    panic!("MessageBox first_result returned None, which is not permitted!");
                }
                return Ok(result);
            } else if let Some(second_button_text) = &self.second_button_text {
                if button.as_str() == second_button_text {
                    let result = (self.second_result.take().unwrap())();
                    if let GameStateChange::None = &result {
                        panic!("MessageBox second_result returned None, which is not permitted!");
                    }
                    return Ok(result);
                }
            }
        }

        Ok(GameStateChange::None)
    }

    fn draw(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        let mut wrapped_root = ConsoleWrapper::new(game_ref.root);
        self.dialog.draw(Position::default(), &mut wrapped_root);

        Ok(())
    }
}
