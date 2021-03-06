use crate::components::map::{Map, Position};
use crate::components::terminal::Symbol;
use bevy::prelude::{Color, Query, Res};
use bevy_ascii_terminal::CharFormat;
use bevy_ascii_terminal::Terminal;

pub(crate) fn update_position(
    map: Res<Map>,
    q: Query<(&Position, &Symbol)>,
    mut term_q: Query<&mut Terminal>,
) {
    let mut terminal = term_q.get_single_mut().unwrap();
    terminal.clear();

    // draw the map first
    map.render(terminal.as_mut());

    // draw entities over it
    for (pos, symbol) in q.iter() {
        if terminal.is_in_bounds([pos.x as i32, pos.y as i32]) {
            terminal.put_char_formatted(
                [pos.x as i32, pos.y as i32],
                symbol.0,
                CharFormat::new(Color::WHITE, Color::BLACK),
            );
        }
    }
}
