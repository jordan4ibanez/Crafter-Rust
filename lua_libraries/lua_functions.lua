dofile("lua_libraries/registration/register_block.lua")

local function check_layers(mod, table_data)
    -- Biome needs all layers.

    -- Top layer.
    assert(table_data.top_layer ~= nil and type(table_data.top_layer) == "string", "BIOME " .. mod .. ":" .. table_data.name .. " NEEDS TOP LAYER DEFINED!")
    
    assert(table_data.top_layer_depth ~= nil and type(table_data.top_layer_depth) == "table", "BIOME " .. mod .. ":" .. table_data.name .. " NEEDS TOP LAYER DEPTH DEFINED!")

    for i = 1,2 do
        assert(type(table_data.top_layer_depth[i]) == "number", "BIOME " .. mod .. ":" .. table_data.name .. " INVALID DATA IN INDEX " .. i .. "OF TOP LAYER DEPTH")
    end

    -- Bottom layer.
    assert(table_data.bottom_layer ~= nil and type(table_data.bottom_layer) == "string", "BIOME " .. mod .. ":" .. table_data.name .. " NEEDS BOTTOM LAYER DEFINED!")
    
    assert(table_data.bottom_layer_depth ~= nil and type(table_data.bottom_layer_depth) == "table", "BIOME " .. mod .. ":" .. table_data.name .. " NEEDS BOTTOM LAYER DEPTH DEFINED!")

    for i = 1,2 do
        assert(type(table_data.bottom_layer_depth[i]) == "number", "BIOME " .. mod .. ":" .. table_data.name .. " INVALID DATA IN INDEX " .. i .. "OF BOTTOM LAYER DEPTH!")
    end

    -- Stone layer.
    assert(table_data.stone_layer ~= nil and type(table_data.stone_layer) == "string", "BIOME " .. mod .. ":" .. table_data.name .. " NEEDS STONE LAYER DEFINED!")
end

crafter.register_biome = function(table_data)
    -- Cache string pointer.
    local mod = current_loading_mod
    table_data.mod = mod

    -- Biome needs a name.
    assert(table_data.name ~= nil, mod .. " IS MISSING A NAME IN ONE OF IT'S BIOMES!")

    -- Check all layers.
    check_layers(mod, table_data)

    crafter.biomes[table_data.name] = table_data
end