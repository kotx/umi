pub mod meta;
pub mod config;
pub mod stats;

use meta::*;
use config::*;
use stats::*;
use serenity::framework::standard::macros::*;

#[group]
#[commands(ping, about)]
pub struct General;

#[group]
#[commands(opt)]
pub struct Config;

#[group]
#[commands(stats)]
pub struct Stats;
