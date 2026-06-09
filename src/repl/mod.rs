use std::panic;

mod command_handler;
mod depth_checker;
mod input_eval;
mod lines_types;
mod logic_loop;
mod output_render;
mod stdlib_helper;
mod syntax_highlighting;
mod utils;

pub fn start_repl() {
    panic::set_hook(Box::new(|_| {}));

    let mut terminal = ratatui::init();
    let result = logic_loop::run_repl(&mut terminal);
    ratatui::restore();

    if let Err(e) = result {
        eprintln!("repl error: {}", e);
    }
}
