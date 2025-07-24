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

use tracing::{Subscriber, level_filters::LevelFilter};
use tracing_subscriber::{
    Registry,
    filter::Targets,
    fmt::{Layer, time::OffsetTime},
    layer::SubscriberExt,
};

pub fn subscriber() -> impl Subscriber {
    let fmt_layer = Layer::default()
        .pretty()
        .with_timer(OffsetTime::local_rfc_3339().unwrap())
        .with_target(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true);
    let targets_layer = Targets::new().with_default(LevelFilter::TRACE);

    Registry::default().with(fmt_layer).with(targets_layer)
}
