use std::env;
use std::fs;
const BLDIR: &str = "/sys/class/backlight";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match &args[1..] {
            [dir,step_size] if dir == "inc" => change_bl(step_size, dir),
            [dir,step_size] if dir == "inc" => change_bl(step_size, dir),
            [case,value] if case == "set" => set_bl(value),
            [case,value] if case == "set" => set_bl(value),
            [dir] if dir == "inc" => change_bl("2", dir),
            [dir] if dir == "dec" => change_bl("2",dir),
            [case] if case == "help" => print_help(),
            _ => ()
        }
    }
}

fn get_device() -> String {
    let dirs = fs::read_dir(BLDIR)
        .expect("Failed to read backglight dir");

    for d in dirs {
        let p: String = d.unwrap().file_name().to_string_lossy().to_string();

        if ["amdgpu_bl0","amdgpu_bl1","acpi_video0"].contains(&p.as_str()) {
            return p;
        }
    }

    String::from("nvidia_0")
}

fn get_max(device: &str) -> u16 {
    let max: u16 = fs::read_to_string(format!("{BLDIR}/{device}/max_brightness"))
        .expect("Failed to read max value")
        .trim()
        .parse()
        .expect("Failed to parse max value");
    max
}

fn get_current(device: &str) -> u16 {
    let current: u16 = fs::read_to_string(format!("{BLDIR}/{device}/brightness"))
        .expect("Failed to read max value")
        .trim()
        .parse()
        .expect("Failed to parse max value");
    current
}

fn calculate_change(current: u16, max: u16, step_size: u16, dir: &str) -> u16 {
    let step: u16 = (max as f32 * (step_size as f32 / 100.0 )) as u16;
    let change: u16 = if dir == "dec" {current.saturating_sub(step)} else {current.saturating_add(step)};

    if change > max {
        max
    } else {
        change
    }

}

fn change_bl(step_size: &str, dir: &str) {
    let step_size: u16 = match step_size.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid step size: use a positive integer");
            return
        },
    };
    let device = get_device();
    let current = get_current(&device);
    let max = get_max(&device);
    let change = calculate_change(current, max, step_size, dir);
    if change != current {
        fs::write(format!("{BLDIR}/{device}/brightness"), format!("{change}")).expect("Couldn't write to brightness file");
    }
}

fn set_bl(val: &str) {
    let val: u16 = match val.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid value: use a positive integer.");
            return
        }
    };
    let device = get_device();
    let current = get_current(&device);
    let max = get_max(&device);
    if (val <= max) & (val != current) {
        fs::write(format!("{BLDIR}/{device}/brightness"), format!("{val}")).expect("Couldn't write to brightness file");
    }
}

fn print_help() {
    let help_str = "blight automatically finds current GPU device, and updates brightness accordingly.\n
blight dec [override step size] - decrease brightness
blight inc [ocerride step size] - increase brightness

Examples:
    blight inc (increases brightness by 2% - default step size)
    blight dec 10 (increases brightness by 10%)
    ";
    println!("{help_str}")
}
