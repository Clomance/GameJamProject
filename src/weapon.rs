
pub enum WeaponType{
    Beam,
    Ball,
}

pub struct Weapon{
    texture:usize,
    damage:u32,
    energy_usage:u32,
    cooldown:u32,
    weapon_type:WeaponType,
}

// Наверное не сюда, но куда-то
// Значения переписал с файла Юли
fn чтобынееругаось(){
let tazer = Weapon{
    texture: 0, //Sometexture
    damage: 2,
    energy_usage: 0,
    cooldown: 3,
    weapon_type: WeaponType::Beam, // по-хорошему melee
};
let blaster = Weapon{
    texture: 0, //Sometexture
    damage: 3,
    energy_usage: 2,
    cooldown: 2,
    weapon_type: WeaponType::Ball,
};
let minilaser = Weapon{
    texture: 0, //Sometexture
    damage: 3,
    energy_usage: 3,
    cooldown: 1,
    weapon_type: WeaponType::Beam,
};
let miniplasma = Weapon{
    texture: 0, //Sometexture
    damage: 5,
    energy_usage: 5,
    cooldown: 2,
    weapon_type: WeaponType::Ball
};
let lasergun = Weapon{
    texture: 0, //Sometexture
    damage: 5,
    energy_usage: 6,
    cooldown: 1,
    weapon_type: WeaponType::Beam
};
let plasmagun = Weapon{
    texture: 0, //Sometexture
    damage: 10,
    energy_usage: 10,
    cooldown: 3,
    weapon_type: WeaponType::Ball
};
}