use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    RunForever,
    RunToAbsPos,
    RunToRelPos,
    RunDirect,
    Stop,
    Reset,
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Command::RunForever => "run-forever",
            Command::RunToAbsPos => "run-to-abs-pos",
            Command::RunToRelPos => "run-to-rel-pos",
            Command::RunDirect => "run-direct",
            Command::Stop => "stop",
            Command::Reset => "reset",
        }
        .fmt(f)
    }
}
