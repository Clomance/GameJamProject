use crate::{DrawableObject, wallpaper_index, main_menu_image_index, GameState};


use lib::{
    ButtonSettings,
    Button,
    ComplexClickable,
    ComplexDrawable,
    ClickableObject,
    ObjectMap
};

use cat_engine::{mouse_cursor, KeyboardButton, MouseButton, Window, PagedWindow, WindowPage, MouseScrollDelta, ModifiersState, window_center};

use cat_engine::graphics::ColourFilter;
use cat_engine::image::RgbaImage;
use lib::colours::{White};
use std::path::PathBuf;

pub struct MainMenu<'a>{
    object_map:&'a mut ObjectMap,
    state:GameState,
}


impl<'a> MainMenu<'a>{
    pub fn new(textures:&Vec<RgbaImage>,object_map:&'a mut ObjectMap,window:&mut PagedWindow)->MainMenu<'a>{
        // Устновка обоев для главного меню
        window.graphics2d().get_textured_object_texture(wallpaper_index).update(&textures[main_menu_image_index]);

        let main_menu = MenuButtons::new(window);
        object_map.add_complex_object(0,main_menu);

        Self{
            object_map,
            state:GameState::Exit
        }
    }
}

impl<'a> WindowPage<'static> for MainMenu<'a>{
    type Window = PagedWindow;
    type Output = GameState;

    fn on_window_close_requested(&mut self, _window: &mut Self::Window) {
    }

    fn on_update_requested(&mut self, _window: &mut Self::Window) {

    }

    fn on_redraw_requested(&mut self, window: &mut Self::Window) {
        window.draw(|parameters, graphics|{
            graphics.draw_textured_object(wallpaper_index,ColourFilter::new_mul([1f32;4]),parameters).unwrap();
            self.object_map.draw(parameters, graphics);
        }).unwrap();
    }

    fn on_mouse_pressed(&mut self, window: &mut Self::Window, button: MouseButton) {
        let cursor_position=unsafe{mouse_cursor.position()};
        match button{
            MouseButton::Left=>{
                if let Some(mut button)=self.object_map.pressed(cursor_position){
                    // game.audio.play_track("button_click");
                    match button{
                        // New game
                        0=>{

                        }
                        // quit
                        1=>{

                        }
                        _=>{

                        }
                    }
                }
            }
            _=>{}
        }
    }

    fn on_mouse_released(&mut self, window: &mut Self::Window, button: MouseButton) {
        let cursor_position=unsafe{mouse_cursor.position()};
        match button{
            MouseButton::Left=>{
                if let Some((button,clicked))=self.object_map.released(cursor_position){

                    match button{
                        0=>{
                            if clicked{
                                self.state=GameState::GamePlay;
                                window.stop_events().unwrap();
                            }
                        }
                        1=> {
                            if clicked {
                                self.state=GameState::Exit;
                                window.stop_events().unwrap();
                            }
                        }

                        _=>{

                        }
                    }
                }
            }
            _=>{}
        }
    }

    fn on_mouse_scrolled(&mut self, _window: &mut Self::Window, _Scroll: MouseScrollDelta) {
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

    }

    #[cfg(feature="file_drop")]
    fn on_file_hovered(&mut self, window: &mut Self::Window, path: PathBuf) {

    }

    #[cfg(feature="file_drop")]
    fn on_file_hovered_canceled(&mut self, window: &mut Self::Window) {

    }

    fn on_event_loop_closed(&mut self, window: &mut Self::Window) -> Self::Output {
        window.graphics2d().clear_simple_object_array();
        window.graphics2d().clear_text_object_array();
        self.object_map.clear();

        self.state.clone()
    }
}


struct MenuButtons{
    button_game:Button,
    button_quit:Button,
}
impl MenuButtons{
    fn new(window:&mut PagedWindow)->Self{

        // Menu buttons
        let confirmation_button_size = [400f32, 100f32];
        let button_game_placement = unsafe{[window_center[0]-200f32, window_center[1], confirmation_button_size[0], confirmation_button_size[1]]};
        let button_game_settings = ButtonSettings::new("НАЧАТЬ ИГРУ", button_game_placement).background_colour([0f32;4]).text_colour(White);
        let button_game = Button::new(button_game_settings, window.graphics2d());
        let button_quit_placement = unsafe{[window_center[0]-200f32, window_center[1] + 120f32, confirmation_button_size[0], confirmation_button_size[1]]};
        let button_quit_settings = ButtonSettings::new("ВЫЙТИ", button_quit_placement).background_colour([0f32;4]).text_colour(White);
        let button_quit = Button::new(button_quit_settings, window.graphics2d());

        Self{
            button_game,
            button_quit,
        }
    }
}

impl ComplexDrawable for MenuButtons{
    fn drawables(&self)->Vec<DrawableObject>{
        let mut drawables = Vec::with_capacity(2);
        drawables.append(&mut self.button_game.drawables());
        drawables.append(&mut self.button_quit.drawables());
        drawables
    }
}
impl ComplexClickable for MenuButtons{
    fn clickables(&self)->Vec<ClickableObject>{
        let mut clickable = Vec::with_capacity(2);
        clickable.append(&mut self.button_game.clickables());
        clickable.append(&mut self.button_quit.clickables());
        clickable
    }
}
