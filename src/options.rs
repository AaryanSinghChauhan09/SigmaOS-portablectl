//! Options model.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode { Default, List, Attach(String, String), Start(String), Stop(String), Remove(String) }

#[derive(Debug, Clone)]
pub struct Options { pub mode: Mode }
impl Default for Options { fn default() -> Self { Self { mode: Mode::Default } } }
#[cfg(test)] mod tests { use super::*; #[test] fn o() { assert_eq!(Options::default().mode, Mode::Default); } }
