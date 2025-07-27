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

use poise::CreateReply;

use crate::{
    CompilerBotContext, CompilerBotError, docker_executor::DockerExecutor, prelude::*,
    utils::extract_code_block,
};

/// Compile and run code
#[poise::command(prefix_command)]
pub async fn compile(
    ctx: CompilerBotContext<'_>,
    #[description = "The language to compile the code in"] language: String,
    #[description = "The code to compile"]
    #[rest]
    code: String,
) -> Result<(), CompilerBotError> {
    let code_block = extract_code_block(&code);
    if code_block.is_none() {
        ctx.say("No code block found").await?;
        return Ok(());
    }

    let code_to_execute = code_block.unwrap();
    let language_lower = language.to_lowercase();

    // Send initial response
    let initial_embed = CreateEmbed::new()
        .title(format!("🔄 Executing {language} code"))
        .description(format!("```{language}\n{code_to_execute}\n```"))
        .color(0xFFFF00); // Yellow for "running"

    let reply = ctx
        .send(CreateReply::default().embed(initial_embed))
        .await?;

    // Execute the code
    let execution_result = DockerExecutor::new()
        .execute(&language_lower, &code_to_execute)
        .await;

    // Prepare the result embed
    let result_embed = match execution_result {
        Ok(result) => {
            let status_emoji = if result.exit_code == Some(0) && !result.timed_out {
                "✅"
            } else if result.timed_out {
                "⏰"
            } else {
                "❌"
            };

            let color = if result.exit_code == Some(0) && !result.timed_out {
                0x00FF00 // Green for success
            } else if result.timed_out {
                0xFF8800 // Orange for timeout
            } else {
                0xFF0000 // Red for error
            };

            let mut embed = CreateEmbed::new()
                .title(format!("{status_emoji} Execution result"))
                .color(color);

            // Add stdout if present
            if !result.stdout.is_empty() {
                let max_len = DockerExecutor::new().config.output.max_output_length;
                let stdout_content = if result.stdout.len() > max_len {
                    format!(
                        "{}{}",
                        &result.stdout[..max_len],
                        DockerExecutor::new().config.output.truncate_suffix
                    )
                } else {
                    result.stdout.clone()
                };
                embed = embed.field("Output", format!("```\n{stdout_content}\n```"), false);
            }

            // Add stderr if present
            if !result.stderr.is_empty() {
                let max_len = DockerExecutor::new().config.output.max_output_length;
                let stderr_content = if result.stderr.len() > max_len {
                    format!(
                        "{}{}",
                        &result.stderr[..max_len],
                        DockerExecutor::new().config.output.truncate_suffix
                    )
                } else {
                    result.stderr.clone()
                };
                embed = embed.field("Error", format!("```\n{stderr_content}\n```"), false);
            }

            // Add execution info
            let mut execution_info = String::new();
            if let Some(code) = result.exit_code {
                execution_info.push_str(&format!("Exit code: {code}\n"));
            }
            if result.timed_out {
                execution_info.push_str("⚠️ Execution timed out\n");
            }
            if !execution_info.is_empty() && result.exit_code != Some(0) {
                embed = embed.field("Execution Info", execution_info, true);
            }

            embed
        }
        Err(error) => {
            CreateEmbed::new()
                .title("❌ Execution failed")
                .description(format!("Failed to execute {language} code"))
                .field("Error", format!("```\n{error}\n```"), false)
                .field(
                    "Source Code",
                    format!("```{language}\n{code_to_execute}\n```"),
                    false,
                )
                .color(0xFF0000) // Red for error
        }
    };

    // Update the message with the result
    reply
        .edit(ctx, CreateReply::default().embed(result_embed))
        .await?;

    Ok(())
}
