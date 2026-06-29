
use constants::{COLORS, PROMPT_SECTION, PROMPT_SECTION_PROMPT_COLOR_KEY};
use helpers::{
    io::write_config,
    menu::{MenuItem, show_menu},
};

pub fn color() {
    let items: Vec<MenuItem> = COLORS
        .iter()
        .map(|(name, c)| MenuItem::colored(*name, *c))
        .collect();

    match show_menu(&items) {
        Some(i) => {
            let (name, _ctcolor) = COLORS[i];
            println!("Color set to {name}");
            write_config(
                PROMPT_SECTION.to_string(),
                PROMPT_SECTION_PROMPT_COLOR_KEY.to_string(),
                name.to_string(),
            );
        }
        None => println!("Cancelled, no color selected."),
    }
}
