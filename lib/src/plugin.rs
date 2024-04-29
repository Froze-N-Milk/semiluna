use std::ops::Deref;

use mlua::{prelude::*, Variadic};

use crate::semiluna::Semiluna;

pub struct Plugin <'lua> (LuaTable<'lua>);

impl<'lua> Deref for Plugin<'lua> {
    type Target = LuaTable<'lua>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'lua> FromLua<'lua> for Plugin<'lua> {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Plugin<'lua>> {
        match value {
            LuaValue::Table(t) => {
                Ok(Plugin ( t ))
            },
            any => Err(LuaError::ToLuaConversionError { from: any.type_name(), to: "Semiluna Plugin", message: Some("Plugin must be registered from a table".to_string()) })
        }
    }
}

impl<'lua> IntoLua<'lua> for Plugin<'lua> {
    fn into_lua(self, _lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::Table(self.0))
    }
}

impl<'lua> Plugin<'lua> {
    const EXPORT_FN_KEY: &'static str = "export";
    const NEW_FN_KEY: &'static str = crate::__CALL;

    pub fn generate_table(lua: &Lua) -> LuaResult<LuaTable> {
        let plugins_metatable = lua.create_table()?;
        plugins_metatable.set(Self::NEW_FN_KEY, lua.create_function(Self::new)?)?;
        plugins_metatable.set(crate::__INDEX, &plugins_metatable)?;
        let plugins = lua.create_table()?;
        plugins.set_metatable(Some(plugins_metatable));
        Ok(plugins)
    }

    fn new(lua: &Lua, _: ()) -> LuaResult<Plugin> {
        let plugin_metatable = lua.create_table()?;
        plugin_metatable.set(crate::SCRIPTS, lua.create_table()?)?;
        plugin_metatable.set(Self::EXPORT_FN_KEY, lua.create_function(export)?)?;
        plugin_metatable.set(crate::__INDEX, &plugin_metatable)?;
        let plugin = lua.create_table()?;
        plugin.set_metatable(Some(plugin_metatable));
        Ok(Plugin(plugin))
    }
}

fn export(lua: &Lua, export_requests: (Plugin, Variadic<LuaValue>)) -> LuaResult<()> {
    let semiluna = Semiluna::obtain(&lua)?;
    let global_scripts = semiluna.scripts()?;
    let scripts = export_requests.0.get::<_, LuaTable>(crate::SCRIPTS)?;
    for key in export_requests.1 {
        if scripts.contains_key(&key)? {
            if global_scripts.contains_key(&key)? {
                return Err(LuaError::runtime(format!("Global script already registed for the name {key:?}.\nUse a renaming pair to export the script to a different global name, {{\"{key:?}\", \"<new_name>\"}}")));
            }
            global_scripts.set(&key, scripts.raw_get::<_, LuaValue>(&key)?)?;
        }
        else {
            return Err(LuaError::runtime(format!("No script found for {key:?}")));
        }
    }
    Ok(())
}
