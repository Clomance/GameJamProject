use std::{
    io::{Read,Write},
    fs::OpenOptions,
};

pub struct GameSettings{
    pub user_name:String,
    pub volume:f32, // Громкость игры, 0 - 128
    pub screenshot:u32, // номер следующего скришота
}

impl GameSettings{
    ///
    pub const fn new()->GameSettings{
        Self{
            user_name:String::new(),
            volume:0f32,
            screenshot:0u32,
        }
    }

    /// Загрузка настроек
    pub fn load()->GameSettings{
        let mut settings=GameSettings::new();
        settings
    }
}