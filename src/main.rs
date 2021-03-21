#![windows_subsystem = "windows"]

#[macro_use]
extern crate lazy_static;
extern crate native_windows_gui as nwg;
#[macro_use]
extern crate native_windows_derive as nwd;
#[macro_use]
extern crate serde;

use std::fmt::{Display, Formatter};

use nwg::{NativeUi, TrayNotificationFlags, Timer, MenuItem, Menu, TrayNotification, Icon, ComboBox, Label, Window, MenuSeparator, GlobalCursor, stop_thread_dispatch, init, dispatch_thread_events, Button, EmbedResource};
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Deserialize, Debug)]
pub struct ChuckFact {
   value: String
}

#[derive(Default, Clone, Copy)]
pub struct ComboBoxModel<T> {
    label: &'static str,
    value: T,
}

static LAST_QUOTE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

lazy_static! {
    static ref DURATION_MODEL: Vec<ComboBoxModel<u32>> = {
        let model = vec![
            ComboBoxModel::new("Every 10 seconds", 10000),
            ComboBoxModel::new("Every 15 seconds", 15000),
            ComboBoxModel::new("Every 20 seconds", 20000),
            ComboBoxModel::new("Every 30 seconds", 30000),
            ComboBoxModel::new("Every 45 seconds", 45000),
            ComboBoxModel::new("Every minute", 60000),
            ComboBoxModel::new("Every 2 minutes", 2 * 60000),
            ComboBoxModel::new("Every 5 minutes", 5 * 60000),
            ComboBoxModel::new("Every 10 minutes", 10 * 60000),
            ComboBoxModel::new("Every 15 minutes", 15 * 60000),
            ComboBoxModel::new("Every 20 minutes", 20 * 60000),
            ComboBoxModel::new("Every 30 minutes", 30 * 60000),
            ComboBoxModel::new("Every 45 minutes", 45 * 60000),
            ComboBoxModel::new("Every hour", 60 * 60000),
        ];
        model
    };
}

impl<T> ComboBoxModel<T> {
    fn new(label: &'static str, value: T) -> ComboBoxModel<T> {
        ComboBoxModel { label, value }
    }
}

impl<T> Display for ComboBoxModel<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

#[derive(Default, NwgUi)]
pub struct ChuckApp {

    #[nwg_resource]
    embed: EmbedResource,

    // window: MessageWindow,
    #[nwg_control(size: (280, 130), center: true, title: "Settings", flags: "WINDOW")]
    window: Window,

    #[nwg_control(position: (10, 10), size: (260, 40), text: "How fast should Chuck speak ?")]
    label_settings: Label,
    #[nwg_control(position: (10, 40), size: (260, 40), selected_index: Some(0), collection: DURATION_MODEL.to_vec())]
    #[nwg_events(OnComboxBoxSelection: [ChuckApp::change_timer_duration])]
    cb_settings: ComboBox<ComboBoxModel<u32>>,
    #[nwg_control(position: (10, 80), size: (260, 40), text: "OK")]
    #[nwg_events(OnButtonClick: [ChuckApp::hide_settings_window])]
    ok_settings: Button,

    #[nwg_resource(source_embed: Some(&data.embed), source_embed_id: 1)]
    icon_app: Icon,

    #[nwg_resource(source_embed: Some(&data.embed), source_embed_id: 2)]
    icon_notif: Icon,

    #[nwg_control(icon: Some(&data.icon_app), tip: Some("Chuck Norust"))]
    #[nwg_events(MousePressLeftUp: [ChuckApp::toggle_settings_visible], OnContextMenu: [ChuckApp::show_menu])]
    tray: TrayNotification,
    #[nwg_control(parent: window, popup: true)]
    tray_menu: Menu,
    #[nwg_control(parent: tray_menu, text: "&Repeat last quote", disabled: true)]
    #[nwg_events(OnMenuItemSelected: [ChuckApp::repeat_last_quote])]
    tray_item1: MenuItem,
    #[nwg_control(parent: tray_menu, text: "&Next quote")]
    #[nwg_events(OnMenuItemSelected: [ChuckApp::next_quote])]
    tray_item2: MenuItem,
    #[nwg_control(parent: tray_menu)]
    tray_item_separator: MenuSeparator,
    #[nwg_control(parent: tray_menu, text: "&Settings...")]
    #[nwg_events(OnMenuItemSelected: [ChuckApp::show_settings_window])]
    tray_item3: MenuItem,
    #[nwg_control(parent: tray_menu)]
    tray_item_separator2: MenuSeparator,
    #[nwg_control(parent: tray_menu, text: "E&xit")]
    #[nwg_events(OnMenuItemSelected: [ChuckApp::exit])]
    tray_item4: MenuItem,
    #[nwg_control(interval: 10000, stopped: false)]
    #[nwg_events(OnTimerTick: [ChuckApp::next_quote])]
    timer: Timer,
}

impl ChuckApp {

    fn toggle_settings_visible(&self) {
        self.window.set_visible(!self.window.visible());
    }

    fn show_settings_window(&self) {
        self.window.set_visible(true);
    }

    fn hide_settings_window(&self) {
        self.window.set_visible(false);
    }

    fn show_menu(&self) {
        let (x, y) = GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn repeat_last_quote(&self) {
        if let Ok(quote) = LAST_QUOTE.lock() {
            self.timer.stop();
            self.tray.show(
                &quote,
                Some("Chuck fact..."),
                Some(TrayNotificationFlags::USER_ICON | TrayNotificationFlags::LARGE_ICON),
                Some(&self.icon_notif),
            );
            self.timer.start();
        }
    }

    fn next_quote(&self) {
        if let Some(quote) = reqwest::blocking::get("https://api.chucknorris.io/jokes/random")
            .and_then(|resp| resp.json::<ChuckFact>())
            .ok()
            .map(|fact| fact.value)
        {
            if quote.len() > 150 { // doesn't fit in win32 notifs
                self.next_quote();
            } else {
                self.tray_item1.set_enabled(true);
                if let Ok(mut last_quote_mutex) = LAST_QUOTE.lock() {
                    *last_quote_mutex = quote.clone();
                }
                self.timer.stop();
                self.tray.show(
                    &quote,
                    Some("Chuck fact..."),
                    Some(TrayNotificationFlags::USER_ICON | TrayNotificationFlags::LARGE_ICON),
                    Some(&self.icon_notif),
                );
                self.timer.start();
            }
        }
    }

    fn change_timer_duration(&self) {
        if let Some(ComboBoxModel { value, label }) = self.cb_settings
            .selection()
            .and_then(|idx| DURATION_MODEL.get(idx))
        {
            println!("sélection de {} : {} ms", label, *value);
            self.timer.stop();
            self.timer.set_interval(*value);
            self.timer.start();
        }
    }

    fn exit(&self) {
        stop_thread_dispatch();
    }
}

fn main() {
    init().expect("Failed to init Native Windows GUI");
    let _ui = ChuckApp::build_ui(Default::default()).expect("Failed to build UI");
    dispatch_thread_events();
}
