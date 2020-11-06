#![allow(non_snake_case,non_upper_case_globals)]

mod settings;
use settings::GameSettings;

mod weapon;
use weapon::*;

mod character;
use character::*;

mod pages;
use pages::*;

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
    text::{
        GlyphCache,
        CachedFont,
        FontOwner,
        Scale,
    },
    support::SyncRawMutPtr,
    KeyboardButton,
    MouseButton,
    ModifiersState,
    MouseScrollDelta,
    PagedWindow,
    WindowPage,
    Window,
    mouse_cursor,
    window_center,
    window_height,
    window_width
};

use std::{
    path::PathBuf,
    time::Instant
};
use cat_engine::glium::backend::glutin::glutin::window::Fullscreen;

const game_name:&'static str="GhostBuster";

const wallpaper_index:usize=0;
//
const loading_screen_wallpaper_path:&'static str="resources/textures/loading_screen_wallpaper.png";
const loading_screen_ghost_texture:&'static str="resources/textures/loading_screen_ghost.png";

//
const main_menu_wallpaper_path:&'static str="resources/textures/main_menu_wallpaper.png";


// Themes
const audio_menu_path:&'static str="resources/audio/main_theme.mp3";
const audio_menu_name:&'static str="main_theme";
// const audio_level_path:&'static str="resources/audio/level_theme.mp3";
// const audio_level_name:&'static str="level_theme";
// const audio_boss_path:&'static str="resources/audio/boss_theme.mp3";
// const audio_boss_name:&'static str="boss_theme";
// SFXs
// const audio_button_path:&'static str="resources/audio/button_click.mp3";
// const audio_button_name:&'static str="button";

// Fonts
const pixel_font_path:&'static str="resources/font/pixel.ttf";

const alphabet:&'static str="АаБбВвГгДдЕеЁёЖжЗзИиЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЪъЫыЬьЭэЮюЯя";

pub static loading:bool=true;

fn main(){
    let face=FontOwner::load(pixel_font_path).unwrap();
    let mut object_map=ObjectMap::new();
    object_map.add_new_layer();

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
    audio_wrapper.load_track(audio_menu_path, audio_menu_name.to_string());

    audio_wrapper.play_track(audio_menu_name);

    let mut window=PagedWindow::new(|mut monitors,settings|{
        settings.general.updates_per_second=50;

        settings.graphics_base_settings.text.glyph_texture_size=[1024;2];

        settings.graphics_base_settings.simple.object_buffer_size=20;
        settings.graphics_base_settings.simple.index_buffer_size=80;
        settings.graphics_base_settings.simple.index_buffer_offset=0;
        settings.graphics_base_settings.simple.vertex_buffer_size=80;
        settings.graphics_base_settings.simple.vertex_buffer_offset=0;

        settings.graphics_base_settings.texture.object_buffer_size=20;
        settings.graphics_base_settings.texture.index_buffer_size=80;
        settings.graphics_base_settings.texture.index_buffer_offset=0;
        settings.graphics_base_settings.texture.vertex_buffer_size=80;
        settings.graphics_base_settings.texture.vertex_buffer_offset=0;

        let monitor=monitors.remove(0);
        settings.window_attributes.fullscreen=Some(Fullscreen::Borderless(Some(monitor)));

        settings.general.initial_colour=Some([1f32;4]);

        settings.vsync=true;
    }).unwrap();

    // Установка шрифта
    let mut glyph_cache=GlyphCache::new_alphabet(face.face(),alphabet,Scale::new(0.1,0.1),window.display());
    let font=CachedFont::raw(face,glyph_cache);

    window.graphics2d().add_font(font);

    let mut image_base=ImageBase::new(White,unsafe{[
        0f32,
        0f32,
        window_width,
        window_height
    ]});

    let mut wallpaper_texture=Texture::from_path(loading_screen_wallpaper_path,window.display()).unwrap();
    window.graphics2d().add_texture(wallpaper_texture);

    // Загрузка обоев
    window.graphics2d().add_textured_object(&image_base,0).unwrap();

    {
        let mut loading_screen=LoadingScreen::new(&mut window);
        // Запуск загрузочного экрана
        window.run_page(&mut loading_screen);
    }

    {
        let mut main_menu=MainMenu::new(&mut object_map,&mut window);
        // Запуск главного меню
        window.run_page(&mut main_menu);
    }
}