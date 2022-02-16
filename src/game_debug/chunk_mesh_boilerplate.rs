

// this is laid out in this manor because it is easier to debug


/*

function ideas:

intake rotation and pass back texture coordinates for up and down

intake size and generate positions with a function


*/


// generic functions to reduce boilerplate

// pushes the array slice into vector
fn assign<T: Copy> (vector: &mut Vec<T>, array: &[T], current_count: &mut i32) {
    array.iter().for_each( | value: &T | {
        vector[*current_count as usize] = *value;
        *current_count += 1;
    });
}

// pushes the adjusted xyz into the vertex data
fn set_pos(pos: &mut [f32], x: f32, y: f32, z: f32) {

    let mut xyz_index: i8 = 0;

    // iterate and modify for xyz values
    pos.iter_mut().for_each( | value: &mut f32 | {
        match xyz_index {
            0 => *value += x,
            1 => *value += y,
            2 => *value += z,
            _ => ()
        }

        xyz_index += 1;

        if xyz_index == 3 {
            xyz_index = 0;
        }
    });
}

// adjusts the indices to the correct value from base
fn adjust_indices(index: &mut [i32], face_count: &mut i32) {

    index.iter_mut().for_each( | value: &mut i32 | {
        *value += *face_count;
    });

    *face_count += 6;
}

// a precalculator for capacity information
pub fn dry_run(pos_count: &mut i32, indice_count: &mut i32, texture_coord_count: &mut i32, colors_count: &mut i32) {
    *pos_count += 18;
    *indice_count += 6;
    *texture_coord_count += 12;
    *colors_count += 18;
}



pub fn face_up(
    positions: &mut Vec<f32>,
    indices: &mut Vec<i32>,
    texture_coordinates: &mut Vec<f32>,
    colors: &mut Vec<f32>,

    pos_count: &mut i32,
    indice_count: &mut i32,
    texture_count: &mut i32,
    color_count: &mut i32,
    face_count: &mut i32,

    x: f32,
    y: f32,
    z: f32,
    light: f32
) {

    // vertex data

    let mut pos: [f32; 18] = [

        // tri 1
        0., 1., 0.,
        0., 1., 1.,
        1., 1., 1.,

        // tri 2
        1., 1., 0.,
        0., 1., 0.,
        1., 1., 1.
    ];

    set_pos(&mut pos, x, y, z);

    assign(positions, &pos, pos_count);

    // index (face/indices) data

    let mut index: [i32; 6] = [
        // tri 1
        0,1,2,

        // tri 2
        3,4,5
    ];

    adjust_indices(&mut index, face_count);
    
    assign(indices, &index, indice_count);

    // texture coordinates

    let texture: [f32; 12] = [

        // tri 1
        0., 0.,
        0., 1.,
        1., 1.,

        // tri 2
        1., 0.,
        0., 0.,
        1., 1.
    ];

    assign(texture_coordinates, &texture, texture_count);



    // light/color data
    // TODO: intake as a parameter
    let color: [f32; 18] = [

        // tri 1
        light, light, light,
        light, light, light,
        light, light, light,

        // tri 2
        light, light, light,
        light, light, light,
        light, light, light,
    ];

    assign(colors, &color, color_count);
}




pub fn face_down(
    positions: &mut Vec<f32>,
    indices: &mut Vec<i32>,
    texture_coordinates: &mut Vec<f32>,
    colors: &mut Vec<f32>,

    pos_count: &mut i32,
    indice_count: &mut i32,
    texture_count: &mut i32,
    color_count: &mut i32,
    face_count: &mut i32,

    x: f32,
    y: f32,
    z: f32,
    light: f32
) {

        // vertex data

        let mut pos: [f32; 18] = [

            // tri 1
            0., 0., 1.,
            0., 0., 0.,
            1., 0., 0.,
    
            // tri 2
            1., 0., 1.,
            0., 0., 1.,
            1., 0., 0.
        ];
    
        set_pos(&mut pos, x, y, z);
    
        assign(positions, &pos, pos_count);
    
    
        // index (face/indices) data
    
        let mut index: [i32; 6] = [
            // tri 1
            0,1,2,
    
            // tri 2
            3,4,5
        ];
    
        adjust_indices(&mut index, face_count);
        
        assign(indices, &index, indice_count);
    
    
        // texture coordinates
    
        let texture: [f32; 12] = [
    
            // tri 1
            0., 1.,
            0., 0.,
            1., 0.,
    
            // tri 2
            1., 1.,
            0., 1.,
            1., 0.
        ];
    
        assign(texture_coordinates, &texture, texture_count);
    
    
    
        // light/color data
        // TODO: intake as a parameter
        let color: [f32; 18] = [
    
            // tri 1
            light, light, light,
            light, light, light,
            light, light, light,

            // tri 2
            light, light, light,
            light, light, light,
            light, light, light,
        ];
    
        assign(colors, &color, color_count);
}




