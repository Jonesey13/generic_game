pub fn vec3_64_to_32(input: [f64; 3]) -> [f32; 3] {
    return [input[0] as f32, input[1] as f32, input[2] as f32];
}

#[allow(dead_code)]
pub fn vec4_64_to_32(input: [f64; 4]) -> [f32; 4] {
    return [input[0] as f32, input[1] as f32, input[2] as f32, input[3] as f32];
}

pub fn mat2_64_to_32(input: [[f64; 2]; 2]) -> [[f32; 2]; 2] {
    return [[input[0][0] as f32, input[0][1] as f32],
             [input[1][0] as f32, input[1][1] as f32]];
}

pub fn mat4_64_to_32(input: [[f64; 4]; 4]) -> [[f32; 4]; 4] {
    return [[input[0][0] as f32, input[0][1] as f32, input[0][2] as f32, input[0][3] as f32],
            [input[1][0] as f32, input[1][1] as f32, input[1][2] as f32, input[1][3] as f32],
            [input[2][0] as f32, input[2][1] as f32, input[2][2] as f32, input[2][3] as f32],
             [input[3][0] as f32, input[3][1] as f32, input[3][2] as f32, input[3][3] as f32]];
}
