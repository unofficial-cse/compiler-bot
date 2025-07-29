/*
 * Compiler-Bot: compiler bot for Unofficial.CSE
 * Copyright (C) 2025  Unofficial.CSE contributors
 *
 * Compiler-Bot is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Compiler-Bot is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with Compiler-Bot.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{collections::HashMap, sync::LazyLock};

use crate::config::SecurityConfig;

mod cpp;
mod python;
mod scala;

pub static LANGUAGES: LazyLock<HashMap<&'static str, Box<dyn Language + Send + Sync>>> =
    LazyLock::new(|| {
        let mut hashmap = HashMap::<&'static str, Box<dyn Language + Send + Sync>>::new();
        hashmap.insert(cpp::Cpp.name(), Box::new(cpp::Cpp));
        hashmap.insert(python::Python.name(), Box::new(python::Python));
        hashmap.insert(scala::Scala.name(), Box::new(scala::Scala));

        hashmap
    });

pub trait Language {
    fn command(&self) -> &'static str;

    fn docker_image(&self) -> &'static str;

    #[allow(dead_code)]
    fn file_extension(&self) -> &'static str;

    fn is_compiled(&self) -> bool;

    fn name(&self) -> &'static str;

    fn security_config(&self) -> SecurityConfig;
}
