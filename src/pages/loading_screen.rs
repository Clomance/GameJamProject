use crate::{
    audio_menu_name,
    audio_menu_path,
    loading_screen_ghost_texture,
    wallpaper_index,
};

use cat_engine::{
    PagedWindow,
    Window,
    WindowPage,
    MouseScrollDelta,
    KeyboardButton,
    MouseButton,
    ModifiersState,

    graphics::{
        ColourFilter,
    },

    texture::{
        Texture,
        ImageBase,
    },
    window_center,
};

use std::{
    thread::{
        spawn,
        JoinHandle,
    },
    path::PathBuf,
};

const ghost_1:usize=1;
const ghost_2:usize=ghost_1+1;

static mut loading:bool=true;

pub struct LoadingScreen{
    frames:usize,
    ghost:usize,
    thread:Option<JoinHandle<()>>,
}

impl LoadingScreen{
    pub fn new(window:&mut PagedWindow)->LoadingScreen{
        // TODO добавить спрайты
        let image_base=ImageBase::new([1f32;4],unsafe{[
            window_center[0]-100f32,
            window_center[1]-100f32,
            200f32,
            200f32
        ]});

        let ghost_texture=Texture::from_path(loading_screen_ghost_texture,window.display()).unwrap();

        let graphics=window.graphics2d();

        graphics.add_texture(ghost_texture);

        graphics.add_textured_object(&image_base,ghost_1).unwrap();
        graphics.add_textured_object(&image_base,ghost_2).unwrap();

        // Создание потока загрузки ресурсов
        let thread=spawn(move|| {
            unsafe{
                loading=false;
            }
        });

        Self{
            frames:0usize,
            ghost:ghost_1,
            thread:Some(thread),
        }
    }
}

impl WindowPage<'static> for LoadingScreen{
    type Window = PagedWindow;
    type Output = ();

    fn on_window_close_requested(&mut self, window: &mut Self::Window) {

    }

    fn on_update_requested(&mut self, window: &mut Self::Window) {
        if unsafe{!loading} {
            if let Some(thread) = self.thread.take() {
                thread.join().expect("Ошибка начальной загрузки");
            }
            window.stop_events();
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
        });
    }

    fn on_mouse_pressed(&mut self, window: &mut Self::Window, button: MouseButton) {

    }

    fn on_mouse_released(&mut self, window: &mut Self::Window, button: MouseButton) {

    }

    fn on_mouse_scrolled(&mut self, window: &mut Self::Window, scroll: MouseScrollDelta) {

    }

    fn on_mouse_moved(&mut self, window: &mut Self::Window, position: [f32; 2]) {

    }

    fn on_keyboard_pressed(&mut self, window: &mut Self::Window, button: KeyboardButton) {

    }

    fn on_keyboard_released(&mut self, window: &mut Self::Window, button: KeyboardButton) {

    }

    fn on_character_recieved(&mut self, window: &mut Self::Window, character: char) {

    }

    fn on_window_resized(&mut self, window: &mut Self::Window, new_size: [u32; 2]) {

    }

    fn on_window_moved(&mut self, window: &mut Self::Window, position: [i32; 2]) {

    }

    fn on_window_focused(&mut self, window: &mut Self::Window, focused: bool) {

    }

    fn on_suspended(&mut self, window: &mut Self::Window) {

    }

    fn on_resumed(&mut self, window: &mut Self::Window) {

    }

    fn on_modifiers_changed(&mut self, window: &mut Self::Window, modifiers: ModifiersState) {
        unimplemented!()
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
        // Удаление всех текстурных объектов (но не их текстур)
        window.graphics2d().delete_last_textured_object();
        window.graphics2d().delete_last_textured_object();
    }
}