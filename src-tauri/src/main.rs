#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Menu, MenuEntry, MenuItem, Submenu};

fn sub_menu_main() -> MenuEntry {
    MenuEntry::Submenu(Submenu::new(
        "Disa",
        Menu::with_items([MenuItem::Hide.into(), MenuItem::Quit.into()]),
    ))
}

fn sub_menu_edit() -> MenuEntry {
    MenuEntry::Submenu(Submenu::new(
        "Edit",
        Menu::with_items([
            MenuItem::Copy.into(),
            MenuItem::Paste.into(),
            MenuItem::Cut.into(),
        ]),
    ))
}

fn main() {
    let ctx = tauri::generate_context!();
    let app = tauri::Builder::default().menu(Menu::with_items([sub_menu_main(), sub_menu_edit()]));
    app.run(ctx).expect("error while running tauri application");
}
