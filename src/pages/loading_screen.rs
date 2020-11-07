use crate::{loading_screen_ghost_texture, wallpaper_index, load_image_scaled, main_menu_wallpaper_path, character_texture_path, load_image_scaled_height, map_background_path, character_height};

use cat_engine::{PagedWindow, Window, WindowPage, MouseScrollDelta, KeyboardButton, MouseButton, ModifiersState, graphics::{
    ColourFilter,
}, texture::{
    Texture,
    ImageBase,
}, window_center, window_width, window_height};

use std::{
    thread::{
        spawn,
        JoinHandle,
    },
};
use cat_engine::image::RgbaImage;
use std::path::PathBuf;
use lib::ImageObject;

const ghost_texture_index:usize=1;
// Текстурные объекты
const ghost_1:usize=1;
const ghost_2:usize=ghost_1+1;

static mut loading:bool=true;

pub struct LoadingScreen{
    frames:usize,
    ghost:usize,
    thread:Option<JoinHandle<Vec<RgbaImage>>>,
    loaded_images:Option<Vec<RgbaImage>>,
}

impl LoadingScreen{
    pub fn new(window:&mut PagedWindow)->LoadingScreen{
        // TODO добавить спрайты
        let image_base=ImageObject::new(unsafe{[
                window_center[0]-100f32,
                window_center[1]-100f32,
                200f32,
                200f32
            ]},
            [
                0f32,
                0f32,
                0.5f32,
                0.5f32
            ],

            [1f32;4]
        );

        let ghost_texture=Texture::from_path(loading_screen_ghost_texture,window.display()).unwrap();

        let graphics=window.graphics2d();

        graphics.add_texture(ghost_texture);

        graphics.add_textured_object(&image_base,1).unwrap();
        graphics.add_textured_object(&image_base,1).unwrap();

        // Создание потока загрузки ресурсов
        let thread=spawn(move|| {
            let mut textures:Vec<RgbaImage>=Vec::with_capacity(10);

            let image_size=unsafe{[window_width as u32,window_height as u32]};

            textures.push(load_image_scaled(main_menu_wallpaper_path,image_size[0],image_size[1]));

            textures.push(load_image_scaled_height(map_background_path,image_size[1]));

            textures.push(load_image_scaled_height(character_texture_path,character_height));

            unsafe{
                loading=false;
            }

            textures
        });

        Self{
            frames:0usize,
            ghost:ghost_1,
            thread:Some(thread),
            loaded_images:None,
        }
    }
}

impl WindowPage<'static> for LoadingScreen{
    type Window = PagedWindow;
    type Output = Vec<RgbaImage>;

    fn on_window_close_requested(&mut self, _window: &mut Self::Window) {

    }

    fn on_update_requested(&mut self, window: &mut Self::Window) {
        if unsafe{!loading} {
            if let Some(thread) = self.thread.take() {
                self.loaded_images=Some(thread.join().expect("Ошибка начальной загрузки"));
            }
            window.stop_events().unwrap();
        }

        if self.frames==10{
            self.ghost=ghost_2;
        }
        else if self.frames==20{
            self.ghost=ghost_1;
            self.frames=0;
        }
        self.frames+=1;
    }

    fn on_redraw_requested(&mut self, window: &mut Self::Window) {
        window.draw(|parameters,graphics|{
            graphics.draw_textured_object(wallpaper_index,ColourFilter::new_mul([1f32;4]),parameters).unwrap();
            graphics.draw_textured_object(self.ghost,ColourFilter::new_mul([1f32;4]),parameters).unwrap();
        }).unwrap();
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
    fn on_file_dropped(&mut self, window: &mut Self::Window, path: PathBuf) {
        unimplemented!()
    }

    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self, window: &mut Self::Window, path: PathBuf) {
        unimplemented!()
    }

    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self, window: &mut Self::Window) {
        unimplemented!()
    }

    fn on_event_loop_closed(&mut self, window: &mut Self::Window) -> Self::Output {
        // Удаление всех текстурных объектов с этой страницы
        window.graphics2d().delete_last_textured_object();
        window.graphics2d().delete_last_textured_object();

        // Удаление текстуры объектов
        window.graphics2d().remove_texture(ghost_texture_index);

        self.loaded_images.take().unwrap()
    }
}