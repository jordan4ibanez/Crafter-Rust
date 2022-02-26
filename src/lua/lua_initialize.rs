use std::env;

use mlua::{Lua, Table};

use super::lua_functions::load_lua_file;

pub fn initialize_lua() -> Lua {
    let lua: Lua = Lua::new();
    
    // tells lua which operating system is being used
    lua.globals().set("operating_system", env::consts::OS).unwrap();

    // loads the lua context (entry point)
    lua.load(&load_lua_file("/lua_libraries/lua_context.lua")).exec().unwrap();

    lua
}