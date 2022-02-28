--[[
crafter.register_block({
    name = "grass",
    draw_type = "normal",
    textures = {
        "grass.png",
        "dirt.png",
        "grass_side.png",
    },
})
]]--

crafter.register_block({
    name = "grass",
    draw_type = "normal",
    textures = {
        "debug_direction.png",
    },
    rotations = {
        0,0,0,0,0,0
    },
    --[[
    flips = {
        0,0,0,0,0,0
    }
    ]]--
})

--[[
crafter.register_block({
    name = "dirt",
    draw_type = "normal",
    textures = {
        "dirt.png",
    },
})

crafter.register_block({
    name = "stone",
    -- draw_type = "block_box",
    draw_type = "normal",
    -- block_box = {0.334343, 0.534654536, 0.44444444      , 1.999999998     , 0.5123123213213     , 1.00000001},
    textures = {
        "stone.png",
    }
})

crafter.register_block({
    name = "cobble",
    draw_type = "normal",
    textures = {
        "cobble.png",
    }
})

crafter.register_block({
    name = "debuggy",
    draw_type = "normal",
    textures = {
        "debug_2.png",
    }
})

]]--