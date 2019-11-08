/*
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

use crate::data::base::{Position, Rectangle};
use tcod::{BackgroundFlag, Color, Console, TextAlignment};

/// Re-implements the `Console` trait from `tcod`, but makes every method object-safe so they can be
/// used as a `dyn` trait.
pub trait SafeConsole {
    /// Returns the default text alignment for the `Console` instance. For all the possible
    /// text alignment options, see the documentation for
    /// [TextAlignment](./enum.TextAlignment.html).
    fn get_alignment(&self) -> TextAlignment;

    /// Sets the default text alignment for the console. For all the possible
    /// text alignment options, see the documentation for
    /// [TextAlignment](./enum.TextAlignment.html).
    fn set_alignment(&mut self, alignment: TextAlignment);

    /// Sets a key color that will be ignored when [blitting](./fn.blit.html) the contents
    /// of this console onto an other (essentially a transparent background color).
    fn set_key_color(&mut self, color: Color);

    /// Returns the width of the console in characters.
    fn width(&self) -> i32;

    /// Returns the height of the console in characters.
    fn height(&self) -> i32;

    /// Return the console's default background color. This is used in
    /// several other methods, like: `clear`, `put_char`, etc.
    fn get_default_background(&mut self) -> Color;

    /// Sets the console's default background color. This is used in several other methods,
    /// like: `clear`, `put_char`, etc.
    fn set_default_background(&mut self, color: Color);

    /// Sets the console's default foreground color. This is used in several printing functions.
    fn set_default_foreground(&mut self, color: Color);

    /// Returns the background color of the cell at the specified coordinates.
    fn get_char_background(&self, position: Position) -> Color;

    /// Returns the foreground color of the cell at the specified coordinates.
    fn get_char_foreground(&self, position: Position) -> Color;

    /// Returns the console's current background flag. For a detailed explanation
    /// of the possible values, see [BackgroundFlag](./enum.BackgroundFlag.html).
    fn get_background_flag(&self) -> BackgroundFlag;

    /// Sets the console's current background flag. For a detailed explanation
    /// of the possible values, see [BackgroundFlag](./enum.BackgroundFlag.html).
    fn set_background_flag(&mut self, background_flag: BackgroundFlag);

    /// Returns the ASCII value of the cell located at `position.x, position.y`
    fn get_char(&self, position: Position) -> char;

    /// Modifies the ASCII value of the cell located at `position.x, position.y`.
    fn set_char(&mut self, position: Position, c: char);

    /// Changes the background color of the specified cell
    fn set_char_background(
        &mut self,
        position: Position,
        color: Color,
        background_flag: BackgroundFlag,
    );

    /// Changes the foreground color of the specified cell
    fn set_char_foreground(&mut self, position: Position, color: Color);

    /// This function modifies every property of the given cell:
    ///
    /// 1. Updates its background color according to the console's default and `background_flag`,
    /// see [BackgroundFlag](./enum.BackgroundFlag.html).
    /// 2. Updates its foreground color based on the default color set in the console
    /// 3. Sets its ASCII value to `glyph`
    fn put_char(&mut self, position: Position, glyph: char, background_flag: BackgroundFlag);

    /// Updates every propert of the given cell using explicit colors for the
    /// background and foreground.
    fn put_char_ex(
        &mut self,
        position: Position,
        glyph: char,
        foreground: Color,
        background: Color,
    );

    /// Clears the console with its default background color
    fn clear(&mut self);

    /// Prints the text at the specified location. The position of the `x` and `y`
    /// coordinates depend on the [TextAlignment](./enum.TextAlignment.html) set in the console:
    ///
    /// * `TextAlignment::Left`: leftmost character of the string
    /// * `TextAlignment::Center`: center character of the sting
    /// * `TextAlignment::Right`: rightmost character of the string
    fn print(&mut self, position: Position, text: &str);

    /// Prints the text at the specified location in a rectangular area with
    /// the dimensions: (width; height). If the text is longer than the width the
    /// newlines will be inserted.
    fn print_rect(&mut self, rectangle: Rectangle, text: &str);

    /// Prints the text at the specified location with an explicit
    /// [BackgroundFlag](./enum.BackgroundFlag.html) and
    /// [TextAlignment](./enum.TextAlignment.html).
    fn print_ex(
        &mut self,
        position: Position,
        background_flag: BackgroundFlag,
        alignment: TextAlignment,
        text: &str,
    );

    /// Combines the functions of `print_ex` and `print_rect`
    fn print_rect_ex(
        &mut self,
        rectangle: Rectangle,
        background_flag: BackgroundFlag,
        alignment: TextAlignment,
        text: &str,
    );

    /// Compute the height of a wrapped text printed using `print_rect` or `print_rect_ex`.
    fn get_height_rect(&self, rectangle: Rectangle, text: &str) -> i32;

    /// Fill a rectangle with the default background colour.
    ///
    /// If `clear` is true, set each cell's character to space (ASCII 32).
    fn rect(&mut self, rectangle: Rectangle, clear: bool, background_flag: BackgroundFlag);

    /// Draw a horizontal line.
    ///
    /// Uses `tcod::chars::HLINE` (ASCII 196) as the line character and
    /// console's default background and foreground colours.
    fn horizontal_line(&mut self, position: Position, length: i32, background_flag: BackgroundFlag);

    /// Draw a vertical line.
    ///
    /// Uses `tcod::chars::VLINE` (ASCII 179) as the line character and
    /// console's default background and foreground colours.
    fn vertical_line(&mut self, position: Position, length: i32, background_flag: BackgroundFlag);

    /// Draw a window frame with an optional title.
    ///
    /// Draws a rectangle (using the rect method) using the suplied background
    /// flag, then draws a rectangle with the console's default foreground
    /// colour.
    ///
    /// If the `title` is specified, it will be printed on top of the rectangle
    /// using inverted colours.
    fn print_frame(
        &mut self,
        rectangle: Rectangle,
        clear: bool,
        background_flag: BackgroundFlag,
        title: Option<&str>,
    );
}

impl<C: Console> SafeConsole for C {
    fn get_alignment(&self) -> TextAlignment {
        Console::get_alignment(self)
    }

    fn set_alignment(&mut self, alignment: TextAlignment) {
        Console::set_alignment(self, alignment)
    }

    fn set_key_color(&mut self, color: Color) {
        Console::set_key_color(self, color);
    }

    fn width(&self) -> i32 {
        Console::width(self)
    }

    fn height(&self) -> i32 {
        Console::height(self)
    }

    fn get_default_background(&mut self) -> Color {
        Console::get_default_background(self)
    }

    fn set_default_background(&mut self, color: Color) {
        Console::set_default_background(self, color)
    }

    fn set_default_foreground(&mut self, color: Color) {
        Console::set_default_foreground(self, color)
    }

    fn get_char_background(&self, position: Position) -> Color {
        Console::get_char_background(self, position.x, position.y)
    }

    fn get_char_foreground(&self, position: Position) -> Color {
        Console::get_char_foreground(self, position.x, position.y)
    }

    fn get_background_flag(&self) -> BackgroundFlag {
        Console::get_background_flag(self)
    }

    fn set_background_flag(&mut self, background_flag: BackgroundFlag) {
        Console::set_background_flag(self, background_flag)
    }

    fn get_char(&self, position: Position) -> char {
        Console::get_char(self, position.x, position.y)
    }

    fn set_char(&mut self, position: Position, c: char) {
        Console::set_char(self, position.x, position.y, c)
    }

    fn set_char_background(
        &mut self,
        position: Position,
        color: Color,
        background_flag: BackgroundFlag,
    ) {
        Console::set_char_background(self, position.x, position.y, color, background_flag)
    }

    fn set_char_foreground(&mut self, position: Position, color: Color) {
        Console::set_char_foreground(self, position.x, position.y, color)
    }

    fn put_char(&mut self, position: Position, glyph: char, background_flag: BackgroundFlag) {
        Console::put_char(self, position.x, position.y, glyph, background_flag)
    }

    fn put_char_ex(
        &mut self,
        position: Position,
        glyph: char,
        foreground: Color,
        background: Color,
    ) {
        Console::put_char_ex(self, position.x, position.y, glyph, foreground, background)
    }

    fn clear(&mut self) {
        Console::clear(self)
    }

    fn print(&mut self, position: Position, text: &str) {
        Console::print(self, position.x, position.y, text)
    }

    fn print_rect(&mut self, rectangle: Rectangle, text: &str) {
        Console::print_rect(
            self,
            rectangle.position.x,
            rectangle.position.y,
            rectangle.size.width,
            rectangle.size.height,
            text,
        )
    }

    fn print_ex(
        &mut self,
        position: Position,
        background_flag: BackgroundFlag,
        alignment: TextAlignment,
        text: &str,
    ) {
        Console::print_ex(
            self,
            position.x,
            position.y,
            background_flag,
            alignment,
            text,
        )
    }

    fn print_rect_ex(
        &mut self,
        rectangle: Rectangle,
        background_flag: BackgroundFlag,
        alignment: TextAlignment,
        text: &str,
    ) {
        Console::print_rect_ex(
            self,
            rectangle.position.x,
            rectangle.position.y,
            rectangle.size.width,
            rectangle.size.height,
            background_flag,
            alignment,
            text,
        )
    }

    fn get_height_rect(&self, rectangle: Rectangle, text: &str) -> i32 {
        Console::get_height_rect(
            self,
            rectangle.position.x,
            rectangle.position.y,
            rectangle.size.width,
            rectangle.size.height,
            text,
        )
    }

    fn rect(&mut self, rectangle: Rectangle, clear: bool, background_flag: BackgroundFlag) {
        Console::rect(
            self,
            rectangle.position.x,
            rectangle.position.y,
            rectangle.size.width,
            rectangle.size.height,
            clear,
            background_flag,
        )
    }

    fn horizontal_line(
        &mut self,
        position: Position,
        length: i32,
        background_flag: BackgroundFlag,
    ) {
        Console::horizontal_line(self, position.x, position.y, length, background_flag)
    }

    fn vertical_line(&mut self, position: Position, length: i32, background_flag: BackgroundFlag) {
        Console::vertical_line(self, position.x, position.y, length, background_flag)
    }

    fn print_frame(
        &mut self,
        rectangle: Rectangle,
        clear: bool,
        background_flag: BackgroundFlag,
        title: Option<&str>,
    ) {
        Console::print_frame(
            self,
            rectangle.position.x,
            rectangle.position.y,
            rectangle.size.width,
            rectangle.size.height,
            clear,
            background_flag,
            title,
        )
    }
}

impl<'a> dyn SafeConsole + 'a {
    /// Because the `blit` function requires a `C: Console` as its destination, we cannot directly
    /// blit into a `dyn SafeConsole`, but instead have to go via the concrete type. This sort of
    /// breaks the abstraction, unfortunately, but what can you do?
    ///
    /// Not that it would be any better if it accepted `dyn Console`, because Rust doesn't have any
    /// way of going from `dyn A` to `dyn B` except through the concrete `C` that implements both of
    /// them, so we'd still have to break the abstraction.
    fn get_console<C: Console + SafeConsole>(&mut self) -> &mut C {
        unsafe { &mut *(self as *mut dyn SafeConsole as *mut C) }
    }

    pub fn blit<C: Console + SafeConsole, S: Console>(
        &mut self,
        source: &S,
        source_rect: Rectangle,
        dest_pos: Position,
        foreground_alpha: f32,
        background_alpha: f32,
    ) {
        tcod::console::blit(
            source,
            source_rect.position.into(),
            source_rect.size.into(),
            self.get_console::<C>(),
            dest_pos.into(),
            foreground_alpha,
            background_alpha,
        );
    }
}
