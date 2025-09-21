use crate::prelude::*;

//static TRACING_BUF: Mutex<Option<VecDeque<String>>> = Mutex::new(None);
//const TRACING_SCROLLBACK: usize = 20;

impl G {
    pub(crate) fn trace_selected(&self) -> Result<()> {
        self.untrace_all();
        for p in self.selected_pawns() {
            p.traced.set(true);
        }

        Ok(())
    }

    pub(crate) fn untrace_all(&self) {
        for p in self.pawns() {
            p.traced.clear();
        }
    }
}

//pub(crate) fn trace_impl<T: Display>(slf: T, msg: String){
//	let line = format!("{slf}: {msg}");
//	let mut buf = TRACING_BUF.lock().unwrap();
//	let buf = buf.get_or_insert_with(VecDeque::default);
//	buf.push_back(line);
//}
