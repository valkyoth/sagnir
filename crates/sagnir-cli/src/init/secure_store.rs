#[cfg(not(unix))]
mod portable;
#[cfg(unix)]
mod unix;

#[cfg(not(unix))]
pub(super) use portable::SecureStore;
#[cfg(unix)]
pub(super) use unix::SecureStore;
