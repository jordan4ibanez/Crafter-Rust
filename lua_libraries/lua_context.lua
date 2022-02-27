--[[

    This is the literal entry point into the lua scope.

    This is why it's called "lua_context.lua". Without this file, the script api does not have anything to work from.

    From here you can freely modify your game to be whatever you want it to be.

    But for now it is going to be Crafter.

    This is extremely WIP so expect it to be insanely bare bones.

]]--

--[[
    This is the base building block of the entire Crafter api.

    Everything from here on out is contained within this table.

    Localizing functions from this table can greatly improve your performance.
]]--

require("lua_libraries.lua_helpers")

crafter = {
    -- Holds block data to be passed into Rust.
    blocks = {},
    -- Localization cached and then cached into table.
    operating_system = get_operating_system(),
    -- Current root directory of the program.
    directory = get_working_directory(),
    -- Caches textures for Rust.
    texture_cache = {}
}

-- This is debug for testing on other operating systems.
print("lua operating system detection: " .. crafter.operating_system);

-- Run the function builder.
dofile("lua_libraries/lua_functions.lua")

-- This is a simple way to hold the file directory without creating an on-disk cache.
current_loading_mod = nil

-- The Windows module loader.
if crafter.operating_system == "windows" then
    -- Open mods folder using built in Windows function.
    local f = io.popen("dir " .. crafter.directory .. "\\mods /b /ad")

    -- Iterate each folder.
    for mod in f:lines() do
        -- This is a global assign.
        current_loading_mod = mod
        -- Run module's entry point.
        dofile(crafter.directory .. "\\mods\\" .. mod .. "\\main.lua")
    end
-- The Linux module loader.
elseif crafter.operating_system == "linux" then

    -- Open mods folder using built in Linux function.
    local pfile = io.popen('ls -a "'.. crafter.directory .. '/mods"')

    -- Iterate each file.
    for mod in pfile:lines() do
        -- If file contains a period, do not run it.
        if not string.find(mod, "%.") then
            -- This is a global assign.
            current_loading_mod = mod
            -- Run module's entry point.
            dofile(crafter.directory .. "/mods/" .. mod .. "/main.lua")
        end
    end
elseif crafter.operating_system == "mac" then
    print("Test this on a mac somehow.")
    print("I'm not even sure if this comes up as mac.")
end


print("--- LUA IS NOW DONE ---")