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
            &["pause"] => self.cmd_pause_or_resume(),
            &["tick"] => self.cmd_tick(),
            &[cmd, ..] => Err(anyhow!("unknown command: {cmd:?}")),
            &[] => Ok(()),
        }
    }


    /// pause button: pause or resume (if already paused)
    fn cmd_pause_or_resume(&mut self) -> Result<()> {
        self.speed = match self.speed {
            0 => 1,
            _ => 0,
        };
        Ok(())
    }

    fn cmd_tick(&mut self) -> Result<()> {
        self.tick_once();
        Ok(())
    }

}
