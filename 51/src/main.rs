use f16::F16;  // assuming your crate is named f16

fn main() {
    let my_f16 = F16::from_f32(12.5_f32);
    println!("The f16 value is: {}", my_f16);
    let back_to_f32: f32 = my_f16.to_f32();
    println!("The converted f32 value is: {}", back_to_f32);
    let result = my_f16 * F16::from_f32(2.0_f32);
    println!("The result of multiplication is: {}", result);
}

