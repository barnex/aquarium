use crate::prelude::*;

impl G {
    /// Consume command buffer, execute all commands.
    /// Powers console mode.
    pub(crate) fn exec_commands(&mut self) {
        let commands = self.commands.drain(..).collect_vec();
        for cmd in commands {
            self.console.print(cmd.to_owned());
            match self.exec_command(&cmd) {
                Ok(()) => {
                    log::info!("command {cmd:?}: OK");
                    self.console.print("OK");
                }
                Err(e) => {
                    log::info!("command {cmd:?}: {e}");
                    self.console.print(format! {"{e}"})
                }
            }
        }
    }

    /// execute a single command
    fn exec_command(&mut self, cmd: &str) -> Result<()> {
        match cmd.trim().split_ascii_whitespace().collect_vec().as_slice() {
            &["sanitycheck" | "sc"] => sanity_check(self),
            &["reset"] => Ok(*self = G::test_world()),
            &["pause" | "pa"] => Ok(toggle(&mut self.paused)),
            &["unpause" | "up"] => Ok(self.paused = false),
            &["tick"] => self.cmd_tick(),
            &["ui" | "toggle_ui"] => Ok(toggle(&mut self.ui.hidden)),
            &["walk" | "show_walkable"] => Ok(toggle(&mut self.debug.show_walkable)),
            &["build" | "show_buildable"] => Ok(toggle(&mut self.debug.show_buildable)),
            &["home" | "show_home"] => Ok(toggle(&mut self.debug.show_home)),
            &["dest" | "show_destination"] => Ok(toggle(&mut self.debug.show_destination)),
            &["downstream" | "show_downstream"] => Ok(toggle(&mut self.debug.show_downstream)),
            &["i" | "inspect_under_cursor"] => Ok(toggle(&mut self.debug.inspect_under_cursor)),
            &["tr" | "trace"] => self.trace_selected(),
            &["ut" | "untrace"] => Ok(self.untrace_all()),
            &["kill"] => Ok(self.selected_pawns().for_each(|p| self.kill_pawn(p))),
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
