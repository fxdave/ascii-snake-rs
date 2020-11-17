use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// Snake Problems
#[derive(Debug)]
pub struct SelfEatingStepError;
impl Display for SelfEatingStepError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "This step is self-eating!")
    }
}
impl Error for SelfEatingStepError {}

#[derive(Debug)]
pub struct SelfTurningDirectionError;
impl Display for SelfTurningDirectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "This direction would cause a self-turning step!")
    }
}
impl Error for SelfTurningDirectionError {}

/// # GameError
#[derive(Debug)]
pub enum GameError {
    KilledByWall,
    SelfTurningDirectionError(SelfTurningDirectionError),
    SelfEatingStepError(SelfEatingStepError),
}

impl From<SelfEatingStepError> for GameError {
    fn from(err: SelfEatingStepError) -> Self {
        Self::SelfEatingStepError(err)
    }
}
impl From<SelfTurningDirectionError> for GameError {
    fn from(err: SelfTurningDirectionError) -> Self {
        Self::SelfTurningDirectionError(err)
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::KilledByWall => write!(f, "You've been killed by the wall"),
            Self::SelfEatingStepError(e) => write!(f, "{}", e),
            Self::SelfTurningDirectionError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for GameError {}
