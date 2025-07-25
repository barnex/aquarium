// Receive text commands from HTML/Javascript.
// E.g. "save" button issues text command "save".
//
// Freetext commands can be made as well (console mode)

use crate::*;

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

/// Execute commands from COMMAND_BUFFER,
/// forward to game state if not JS-specific.
pub(crate) fn exec_commands(state: &mut State) {
    for cmd in COMMAND_BUFFER.lock().unwrap().drain(..) {
        match exec_command(state, &cmd) {
            Ok(()) => log::info!("command {cmd:?}: OK"),
            Err(e) => log::info!("command {cmd:?}: {e:?}"),
        }
    }
    state.commands.extend(COMMAND_BUFFER.lock().unwrap().drain(..))
}

fn exec_command(state: &mut State, cmd: &str) -> JsResult<()> {
    match cmd.trim().split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
        &["save_reload"] => Ok(save_reload(state)),
        _ => Ok(state.commands.push_back(cmd.to_owned())),
    }
}

fn save_reload(state: &State) {
    save_game(state);
    window().location().reload();
}
