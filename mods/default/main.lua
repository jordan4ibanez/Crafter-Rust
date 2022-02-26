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
    block_box = {0.1, 1.2, 2.3432442, 3.222444, 4.555555, 5.95},
    textures = {
        "debug_alpha.png",
    }
})
