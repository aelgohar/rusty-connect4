#![recursion_limit = "100000"]
mod Connect4Computer;
mod Connect4Human;
mod HowToConnect4;
mod HowToToot;
mod ScoreBoard;
mod Scores;
mod TootOttoComputer;
mod TootOttoHuman;
mod Welcome;
mod display_window;
mod page;
mod sidebar;

pub use crate::{
    page::{Page, PageProps},
    sidebar::{Sidebar, SidebarProps},
};
