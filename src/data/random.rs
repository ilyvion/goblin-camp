/*
    Copyright 2010-2011 Ilkka Halila
    Copyright 2019 Alexander Krivács Schrøder

    This file is part of Goblin Camp.

    Goblin Camp is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Goblin Camp is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Goblin Camp.  If not, see <https://www.gnu.org/licenses/>.
*/

use rand::{SeedableRng, RngCore, Rng};
use rand::rngs::StdRng;
use std::time::SystemTime;
use crate::coordinate::{Coordinate, Axis};

/// Represents the sign of a numeric value, either `Positive` or `Negative`.
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Sign {
    /// Represents a positive sign for a numeric value.
    Positive,
    /// Represents a negative sign for a numeric value.
    Negative,
}

/// Represents a group of useful random value generator methods.
pub trait Generator {
    /// Generates a value between start and end, inclusive (\[start, end\]).
    ///
    /// # Arguments
    ///
    /// * `start` - The lowest number to possibly generate.
    /// * `end` - The highest number to possibly generate.
    ///
    /// # Panics
    /// * If `start > end`.
    fn generate(&mut self, start: i32, end: i32) -> i32;

    /// Generates a value between 0 and end, inclusive (\[0, end\]).
    ///
    /// # Arguments
    ///
    /// * `end` - The highest number to possibly generate.
    ///
    /// # Panics
    /// * If `end < 0`.
    fn generate_up_to(&mut self, end: i32) -> i32;

    /// Generates a value between 0.0 and 1.0, inclusive (\[0.0, 1.0\]).
    fn generate_floating(&mut self) -> f64;

    /// Generates a value of `true` or `false`.
    fn generate_bool(&mut self) -> bool;

    /// Generates a value of `Positive` or `Negative`.
    fn generate_sign(&mut self) -> Sign;

    /// Generates a coordinate inside the rectangular area delineated by `origin` and `extent`,
    /// excluding the coordinates `extent` itself, i.e. [origin, extent).
    fn generate_coordinate_within_extent(&mut self, origin: Coordinate, extent: Coordinate) -> Coordinate;

    /// Generates a coordinate inside the rectangular area delineated by `(0, 0)` and `extent`,
    /// excluding the coordinates of and `extent` itself, i.e. [(0, 0), extent).
    fn generate_coordinate_within_origin_extent(&mut self, extent: Coordinate) -> Coordinate;

    /// Generates a coordinate which is up to `distance` away per axis from `origin`.
    ///
    /// # Panics
    /// * If `distance < 0`.
    fn generate_coordinate_within_distance(&mut self, origin: Coordinate, distance: i32) -> Coordinate;

    /// Generates a coordinate which is up to `distance` away per axis from `(0, 0)`.
    ///
    /// # Panics
    /// * If `distance < 0`.
    fn generate_coordinate_within_origin_distance(&mut self, distance: i32) -> Coordinate;

    /// Generates a coordinate inside a rectangle delineated by its `low` and `high` corners,
    /// both included.
    fn generate_coordinate_within_rectangle(&mut self, low: Coordinate, high: Coordinate) -> Coordinate;

    /// Returns a `Dice` based on this generator.
    ///
    /// # Arguments
    ///
    /// * `faces` - How many faces per die.
    /// * `rolls` - How many dice to roll.
    /// * `multiplier` - What to multiply the dice roll by.
    /// * `offset` - What to offset the dice roll by.
    fn get_dice(&mut self, faces: u32, rolls: u32, multiplier: f64, offset: f64) -> Dice<Self> where Self: Sized {
        Dice::new(self, faces, rolls, multiplier, offset)
    }
}

/// A random number generator utility.
pub struct DefaultGenerator<R: RngCore> {
    rng: R,
    seed: u64,
}

impl<S: RngCore + SeedableRng> DefaultGenerator<S> {
    /// Returns a unix timestamp, which is used as the default seed when none is otherwise provided.
    fn default_seed() -> u64 {
        let now = SystemTime::now();
        let duration_since = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        duration_since.as_secs()
    }

    /// Creates a new generator with a custom RNG and the default seed.
    pub fn new() -> Self {
        Self::new_with_seed(Self::default_seed())
    }

    /// Creates a generator with a custom RNG and a custom seed.
    pub fn new_with_seed(seed: u64) -> Self {
        Self {
            rng: S::seed_from_u64(seed),
            seed,
        }
    }

    /// Returns the seed used in this generator.
    pub fn seed(&self) -> u64 { self.seed }

    /// Reseeds the generator.
    pub fn reseed(&mut self, seed: u64) {
        self.rng = S::seed_from_u64(seed);
        self.seed = seed;
    }
}

impl DefaultGenerator<StdRng> {
    /// Creates a generator with the default RNG and a custom seed.
    pub fn default_with_seed(seed: u64) -> Self {
        Self::new_with_seed(seed)
    }
}

impl Default for DefaultGenerator<StdRng> {
    /// Creates a generator with the default RNG and seed.
    fn default() -> Self {
        Self::new()
    }
}

impl<R: RngCore> Generator for DefaultGenerator<R> {
    fn generate(&mut self, start: i32, end: i32) -> i32 {
        self.rng.gen_range(start, end + 1)
    }

    fn generate_up_to(&mut self, end: i32) -> i32 {
        self.generate(0, end)
    }

    fn generate_floating(&mut self) -> f64 {
        self.rng.gen()
    }

