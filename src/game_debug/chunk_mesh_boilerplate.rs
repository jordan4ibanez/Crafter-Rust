

// this is laid out in this manor because it is easier to debug


/*

function ideas:

intake rotation and pass back texture coordinates for up and down

intake size and generate positions with a function


*/


// generic functions to reduce boilerplate

// pushes the array slice into vector
fn push<T: Copy>(vector: &mut Vec<T>, array: &[T]) {
    array.iter().for_each( | value: &T | {
        vector.push(*value);
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
fn adjust_indices(index: &mut [i32], positions: &Vec<f32>) {
    let adjustment: i32 = ( positions.len() as i32 - 18 ) / 3;
    index.iter_mut().for_each( | value: &mut i32 | {
        *value += adjustment;
    });
}




pub fn face_up(positions: &mut Vec<f32>, indices: &mut Vec<i32>, texture_coordinates: &mut Vec<f32>, colors: &mut Vec<f32>, x: f32, y: f32, z: f32) {

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

    push(positions, &pos);


    // index (face/indices) data

    let mut index: [i32; 6] = [
        // tri 1
        0,1,2,

        // tri 2
        3,4,5
    ];

    adjust_indices(&mut index, &positions);
    
    push(indices, &index);


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

    push(texture_coordinates, &texture);



    // light/color data
    // TODO: intake as a parameter
    let color: [f32; 18] = [

        // tri 1
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,

        // tri 2
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ];

    push(colors, &color);
}




pub fn face_down(positions: &mut Vec<f32>, indices: &mut Vec<i32>, texture_coordinates: &mut Vec<f32>, colors: &mut Vec<f32>, x: f32, y: f32, z: f32) {

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
    
        push(positions, &pos);
    
    
        // index (face/indices) data
    
        let mut index: [i32; 6] = [
            // tri 1
            0,1,2,
    
            // tri 2
            3,4,5
        ];
    
        adjust_indices(&mut index, &positions);
        
        push(indices, &index);
    
    
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
    
        push(texture_coordinates, &texture);
    
    
    
        // light/color data
        // TODO: intake as a parameter
        let color: [f32; 18] = [
    
            // tri 1
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,
    
            // tri 2
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,
        ];
    
        push(colors, &color);
}




pub fn face_south(positions: &mut Vec<f32>, indices: &mut Vec<i32>, texture_coordinates: &mut Vec<f32>, colors: &mut Vec<f32>, x: f32, y: f32, z: f32) {

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

    push(positions, &pos);


    // index (face/indices) data

    let mut index: [i32; 6] = [
        // tri 1
        0,1,2,

        // tri 2
        3,4,5
    ];

    adjust_indices(&mut index, &positions);
    
    push(indices, &index);


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

    push(texture_coordinates, &texture);



    // light/color data
    // TODO: intake as a parameter
    let color: [f32; 18] = [

        // tri 1
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,

        // tri 2
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ];

    push(colors, &color);
}




pub fn face_north(positions: &mut Vec<f32>, indices: &mut Vec<i32>, texture_coordinates: &mut Vec<f32>, colors: &mut Vec<f32>, x: f32, y: f32, z: f32) {
    
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

    push(positions, &pos);


    // index (face/indices) data

    let mut index: [i32; 6] = [
        // tri 1
        0,1,2,

        // tri 2
        3,4,5
    ];

    adjust_indices(&mut index, &positions);
    
    push(indices, &index);


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

    push(texture_coordinates, &texture);



    // light/color data
    // TODO: intake as a parameter
    let color: [f32; 18] = [

        // tri 1
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,

        // tri 2
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ];

    push(colors, &color);
}




pub fn face_west(positions: &mut Vec<f32>, indices: &mut Vec<i32>, texture_coordinates: &mut Vec<f32>, colors: &mut Vec<f32>, x: f32, y: f32, z: f32) {
    
    // vertex data

    let mut pos: [f32; 18] = [

        // tri 1
        0., 0., 0.,
        0., 0., 0.,
        0., 0., 0.,

        // tri 2
        0., 0., 0.,
        0., 0., 0.,
        0., 0., 0.
    ];

    set_pos(&mut pos, x, y, z);

    push(positions, &pos);


    // index (face/indices) data

    let mut index: [i32; 6] = [
        // tri 1
        0,1,2,

        // tri 2
        3,4,5
    ];

    adjust_indices(&mut index, &positions);
    
    push(indices, &index);


    // texture coordinates

    let texture: [f32; 12] = [

        // tri 1
        0., 0.,
        0., 0.,
        0., 0.,

        // tri 2
        0., 0.,
        0., 0.,
        0., 0.
    ];

    push(texture_coordinates, &texture);



    // light/color data
    // TODO: intake as a parameter
    let color: [f32; 18] = [

        // tri 1
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,

        // tri 2
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ];

    push(colors, &color);
}




pub fn face_east(positions: &mut Vec<f32>, indices: &mut Vec<i32>, texture_coordinates: &mut Vec<f32>, colors: &mut Vec<f32>, x: f32, y: f32, z: f32) {
    
    // vertex data

    let mut pos: [f32; 18] = [

        // tri 1
        0., 0., 0.,
        0., 0., 0.,
        0., 0., 0.,

        // tri 2
        0., 0., 0.,
        0., 0., 0.,
        0., 0., 0.
    ];

    set_pos(&mut pos, x, y, z);

    push(positions, &pos);


    // index (face/indices) data

    let mut index: [i32; 6] = [
        // tri 1
        0,1,2,

        // tri 2
        3,4,5
    ];

    adjust_indices(&mut index, &positions);
    
    push(indices, &index);


    // texture coordinates

    let texture: [f32; 12] = [

        // tri 1
        0., 0.,
        0., 0.,
        0., 0.,

        // tri 2
        0., 0.,
        0., 0.,
        0., 0.
    ];

    push(texture_coordinates, &texture);



    // light/color data
    // TODO: intake as a parameter
    let color: [f32; 18] = [

        // tri 1
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,

        // tri 2
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ];

    push(colors, &color);
}