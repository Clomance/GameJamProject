
pub enum WeaponType{
    Beam,
    Ball,
}

pub struct Weapon{
    texture:usize,
    damage:u32,
    weapon_type:WeaponType,
}