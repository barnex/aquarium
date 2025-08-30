use crate::prelude::*;

impl G {
    /// Consume command buffer, execute all commands.
    /// Powers console mode.
    pub(crate) fn exec_commands(&mut self) {
        let commands = self.commands.drain(..).collect_vec();
        for cmd in commands {
            match self.exec_command(&cmd) {
                Ok(()) => log::info!("command {cmd:?}: OK"),
                Err(e) => log::info!("command {cmd:?}: {e}"),
            }
        }
    }

    /// execute a single command
    fn exec_command(&mut self, cmd: &str) -> Result<()> {
        match cmd.trim().split_ascii_whitespace().collect_vec().as_slice() {
            &["pause"] => Ok(toggle(&mut self.paused)),
            &["tick"] => self.cmd_tick(),
            &["toggle_ui"] => Ok(toggle(&mut self.ui.hidden)),
            &["show_walkable"] => Ok(toggle(&mut self.debug.show_walkable)),
            &["show_buildable"] => Ok(toggle(&mut self.debug.show_buildable)),
            &["show_home"] => Ok(toggle(&mut self.debug.show_home)),
            &["show_destination"] => Ok(toggle(&mut self.debug.show_destination)),
            &["show_downstream"] => Ok(toggle(&mut self.debug.show_downstream)),
            &["inspect_under_cursor"] => Ok(toggle(&mut self.debug.inspect_under_cursor)),
            &[cmd, ..] => Err(anyhow!("unknown command: {cmd:?}")),
            &[] => Ok(()),
        }
    }

    fn cmd_tick(&mut self) -> Result<()> {
        self.major_tick();
        Ok(())
    }
}

fn toggle(v: &mut bool) {
    *v = !*v
}
