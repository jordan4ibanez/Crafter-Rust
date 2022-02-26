-- This allows module creators to register blocks easily.
crafter.register_block = function(table_data)
    table_data.mod = current_loading_mod
    crafter.blocks[table_data.name] = table_data
end