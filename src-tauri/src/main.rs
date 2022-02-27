#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Menu, MenuEntry, MenuItem, Submenu};

mod assembler;
mod disassembler;

#[tauri::command]
async fn cmd_assemble(msg: String) -> Result<String, String> {
    assembler::assembly_to_hex(&msg).map_err(|e| format!("{:?}", e))
}

#[tauri::command]
async fn cmd_disassemble(msg: String) -> Result<String, String> {
    disassembler::hex_to_assembly(&msg).map_err(|e| format!("{:?}", e))
}

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
    let app = tauri::Builder::default()
        .menu(Menu::with_items([sub_menu_main(), sub_menu_edit()]))
        .invoke_handler(tauri::generate_handler![cmd_assemble, cmd_disassemble]);
    app.run(ctx).expect("error while running tauri application");
}