pub fn face_south(
    positions: &mut Vec<f32>,
    indices: &mut Vec<i32>,
    texture_coordinates: &mut Vec<f32>,
    colors: &mut Vec<f32>,

    pos_count: &mut i32,
    indice_count: &mut i32,
    texture_count: &mut i32,
    color_count: &mut i32,
    face_count: &mut i32,

    x: f32,
    y: f32,
    z: f32,
    light: f32
) {

    // vertex data

    let mut pos: [f32; 18] = [

        // tri 1
        0., 1., 1.,
        0., 0., 1.,
        1., 0., 1.,

        // tri 2
        1., 1., 1.,
        0., 1., 1.,
        1., 0., 1.
    ];

    set_pos(&mut pos, x, y, z);

    assign(positions, &pos, pos_count);


    // index (face/indices) data

    let mut index: [i32; 6] = [
        // tri 1
        0,1,2,

        // tri 2
        3,4,5
    ];

    adjust_indices(&mut index, face_count);
    
    assign(indices, &index, indice_count);


    // texture coordinates

    let texture: [f32; 12] = [

        // tri 1
        0., 1.,
        0., 0.,
        1., 0.,

        // tri 2
        1., 1.,
        0., 1.,
        1., 0.
    ];

    assign(texture_coordinates, &texture, texture_count);



    // light/color data
    // TODO: intake as a parameter
    let color: [f32; 18] = [

        // tri 1
        light, light, light,
        light, light, light,
        light, light, light,

        // tri 2
        light, light, light,
        light, light, light,
        light, light, light,
    ];

    assign(colors, &color, color_count);
}




pub fn face_north(
    positions: &mut Vec<f32>,
    indices: &mut Vec<i32>,
    texture_coordinates: &mut Vec<f32>,
    colors: &mut Vec<f32>,

    pos_count: &mut i32,
    indice_count: &mut i32,
    texture_count: &mut i32,
    color_count: &mut i32,
    face_count: &mut i32,

    x: f32,
    y: f32,
    z: f32,
    light: f32
) {
    
    // vertex data

    let mut pos: [f32; 18] = [

        // tri 1
        0., 0., 0.,
        0., 1., 0.,
        1., 1., 0.,

        // tri 2
        1., 0., 0.,
        0., 0., 0.,
        1., 1., 0.
    ];

    set_pos(&mut pos, x, y, z);

    assign(positions, &pos, pos_count);


    // index (face/indices) data

    let mut index: [i32; 6] = [
        // tri 1
        0,1,2,

        // tri 2
        3,4,5
    ];

    adjust_indices(&mut index, face_count);
    
    assign(indices, &index, indice_count);


    // texture coordinates

    let texture: [f32; 12] = [

        // tri 1
        0., 0.,
        0., 1.,
        1., 1.,

        // tri 2
        1., 0.,
        0., 0.,
        1., 1.
    ];

    assign(texture_coordinates, &texture, texture_count);



    // light/color data
    // TODO: intake as a parameter
    let color: [f32; 18] = [

        // tri 1
        light, light, light,
        light, light, light,
        light, light, light,

        // tri 2
        light, light, light,
        light, light, light,
        light, light, light,
    ];

    assign(colors, &color, color_count);
}




