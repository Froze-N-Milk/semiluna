use std::{fmt::{Debug, Display}, io::{self, Write}, usize};

use crossterm::terminal;
use mlua::prelude::*;

pub type Result<T> = core::result::Result<T, self::Error>;

#[derive(Debug)]
pub enum Error {
    Other(Box<dyn std::error::Error>),
    IO(io::Error),
    Size(&'static str),
}
impl std::error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(err) => std::fmt::Display::fmt(err, f),
            Self::IO(io_err) => std::fmt::Display::fmt(io_err, f),
            Self::Size(s) => write!(f, "{s}"),
        }
    }
}

impl From<io::Error> for self::Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

pub fn build_lua_table <'lua> (lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
    todo!()
}

pub fn full_width_boxed_text (write: &mut impl Write, buf: impl AsRef<str>, rows: u16) -> Result<()> {
    let (cols, _) = terminal::size()?;
    boxed_text(write, buf, cols, rows)
}

pub fn boxed_text (write: &mut impl Write, buf: impl AsRef<str>, cols: u16, rows: u16) -> Result<()> {
    let buf = buf.as_ref();

    if cols < 12 {
        return Err(Error::Size("Window too narrow for Semiluna"));
    }

    if rows < 3 {
        return Err(Error::Size("Window too short for Semiluna"));
    }

    let width: usize = (cols - 3).into();

    write.write("▄".repeat(cols.clone().into()).as_bytes())?;

    let lines = buf.lines()
        .map(str::split_whitespace);

    for mut line in lines {
        write.write("█ ".as_bytes())?;
        let mut line_len: usize = 0;
        let mut local_line = line.clone();
        let mut count: usize = 0;
        while let Some(word) = line.next() {
            line_len += word.len() + 1;
            count += 1;
            if line_len > width {
                line_len -= word.len() + 1;
                count -= 1;
                if word.len() > width - 1 {
                    write.write(format!("{:^1$}...", word, width).as_bytes())?;
                }
                else {
                    let spaces = (width - line_len) / 2;
                    for _ in 0..spaces {
                        write.write(" ".as_bytes())?;
                    }
                    for _ in 0..count {
                        write.write(local_line.next().unwrap().as_bytes())?;
                        write.write(" ".as_bytes())?;
                    }
                    let spaces = (width - line_len).div_ceil(2);
                    for _ in 0..spaces {
                        write.write(" ".as_bytes())?;
                    }
                    write.write("█\n".as_bytes())?;
                }
                local_line = line.clone();
                count = 0;
                line_len = 0;
            }
        }
        if line_len > 0 {
            let spaces = (width - line_len) / 2;
            for _ in 0..spaces {
                write.write(" ".as_bytes())?;
            }
            for _ in 0..count {
                write.write(local_line.next().unwrap().as_bytes())?;
                write.write(" ".as_bytes())?;
            }
            let spaces = (width - line_len).div_ceil(2);
            for _ in 0..spaces {
                write.write(" ".as_bytes())?;
            }
            write.write("█\n".as_bytes())?;
        }
    }

    write.write("▀".repeat(cols.clone().into()).as_bytes())?;

    Ok(())
}

// todo:
// ui stack
// vec of enums?
// enum at least describes element behaviour when not the focus: hide_self, hide_all, collapse
// in-focus element takes up the remainder of space

pub trait UI : std::io::Write {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn write_safe(&mut self, buf: &str) -> Result<()> {
        let width = self.width();
        if buf.len() < self.width() {
            write!(self, "{buf:^width$}")?;
        }
        Ok(())
    }
    fn bordered_write_safe(&mut self, buf: &str) -> Result<()> {
        write!(self, "█")?;
        self.write_safe(buf)?;
        write!(self, "█")?;
        Ok(())
    }
}
