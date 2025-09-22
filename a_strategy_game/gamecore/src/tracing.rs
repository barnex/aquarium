use crate::prelude::*;

#[macro_export]
macro_rules! trace {
    ($slf:expr) => {
        #[cfg(debug_assertions)]
        {
            let slf = $slf;
            if slf.traced.get(){
                log::trace!("{slf}: {}", caller!());
            }
        }
    };
    ($slf:expr, $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            let slf = $slf;
            if slf.traced.get(){
                log::trace!("{slf}: {}: {}", caller!(), format!($($arg)*));
            }
        }
    };
}

impl G {
    pub(crate) fn trace_selected(&self) -> Result<()> {
        self.untrace_all();
        let mut count = 0;
        for p in self.selected_pawns().inspect(|_| count += 1) {
            p.traced.set(true);
        }
        log::trace!("tracing {count} entities");

        Ok(())
    }

    pub(crate) fn untrace_all(&self) {
        for p in self.pawns() {
            p.traced.clear();
        }
    }
}
