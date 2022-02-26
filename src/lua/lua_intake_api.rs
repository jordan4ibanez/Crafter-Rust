use mlua::{Lua, Table};

use crate::{blocks::blocks::BlockComponentSystem, graphics::mesh_component_system::MeshComponentSystem};


pub fn intake_api_values(lua: &Lua, mcs: &mut MeshComponentSystem, bcs: &mut BlockComponentSystem) {

    // this follows the same pattern as lua
    let crafter: Table = lua.globals().raw_get("crafter").unwrap();
    let blocks: Table = crafter.get("blocks").unwrap();
    let texture_cache: Table = crafter.get("texture_cache").unwrap();


    println!("-------BEGINNING TEST OF API TRANSLATION ------------");

    // this is done imperatively because it's easier to understand the program flow

    let mut cached_table_values: Vec<(String, String)> = Vec::new();

    // iterate the base of the texture cache table - crafter.texture_cache
    for module_name in texture_cache.pairs::<String, Table>() {

        // the module_name_unwrapped.0 is the module name
        let module_name_unwrapped: (String, Table) = module_name.unwrap();

        println!("{}", module_name_unwrapped.0);

        // iterate through textures in module table - crafter.texture_cache[texture]
        for texture_name in module_name_unwrapped.1.pairs::<u32, String>() {
            // 0 - index | 1 - texture name (.png)
            let texture_name_unwrapped: (u32, String) = texture_name.unwrap();

            println!("{} | {}", texture_name_unwrapped.0, texture_name_unwrapped.1);

            // insert the values into the vector tuple

            cached_table_values.push((module_name_unwrapped.0.clone(), texture_name_unwrapped.1));

        }
    }

    println!("{:#?}", cached_table_values);

    // (number of textures, biggest_width, biggest height)
    // let config_values: (u32, u32, u32) = configure_texture_atlas();
    


    // iterate blocks to be put into Block Component System

    // iterating crafter.blocks
    /*
    for blocks in blocks.pairs::<String, Table>() {

        let unwrapped_blocks: (String, Table) = blocks.unwrap();




    }
    */
}