#![allow(non_snake_case,non_upper_case_globals)]

mod settings;
use settings::GameSettings;

mod weapon;
use weapon::*;

mod character;
use character::*;

mod wallpaper;
use wallpaper::Wallpaper;

use lib ::{
    *,
    colours::*,
};

use cat_engine::{
    audio::{
        AudioSettings,
        Audio,
        cpal,
        cpal::traits::HostTrait,
        cpal::traits::DeviceTrait,
        AudioWrapper,
    },

    graphics::{
        ColourFilter,
        Graphics,
        DrawType,
        ObjectType,
    },

    image::{
        RgbaImage,
    },

    texture::{
        ImageBase,
        Texture,
    },

    support::SyncRawMutPtr,

    KeyboardButton,
    MouseButton,
    ModifiersState,
    MouseScrollDelta,

    PagedWindow,

    // statics
    mouse_cursor,
    window_center,

};

use std::time::Instant;

const game_name:&'static str="GhostBuster";

const audio_menu_path:&'static str="resources/audio/main_theme.mp3";
const audio_menu_name:&'static str="main_theme";

fn main(){
    // Настройки игры
    let game_settings=GameSettings::new();
    // Настройки аудио системы
    let settings=AudioSettings::new();
    let host=cpal::default_host();
    let audio=Audio::new(host,|host|{
            host.default_output_device().unwrap()
        },
        |device|{
            device.default_output_format().unwrap()
        },
        settings
    ).unwrap();

    let mut audio_wrapper=AudioWrapper::new(audio);

    let mut audio_wrapper_ref=SyncRawMutPtr::new(&mut audio_wrapper);

    let loading_function=move||{
        let audio_wrapper_mut=audio_wrapper_ref.as_mut();

        audio_wrapper_mut.load_track(audio_menu_path,audio_menu_name.to_string());
    };

    let mut window=PagedWindow::new(|_,s|{
        s.vsync=true;
    }).unwrap();

    let mut game=Game::new(settings);
}


struct Game{
    game_start_time:Instant,
    settings:GameSettings,
    audio:AudioWrapper,

    frames:u64,

    // Карта объектов
    // Отрисовываемые объекты и зоны активные для клика
    object_map:ObjectMap,
    saved_drawables:Vec<DrawableObject>,

    //
    prerendering:fn(&mut Self),
    updates:fn(&mut Self,&mut PagedWindow),
    click_handler:fn(&mut Self,bool,MouseButton,&mut PagedWindow),
    keyboard_handler:fn(&mut Self,bool,KeyboardButton,&mut PagedWindow),
}

impl Game{
    pub fn new<F:FnOnce()->Vec<RgbaImage>+Send+'static>(settings:GameSettings,window:&mut PagedWindow,background:F)->Game{
        // Объекты интерфейса
        let mut saved_drawables=Vec::with_capacity(10);
        let mut object_map=ObjectMap::new();

        object_map.add_new_layer();

        let mut image_base=ImageBase::new(White,unsafe{[
            window_center[0]-100f32,
            window_center[1]-100f32,
            200f32,
            200f32
        ]});

        let cat=Texture::from_path("./resources/images/cat.png",window.display()).unwrap();
        let cat=window.graphics2d().add_textured_object(&image_base,cat).unwrap();

        saved_drawables.push(DrawableObject::new(cat,ObjectType::Textured,DrawType::Common));
        object_map.add_raw_simple_drawable_object(0,cat,ObjectType::Textured,DrawType::Common);

        let cat_eyes_closed=Texture::from_path("./resources/images/cat_eyes_closed.png",window.display()).unwrap();
        let cat_eyes_closed=window.graphics2d().add_textured_object(&image_base,cat_eyes_closed).unwrap();

        saved_drawables.push(DrawableObject::new(cat_eyes_closed,ObjectType::Textured,DrawType::Common));

        image_base.set_rect(unsafe{[
            window_center[0]-200f32,
            window_center[1]-200f32,
            400f32,
            400f32
        ]});

        let gear=Texture::from_path("./resources/images/gear.png",window.display()).unwrap();
        let gear=window.graphics2d().add_textured_object(&image_base,gear).unwrap();

        object_map.add_raw_simple_drawable_object(0,gear,ObjectType::Textured,DrawType::Rotating((0f32,unsafe{window_center})));

        // Подключение аудио системы
        let audio=Audio::default(AudioSettings::new()).unwrap();
        let mut audio = AudioWrapper::new(audio);
        audio.load_track("./resources/music/audio.mp3", "main_theme".to_string());
        audio.load_track("./resources/music/button.mp3", "button_click".to_string());
        audio.load_track("./resources/music/click.mp3", "mouse_click".to_string());


        let thread=std::thread::spawn(background);

