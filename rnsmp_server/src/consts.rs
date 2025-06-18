use std::time::Duration;

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)] pub const BUILD_TYPE: &str = "debug";
#[cfg(not(debug_assertions))] pub const BUILD_TYPE: &str = "release";
pub const BUILD_ID: &str = env!("BUILD_ID");

pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const LICENSE_NAME: &str = "ANTI-CAPITALIST SOFTWARE LICENSE v 1.4";
pub const LICENSE_CONTENTS: &str = include_str!("../../LICENSE");
pub const CREDITS: &str = include_str!("../../credits.txt");
pub const MOTD: &str = "Have fun. ♥";

pub const DEFAULT_PORT: &i32 = &7273;
pub const DEFAULT_TICK_DURATION: &Duration = &Duration::from_millis(1000 / 64);

pub const LOG_HELP: &str = "Use \"help\".";
pub const LOG_INVALID_ARGS: &str = "Invalid arguments.";