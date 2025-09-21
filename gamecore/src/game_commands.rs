use crate::prelude::*;

impl G {
    /// Consume command buffer, execute all commands.
    /// Powers console mode.
    pub(crate) fn exec_commands(&mut self) {
        let commands = self.commands.drain(..).collect_vec();
        for cmd in commands {
            self.print_to_console(cmd.to_owned());
            match self.exec_command(&cmd) {
                Ok(()) => {
                    log::info!("command {cmd:?}: OK");
                    self.print_to_console("OK");
                }
                Err(e) => {
                    log::info!("command {cmd:?}: {e}");
                    self.print_to_console(format! {"{e}"})
                }
            }
        }
    }

    /// execute a single command
    fn exec_command(&mut self, cmd: &str) -> Result<()> {
        match cmd.trim().split_ascii_whitespace().collect_vec().as_slice() {
            &["pause"] => Ok(toggle(&mut self.paused)),
            &["tick"] => self.cmd_tick(),
            &["ui" | "toggle_ui"] => Ok(toggle(&mut self.ui.hidden)),
            &["walk" | "show_walkable"] => Ok(toggle(&mut self.debug.show_walkable)),
            &["build" | "show_buildable"] => Ok(toggle(&mut self.debug.show_buildable)),
            &["home" | "show_home"] => Ok(toggle(&mut self.debug.show_home)),
            &["dest" | "show_destination"] => Ok(toggle(&mut self.debug.show_destination)),
            &["downstream" | "show_downstream"] => Ok(toggle(&mut self.debug.show_downstream)),
            &["i" | "inspect_under_cursor"] => Ok(toggle(&mut self.debug.inspect_under_cursor)),
            &["t" | "trace"] => self.trace_selected(),
            &[cmd, ..] => Err(anyhow!("unknown command: {cmd:?}")),
            &[] => Ok(()),
        }
    }

    fn cmd_tick(&mut self) -> Result<()> {
        self.major_tick();
        Ok(())
    }
}

pub(crate) fn toggle(v: &mut bool) {
    *v = !*v
}
