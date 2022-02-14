use cgmath::Matrix4;

// Remember that cgmath is basically just joml

// I will implement ortholinear rendering later
pub struct Transformation {
    projection_matrix: Matrix4<f64>,
    model_matrix: Matrix4<f64>,
    view_matrix: Matrix4<f64>,
}




pub fn new() -> Transformation {
    let t = Transformation {
        projection_matrix: Matrix4::<f64>::from_scale(0.0),
        model_matrix: Matrix4::<f64>::from_scale(0.0),
        view_matrix: Matrix4::<f64>::from_scale(0.0)
    };



    for x in 0..4 {
        for y in 0..4 {
            println!("{}", t.projection_matrix[x][y]);
        }
    }

    t
}