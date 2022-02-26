crafter.register_block({
    name = "dirt",
    draw_type = "normal",
    textures = {
        "dirt.png",
        "dirt.png",
        "dirt.png",
        "dirt.png",
        "dirt.png",
        "dirt.png",
    }
})

crafter.register_block({
    name = "stone",
    draw_type = "block_box",
    block_box = {0.0, 1.0, 2.0, 3.0, 4.0, 5.0},
    textures = {
        "debug_alpha.png",
    }
})
