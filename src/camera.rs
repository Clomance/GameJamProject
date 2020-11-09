#[derive(Clone,Copy,PartialEq)]
pub enum Movement{
    None,

    Up,
    Down,
    Right,
    Left,

    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Movement{
    pub fn moving(&self,position:&mut [f32;2],velocity:f32)->bool{
        match self{
            Movement::None=>false,

            Movement::Up=>{
                position[1]-=velocity;
                true
            }
            Movement::Down=>{
                position[1]+=velocity;
                true
            }
            Movement::Right=>{
                position[0]+=velocity;
                true
            }
            Movement::Left=>{
                position[0]-=velocity;
                true
            }

            Movement::UpRight=>{
                let half_velocity=velocity/2f32;
                position[1]-=half_velocity;
                position[0]+=half_velocity;
                true
            }
            Movement::UpLeft=>{
                let half_velocity=velocity/2f32;
                position[1]-=half_velocity;
                position[0]-=half_velocity;
                true
            }
            Movement::DownRight=>{
                let half_velocity=velocity/2f32;
                position[1]+=half_velocity;
                position[0]+=half_velocity;
                true
            }
            Movement::DownLeft=>{
                let half_velocity=velocity/2f32;
                position[1]+=half_velocity;
                position[0]-=half_velocity;
                true
            }
        }
    }

    pub fn move_up_enable(&mut self){
        *self=match self{
            Movement::None=>Movement::Up,

            Movement::Down=>Movement::None,
            Movement::Right=>Movement::UpRight,
            Movement::Left=>Movement::UpLeft,

            Movement::DownRight=>Movement::Right,
            Movement::DownLeft=>Movement::Left,

            _=>*self
        }
    }

    pub fn move_up_disable(&mut self){
        *self=match self{
            Movement::None=>Movement::Down,

            Movement::Up=>Movement::None,
            Movement::Right=>Movement::DownRight,
            Movement::Left=>Movement::DownLeft,

            Movement::UpRight=>Movement::Right,
            Movement::UpLeft=>Movement::Left,

            _=>*self
        }
    }

    pub fn move_down_enable(&mut self){
        *self=match self{
            Movement::None=>Movement::Down,

            Movement::Up=>Movement::None,
            Movement::Right=>Movement::DownRight,
            Movement::Left=>Movement::DownLeft,

            Movement::UpRight=>Movement::Right,
            Movement::UpLeft=>Movement::Left,

            _=>*self
        }
    }

    pub fn move_down_disable(&mut self){
        *self=match self{
            Movement::None=>Movement::Up,

            Movement::Down=>Movement::None,
            Movement::Right=>Movement::UpRight,
            Movement::Left=>Movement::UpLeft,

            Movement::DownRight=>Movement::Right,
            Movement::DownLeft=>Movement::Left,

            _=>*self
        }
    }

    pub fn move_right_enable(&mut self){
        *self=match self{
            Movement::None=>Movement::Right,

            Movement::Up=>Movement::UpRight,
            Movement::Down=>Movement::DownRight,
            Movement::Left=>Movement::None,

            Movement::UpLeft=>Movement::Up,
            Movement::DownLeft=>Movement::Down,

            _=>*self
        }
    }

    pub fn move_right_disable(&mut self){
        *self=match self{
            Movement::None=>Movement::Left,

            Movement::Up=>Movement::UpLeft,
            Movement::Down=>Movement::DownLeft,
            Movement::Right=>Movement::None,

            Movement::UpRight=>Movement::Up,
            Movement::DownRight=>Movement::Down,

            _=>*self
        }
    }

    pub fn move_left_enable(&mut self){
        *self=match self{
            Movement::None=>Movement::Left,

            Movement::Up=>Movement::UpLeft,
            Movement::Down=>Movement::DownLeft,
            Movement::Right=>Movement::None,

            Movement::UpRight=>Movement::Up,
            Movement::DownRight=>Movement::Down,

            _=>*self
        }
    }

    /// Used only after the `move_left_enable` function had been called.
    pub fn move_left_disable(&mut self){
        *self=match self{
            Movement::None=>Movement::Right,

            Movement::Up=>Movement::UpRight,
            Movement::Down=>Movement::DownRight,
            Movement::Left=>Movement::None,

            Movement::UpLeft=>Movement::Up,
            Movement::DownLeft=>Movement::Down,

            _=>*self
        }
    }
}

pub struct Camera{
    /// Window coords.
    pub position:[f32;2],

    pub velocity:f32,
    pub movement:Movement,
}

impl Camera{
    pub fn new(velocity:f32)->Camera{
        Self{
            position:[0f32;2],

            velocity,
            movement:Movement::None,
        }
    }

    pub fn moving(&mut self)->bool{
        self.movement.moving(&mut self.position,self.velocity)
    }
}