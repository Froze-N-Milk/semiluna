use std::{fs, io::{self, Write}};

use crossterm::{cursor, execute, terminal};
use mlua::prelude::*;
use lib::{self, semiluna::Semiluna};

fn main() -> LuaResult<()> {
    let lua = Lua::new();
    let semiluna = Semiluna::obtain(&lua)?;

    // todo:
    // load config from init.semiluna
    // enter interactive shell-like state where loaded scripts can be executed and queried, etc...

    lua.load(
        fs::read("init.semiluna.lua").expect("Unable to read init.semiluna file, check that it exists in this directory")
    ).exec()?;

    execute!(io::stdout(), terminal::EnterAlternateScreen, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).unwrap();

    // todo fix expect here
    lib::ui::full_width_boxed_text(&mut io::stdout(), "Semiluna", 3).expect("WRITING FAILED HAHAH");
    io::stdout().flush()?;

    let scripts = semiluna.scripts_hashmap()?;

    println!("The following top level scripts are available:");
    for key in scripts.keys() {
        println!("{}", key);
    }

    println!();
    println!("enter 'q' to quit");

    let mut input = String::new();
    while input != "q" {
        if let Some(script) = scripts.get(&input) {
            println!("executing {input}");
            script.exec()?;
            println!("done");
        }
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        input = input[..input.len() - 1].to_owned();
    }

    execute!(io::stdout(), terminal::LeaveAlternateScreen).unwrap();

    Ok(())
}

