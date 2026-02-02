//! Util.

#[allow(unused)]
#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("crossterm error")]
    Crossterm,
}

#[cfg(feature = "crossterm")]
pub struct MouseCapture;

#[cfg(feature = "crossterm")]
impl MouseCapture {
    pub fn new() -> Result<Self, Error> {
        if let Err(_) = crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)
        {
            return Err(Error::Crossterm);
        }
        return Ok(Self);
    }
}

#[cfg(feature = "crossterm")]
impl Drop for MouseCapture {
    fn drop(&mut self) {
        if let Err(e) =
            crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)
        {
            tracing::error!("Failed to restore terminal keyboard: {e}");
        }
    }
}
