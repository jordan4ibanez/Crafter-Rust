dofile("lua_libraries/registration/register_block.lua")

crafter.register_biome = function(table_data)
    -- Cache string pointer.
    local mod = current_loading_mod
    table_data.mod = mod

    assert(table_data.name ~= nil, mod .. " IS MISSING A NAME IN ONE OF IT'S BIOMES!")

    crafter.biomes[table_data.name] = table_data
end