use rand::Rng;
use std::io;

struct Target {
    start_pos: f64,
    end_pos: f64,
    width: i32,
}

struct Shot {
    x_pos: f64,
    hit_target: bool,
}

fn cannon_game() {
    let target = place_target_zone();

    println!("Welcome to ´Hit the target`!");
    println!("The target zone is placed {}m ahead of you and the zone is {}m wide!", target.start_pos, target.width);
    println!("You have 10 tries to hit the target with your cannon!");
    println!("Start firing!");

    let total_attempts = 10;
    let mut attempt = 0;
    let mut won_game = false;

    while attempt < total_attempts && !won_game {
        attempt += 1;
        println!("Attempt: {}", attempt);

        let shot_power = input_shot_power();
        let shot_angle = input_shot_angle();
        let shot = simulate_ball_trajectory(shot_power, shot_angle, &target);

        println!("The shot landed at {}m", shot.x_pos);
        won_game = shot.hit_target;
        if won_game {
            println!("Congratulations! You hit the target at {}m!", target.start_pos);
        }
        else if !won_game && attempt < total_attempts {
            println!("The target is placed {}m away from you", target.start_pos);
        }
        else if attempt == total_attempts {
            println!("I`m sorry - you lost! You have used up all your attempts!")
        }

    }
}


fn place_target_zone() -> Target {
    let mut rng = rand::thread_rng();

    let max_distance = 1000;
    let target_zone_width = 10;

    let max_start_pos = max_distance - target_zone_width;
    let start_pos = rng.gen_range(10..=max_start_pos) as f64;
    let end_pos = start_pos + target_zone_width as f64;

    Target { start_pos, end_pos, width: target_zone_width }
}

fn input_shot_power() -> f64 {
    loop {
        let mut shot_power_input = String::new();
        println!("Enter shot power in m/s ");
        io::stdin()
            .read_line(&mut shot_power_input)
            .expect("Failed to read shot power");
        if let Ok(shot_power) = shot_power_input.trim().parse::<f64>() {
            if shot_power <= 0.0 {
                println!("Please enter a value larger than 0");
            }
            else {
                return shot_power;
            }
        }
        else {
            println!("Please enter a number")
        }
    }
}

fn input_shot_angle() -> f64 {
    loop {
        let mut shot_angle_input = String::new();
        println!("Enter shot angle in degrees ");
        io::stdin()
            .read_line(&mut shot_angle_input)
            .expect("Failed to read shot angle");
        if let Ok(shot_angle) = shot_angle_input.trim().parse::<f64>() {
            if shot_angle <= 0.0 || shot_angle >= 90.0 {
                println!("Please enter a value larger than 0 and smaller than 90");
            }
            else {
                return shot_angle;
            }
        }
        else {
            println!("Please enter a number")
        }
    }
}

fn simulate_ball_trajectory(shot_power_ms: f64, shot_angle_degree: f64, target: &Target) -> Shot {
    let dt = 0.001; // seconds
    let gravity = 9.81;

    let shot_angle_rad = shot_angle_degree * std::f64::consts::PI / 180.0;
    let v_x = shot_power_ms * shot_angle_rad.sin();
    let mut v_y = shot_power_ms * shot_angle_rad.cos();

    let mut hit_ground = false;
    let mut x_pos = 0.0;
    let mut y_pos = 0.0;

    while !hit_ground {
        x_pos += v_x * dt;
        y_pos += v_y * dt;
        v_y -= gravity * dt;
        hit_ground = y_pos <= 0.0;
    }
    let hit_target = x_pos >= target.start_pos &&  x_pos <= target.end_pos;
    Shot {x_pos, hit_target}
}
