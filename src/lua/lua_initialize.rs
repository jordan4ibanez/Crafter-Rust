use mlua::Lua;

use super::lua_functions::load_lua_file;

pub fn initialize_lua() {
    let lua: Lua = Lua::new();

    lua.load(&load_lua_file("/lua_libraries/lua_context.lua")).exec().unwrap();
}