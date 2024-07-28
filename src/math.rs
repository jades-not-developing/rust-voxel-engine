use nalgebra_glm as glm;

pub fn create_transformation_matrix(
    translation: glm::Vec3,
    rotation: (f32, f32, f32),
    scale: f32,
) -> glm::Mat4 {
    let (rx, ry, rz) = rotation;

    let matrix = glm::Mat4::identity();
    let matrix = glm::translate(&matrix, &translation);
    let matrix = glm::rotate(&matrix, rx.to_radians(), &glm::vec3(1., 0., 0.));
    let matrix = glm::rotate(&matrix, ry.to_radians(), &glm::vec3(0., 1., 0.));
    let matrix = glm::rotate(&matrix, rz.to_radians(), &glm::vec3(0., 0., 1.));
    let matrix = glm::scale(&matrix, &glm::vec3(scale, scale, scale));

    matrix
}
