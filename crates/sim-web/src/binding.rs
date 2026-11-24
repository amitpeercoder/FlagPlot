//! The binding of the door for JavaScript (decision 0229, ruling 3): one
//! small module, compiled for the browser's target alone, each bound function
//! one call of the door. The one door of a module is held in a
//! `thread_local!` cell; no class is exported.
//!
//! A `Result` that is an `Err` is given to JavaScript as an error carrying
//! the door's words (ruling 2 of the architect's rulings on pull request
//! 732). A panic ends the module with no words (ruling 27): no hook is
//! installed that would print them.

use std::cell::RefCell;

use wasm_bindgen::prelude::*;

use crate::Door;

thread_local! {
    static DOOR: RefCell<Door> = RefCell::new(Door::new());
}

fn words(e: String) -> JsError {
    JsError::new(&e)
}

/// The rules, state and doctrine versions the module was built with.
#[wasm_bindgen]
pub fn versions() -> String {
    DOOR.with(|d| d.borrow().versions())
}

/// Opens the game: the pack's bytes and a request.
#[wasm_bindgen]
pub fn begin(pack: &[u8], request: &str) -> Result<(), JsError> {
    DOOR.with(|d| d.borrow_mut().begin(pack, request))
        .map_err(words)
}

/// The seat's tools, as `tools/list` gives them.
#[wasm_bindgen]
pub fn tools() -> Result<String, JsError> {
    DOOR.with(|d| d.borrow().tools()).map_err(words)
}

/// One tool call: `{"ok": ..}` or `{"refused": ..}`.
#[wasm_bindgen]
pub fn call(tool: &str, arguments: &str) -> Result<String, JsError> {
    DOOR.with(|d| d.borrow_mut().call(tool, arguments))
        .map_err(words)
}

/// The game file of the game as it stands.
#[wasm_bindgen]
pub fn game_file() -> Result<String, JsError> {
    DOOR.with(|d| d.borrow().game_file()).map_err(words)
}

/// Opens the game by replaying a game file.
#[wasm_bindgen]
pub fn resume(pack: &[u8], game_file: &str) -> Result<(), JsError> {
    DOOR.with(|d| d.borrow_mut().resume(pack, game_file))
        .map_err(words)
}

/// The saved game, refused until the game is over.
#[wasm_bindgen]
pub fn saved_game() -> Result<String, JsError> {
    DOOR.with(|d| d.borrow().saved_game()).map_err(words)
}
