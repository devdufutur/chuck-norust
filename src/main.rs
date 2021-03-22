#![windows_subsystem = "windows"]

mod gui;
mod combobox_model;
mod common;
mod winreg;

#[macro_use]
extern crate lazy_static;
extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

#[macro_use]
extern crate serde;

use nwg::{init, dispatch_thread_events, NativeUi};

fn main() {
    init().expect("Failed to init Native Windows GUI");
    let _ui = gui::ChuckApp::build_ui(Default::default()).expect("Failed to build UI");
    dispatch_thread_events();
}
