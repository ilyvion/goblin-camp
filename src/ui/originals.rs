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

use crate::ui::drawable::Drawable;
use std::marker::PhantomData;

/// Implement for types that store `Drawable`s in a `Box<dyn Drawable>` fashion, where they are able
/// to hand back the original type based on a given token of type `T`.
pub trait HoldingOriginals<T> {
    fn get_component(&mut self, token: &T) -> &mut dyn Drawable;

    /// Returns the original `D` based on the data in the `O`.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee that the `Original<D>` came from the implementer of this
    /// trait, and that the implementer is in such a state as to access it through the `Original<D>`
    /// will correctly return a `D`.
    #[allow(unsafe_code)]
    unsafe fn get_original_component<D: Drawable, O: Original<D, T>>(&mut self, io: &O) -> &mut D
    where
        Self: std::marker::Sized,
    {
        io.original(self)
    }
}

pub trait Original<D: Drawable, T> {
    fn token(&self) -> &T;

    /// Returns the original `D` kept in the `H` from which this `Original<D>` originated.
    ///
    /// # Safety
    ///
    /// It is up to the caller to guarantee that this `Original<D>` came from the `H`, and that the
    /// `H` is in such a state as to access it through this `Original<D>` will correctly
    /// return a `D`.
    #[allow(unsafe_code)]
    unsafe fn original<'o, H: HoldingOriginals<T>>(&self, h: &'o mut H) -> &'o mut D {
        let drawable = h.get_component(self.token());
        &mut *(drawable as *mut dyn Drawable as *mut D)
    }
}

pub struct IndexedOriginal<D: Drawable> {
    index: usize,
    originally: PhantomData<D>,
}

impl<D: Drawable> IndexedOriginal<D> {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            originally: PhantomData,
        }
    }
}

impl<D: Drawable> Original<D, usize> for IndexedOriginal<D> {
    fn token(&self) -> &usize {
        &self.index
    }
}

#[macro_export]
macro_rules! indexed_original_impl {
    ($ty:ident, $name:ident) => {
        $crate::indexed_original_impl!($ty<>, $name);
    };
    ($ty:ident < $( $N:ident $(: $b0:ident $(+$b:ident)* )? ),* >, $name:ident) => {
        impl< $( $N $(: $b0 $(+$b)* )? ),* > $crate::ui::originals::IndexedOriginal<$ty< $( $N ),* >> {
            pub fn $name<'o, H: $crate::ui::originals::HoldingOriginals<usize>>(&mut self, h: &'o mut H) -> &'o mut $ty< $( $N ),* > {
                #[allow(unsafe_code)]
                unsafe { $crate::ui::originals::Original::original(self, h) }
            }
        }
    };
}
