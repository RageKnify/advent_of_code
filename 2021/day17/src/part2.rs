use std::collections::HashSet;
use std::ops::RangeInclusive;

fn calculate_y_steps(mut velocity: i32, target: &RangeInclusive<i32>) -> Vec<usize> {
    let mut steps = 0;
    let mut y = 0;
    let mut res = Vec::new();
    loop {
        if y < *target.start() {
            return res;
        }
        if target.contains(&y) {
            res.push(steps);
        }
        y += velocity;
        velocity -= 1;
        steps += 1;
    }
}

fn calculate_x(steps: usize, target: &RangeInclusive<i32>) -> Vec<i32> {
    fn helper(mut v_x: i32, steps: usize) -> i32 {
        let mut x: i32 = 0;
        for _ in 0..steps {
            if v_x == 0 {
                break;
            }
            x += v_x;
            v_x -= 1;
        }
        x
    }
    let mut res = Vec::new();
    for v_x in (1..1000).rev() {
        let final_x = helper(v_x, steps);
        if target.contains(&final_x) {
            res.push(v_x);
        }
    }
    res
}

fn main() {
    // input = "target area: x=135..155, y=-102..-78"
    let x_range = 135..=155;
    let y_range = -102..=-78;

    let mut solutions = HashSet::new();
    for v_y in (-1000..1000).rev() {
        let step_values = calculate_y_steps(v_y, &y_range);
        for steps in step_values {
            for v_x in calculate_x(steps, &x_range) {
                solutions.insert((v_y, v_x));
            }
        }
    }

    println!("Result = {}", solutions.len());
}
