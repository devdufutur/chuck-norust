#![windows_subsystem = "windows"]

#[macro_use]
extern crate lazy_static;
extern crate native_windows_gui as nwg;
#[macro_use]
extern crate serde;

use std::fmt::{Display, Formatter};

use nwg::{NativeUi, TrayNotificationFlags, Timer, MenuItem, Menu, TrayNotification, Icon, ComboBox, Label, Window, MenuSeparator, GlobalCursor, stop_thread_dispatch, init, dispatch_thread_events, Button};
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

#[derive(Default)]
pub struct ChuckApp {
    // window: MessageWindow,
    window: Window,
    label_settings: Label,
    cb_settings: ComboBox<ComboBoxModel<u32>>,
    ok_settings: Button,
    icon: Icon,
    icon_notif: Icon,
    tray: TrayNotification,
    tray_menu: Menu,
    tray_item1: MenuItem,
    tray_item2: MenuItem,
    tray_item3: MenuItem,
    tray_item4: MenuItem,
    timer: Timer,
    tray_item_separator: MenuSeparator,
    tray_item_separator2: MenuSeparator,
}

impl ChuckApp {
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

    fn exit(&self) {
        stop_thread_dispatch();
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod system_tray_ui {
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    use native_windows_gui as nwg;
    use nwg::{ComboBox, Label, Window, EventHandler, EmbedResource, MousePressEvent, full_bind_event_handler, unbind_event_handler};

    use super::*;

    pub struct ChuckAppUi {
        inner: Rc<ChuckApp>,
        default_handler: RefCell<Vec<EventHandler>>,
    }

    impl NativeUi<ChuckAppUi> for ChuckApp {
        fn build_ui(mut data: ChuckApp) -> Result<ChuckAppUi, nwg::NwgError> {
            use nwg::Event as E;

            // embedded icons
            if let Some(icon) = EmbedResource::load(None)?.icon(1, None) {
                data.icon = icon;
            }
            if let Some(icon_notif) = EmbedResource::load(None)?.icon(2, None) {
                data.icon_notif = icon_notif;
            }

            Window::builder()
                .size((280, 130))
                .title("Settings")
                .center(true)
                .flags(nwg::WindowFlags::WINDOW) // invisible
                .build(&mut data.window)?;

            TrayNotification::builder()
                .parent(&data.window)
                .icon(Some(&data.icon))
                .tip(Some("Chuck Norust"))
                .build(&mut data.tray)?;

            Menu::builder()
                .popup(true)
                .parent(&data.window)
                .build(&mut data.tray_menu)?;

            MenuItem::builder()
                .text("&Repeat last quote")
                .disabled(true)
                .parent(&data.tray_menu)
                .build(&mut data.tray_item1)?;

            MenuItem::builder()
                .text("&Next quote")
                .parent(&data.tray_menu)
                .build(&mut data.tray_item2)?;

            MenuSeparator::builder()
                .parent(&data.tray_menu)
                .build(&mut data.tray_item_separator)?;

            MenuItem::builder()
                .text("&Settings")
                .parent(&data.tray_menu)
                .build(&mut data.tray_item3)?;

            MenuSeparator::builder()
                .parent(&data.tray_menu)
                .build(&mut data.tray_item_separator2)?;

            MenuItem::builder()
                .text("E&xit")
                .parent(&data.tray_menu)
                .build(&mut data.tray_item4)?;

            Timer::builder()
                .interval(10000)
                .parent(&data.window)
                .stopped(false)
                .build(&mut data.timer)?;

            Label::builder()
                .parent(&data.window)
                .position((10, 10))
                .size((260, 40))
                .text("How fast should Chuck speak ?")
                .build(&mut data.label_settings)?;

            ComboBox::builder()
                .parent(&data.window)
                .position((10, 40))
                .size((260, 40))
                .selected_index(Some(0))
                .collection(DURATION_MODEL.to_vec())
                .build(&mut data.cb_settings)?;

            Button::builder()
                .parent(&data.window)
                .position((10, 80))
                .size((260, 40))
                .text("OK")
                .build(&mut data.ok_settings)?;

            // Wrap-up
            let ui = ChuckAppUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(evt_ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => {
                            if &handle == &evt_ui.ok_settings {
                                evt_ui.window.set_visible(false);
                            }
                        }
                        E::OnComboxBoxSelection => {
                            if let Some(ComboBoxModel { value, label }) = evt_ui
                                .cb_settings
                                .selection()
                                .and_then(|idx| DURATION_MODEL.get(idx))
                            {
                                println!("sélection de {} : {} ms", label, *value);
                                evt_ui.timer.stop();
                                evt_ui.timer.set_interval(*value);
                                evt_ui.timer.start();
                            }
                        }
                        E::OnTimerTick => {
                            if &handle == &evt_ui.timer {
                                evt_ui.next_quote();
                            }
                        }
                        E::OnMousePress(MousePressEvent::MousePressLeftUp) => {
                            if &handle == &evt_ui.tray {
                                evt_ui.window.set_visible(!evt_ui.window.visible());
                            }
                        }
                        E::OnContextMenu => {
                            if &handle == &evt_ui.tray {
                                ChuckApp::show_menu(&evt_ui);
                            }
                        }
                        E::OnMenuItemSelected => {
                            if &handle == &evt_ui.tray_item1 {
                                evt_ui.repeat_last_quote();
                            } else if &handle == &evt_ui.tray_item2 {
                                evt_ui.next_quote();
                            } else if &handle == &evt_ui.tray_item3 {
                                evt_ui.window.set_visible(true);
                            } else if &handle == &evt_ui.tray_item4 {
                                evt_ui.exit();
                            }
                        }
                        _ => {}
                    }
                }
            };

            ui.default_handler
                .borrow_mut()
                .push(full_bind_event_handler(
                    &ui.window.handle,
                    handle_events,
                ));

            return Ok(ui);
        }
    }

    impl Drop for ChuckAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let mut handlers = self.default_handler.borrow_mut();
            for handler in handlers.drain(0..) {
                unbind_event_handler(&handler);
            }
        }
    }

    impl Deref for ChuckAppUi {
        type Target = ChuckApp;

        fn deref(&self) -> &ChuckApp {
            &self.inner
        }
    }
}

fn main() {
    init().expect("Failed to init Native Windows GUI");
    let _ui = ChuckApp::build_ui(Default::default()).expect("Failed to build UI");
    dispatch_thread_events();
}
