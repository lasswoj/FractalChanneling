/*
 * Copyright (c) 2021 Authors <https://github.com/lasswoj/FractalChanneling/blob/main/Authors.md>
 * 
 * Created Date: Tuesday, October 5th 2021, 10:35:54 pm
 * Author: Authors
 * 
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
use serde::Deserialize;
use std::fs;
use std::str::FromStr;
use toml::de::Error;

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub password: String,
}

pub type ParseError = Error;
// pub type ParseError = Infallible;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub profile: Option<Profile>,
}

impl FromStr for Config {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Self, ParseError> {
        return toml::from_str(input);
    }

}


impl Config{
    pub fn from_file(filename: &str)-> Result<Self, ParseError>{
        let data = fs::read_to_string(filename).expect("Unable to read file");
        let ret = Config::from_str(&*data);
        return ret;
    }
}

