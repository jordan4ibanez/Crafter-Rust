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

-- This allows module creators to register blocks easily.
crafter.register_block = function(table_data)
    -- Cache string pointer.
    local mod = current_loading_mod
    table_data.mod = mod

    -- Create streamlined texture cache for Rust to work with.
    cache_texture_to_load(mod, table_data.textures)

    --[[
    Automatically repeats the texture.

    This is useful when defining simple blocks like dirt or stone.
    ]]--
    repeat_texture(table_data.textures)

    crafter.blocks[table_data.name] = table_data
end

