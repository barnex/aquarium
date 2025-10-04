// Receive text commands from HTML/Javascript.
// E.g. "save" button issues text command "save".
use crate::*;

/// Commands are queued here (e.g. on button click),
/// to be processed on each tick.
pub(crate) static COMMAND_BUFFER: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

/// Exposed to JavaScript as
/// 	window.wasmBindings.cmd(...)
/// Allows HTML elements to send commands. E.g.
/// 	<button onclick='window.wasmBindings.cmd("save")'>💾</button><br/>
///
#[wasm_bindgen]
pub fn cmd(cmd: String) {
    // 👇 main loop polls this every frame
    COMMAND_BUFFER.lock().unwrap().push_back(cmd);
}

/// Execute commands consumed from COMMAND_BUFFER,
/// forward to game state if not JS-specific.
pub(crate) fn exec_pending_commands<G: GameCore>(state: &mut G) {
    for cmd in COMMAND_BUFFER.lock().unwrap().drain(..) {
        match exec_command(state, &cmd) {
            Ok(()) => log::info!("js command {cmd:?}: OK"),
            Err(e) => log::info!("js command {cmd:?}: {e:?}"),
        }
    }
}

// Execute a single command.
// If unknown, forward to the game state.
fn exec_command<G: GameCore>(state: &mut G, cmd: &str) -> JsResult<()> {
    match cmd.trim().split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
        &["save"] => Ok(save_game(state)),
        &["reset"] => Ok(reset(state)),
        &["save_reload"] => Ok(save_reload(state)),
        &["toggle_large"] => Ok(toggle_large()),
        &["screenshot"] => download_screenshot("screenshot.png"),
        _ => Err("unknown command".into()),
    }
}

// toggle between large & small canvas size.
fn toggle_large() {
    let canvas = get_element_by_id::<HtmlCanvasElement>("canvas");
    let screen_width = window().inner_width().unwrap().as_f64().unwrap();
    let screen_height = window().inner_height().unwrap().as_f64().unwrap();

    let margin_x = 2.0; // +2 for 1px margin on each side.
    let margin_y = (canvas.client_top() + 2) as f64;
    let new_size = ((screen_width - margin_x) as u32, (screen_height - margin_y) as u32);
    let curr_size = (canvas.width(), canvas.height());

    if new_size != curr_size {
        canvas.set_width(new_size.0);
        canvas.set_height(new_size.1);
    } else {
        // was already at large size: toggle back to small
        canvas.set_height(320);
        canvas.set_width(480);
    }
}

// save + reload command
fn save_reload<G: GameCore>(state: &G) {
    save_game(state);
    window().location().reload().expect("reload");
}

// reset gamestate command
fn reset<G: GameCore>(state: &mut G) {
    state.reset()
}
