use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    RunForever,
    RunToAbsPos,
    RunToRelPos,
    RunTimed,
    RunDirect,
    Stop,
    Reset,
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        use Command::*;
        match self {
            RunForever => "run-forever",
            RunToAbsPos => "run-to-abs-pos",
            RunToRelPos => "run-to-rel-pos",
            RunTimed => "run-timed",
            RunDirect => "run-direct",
            Stop => "stop",
            Reset => "reset",
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}
