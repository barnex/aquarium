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

/// Forward COMMAND_BUFFER to
pub(crate) fn record_commands(state: &mut State) {
    state.commands.extend(COMMAND_BUFFER.lock().unwrap().drain(..))
}
