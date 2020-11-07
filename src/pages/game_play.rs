use cat_engine::{WindowPage, MouseScrollDelta, KeyboardButton, ModifiersState, MouseButton, PagedWindow, Window, window_center, window_height, window_rect};
use cat_engine::image::RgbaImage;
use cat_engine::texture::{Texture, ImageBase};
use crate::{map_background_index, character_image_index};
use lib::colours::White;
use cat_engine::graphics::ColourFilter;

pub struct GamePlay{
    //character:Character,
    //map:Map,

}

const map_background_texture_index:usize=1;
// (текстурного объекта)
const map_background1_index:usize=0;

const character_texture_index:usize=1;
// Индекс персонажа (текстурного объекта)
const character_index:usize=1;



impl GamePlay {
    pub fn new(textures:&Vec<RgbaImage>,window:&mut PagedWindow)->GamePlay{
        // Текстуры карты
        let map_background=ImageBase::new(White,window_rect());
        let background_texture=Texture::from_image(&textures[map_background_index],window.display()).unwrap();
        window.graphics2d().add_texture(background_texture);
        window.graphics2d().add_textured_object(&map_background,map_background_texture_index).unwrap();


        let character_image=&textures[character_image_index];
        let rect={
            let size=character_image.dimensions();
            let size=[size.0 as f32,size.1 as f32];

            unsafe{[
                window_center[0]-size[0],
                window_height,
                size[0],
                size[1]
            ]}
        };
        let character=ImageBase::new(White,rect);

        let character_texture=Texture::from_image(character_image,window.display()).unwrap();
        window.graphics2d().add_texture(character_texture);
        window.graphics2d().add_textured_object(&character,character_texture_index).unwrap();

        Self{

        }
    }
}

impl WindowPage<'static> for GamePlay{
    type Window = PagedWindow;
    type Output = ();

    fn on_window_close_requested(&mut self, _window: &mut Self::Window) {
    }

    fn on_update_requested(&mut self, _window: &mut Self::Window) {
    }

    fn on_redraw_requested(&mut self, window: &mut Self::Window) {
        window.draw(|parameters, graphics|{
            graphics.draw_textured_object(character_index,ColourFilter::new_mul([1f32;4]),parameters).unwrap();
        });
    }

    fn on_mouse_pressed(&mut self, _window: &mut Self::Window, _button: MouseButton) {
    }

    fn on_mouse_released(&mut self, _window: &mut Self::Window, _button: MouseButton) {
    }

    fn on_mouse_scrolled(&mut self, _window: &mut Self::Window, _scroll: MouseScrollDelta) {
    }

    fn on_mouse_moved(&mut self, _window: &mut Self::Window, _position: [f32; 2]) {
    }

    fn on_keyboard_pressed(&mut self, _window: &mut Self::Window, _button: KeyboardButton) {
    }

    fn on_keyboard_released(&mut self, _window: &mut Self::Window, _button: KeyboardButton) {
    }

    fn on_character_recieved(&mut self, _window: &mut Self::Window, _character: char) {
    }

    fn on_window_resized(&mut self, _window: &mut Self::Window, _new_size: [u32; 2]) {
    }

    fn on_window_moved(&mut self, _window: &mut Self::Window, _position: [i32; 2]) {
    }

    fn on_window_focused(&mut self, _window: &mut Self::Window, _focused: bool) {
    }

    fn on_suspended(&mut self, _window: &mut Self::Window) {
    }

    fn on_resumed(&mut self, _window: &mut Self::Window) {
    }

    fn on_modifiers_changed(&mut self, _window: &mut Self::Window, _modifiers: ModifiersState) {
    }

    #[cfg(feature="file_drop")]
    fn on_file_dropped(&mut self, _window: &mut Self::Window, _path: PathBuf) {
    }
    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self, _window: &mut Self::Window, _path: PathBuf) {
    }
    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self, _window: &mut Self::Window) {
    }

    fn on_event_loop_closed(&mut self, _window: &mut Self::Window) -> Self::Output {
    }
}