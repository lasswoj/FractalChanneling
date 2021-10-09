/*
 *  Copyright (C) 1991, 1992, 1993 Free Software Foundation, Inc.
 * 
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 * 
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 * 
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 * 
 * Modified by the Fractal Team and others 2021.  See the Authors.md
 * file for a list of people on the Fractal Team.
 */
extern crate strum;



use strum_macros::{EnumString,EnumVariantNames};
use strum::VariantNames;
#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "PascalCase")]
pub enum Actions {
    Add,
    Remove,
    Select,
    Read,
    List,
    Put,
    Append,
    Wrong
}

pub const NAMES:&'static [&'static str] = Actions::VARIANTS;

pub trait ActionsTr: Sized {
    /// The associated error which can be returned from parsing.
    fn try_again(line: String) -> Self;
    fn get_action() -> Self; 
}


