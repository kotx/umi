pub mod meta;
pub mod config;
pub mod stats;
pub mod markov;
pub mod collect;

use self::{
    config::*,
    markov::*,
    meta::*,
    stats::*,
    collect::*,
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

#[group]
#[commands(collect)]
pub struct Data;
