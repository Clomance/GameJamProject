#![allow(non_snake_case,non_upper_case_globals)]

mod settings;
use settings::GameSettings;

mod weapon;
use weapon::*;

mod character;
use character::*;

mod pages;
use pages::*;

mod map;
mod phys_engine;
pub use phys_engine::*;

use map::Map;

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
    PagedWindow,
    Window,
    window_height,
    window_width
};

use std::{
    path::PathBuf,
    time::Instant
};
use cat_engine::glium::backend::glutin::glutin::window::Fullscreen;
use cat_engine::glium::glutin::window::Icon;
use cat_engine::image::GenericImageView;

const game_name:&'static str="GhostBuster";

// Индекс обоев (объекта) в кэше
const wallpaper_index:usize=0;
// Индекс картинки обоев в массиве картинок
const main_menu_image_index:usize=0;
// Индекс картинки задника карты в массиве картинок
const map_background_index:usize=1;
// Индекс картинки персонажа в массиве картинок
const character_image_index:usize=2;


//
const loading_screen_wallpaper_path:&'static str="resources/textures/loading_screen_wallpaper.png";
const loading_screen_ghost_texture:&'static str="resources/textures/loading_screen_ghost.png";

//
const main_menu_wallpaper_path:&'static str="resources/textures/main_menu_wallpaper.png";

const map_background_path:&'static str="resources/textures/map.png";
const character_texture_path:&'static str="resources/textures/character.png";
const weaponry_texture_path:&'static str="resources/textures/weaponry.png";

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
    //let game_settings=GameSettings::new();

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

    //let window_icon=load_window_icon();

    let mut audio_wrapper=AudioWrapper::new(audio);
    audio_wrapper.load_track(audio_menu_path, audio_menu_name.to_string());

    // Создание окна
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

    // Проигрывание мелодии
    audio_wrapper.play_track(audio_menu_name);

    let mut textures:Vec<RgbaImage>={
        let mut loading_screen=LoadingScreen::new(&mut window);
        // Запуск загрузочного экрана
        window.run_page(&mut loading_screen)
    };

    {
        let mut main_menu=MainMenu::new(&textures,&mut object_map,&mut window);
        // Запуск главного меню
        window.run_page(&mut main_menu);
    }

    {
        let mut game_play=GamePlay::new(&textures,&mut window);
        // Запуск геймплея
        window.run_page(&mut game_play);
    }
}

/// Загрузка иконки окна
fn load_window_icon()->Icon{
    let image=cat_engine::image::open("./resources/images/window_icon.png").unwrap();
    let vec=image.to_bytes();
    let (width,height)=image.dimensions();

    Icon::from_rgba(vec,width,height).unwrap()
}

pub fn load_image<P:AsRef<std::path::Path>>(path:P)->RgbaImage{
    let mut image=cat_engine::image::open(path).unwrap();

    if let cat_engine::image::DynamicImage::ImageRgba8(image)=image{
        image
    }
    else{
        image.into_rgba()
    }
}

// Загрузка изображений
pub fn load_image_scaled<P:AsRef<std::path::Path>>(path:P,width:u32,height:u32)->RgbaImage{
    let mut image=cat_engine::image::open(path).unwrap();

    image=image.resize_exact(width,height,cat_engine::image::imageops::FilterType::Gaussian);
    if let cat_engine::image::DynamicImage::ImageRgba8(image)=image{
        image
    }
    else{
        image.into_rgba()
    }
}

pub fn load_image_scaled_height<P:AsRef<std::path::Path>>(path:P,height:u32)->RgbaImage{
    let mut image=cat_engine::image::open(path).unwrap();

    let mut image_dimensions=image.dimensions();

    let scale=image_dimensions.1 as f32/height as f32;
    let width=(image_dimensions.0 as f32 * scale).ceil() as u32;

    image=image.resize_exact(width,height,cat_engine::image::imageops::FilterType::Gaussian);
    if let cat_engine::image::DynamicImage::ImageRgba8(image)=image{
        image
    }
    else{
        image.into_rgba()
    }
}