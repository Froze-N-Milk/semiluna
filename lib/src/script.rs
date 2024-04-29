use std::ops::Deref;

use mlua::prelude::*;

pub struct Script <'lua> (LuaTable<'lua>);

impl<'lua> Deref for Script<'lua> {
    type Target = LuaTable<'lua>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'lua> FromLua<'lua> for Script<'lua> {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Script<'lua>> {
        match value {
            LuaValue::Table(t) => {
                Ok(Script ( t ))
            },
            any => Err(LuaError::ToLuaConversionError { from: any.type_name(), to: "Semiluna Script", message: Some("Script must be registered from a table".to_string()) })
        }
    }
}

impl<'lua> IntoLua<'lua> for Script<'lua> {
    fn into_lua(self, _lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::Table(self.0))
    }
}

impl Script<'_> {
    pub fn dependencies(&self) -> LuaResult<Vec<Script>> {
        self.get("dependencies")
    }

    pub fn exec(&self) -> LuaResult<()> {
        self.dependencies()?.into_iter().try_for_each(|d| d.exec())?;
        self.call_function::<_, ()>("exec", ())
    }
}