        Self{
            audio:audio,
            settings:settings,
            wallpaper:Wallpaper::Colour(White),
            images:Vec::new(),

            frames:0u64,
            thread:Some(thread),

            saved_drawables,
            object_map,

            prerendering:Game::empty_prerendering,
            updates:Game::loading_updates,
            click_handler:Game::empty_click_handler,
            keyboard_handler:Game::empty_keyboard_handler,
        }
    }

    pub fn loading_updates(&mut self,window:&mut PagedWindow){
        if unsafe{!loading}{
            if let Some(thread)=self.thread.take(){
                self.images=thread.join().expect("Ошибка начальной загрузки");
            }

            self.saved_drawables.clear();
            // Отчистка слоёв
            self.object_map.clear_layers();

            for _ in 0..3{
                window.graphics2d().delete_last_textured_object();
            }
            self.audio.play_track("main_theme");

            return set_main_menu(self,window)
        }

        if let DrawType::Rotating((angle,_))=&mut self.object_map.get_drawable(0,1).draw_type{
            *angle+=0.05f32;
        }

        if self.frames==20{
            self.object_map.set_drawable(0,0,self.saved_drawables[1].clone());
            // self.cat_eyes_closed
        }
        else{
            if self.frames==30{
                // self.cat
                self.object_map.set_drawable(0,0,self.saved_drawables[0].clone());
                self.frames=0;
            }
        };

        self.frames+=1;
    }

    pub fn empty_prerendering(&mut self){

    }

    pub fn empty_updates(&mut self,_window:&mut PagedWindow){

    }

    pub fn empty_click_handler(&mut self,_pressed:bool,_button:MouseButton,_window:&mut PagedWindow){

    }

    pub fn empty_keyboard_handler(&mut self,_:bool,_:KeyboardButton,_:&mut PagedWindow){

    }
}


impl WindowPage<'static> for Game{
    type Window=PagedWindow;

    type Output=();

    fn on_window_close_requested(&mut self,_window:&mut PagedWindow){

    }

    fn on_update_requested(&mut self,window:&mut PagedWindow){
        (self.updates)(self,window)
    }

    fn on_redraw_requested(&mut self,window:&mut PagedWindow){
        (self.prerendering)(self);

        window.draw(|parameters,graphics|{
            let [dx,dy]=unsafe{mouse_cursor.center_radius()};
            // Рендеринг фона
            if let Wallpaper::Colour(colour)=self.wallpaper{
                // Заполнение цветом
                graphics.clear_colour(colour)
            }
            else{
                // Заполнение картинкой
                let shift=[
                    dx/wallpaper_movement_scale,
                    dy/wallpaper_movement_scale,
                ];
                graphics.draw_shift_textured_object(wallpaper,shift,ColourFilter::new_mul([1f32;4]),parameters).unwrap();
            }

            // Рендеринг объектов
            self.object_map.draw(parameters,graphics);

            // Рендеринг курсора
            graphics.draw_shift_textured_object(mouse_cursor_icon,[dx,dy],ColourFilter::new_mul([1f32;4]),parameters).unwrap();
        }).unwrap();
    }

    fn on_mouse_pressed(&mut self,window:&mut PagedWindow,button:MouseButton){
        self.audio.play_track("mouse_click");
        (self.click_handler)(self,true,button,window)
    }
    fn on_mouse_released(&mut self,window:&mut PagedWindow,button:MouseButton){
        (self.click_handler)(self,false,button,window)
    }
    fn on_mouse_moved(&mut self,_window:&mut PagedWindow,_:[f32;2]){}
    fn on_mouse_scrolled(&mut self,_window:&mut PagedWindow,_:MouseScrollDelta){}

    fn on_keyboard_pressed(&mut self,window:&mut PagedWindow,button:KeyboardButton){
        (self.keyboard_handler) (self,true,button,window)
    }
    fn on_keyboard_released(&mut self,window:&mut PagedWindow,button:KeyboardButton){
        (self.keyboard_handler) (self,false,button,window)

    }

    fn on_character_recieved(&mut self,_window:&mut PagedWindow,_character:char){}

    fn on_modifiers_changed(&mut self,_window:&mut PagedWindow,_modifiers:ModifiersState){}

    fn on_window_resized(&mut self,_window:&mut PagedWindow,_new_size:[u32;2]){}

    fn on_suspended(&mut self,_window:&mut PagedWindow){}
    fn on_resumed(&mut self,_window:&mut PagedWindow){}

    fn on_window_moved(&mut self,_window:&mut PagedWindow,_:[i32;2]){}

    fn on_window_focused(&mut self,_window:&mut PagedWindow,_:bool){}

    fn on_event_loop_closed(&mut self,_window:&mut Self::Window){}
}