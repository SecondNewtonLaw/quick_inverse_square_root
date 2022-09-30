pub enum AccuracyLevel {
    Low = 1,
    Mid = 2,
    High = 3,
}

impl AccuracyLevel {
    fn get_accuracy(&self) -> i8 {
        match self {
            AccuracyLevel::Low => 1,
            AccuracyLevel::Mid => 2,
            AccuracyLevel::High => 3,
            //_ => panic!("Invalid accuracy level!"),
        }
    }
    pub(crate) fn get_from_number(num: i8) -> AccuracyLevel {
        match num {
            1 => AccuracyLevel::Low,
            2 => AccuracyLevel::Mid,
            3 => AccuracyLevel::High,
            _ => panic!("Invalid integer!"),
        }
    }
}

pub fn q_isqrt(number: f32, accuracy: AccuracyLevel) -> f32 {
    // Set tmp vars.
    let mut tmp_y = number;
    unsafe {
        let mut tmp_i: i32;
        tmp_i = std::mem::transmute(tmp_y); // Evil bit point hack. -> Transmute to get from f32 to i32.
        tmp_i = 0x5f3759df - (tmp_i >> 1); // What the fuck.
        tmp_y = std::mem::transmute(tmp_i); // reverse conversion.
    }
    for _ in 0..(accuracy.get_accuracy()) {
        // 1 is ok, 3 for best accuracy
        let tmp_x2: f32 = number * 0.5;
        const THREE_HALFS: f32 = 1.5;
        tmp_y *= THREE_HALFS - (tmp_x2 * (tmp_y * tmp_y)); // accuracy++
    }
    return tmp_y;
}
