use hike_game::actions::Action;

use super::{InputState, GraphicsBackend, SpriteColor};
use super::buttons::Button;

pub struct ModalData {
    pub text: String,
    pub choices: Vec<(String, Option<Box<dyn Action>>)>
}

pub fn draw_modal(
    backend: &dyn GraphicsBackend,
    input_state: &InputState,
    data: &ModalData
) -> Option<usize> {
    let viewport_size = backend.viewport_size();
    let mut clicked = None;

    // for (i, entry) in data.choices.iter().enumerate() {
    //     if Button::new(
    //             viewport_size.x / 2. - 200.,
    //             viewport_size.y / 2. - 50. + i as f32 * 50.,
    //             400.,
    //             40.,
    //         )
    //         .with_color(SpriteColor(0, 0, 0, 0))
    //         .with_text(
    //             &entry.0,
    //             SpriteColor(255, 255, 255, 255),
    //             24
    //         )
    //         .draw(backend)
    //         .clicked(input_state) {
    //             clicked = Some(i)
    //         }
    // }

    clicked
}
