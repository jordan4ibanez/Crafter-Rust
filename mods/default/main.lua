
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


crafter.register_biome({
    name = "grass_lands",

    top_layer = "grass",
    top_layer_depth = {1,1},

    bottom_layer = "dirt",
    bottom_layer_depth = {3,5},

    stone_layer = "stone",

    ores = {
        coal = {
            depth = {2, 100},
            frequency = 50,
        },
        iron = {
            depth = {2,100},
            frequency = 40,
        }
    },

    -- How high or low the terrain can fluctuate.
    terrain_noise_multiplier = 30;

    -- How often the terrain fluctuates.
    terrain_frequency = 0.005,

    caves = true,

    -- Minimum and maximum noise for cave to be carved.
    cave_heat = {0.05, 0.30},

    rain = true,
})