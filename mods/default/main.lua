
crafter.register_block({
    name = "grass",
    draw_type = "normal",
    textures = {
        "grass.png",
        "dirt.png",
        "grass_side.png"
    },
})



crafter.register_block({
    name = "dirt",
    draw_type = "normal",
    textures = {
        "dirt.png",
    },
})


crafter.register_block({
    name = "stone",
    draw_type = "normal",
    textures = {
        "stone.png",
    },
})

crafter.register_block({
    name = "sand",
    draw_type = "normal",
    textures = {
        "sand.png"
    }
})

crafter.register_block({
    name = "coal_ore",
    draw_type = "normal",
    textures = {
        "coal_ore.png",
    },
})

crafter.register_block({
    name = "iron_ore",
    draw_type = "normal",
    textures = {
        "iron_ore.png",
    },
})

crafter.register_block({
    name = "bedrock",
    draw_type = "normal",
    textures = {
        "bedrock.png",
    },
})

crafter.register_biome({
    name = "grass_lands",

    biome_noise_params = {
        heat_min = 0.0,
        heat_max = 1.0,
        -- Multiplies the output of the noise value. 2.0 means -2.0 to 2.0 for your heat values.
        scale = 1.0,
        -- How often the terrain fluctuates.
        frequency = 0.05,
    },

    -- How high or low the terrain can fluctuate.
    terrain_height_flux = 20;


    top_layer = "grass",
    top_layer_depth = {1,1}, -- Min, Max

    bottom_layer = "air",
    bottom_layer_depth = {3,5}, -- Min, Max

    stone_layer = "air",

    bedrock_layer = "bedrock",

    ores = {
        coal_ore = {
            depth = {2, 100},
            heat = {0.61, 0.66},
            frequency = 0.038,
        },
        iron_ore = {
            depth = {2,100},
            heat = {0.66, 0.71},
            frequency = 0.039,
        }
    },


    -- Defines if there is cave generation.
    caves = false,

    -- Cave parameters.
    cave_noise_params = {
        -- Caves will be carved within the min and max.
        heat_min = -5,
        heat_max = -3,
        -- Multiplies the output noise value. 2.0 means -2.0 to 2.0 for your heat values.
        scale = 5.0,
        -- How often cave carving fluctuates.
        frequency = 0.056,
    },

    -- Defines if there is rain.
    rain = true,
})

--[[
crafter.register_biome({
    name = "dessert",

    biome_heat = {-1.0, 0.0},

    top_layer = "sand",
    top_layer_depth = {1,1}, -- Min, Max

    bottom_layer = "sand",
    bottom_layer_depth = {3,5}, -- Min, Max

    stone_layer = "stone",

    bedrock_layer = "bedrock",

    ores = {
        coal_ore = {
            depth = {2, 100},
            heat = {0.61, 0.66},
            frequency = 0.038,
        },
        iron_ore = {
            depth = {2,100},
            heat = {0.66, 0.71},
            frequency = 0.039,
        }
    },

    -- How high or low the terrain can fluctuate.
    terrain_noise_multiplier = 10;

    -- How often the terrain fluctuates.
    terrain_frequency = 0.01,

    -- Defines if there is cave generation.
    caves = true,

    -- Minimum and maximum noise for cave to be carved.
    -- Caves will be carved OUTSIDE of the min and max.
    cave_heat = {-0.8, 0.8},

    -- How often cave carving fluctuates.
    cave_frequency = 0.005,

    -- Defines if there is rain.
    rain = true,
})
]]--