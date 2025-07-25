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
use poise::CreateReply;
use poise::serenity_prelude::CreateEmbed;

/// Show this help menu
#[poise::command(prefix_command, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
         ctx,
         command.as_deref(),
         poise::builtins::HelpConfiguration {
             extra_text_at_bottom: "Compiler-Bot is a bot that compiles and runs code for you. It supports multiple languages.",
             ..Default::default()
         },
     )
     .await?;
    Ok(())
}

/// List supported languages for compilation
#[poise::command(prefix_command, slash_command)]
pub async fn languages(ctx: Context<'_>) -> Result<(), Error> {
    // TODO: list supported languages from runtime config
    let supported_languages = ["c", "cpp", "scala", "python", "javascript", "typescript"];

    let language_list = supported_languages
        .iter()
        .map(|lang| format!("`{lang}`"))
        .collect::<Vec<_>>()
        .join(", ");

    let embed = CreateEmbed::new()
        .title("Supported Programming Languages")
        .description(language_list);

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
