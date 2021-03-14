pub mod meta;
pub mod config;
pub mod stats;
pub mod markov;

use self::{
    config::*,
    markov::*,
    meta::*,
    stats::*,
};
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

#[group]
#[commands(markov, search, graph)]
pub struct Markov;
