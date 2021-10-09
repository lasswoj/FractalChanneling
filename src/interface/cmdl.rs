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
use super::base::{Actions, ActionsTr, NAMES};
use std::io::{stdin,stdout,Write};
use std::str::FromStr;

pub enum CMDActions{
    Actions(Actions)
}

impl FromStr for CMDActions {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            _ if NAMES.contains(&input) => Ok(Self::Actions(Actions::from_str(input).unwrap())),
            // "Add"  => Ok(Self::Actions(Actions::Add)),
            // "Append"  => Ok(Self::Actions(Actions::Append)),
            // "List"  => Ok(Self::Actions(Actions::List)),
            // "Put"  => Ok(Self::Actions(Actions::Put)),
            // "Read"  => Ok(Self::Actions(Actions::Read)),
            // "Remove"  => Ok(Self::Actions(Actions::Remove)),
            // "Select"  => Ok(Self::Actions(Actions::Select)),
            _      => Err(()),
        }
    }
}


impl ActionsTr for CMDActions{
    fn try_again(mut line: String) -> CMDActions{
        let mut action = CMDActions::Actions(Actions::Wrong);
        while matches!(action, CMDActions::Actions(Actions::Wrong)){
            print!("unsupported action :{}\n", line);
            line = String::new();
            let _=stdout().flush();
            let len = stdin().read_line(&mut line).expect("Did not enter a correct string");
            let v = line[0..len-1].split(" ").collect::<Vec<&str>>(); 
            action = match Self::from_str(v[0]){
                    Ok(ac) => ac,
                    Err(_) => CMDActions::Actions(Actions::Wrong)
            };
        }
        return action;
    }
    fn get_action() -> CMDActions{
            print!("provide action\n");
            let mut line = String::new();
            let _=stdout().flush();
            let len = stdin().read_line(&mut line).expect("Did not enter a correct string");
            let v = line[0..len-1].split(" ").collect::<Vec<&str>>(); 
            let mut action = match Self::from_str(v[0]){
                Ok(ac) => ac,
                Err(_) => Self::try_again(line)
            };
            return  action;
        }
    }