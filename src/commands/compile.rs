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

use crate::{Context, Error};

/// Compile and run code
#[poise::command(prefix_command, slash_command)]
pub async fn compile(
    ctx: Context<'_>,
    #[description = "The language to compile the code in"] language: String,
    #[description = "The code to compile"]
    #[rest]
    code: String,
) -> Result<(), Error> {
    println!("Compiling code in {language} language");
    println!("Code: {code}");

    ctx.say(&format!("Compiling code in {language} language"))
        .await?;

    // TODO: compile and runthe code

    Ok(())
}
