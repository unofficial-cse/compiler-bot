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

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotConfig {
    pub security: SecurityConfig,
    pub output: OutputConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub cpu_limit: String,
    pub memory_limit: String,
    pub pids_limit: u32,
    pub file_descriptor_limit: String,
    pub disable_network: bool,
    pub timeout_duration: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    pub max_output_length: usize,
    pub truncate_suffix: String,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            security: SecurityConfig {
                cpu_limit: "0.25".into(),
                memory_limit: "128m".into(),
                pids_limit: 100,
                file_descriptor_limit: "64:64".into(),
                disable_network: true,
                timeout_duration: 60,
            },
            output: OutputConfig {
                max_output_length: 1000,
                truncate_suffix: "...\n(truncated)".into(),
            },
        }
    }
}
