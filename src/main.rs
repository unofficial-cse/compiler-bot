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

use std::env;

use dotenvy::dotenv;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use serenity::client::ClientBuilder;

use prelude::*;

mod prelude;
mod utils;

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    
    tracing::subscriber::set_global_default(utils::subscriber()).unwrap();

    let Ok(token) = env::var("BOT_TOKEN") else {
        tracing::error!("BOT_TOKEN environment variable is not set");
        return;
    };

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![],
            on_error: |_| {
                Box::pin(async move {
                    // TODO: add error handler
                })
            },
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".into()), // TODO: bikeshed on prefix
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(move |context, _, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(context, &framework.options().commands).await
            })
        })
        .build();

    let intents = GatewayIntents::all(); // TODO: detemrine actually required intents
    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}
