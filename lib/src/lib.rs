#![feature(iter_array_chunks)]
#![feature(iter_advance_by)]

pub mod script;
pub mod plugin;
pub mod util;
pub mod semiluna;
pub mod ui;

const SCRIPTS: &'static str = "scripts";
const PLUGINS: &'static str = "plugins";
const __INDEX: &'static str = "__index";
const __CALL: &'static str = "__call";

