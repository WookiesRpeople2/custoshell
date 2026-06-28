use engine::state::ShellState;
use constants::{COLORS, PROMPT_SECTION, PROMPT_SECTION_PROMPT_COLOR_KEY};
use crossterm::style::Color as CtColor;
use helpers::{
    io::write_config,
    menu::{MenuItem, show_menu},
};

pub fn color(state: &mut ShellState) {
    let items: Vec<MenuItem> = COLORS
        .iter()
        .map(|(name, c)| MenuItem::colored(*name, *c))
        .collect();

    match show_menu(&items) {
        Some(i) => {
            println!("The current color of the shell: {:?}", state.promt_color);
            let (name, _ctcolor) = COLORS[i];
            println!("Color set to {name}");
            write_config(
                PROMPT_SECTION.to_string(),
                PROMPT_SECTION_PROMPT_COLOR_KEY.to_string(),
                name.to_string(),
            );

            state.promt_color = name.to_string();
            assert!(state.promt_color == name.to_string());
        }
        None => println!("Cancelled, no color selected."),
    }
}

pub fn color_from_name(name: &str) -> CtColor {
    COLORS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, c)| *c)
        .unwrap_or(CtColor::White)
}
