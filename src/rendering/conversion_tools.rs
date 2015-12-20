pub fn vec3_64_to_32(input: [f64; 3]) -> [f32; 3] {
    return [input[0] as f32, input[1] as f32, input[2] as f32];
}


pub fn mat2_64_to_32(input: [[f64; 2]; 2]) -> [[f32; 2]; 2] {
    return [[input[0][0] as f32, input[0][1] as f32],
             [input[1][0] as f32, input[1][1] as f32]];
}
