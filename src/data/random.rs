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

use crate::coordinate::{Axis, Coordinate};
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};
use std::convert::TryInto;
use std::time::SystemTime;
use tcod::random::Algo;

/// Trait used to provide all the possible values of an `enum`, which is then used by the
/// [`auto_select`] method on the `Generator` to automatically provide a random value of said
/// enum to the user.
///
/// [`auto_select`]: trait.Generator.html#method.auto_select
pub trait Selection
where
    Self: Sized + Copy,
{
    /// Returns all the possible values of this type
    fn get_choices() -> &'static [Self];
}

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
    fn generate_integer(&mut self, start: i32, end: i32) -> i32;

    /// Generates a value between 0 and end, inclusive (\[0, end\]).
    ///
    /// # Arguments
    ///
    /// * `end` - The highest number to possibly generate.
    ///
    /// # Panics
    /// * If `end < 0`.
    fn generate_integer_up_to(&mut self, end: i32) -> i32 {
        self.generate_integer(0, end)
    }

    /// Generates a value between 0.0 and 1.0, inclusive (\[0.0, 1.0\]).
    fn generate_floating(&mut self) -> f64;

    /// Generates a value of `true` or `false`.
    fn generate_bool(&mut self) -> bool;

    /// Generates a value of `Positive` or `Negative`.
    fn generate_sign(&mut self) -> Sign {
        if self.generate_bool() {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }

    /// Generates a coordinate inside the rectangular area delineated by `origin` and `extent`,
    /// excluding the coordinates `extent` itself, i.e. [origin, extent).
    fn generate_coordinate_within_extent(
        &mut self,
        origin: Coordinate,
        extent: Coordinate,
    ) -> Coordinate {
        let mut res = origin;
        Axis::both().for_each(|a| res[a] += self.generate_integer_up_to(extent[a] - 1));

        res
    }

    /// Generates a coordinate inside the rectangular area delineated by `(0, 0)` and `extent`,
    /// excluding the coordinates of and `extent` itself, i.e. [(0, 0), extent).
    fn generate_coordinate_within_origin_extent(&mut self, extent: Coordinate) -> Coordinate {
        self.generate_coordinate_within_extent(Coordinate::ORIGIN, extent)
    }

    /// Generates a coordinate which is up to `distance` away per axis from `origin`.
    ///
    /// # Panics
    /// * If `distance < 0`.
    fn generate_coordinate_within_distance(
        &mut self,
        origin: Coordinate,
        distance: i32,
    ) -> Coordinate {
        let mut res = origin;
        Axis::both().for_each(|a| res[a] += self.generate_integer(-distance, distance));

        res
    }

    /// Generates a coordinate which is up to `distance` away per axis from `(0, 0)`.
    ///
    /// # Panics
    /// * If `distance < 0`.
    fn generate_coordinate_within_origin_distance(&mut self, distance: i32) -> Coordinate {
        self.generate_coordinate_within_distance(Coordinate::ORIGIN, distance)
    }

    /// Generates a coordinate inside a rectangle delineated by its `low` and `high` corners,
    /// both included.
    fn generate_coordinate_within_rectangle(
        &mut self,
        low: Coordinate,
        high: Coordinate,
    ) -> Coordinate {
        let mut res = Coordinate::default();
        Axis::both().for_each(|a| res[a] += self.generate_integer(low[a], high[a]));

        res
    }

    /// Returns a `Dice` based on this generator.
    ///
    /// # Arguments
    ///
    /// * `faces` - How many faces per die.
    /// * `rolls` - How many dice to roll.
    /// * `multiplier` - What to multiply the dice roll by.
    /// * `offset` - What to offset the dice roll by.
    fn get_dice(&mut self, faces: u32, rolls: u32, multiplier: f64, offset: f64) -> Dice<Self>
    where
        Self: Sized,
    {
        Dice::new(self, faces, rolls, multiplier, offset)
    }

    /// Returns a reference to a tcod [`Rng`], which should generally be seeded by the same seed
    /// as the generator itself, if possible, or if not, a seed that is deterministically derived
    /// from the generator's own seed.
    ///
    /// [`Rng`]: /tcod/random/struct.Rng.html
    fn get_tcod_rng(&self) -> &tcod::random::Rng;
}

impl<'a> dyn Generator + 'a {
    /// Given a slice of choices, this method selects one of them by random and returns it
    pub fn select<T: Copy>(&mut self, choices: &[T]) -> T {
        *self.select_by_ref(choices)
    }

