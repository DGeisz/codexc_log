use std::env;

fn base_check(level: usize, check_type: String) -> bool {
    let index_option = env::args().position(|arg| arg == check_type);

    if let Some(index) = index_option {
        let env_args = env::args().collect::<Vec<String>>();
        let true_level_option = env_args.get(index + 1);

        if let Some(true_level_string) = true_level_option {
            let true_level = true_level_string.parse::<usize>().unwrap();

            true_level >= level
        } else {
            false
        }
    } else {
        false
    }
}

pub fn init(level: usize) -> bool {
    base_check(level, "init".into())
}

pub fn init_connections(level: usize) -> bool {
    base_check(level, "init_connections".into())
}

pub fn run(level: usize) -> bool {
    base_check(level, "run".into())
}

pub fn run_state(level: usize) -> bool {
    base_check(level, "run_state".into())
}

pub fn output(level: usize) -> bool {
    base_check(level, "output".into())
}

pub fn output_state(level: usize) -> bool {
    base_check(level, "output_state".into())
}
