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

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Command::*;
        f.write_str(match self {
            RunForever => "run-forever",
            RunToAbsPos => "run-to-abs-pos",
            RunToRelPos => "run-to-rel-pos",
            RunTimed => "run-timed",
            RunDirect => "run-direct",
            Stop => "stop",
            Reset => "reset",
        })
    }
}
