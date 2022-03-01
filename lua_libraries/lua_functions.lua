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

local function automate_and_check_biome_parameters(mod, table_data)
    
    -- Default to 30 if forgotten.
    if table_data.terrain_noise_multiplier == nil then
        table_data.terrain_noise_multiplier = 30
    end

    -- Can only be number.
    assert(type(table_data.terrain_noise_multiplier) == "number", "BIOME " .. mod .. ":" .. table_data.name .. " NEEDS A NUMBER AS terrain_noise_multiplier!")

    -- Default to 0.005 if forgotten.
    if table_data.terrain_frequency == nil then
        table_data.terrain_frequency = 0.005
    end

    assert(type(table_data.terrain_frequency) == "number", "BIOME " .. mod .. ":" .. table_data.name .. " NEEDS A NUMBER AS terrain_frequency!")
    
    -- Default to true.
    if table_data.caves == nil then
        table_data.caves = true
    end

    assert(type(table_data.caves) == "boolean", "BIOME " .. mod .. ":" .. table_data.name .. " NEEDS A BOOLEAN AS caves!")


    -- Check cave parameters.
    
    -- Default to 0.05 min, 0.30 max
    if table_data.cave_heat == nil then
        table_data.cave_heat = {0.05, 0.30}
    end

    assert(type(table_data.cave_heat) == "table", "BIOME " .. mod .. ":" .. table_data.name .. " NEEDS A TABLE AS cave_heat!")

    -- Check type
    for i = 1,2 do
        assert(type(table_data.cave_heat[i]) == "number", "BIOME " .. mod .. ":" .. table_data.name .. " HAS INVALID DATA IN INDEX " .. i .. " OF cave_heat!")
    end

    -- Automate weather.

    if table_data.rain == nil then
        table_data.rain = false
    end

    if table_data.snow == nil then
        table_data.snow = false
    end

end

crafter.register_biome = function(table_data)
    -- Cache string pointer.
    local mod = current_loading_mod
    table_data.mod = mod

    -- Biome needs a name.
    assert(table_data.name ~= nil, mod .. " IS MISSING A NAME IN ONE OF IT'S BIOMES!")

    -- Check all layers.
    check_layers(mod, table_data)

    -- Check and automate biome parameters.
    automate_and_check_biome_parameters(mod, table_data)

    crafter.biomes[table_data.name] = table_data
end