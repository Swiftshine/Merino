#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod merino;
use merino::MerinoApp;

fn main() -> Result<(), eframe::Error> {
    MerinoApp::run()
}