    fn generate_bool(&mut self) -> bool {
        self.rng.gen()
    }

    fn generate_sign(&mut self) -> Sign {
        if self.generate_bool() {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }

    fn generate_coordinate_within_extent(&mut self, origin: Coordinate, extent: Coordinate) -> Coordinate {
        let mut res = origin;
        Axis::both().for_each(|a| res[a] += self.generate_up_to(extent[a] - 1));

        res
    }

    fn generate_coordinate_within_origin_extent(&mut self, extent: Coordinate) -> Coordinate {
        self.generate_coordinate_within_extent(Coordinate::ORIGIN, extent)
    }

    fn generate_coordinate_within_distance(&mut self, origin: Coordinate, distance: i32) -> Coordinate {
        let mut res = origin;
        Axis::both().for_each(|a| res[a] += self.generate(-distance, distance));

        res
    }

    fn generate_coordinate_within_origin_distance(&mut self, distance: i32) -> Coordinate {
        self.generate_coordinate_within_distance(Coordinate::ORIGIN, distance)
    }

    fn generate_coordinate_within_rectangle(&mut self, low: Coordinate, high: Coordinate) -> Coordinate {
        let mut res = Coordinate::default();
        Axis::both().for_each(|a| res[a] += self.generate(low[a], high[a]));

        res
    }
}

/// Represents a collection of dice plus rules for how to use those dice to generate a number.
pub struct Dice<'g, G: Generator> {
    generator: &'g mut G,
    faces: u32,
    rolls: u32,
    multiplier: f64,
    offset: f64,
}

impl<'g, G: Generator> Dice<'g, G> {
    /// Returns a collection of dice with the given calculation rules.
    ///
    /// # Arguments
    ///
    /// * `generator` - The [`Generator`] used to pick which value each die roll lands on.
    /// * `faces` - How many faces per die (0 gets interpreted as 1).
    /// * `rolls` - How many dice to roll (0 gets interpreted as 1).
    /// * `multiplier` - What to multiply the dice roll by.
    /// * `offset` - What to offset the dice roll by.
    ///
    /// [`Generator`]: trait.Generator.html
    pub fn new(generator: &'g mut G, faces: u32, rolls: u32, multiplier: f64, offset: f64) -> Self {
        Self { generator, faces: faces.max(1), rolls: rolls.max(1), multiplier, offset }
    }

    /// Rolls the dice and returns the outcome.
    ///
    /// # Example
    /// ```
    ///# use crate::goblin_camp::data::random::{DefaultGenerator, Dice};
    ///# let mut generator = DefaultGenerator::default();
    /// let mut dice = Dice::new(&mut generator, 20, 2, 2., 10.);
    /// println!("{}", dice.roll());
    /// ```
    pub fn roll(&mut self) -> u32 {
        let result =
            (0..self.rolls).fold(0, |r, _|
                r + self.generator.generate(1, self.faces.max(1) as i32));

        (f64::from(result) * self.multiplier + self.offset).round() as u32
    }

    /// The maximum possible roll outcome for these dice.
    ///
    /// # Example
    /// ```
    ///# use crate::goblin_camp::data::random::{DefaultGenerator, Dice};
    ///# let mut generator = DefaultGenerator::default();
    /// let dice = Dice::new(&mut generator, 20, 2, 2., 10.);
    /// assert_eq!(90, dice.max());
    /// ```
    pub fn max(&self) -> u32 {
        (f64::from(self.faces * self.rolls) * self.multiplier + self.offset).round() as u32
    }

    /// The minimum possible roll outcome for these dice.
    ///
    /// # Example
    /// ```
    ///# use crate::goblin_camp::data::random::{DefaultGenerator, Dice};
    ///# let mut generator = DefaultGenerator::default();
    /// let dice = Dice::new(&mut generator, 20, 2, 2., 10.);
    /// assert_eq!(14, dice.min());
    /// ```
    pub fn min(&self) -> u32 {
        (f64::from(self.rolls) * self.multiplier + self.offset).round() as u32
    }
}

/// An implementation of a [`Generator`] that always returns the same value. Mostly useful for testing
/// situations that uses a [`Generator`].
///
/// [`Generator`]: trait.Generator.html
pub struct StaticGenerator(());

impl Default for StaticGenerator {
    /// Creates a new static generator.
    fn default() -> Self { Self(()) }
}

impl Generator for StaticGenerator {
    fn generate(&mut self, start: i32, _: i32) -> i32 {
        start
    }

    fn generate_up_to(&mut self, end: i32) -> i32 {
        end
    }

    fn generate_floating(&mut self) -> f64 {
        0.5
    }

    fn generate_bool(&mut self) -> bool {
        true
    }

    fn generate_sign(&mut self) -> Sign {
        Sign::Positive
    }

    fn generate_coordinate_within_extent(&mut self, origin: Coordinate, extent: Coordinate) -> Coordinate {
        origin + extent - 1
    }

    fn generate_coordinate_within_origin_extent(&mut self, extent: Coordinate) -> Coordinate {
        extent
    }

    fn generate_coordinate_within_distance(&mut self, origin: Coordinate, distance: i32) -> Coordinate {
        origin + distance
    }

    fn generate_coordinate_within_origin_distance(&mut self, distance: i32) -> Coordinate {
        Coordinate::ORIGIN + distance
    }

    fn generate_coordinate_within_rectangle(&mut self, low: Coordinate, high: Coordinate) -> Coordinate {
        low + high
    }
}
