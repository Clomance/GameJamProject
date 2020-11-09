use super::{
    Movement,
};

pub struct Character{
    pub position:[f32;2],

    pub velocity:f32,
    pub movement:Movement,
}

impl Character{
    pub fn new(velocity:f32)->Character{
        Self{
            position:[0f32;2],
            velocity,
            movement:Movement::None
        }
    }

    pub fn moving(&mut self)->bool{
        self.movement.moving(&mut self.position,self.velocity)
    }
}