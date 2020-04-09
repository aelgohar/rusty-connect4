#![recursion_limit = "100000"]
#![allow(warnings)]
mod Connect4Computer;
mod Connect4Human;
mod HowToConnect4;
mod HowToToot;
mod ScoreBoard;
mod Scores;
mod TootOttoComputer;
mod TootOttoHuman;
mod Welcome;
mod canvas;
mod display_window;
mod page;
mod player;
mod sidebar;
mod toot_canvas;

#[macro_use]
extern crate stdweb;

pub use crate::{
    canvas::CanvasModel,
    page::{Page, PageProps},
    sidebar::{Sidebar, SidebarProps},
};
