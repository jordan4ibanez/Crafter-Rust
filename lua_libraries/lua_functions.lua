-- Lua Locals.
local 
insert      , dump
=
table.insert, dump

local function cache_texture_to_load(mod, texture_table)

    -- Do not iterate empty or non-existent texture table.
    if texture_table == nil then
        return
    end

    -- Cache table pointer.
    local cache = crafter.texture_cache

    -- Create a texture cache table if non-existent.
    if cache[mod] == nil then
        cache[mod] = {}
    end


    -- Run through textures and see if they need to be cached.
    for _,current_texture in ipairs(texture_table) do

        local found = false

        -- Check against existing textures in all mods.
        for _,mod_table in pairs(cache) do
            for _,cached_texture in ipairs(mod_table) do
                if current_texture == cached_texture then
                    found = true
                end
            end
        end

        if not found then
            insert(cache[mod], current_texture)
        end
    end
end

-- This function repeats the last texture in the table if the table length is less than 6.
local function repeat_texture(texture_table)
    if #texture_table < 6 then
        local repeating_texture = texture_table[#texture_table]
        for i = #texture_table + 1, 6 do
            texture_table[i] = repeating_texture
        end
    end
end

local function check_block_box(block_name, table_data)
    -- Reduce redundant data.
    if table_data.draw_type ~= "block_box" then
        table_data.block_box = nil
    elseif table_data.block_box ~= nil then
        assert(#table_data.block_box >= 6, block_name .. " BLOCK BOX MUST BE AT LEAST 6 POINTS!")
        assert(#table_data.block_box % 6 == 0, block_name .. " MUST HAVE 6 POINTS IN EACH BLOCK SHAPE!")
    end
end

-- This requires the entire table pointer.
local function check_block_rotations(block_name, table_data)

    -- Automate rotations generation so modder does not have to think about it.
    if table_data.rotations == nil then
        table_data.rotations = {0,0,0, 0,0,0}
    -- Check defined data.
    else
        -- Check if 6 points.
        assert(#table_data.rotations == 6 == true, block_name .. " ROTATIONS MUST BE EXACTLY 6 POINTS!")

        -- Floor data just in case a modder goes crazy. Assume correct length
        for i = 1,6 do
            -- Limit the data value. (0 through 3)
            assert(
                table_data.rotations[i] >= 0 and
                table_data.rotations[i] <= 3,
                block_name .. " ROTATION INDEX " .. tostring(i) .. " OUT OF BOUNDS! ROTATIONS ARE LIMITED TO 0 THROUGH 3!"
            )
            table_data.rotations[i] = math.floor(table_data.rotations[i])
        end
    end

end

-- This allows module creators to register blocks easily.
crafter.register_block = function(table_data)
    -- Cache string pointer.
    local mod = current_loading_mod
    table_data.mod = mod


    -- Check that the block_box has 6 points in each shape
    check_block_box(table_data.name, table_data)

    -- Create streamlined texture cache for Rust to work with.
    cache_texture_to_load(mod, table_data.textures)

    -- Automate rotations, rotations check, and rotations data limiter.
    check_block_rotations(table_data.name, table_data)

    --[[
    Automatically repeats the texture.

    This is useful when defining simple blocks like dirt or stone.
    ]]--
    repeat_texture(table_data.textures)

    crafter.blocks[table_data.name] = table_data
end

