use std::ops::RangeInclusive;

fn calculate_y_steps(mut velocity: i32, target: &RangeInclusive<i32>) -> Option<usize> {
    let mut steps = 0;
    let mut y = 0;
    loop {
        if y < *target.start() {
            return None;
        }
        if target.contains(&y) {
            return Some(steps);
        }
        y += velocity;
        velocity -= 1;
        steps += 1;
    }
}

fn calculate_x(steps: usize, target: &RangeInclusive<i32>) -> Option<i32> {
    fn helper(mut v_x: usize, steps: usize) -> i32 {
        let mut x: i32 = 0;
        for _ in 0..steps {
            if v_x == 0 {
                break;
            }
            x += v_x as i32;
            v_x -= 1;
        }
        x
    }
    for v_x in 1..20 {
        let final_x = helper(v_x, steps);
        if target.contains(&final_x) {
            return Some(v_x as i32);
        }
    }
    None
}

fn calculate_max_y(mut v_y: i32) -> i32 {
    let mut prev_y = 0;
    let mut y = 0;
    loop {
        if prev_y > y {
            return prev_y;
        }
        prev_y = y;
        y += v_y;
        v_y -= 1;
    }
}

fn main() {
    // input = "target area: x=135..155, y=-102..-78"
    let x_range = 135..=155;
    let y_range = -102..=-78;

    for v_y in (0..200).rev() {
        if let Some(steps) = calculate_y_steps(v_y, &y_range) {
            if let Some(_v_x) = calculate_x(steps, &x_range) {
                let max_y = calculate_max_y(v_y);
                println!("Result = {}", max_y);
                return;
            }
        }
    }

    println!("Failed :(");
}
