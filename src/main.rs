#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod discord_bot;
mod web;

use discord_bot::*;

#[tokio::main]
async fn main() {
    web::route_network();
    bot::start().await;
}