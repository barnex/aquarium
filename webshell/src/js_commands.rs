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
pub(crate) fn exec_pending_commands(state: &mut State) {
    for cmd in COMMAND_BUFFER.lock().unwrap().drain(..) {
        match exec_command(state, &cmd) {
            Ok(()) => log::info!("command {cmd:?}: OK"),
            Err(e) => log::info!("command {cmd:?}: {e:?}"),
        }
    }
}

// Execute a single command.
// If unknown, forward to the game state.
fn exec_command(state: &mut State, cmd: &str) -> JsResult<()> {
    match cmd.trim().split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
        &["save"] => Ok(save_game(state)),
        &["reset"] => Ok(reset(state)),
        &["save_reload"] => Ok(save_reload(state)),
        // 👇 unknown command: forward to gamestate
        _ => Ok(state.commands.push_back(cmd.to_owned())),
    }
}

// save + reload command
fn save_reload(state: &State) {
    save_game(state);
    window().location().reload().expect("reload");
}

// reset gamestate command
fn reset(state: &mut State) {
    *state = State::new();
}
