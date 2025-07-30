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

use super::Language;
use crate::config::SecurityConfig;

pub struct Cpp;

impl Language for Cpp {
    fn command(&self) -> &'static str {
        "bash -c 'cat > /output.cpp && g++ -std=c++17 -Wall -Wextra -o /output /output.cpp && /output'"
    }

    fn docker_image(&self) -> &'static str {
        "compiler-bot-cpp-rt:latest"
    }

    fn file_extension(&self) -> &'static str {
        "cpp"
    }

    fn is_compiled(&self) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "cpp"
    }

    fn security_config(&self) -> SecurityConfig {
        SecurityConfig::default()
    }
}
