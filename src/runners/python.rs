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

pub struct Python;

impl Language for Python {
    fn command(&self) -> &'static str {
        "python3"
    }

    fn docker_image(&self) -> &'static str {
        "compiler-bot-python-rt:latest"
    }

    fn file_extension(&self) -> &'static str {
        "py"
    }

    fn is_compiled(&self) -> bool {
        false
    }

    fn name(&self) -> &'static str {
        "python"
    }
}
