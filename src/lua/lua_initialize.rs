use std::env;

use mlua::{Lua, Error};

use crate::helper::helper_functions::get_path_string;

use super::lua_functions::load_lua_file;

pub fn initialize_lua() -> Lua {
    let lua: Lua = Lua::new();
    
    // tells lua which operating system is being used
    lua.globals().set("operating_system", env::consts::OS).unwrap();

    // tells lua where the root of the folder is
    lua.globals().set("current_working_directory", get_path_string()).unwrap();

    /*
    loads the lua context (entry point)
    this is an mlua::Error enum
    */
    let lua_error: Result<(), Error> = lua.load(&load_lua_file("/lua_libraries/lua_context.lua")).exec();

    match lua_error {
        Ok(_) => (),
        Err(error) => panic!("{}", error.to_string())
    }

    lua
}