pub fn face_west(
    positions: &mut Vec<f32>,
    indices: &mut Vec<i32>,
    texture_coordinates: &mut Vec<f32>,
    colors: &mut Vec<f32>,

    pos_count: &mut i32,
    indice_count: &mut i32,
    texture_count: &mut i32,
    color_count: &mut i32,
    face_count: &mut i32,

    x: f32,
    y: f32,
    z: f32,
    light: f32
) {
    
    // vertex data

    let mut pos: [f32; 18] = [

        // tri 1
        1., 0., 1.,
        1., 0., 0.,
        1., 1., 0.,

        // tri 2
        1., 1., 1.,
        1., 0., 1.,
        1., 1., 0.
    ];

    set_pos(&mut pos, x, y, z);

    assign(positions, &pos, pos_count);


    // index (face/indices) data

    let mut index: [i32; 6] = [
        // tri 1
        0,1,2,

        // tri 2
        3,4,5
    ];

    adjust_indices(&mut index, face_count);
    
    assign(indices, &index, indice_count);


    // texture coordinates

    let texture: [f32; 12] = [

        // tri 1
        0., 0.,
        0., 1.,
        1., 1.,

        // tri 2
        1., 0.,
        0., 0.,
        1., 1.
    ];

    assign(texture_coordinates, &texture, texture_count);



    // light/color data
    // TODO: intake as a parameter
    let color: [f32; 18] = [

        // tri 1
        light, light, light,
        light, light, light,
        light, light, light,

        // tri 2
        light, light, light,
        light, light, light,
        light, light, light,
    ];

    assign(colors, &color, color_count);
}




pub fn face_east(
    positions: &mut Vec<f32>,
    indices: &mut Vec<i32>,
    texture_coordinates: &mut Vec<f32>,
    colors: &mut Vec<f32>,

    pos_count: &mut i32,
    indice_count: &mut i32,
    texture_count: &mut i32,
    color_count: &mut i32,
    face_count: &mut i32,

    x: f32,
    y: f32,
    z: f32,
    light: f32
) {
    
    // vertex data

    let mut pos: [f32; 18] = [

        // tri 1
        0., 0., 0.,
        0., 0., 1.,
        0., 1., 1.,

        // tri 2
        0., 1., 0.,
        0., 0., 0.,
        0., 1., 1.
    ];

    set_pos(&mut pos, x, y, z);

    assign(positions, &pos, pos_count);


    // index (face/indices) data

    let mut index: [i32; 6] = [
        // tri 1
        0,1,2,

        // tri 2
        3,4,5
    ];

    adjust_indices(&mut index, face_count);
    
    assign(indices, &index, indice_count);


    // texture coordinates

    let texture: [f32; 12] = [

        // tri 1
        0., 1.,
        0., 0.,
        1., 0.,

        // tri 2
        1., 1.,
        0., 1.,
        1., 0.
    ];

    assign(texture_coordinates, &texture, texture_count);



    // light/color data
    // TODO: intake as a parameter
    let color: [f32; 18] = [

        // tri 1
        light, light, light,
        light, light, light,
        light, light, light,

        // tri 2
        light, light, light,
        light, light, light,
        light, light, light,
    ];

    assign(colors, &color, color_count);
}

// the packed boilerplate to allow a single function call
pub fn add_block(
    positions: &mut Vec<f32>,
    indices: &mut Vec<i32>,
    texture_coordinates: &mut Vec<f32>,
    colors: &mut Vec<f32>,

    pos_count: &mut i32,
    indice_count: &mut i32,
    texture_count: &mut i32,
    color_count: &mut i32,
    face_count: &mut i32,

    x: f32,
    y: f32,
    z: f32,
    light: f32
) {
    face_up(
        positions,
        indices,
        texture_coordinates,
        colors,

        pos_count,
        indice_count,
        texture_count,
        color_count,
        face_count,

        x,
        y,
        z,
        light
    );

    face_down(
        positions,
        indices,
        texture_coordinates,
        colors,

        pos_count,
        indice_count,
        texture_count,
        color_count,
        face_count,

        x,
        y,
        z,
        light
    );

    face_south(
        positions,
        indices,
        texture_coordinates,
        colors,

        pos_count,
        indice_count,
        texture_count,
        color_count,
        face_count,

        x,
        y,
        z,
        light
    );


    face_north(
        positions,
        indices,
        texture_coordinates,
        colors,

        pos_count,
        indice_count,
        texture_count,
        color_count,
        face_count,

        x,
        y,
        z,
        light
    );


    face_west(
        positions,
        indices,
        texture_coordinates,
        colors,

        pos_count,
        indice_count,
        texture_count,
        color_count,
        face_count,

        x,
        y,
        z,
        light
    );


    face_east(
        positions,
        indices,
        texture_coordinates,
        colors,

        pos_count,
        indice_count,
        texture_count,
        color_count,
        face_count,

        x,
        y,
        z,
        light
    );
}