-- Lua Locals.
local insert = table.insert

local function cache_texture_to_load(mod, texture_table)

    -- Do not iterate empty or non-existent texture table.
    if texture_table == nil then
        return
    end

    -- Create a texture cache table if non-existent.
    if crafter.texture_cache[mod] == nil then
        crafter.texture_cache[mod] = {}
    end

    -- Cache table pointer.
    local cache = crafter.texture_cache[mod]

    -- Run through textures and see if they need to be cached.
    for _,current_texture in ipairs(texture_table) do

        local found = false

        -- Check against existing values.
        for _,cached_texture in ipairs(cache) do
            if current_texture == cached_texture then
                found = true
            end
        end

        if not found then
            insert(cache, current_texture)
        end
    end
end

-- This allows module creators to register blocks easily.
crafter.register_block = function(table_data)
    -- Cache string pointer.
    local mod = current_loading_mod
    table_data.mod = mod

    cache_texture_to_load(mod, table_data.textures)

    crafter.blocks[table_data.name] = table_data
end

