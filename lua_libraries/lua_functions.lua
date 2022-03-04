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

    -- Bedrock layer.
    assert(table_data.bedrock_layer ~= nil and type(table_data.bedrock_layer) == "string", "BIOME " .. mod .. ":" .. table_data.name .. " NEEDS BEDROCK LAYER DEFINED!")
end

local function automate_and_check_biome_parameters(mod, table_data)
    
    -- Force biome heat.
    assert(table_data.biome_heat ~= nil, mod .. ":" .. table_data.name .. " NEEDS A biome_heat DEFINITION!")
    assert(type(table_data.biome_heat) == "table", mod .. ":" .. table_data.name .. " NEEDS A 2 ELEMENT TABLE AS biome_heat!")
    assert(#table_data.biome_heat == 2, mod .. ":" .. table_data.name .. " NEEDS 2 ELEMENTS AS biome_heat!")

    for i = 1,2 do
        assert(type(table_data.biome_heat[i]) == "number", mod .. ":" .. table_data.name .. " HAS INVALID DATA IN biome_heat IN ELEMENT " .. i .. "!")
    end


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


local function check_biome_ores(mod, table_data)

    if table_data.ores == nil then
        return
    end

    -- Table data pointer.
    local ores = table_data.ores

    for name, data in pairs(ores) do
        
        -- Make sure depth exists and is correct.
        assert(data.depth ~= nil, mod .. ":" .. table_data.name .. " IS MISSING depth FOR ORE " .. name .. "!")
        assert(type(data.depth) == "table", mod .. ":" .. table_data.name .. " HAS INCORRECT DATA FOR depth FOR ORE " .. name .. "!")
        assert(#data.depth == 2, mod .. ":" .. table_data.name .. " SHOULD HAVE TABLE LENGTH 2 IN depth FOR ORE " .. name .. "!")

        for i = 1,2 do
            assert(type(data.depth[i]) == "number", mod .. ":" .. table_data.name .. " HAS INVALID DATA IN depth IN INDEX " .. i .. "!")
        end


        -- Make sure heat exists and is correct.
        assert(data.heat ~= nil, mod .. ":" .. table_data.name .. " IS MISSING heat FOR ORE " .. name .. "!")
        assert(type(data.heat) == "table", mod .. ":" .. table_data.name .. " HAS INCORRECT DATA FOR heat FOR ORE " .. name .. "!")
        assert(#data.heat == 2, mod .. ":" .. table_data.name .. " SHOULD HAVE TABLE LENGTH 2 IN heat FOR ORE " .. name .. "!")

        for i = 1,2 do
            assert(type(data.heat[i]) == "number", mod .. ":" .. table_data.name .. " HAS INVALID DATA IN heat IN INDEX " .. i .. "!")
        end

        -- Make sure frequency exists and is correct.
        assert(data.frequency ~= nil, mod .. ":" .. table_data.name .. " IS MISSING frequency FOR ORE " .. name .. "!")
        assert(type(data.frequency) == "number", mod .. ":" .. table_data.name .. " HAS INCORRECT DATA FOR frequency FOR ORE " .. name .. "!")

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

    -- Check cave generation frequency parameters.
    assert(table_data.cave_frequency ~= nil, mod .. ":" .. table_data.name .. " IS MISSING cave_frequency!")
    assert(type(table_data.cave_frequency) == "number", mod .. ":" .. table_data.name .. " HAS INCORRECT DATA FOR cave_frequency! NEEDS A NUMBER!")

    -- Check biome ore definition is correct.
    check_biome_ores(mod, table_data)

    crafter.biomes[table_data.name] = table_data
end











-- Make sure that all biomes contain valid blocks.
function double_check_biome_blocks(mod, name, biome, blocks)

    local layers = {["TOP"] = biome.top_layer, ["BOTTOM"] = biome.bottom_layer, ["STONE"] = biome.stone_layer, ["BEDROCK"] = biome.bedrock_layer}

    for layer_name,defined_name in pairs(layers) do

        local found = false;
        for block_name,_ in pairs(blocks) do
            -- Positive check lock.
            if defined_name == block_name then
                found = true
            end
        end

        assert(found == true, "BIOME " .. mod .. ":" .. name .. " CONTAINS AN UNDEFINED BLOCK: " .. defined_name .. " IN " .. layer_name .. " LAYER!")
    end
end

-- Make sure that all biomes contain valid ores.
function double_check_biome_ores(mod, name, ores, blocks)
    -- No ore defined. Do nothing.
    if ores == nil then
        return
    end

    assert(type(ores) == "table", mod .. ":" .. name .. " HAS THE INCORRECT TYPE OF DATA AS ores! REQUIRED: table, PROVIDED: " .. type(ores) .. "!")

    for ore_block,_ in pairs(ores) do

        local found = false;
        for block_name,_ in pairs(blocks) do
            -- Positive check lock.
            if ore_block == block_name then
                found = true
            end
        end

        assert(found == true, "BIOME " .. mod .. ":" .. name .. " CONTAINS AN UNDEFINED ORE: " .. ore_block .. "!")
    end
end