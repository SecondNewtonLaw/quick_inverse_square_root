mod math;

use math::*;

fn main() {
    let isqrt_non_quake = 1f32 / (69420 as f32).sqrt();

    println!(
        "Inverse Root Square of 69420 is (std lib stuff): {}",
        isqrt_non_quake,
    );

    for accuracy in 1..4 {
        let isqrt_quakeish: f32 =
            q_isqrt(69420 as f32, math::AccuracyLevel::get_from_number(accuracy));

        println!(
            "Inverse Root Square of 69420 is (quake III arena stuff) (Accuracy level {}): {}",
            accuracy, isqrt_quakeish,
        );
    }
}
