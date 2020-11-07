use cat_engine::{WindowPage, MouseScrollDelta, KeyboardButton, ModifiersState, MouseButton, PagedWindow, Window, window_center, window_height, window_rect, window_width};
use cat_engine::image::RgbaImage;
use cat_engine::texture::{Texture, ImageBase};
use crate::{map_background_index, character_image_index};
use lib::colours::{White, Cyan};
use cat_engine::graphics::{ColourFilter, ObjectType, DrawType};
use std::path::PathBuf;
use cat_engine::shapes::{RectangleWithBorder, Rectangle};
use lib::{TextView, TextViewSettings, Button, ButtonSettings, ComplexDrawable, DrawableObject, Drawable};

pub struct GamePlay{
    //character:Character,
    //map:Map,
}

const map_background_texture_index:usize=1;
// (текстурного объекта)
const map_background1_index:usize=1;
const map_background2_index:usize=2;

const character_texture_index:usize=2;
// Индекс персонажа (текстурного объекта)
const character_index:usize=3;



impl GamePlay {
    pub fn new(textures:&Vec<RgbaImage>,window:&mut PagedWindow)->GamePlay{
        // Текстуры карты
        let map_background1=ImageBase::new(White,window_rect());
        let map_background2=ImageBase::new(White,window_rect());

        let background_texture=Texture::from_image(&textures[map_background_index],window.display()).unwrap();
        window.graphics2d().add_texture(background_texture);

        window.graphics2d().add_textured_object(&map_background1,map_background_texture_index).unwrap();
        window.graphics2d().add_textured_object(&map_background2,map_background_texture_index).unwrap();


        // Персонаж
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
            graphics.draw_textured_object(map_background1_index,ColourFilter::new_mul([1f32;4]),parameters).unwrap();
            graphics.draw_textured_object(map_background2_index,ColourFilter::new_mul([1f32;4]),parameters).unwrap();
            graphics.draw_textured_object(character_index,ColourFilter::new_mul([1f32;4]),parameters).unwrap();
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

struct Hud{
    health_bar_size: RectangleWithBorder,
    energy_bar_size: RectangleWithBorder,
    health_bar: Rectangle,
    energy_bar: Rectangle,
    health_text:TextView,
    energy_text:TextView,
}
impl Hud{
    fn new(&mut window:PagedWindow)->Self{
        let health_bar_size=[40f32, 40f32,450f32,50f32];
        let energy_bar_size=unsafe{[window_width-40f32-450f32,40f32,450f32,50f32]};
        let health_bar_background=RectangleWithBorder::raw(health_bar_size,[1f32,1f32,1f32,0.3], 1f32,White);
        let energy_bar_background=RectangleWithBorder::raw(energy_bar_size, [1f32,1f32,1f32,0.3],1f32,White);
        let health_text_settings = TextViewSettings::new("Здроровье",[health_bar_size[0] + health_bar_size[3] + 5f32, health_bar_size[2], 150f32, 40f32]);
        let health_text=TextView::new(health_text_settings,window.graphics2d());
        let energy_text_settings = TextViewSettings::new("Энергия",[energy_bar_size[0]  - 150f32, energy_bar_size[2], 150f32, 40f32]);
        let energy_text=TextView::new(energy_text_settings,window.graphics2d());
        let mut health_bar = Rectangle::new(health_bar_size, [0f32,1f32,0f32,1f32]); //
        let mut energy_bar = Rectangle::new(energy_bar_size, Cyan);
        Self{
            health_bar_size: health_bar_background,
            energy_bar_size: energy_bar_background,
            health_bar,
            energy_bar,
            health_text,
            energy_text
        }
    }
    fn update(&self, health:f32, energy:f32)->Self{
    }
}
struct PauseMenuDialog{
    button_yes:Button,
    button_no:Button,
    header:TextView,
    dialog_box:usize,
}
impl PauseMenuDialog {
    fn new(window: &mut PagedWindow) -> Self {
        // making a box for dialog to fit in
        let window_size = unsafe { [window_width, window_height] };
        let rect_size = [window_size[0] / 2f32 - 200f32, window_size[1] / 2f32 - 100f32, 400f32, 200f32];
        let dialog_box_rect = Rectangle::new(rect_size, [1.0, 0.545, 0.349, 0.75]); // Uses Bleak_orange with lowered alpha
        let dialog_box_rect_index = window.graphics2d().add_simple_object(&dialog_box_rect).unwrap();

        // making confirmation text
        let confirmation_text_settings = TextViewSettings::new("Выйти в меню?", [window_size[0] / 2f32 - 200f32 + 5f32, window_size[1] / 2f32 - 100f32 + 5f32, 195f32, 20f32]);
        let confirmation_text = TextView::new(confirmation_text_settings, window.graphics2d());

        // making confirmation buttons
        let confirmation_button_size = [window_size[0] / 16f32, window_size[1] / 10f32];
        let confirmation_button_yes_placement = [window_size[0] / 2f32 - 200f32 + 20f32, window_size[1] / 2f32 - 20f32, confirmation_button_size[0], confirmation_button_size[1]];
        let confirmation_button_yes_settings = ButtonSettings::new("Да", confirmation_button_yes_placement);
        let confirmation_button_yes = Button::new(confirmation_button_yes_settings, window.graphics2d());
        let confirmation_button_no_placement = [window_size[0] / 2f32 + 175f32 - confirmation_button_size[1], window_size[1] / 2f32 - 20f32, confirmation_button_size[0], confirmation_button_size[1]];
        let confirmation_button_no_settings = ButtonSettings::new("Нет", confirmation_button_no_placement);
        let confirmation_button_no = Button::new(confirmation_button_no_settings, window.graphics2d());
        Self {
            button_yes: confirmation_button_yes,
            button_no: confirmation_button_no,
            header: confirmation_text,
            dialog_box: dialog_box_rect_index
        }
    }
}
impl ComplexDrawable for PauseMenuDialog{
    fn drawables(&self)->Vec<DrawableObject>{
        let mut drawables = Vec::with_capacity(4);
        let drawable = DrawableObject{
            index: self.dialog_box,
            object_type: ObjectType::Simple,
            draw_type: DrawType::Common,
        };
        drawables.push(drawable);
        drawables.push(self.header.drawable());
        drawables.append(&mut self.button_yes.drawables());
        drawables.append(&mut self.button_no.drawables());
        drawables
    }
}
impl ComplexClickable for ExitConfirmationDialogue{
    fn clickables(&self)->Vec<ClickableObject>{
        let mut clickable = Vec::with_capacity(2);
        clickable.append(&mut self.button_yes.clickables());
        clickable.append(&mut self.button_no.clickables());
        clickable
    }

}