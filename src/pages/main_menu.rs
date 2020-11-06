use crate::{
    game_name,
    // structs
    Drawable,
    DrawableObject,
};


use lib::{
    Menu,
    MenuSettings,
    TextView,
    TextViewSettings,
    ButtonSettings,
    Button,
    ComplexClickable,
    ComplexDrawable,
    ClickableObject,
    ObjectMap
};

use cat_engine::{
    window_height,
    window_width,
    mouse_cursor,
    WindowEvent,
    KeyboardButton,
    MouseButton,
    glium::DrawParameters,
    Window,
    audio::Audio, shapes::*, DefaultWindow, PagedWindow,
    graphics::{
        Graphics,
        DrawType,
        ObjectType
    },
    WindowPage,
    MouseScrollDelta,
    ModifiersState
};

use lib::colours::Bleak_orange;
use std::path::PathBuf;

// Индекс картинки для главного меню
// Пока что так
const main_menu_image:usize=0;

pub struct MainMenu<'a>{
    object_map:&'a mut ObjectMap
}


impl<'a> MainMenu<'a>{
    pub fn new(object_map:&'a mut ObjectMap,window:&mut PagedWindow)->MainMenu<'a>{
        // Устновка обоев для главного меню
        // window.graphics2d().get_textured_object_texture(wallpaper).update(/);

        object_map.add_new_layer();
        object_map.set_layer_enabled(0,false);

        let main_menu = MenuButtons::new(window);
        object_map.add_complex_object(0,main_menu);

        Self{
            object_map,
        }
    }
}

impl<'a> WindowPage<'static> for MainMenu<'a>{
    type Window = PagedWindow;
    type Output = ();

    fn on_window_close_requested(&mut self, window: &mut Self::Window) {
    }

    fn on_update_requested(&mut self, window: &mut Self::Window) {
    }

    fn on_redraw_requested(&mut self, window: &mut Self::Window) {
        window.draw(|parameters, graphics|{
            self.object_map.draw(parameters, graphics);
        });
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
                            println!("pressed")
                        }
                        // quit
                        1=>{
                            println!("pressed");
                            window.stop_events();
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
                if let Some((mut button,clicked))=self.object_map.released(cursor_position){

                    match button{
                        0=>{
                            if clicked{
                                println!("continue")
                            }
                        }
                        1=> {
                            if clicked {
                                println!("continue")
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

    fn on_mouse_scrolled(&mut self, window: &mut Self::Window, Scroll: MouseScrollDelta) {
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
        let button_game_placement = [1920f32/920f32-200f32, 1080f32/2f32, confirmation_button_size[0], confirmation_button_size[1]];
        let button_game_settings = ButtonSettings::new("НАЧАТь ИГРУ", button_game_placement);
        let button_game = Button::new(button_game_settings, window.graphics2d());
        let button_quit_placement = [1920f32/920f32-200f32, 1080f32/2f32 + 120f32, confirmation_button_size[0], confirmation_button_size[1]];
        let button_quit_settings = ButtonSettings::new("ВЫЙТИ", button_quit_placement);
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
