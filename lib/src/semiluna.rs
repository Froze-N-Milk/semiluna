use std::collections::HashMap;

use mlua::prelude::*;

use crate::{plugin, script::Script};

pub struct Semiluna<'lua>(LuaTable<'lua>);

impl<'lua> std::ops::Deref for Semiluna<'lua> {
    type Target = LuaTable<'lua>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'lua> Semiluna<'lua> {
    const SELF_KEY: &'static str = "semiluna";

    pub fn obtain(lua: &'lua Lua) -> LuaResult<Self> {
        let semiluna = lua.globals()
            .raw_get::<_, LuaValue>(Self::SELF_KEY)?;

        let semiluna = Semiluna(match semiluna {
            LuaValue::Table(table) => table,
            LuaValue::Nil => {
                let table = Self::generate_semiluna_table(lua)?;
                lua.globals().raw_set(Self::SELF_KEY, &table)?;
                table
            },
            any => Err(LuaError::FromLuaConversionError { from: any.type_name(), to: "Semiluna", message: None })?,
        });

        Ok(semiluna)
    }

    fn generate_semiluna_table(lua: &Lua) -> LuaResult<LuaTable> {
        let semiluna = lua.create_table()?;
        semiluna.set(crate::PLUGINS, plugin::Plugin::generate_table(&lua)?)?;
        semiluna.set(crate::SCRIPTS, lua.create_table()?)?;
        Ok(semiluna)
    }

    pub fn scripts(&self) -> LuaResult<LuaTable> {
        self.get(crate::SCRIPTS)
    }

    /// generates a [HashMap] of the current table, inserting values will not also insert it into
    /// the underlying lua table
    pub fn scripts_hashmap(&self) -> LuaResult<HashMap<String, Script>> {
        self.get(crate::SCRIPTS)
    }
}