    /// Given a slice of choices, this method selects one of them by random and returns it
    /// by reference
    pub fn select_by_ref<'c, T>(&mut self, choices: &'c [T]) -> &'c T {
        let end: i32 = (choices.len() - 1)
            .try_into()
            .expect("Number of choices too large");
        choices.get(self.generate_integer(0, end) as usize).unwrap()
    }

    /// To accommodate ease-of-use when it comes to picking an enum value by random, this method
    /// in conjuction with the `Selection` trait makes that super easy to accomplish. First,
    /// implement the `Selection` trait for your enum, which also has to implement `Copy`:
    /// ```
    /// use goblin_camp_revival::data::random::Selection;
    ///
    /// #[derive(Copy, Clone)]
    /// pub enum Foo {
    ///     Bar,
    ///     Baz,
    /// }
    /// const FOO_VALUES: [Foo; 2] = [Foo::Bar, Foo::Baz];
    /// impl Selection for Foo {
    ///     fn get_choices() -> &'static [Self] {
    ///         &FOO_VALUES
    ///     }
    /// }
    /// ```
    /// by doing this, you can now call `auto_select()` for any value that expects a `Foo`, and
    /// it'll give you a random enum value:
    /// ```
    /// # use goblin_camp_revival::data::random::Selection;
    /// #
    /// # #[derive(Copy, Clone)]
    /// # pub enum Foo {
    /// #    Bar,
    /// #    Baz,
    /// # }
    /// # const FOO_VALUES: [Foo; 2] = [Foo::Bar, Foo::Baz];
    /// # impl Selection for Foo {
    /// #    fn get_choices() -> &'static [Self] {
    /// #        &FOO_VALUES
    /// #    }
    /// # }
    /// #
    /// use goblin_camp_revival::data::random::{DefaultGenerator, Generator};
    ///
    /// let mut default_generator = DefaultGenerator::default();
    /// let generator: &mut dyn Generator = &mut default_generator;
    /// let foo: Foo = generator.auto_select();
    /// ```
    pub fn auto_select<S: Selection + 'static>(&mut self) -> S
    where
        S: Copy,
    {
        self.select(S::get_choices())
    }

    /// Generates a `u8` using the [`generate`] method.
    ///
    /// [`generate`]: #method.generate
    pub fn generate_u8(&mut self, start: u8, end: u8) -> u8 {
        self.generate_integer(i32::from(start), i32::from(end)) as u8
    }

    /// Generates a `u8` using the [`generate_up_to`] method.
    ///
    /// [`generate_up_to`]: #method.generate_up_to
    pub fn generate_up_to_u8(&mut self, end: u8) -> u8 {
        self.generate_integer_up_to(i32::from(end)) as u8
    }
}

/// A random number generator utility.
pub struct DefaultGenerator<R: RngCore> {
    rng: R,
    seed: u64,
    tcod_rng: tcod::random::Rng,
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
            //
            tcod_rng: tcod::random::Rng::new_with_seed(
                Algo::CMWC,
                (seed % (u64::from(u32::max_value()) + 1)) as u32,
            ),
        }
    }

    /// Returns the seed used in this generator.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Reseeds the generator.
    pub fn reseed(&mut self, seed: u64) {
        self.rng = S::seed_from_u64(seed);
        self.seed = seed;
    }

    /// Reseeds the generator with the default seed.
    pub fn reseed_with_default(&mut self) {
        self.reseed(Self::default_seed())
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
    fn generate_integer(&mut self, start: i32, end: i32) -> i32 {
        self.rng.gen_range(start, end + 1)
    }

    fn generate_floating(&mut self) -> f64 {
        self.rng.gen()
    }

    fn generate_bool(&mut self) -> bool {
        self.rng.gen()
    }

    fn get_tcod_rng(&self) -> &tcod::random::Rng {
        &self.tcod_rng
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
        Self {
            generator,
            faces: faces.max(1),
            rolls: rolls.max(1),
            multiplier,
            offset,
        }
    }

    /// Rolls the dice and returns the outcome.
    ///
    /// # Example
    /// ```
    ///# use crate::goblin_camp_revival::data::random::{DefaultGenerator, Dice};
    ///# let mut generator = DefaultGenerator::default();
    /// let mut dice = Dice::new(&mut generator, 20, 2, 2., 10.);
    /// println!("{}", dice.roll());
    /// ```
    pub fn roll(&mut self) -> u32 {
        let result = (0..self.rolls).fold(0, |r, _| {
            r + self.generator.generate_integer(1, self.faces.max(1) as i32)
        });

        (f64::from(result) * self.multiplier + self.offset).round() as u32
    }

    /// The maximum possible roll outcome for these dice.
    ///
    /// # Example
    /// ```
    ///# use crate::goblin_camp_revival::data::random::{DefaultGenerator, Dice};
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
    ///# use crate::goblin_camp_revival::data::random::{DefaultGenerator, Dice};
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
pub struct StaticGenerator(tcod::random::Rng);

impl Default for StaticGenerator {
    /// Creates a new static generator.
    fn default() -> Self {
        Self(tcod::random::Rng::new_with_seed(Algo::CMWC, 0))
    }
}

impl Generator for StaticGenerator {
    fn generate_integer(&mut self, start: i32, _: i32) -> i32 {
        start
    }

    fn generate_integer_up_to(&mut self, end: i32) -> i32 {
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

    fn generate_coordinate_within_extent(
        &mut self,
        origin: Coordinate,
        extent: Coordinate,
    ) -> Coordinate {
        origin + extent - 1
    }

    fn generate_coordinate_within_origin_extent(&mut self, extent: Coordinate) -> Coordinate {
        extent
    }

    fn generate_coordinate_within_distance(
        &mut self,
        origin: Coordinate,
        distance: i32,
    ) -> Coordinate {
        origin + distance
    }

    fn generate_coordinate_within_origin_distance(&mut self, distance: i32) -> Coordinate {
        Coordinate::ORIGIN + distance
    }

    fn generate_coordinate_within_rectangle(
        &mut self,
        low: Coordinate,
        high: Coordinate,
    ) -> Coordinate {
        low + high
    }

    fn get_tcod_rng(&self) -> &tcod::random::Rng {
        &self.0
    }
}
