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

pub struct Camera{
    /// Window coords.
    pub position:[f32;2],

    pub velocity:f32,
    movement:Movement,
}

impl Camera{
    pub fn new(velocity:f32)->Camera{
        Self{
            position:[0f32;2],

            velocity,
            movement:Movement::None,
        }
    }

    pub fn update(&mut self){
        match self.movement{
            Movement::None=>{}

            Movement::Up=>self.position[1]-=self.velocity,
            Movement::Down=>self.position[1]+=self.velocity,
            Movement::Right=>self.position[0]+=self.velocity,
            Movement::Left=>self.position[0]-=self.velocity,

            Movement::UpRight=>{
                let half_velocity=self.velocity/2f32;
                self.position[1]-=half_velocity;
                self.position[0]+=half_velocity
            }
            Movement::UpLeft=>{
                let half_velocity=self.velocity/2f32;
                self.position[1]-=half_velocity;
                self.position[0]-=half_velocity
            }
            Movement::DownRight=>{
                let half_velocity=self.velocity/2f32;
                self.position[1]-=half_velocity;
                self.position[0]+=half_velocity
            }
            Movement::DownLeft=>{
                let half_velocity=self.velocity/2f32;
                self.position[1]-=half_velocity;
                self.position[0]-=half_velocity
            }
        }
    }

    pub fn move_up_enable(&mut self){
        self.movement=match self.movement{
            Movement::None=>Movement::Up,

            Movement::Down=>Movement::None,
            Movement::Right=>Movement::UpRight,
            Movement::Left=>Movement::UpLeft,

            Movement::DownRight=>Movement::Right,
            Movement::DownLeft=>Movement::Left,

            _=>self.movement
        }
    }

    pub fn move_up_disable(&mut self){
        self.movement=match self.movement{
            Movement::None=>Movement::Down,

            Movement::Up=>Movement::None,
            Movement::Right=>Movement::DownRight,
            Movement::Left=>Movement::DownLeft,

            Movement::UpRight=>Movement::Right,
            Movement::UpLeft=>Movement::Left,

            _=>self.movement
        }
    }

    pub fn move_down_enable(&mut self){
        self.movement=match self.movement{
            Movement::None=>Movement::Down,

            Movement::Up=>Movement::None,
            Movement::Right=>Movement::DownRight,
            Movement::Left=>Movement::DownLeft,

            Movement::UpRight=>Movement::Right,
            Movement::UpLeft=>Movement::Left,

            _=>self.movement
        }
    }

    pub fn move_down_disable(&mut self){
        self.movement=match self.movement{
            Movement::None=>Movement::Up,

            Movement::Down=>Movement::None,
            Movement::Right=>Movement::UpRight,
            Movement::Left=>Movement::UpLeft,

            Movement::DownRight=>Movement::Right,
            Movement::DownLeft=>Movement::Left,

            _=>self.movement
        }
    }

    pub fn move_right_enable(&mut self){
        self.movement=match self.movement{
            Movement::None=>Movement::Right,

            Movement::Up=>Movement::UpRight,
            Movement::Down=>Movement::DownRight,
            Movement::Left=>Movement::None,

            Movement::UpLeft=>Movement::Up,
            Movement::DownLeft=>Movement::Down,

            _=>self.movement
        }
    }

    pub fn move_right_disable(&mut self){
        self.movement=match self.movement{
            Movement::None=>Movement::Left,

            Movement::Up=>Movement::UpLeft,
            Movement::Down=>Movement::DownLeft,
            Movement::Right=>Movement::None,

            Movement::UpRight=>Movement::Up,
            Movement::DownRight=>Movement::Down,

            _=>self.movement
        }
    }

    pub fn move_left_enable(&mut self){
        self.movement=match self.movement{
            Movement::None=>Movement::Left,

            Movement::Up=>Movement::UpLeft,
            Movement::Down=>Movement::DownLeft,
            Movement::Right=>Movement::None,

            Movement::UpRight=>Movement::Up,
            Movement::DownRight=>Movement::Down,

            _=>self.movement
        }
    }

    /// Used only after the `move_left_enable` function had been called.
    pub fn move_left_disable(&mut self){
        self.movement=match self.movement{
            Movement::None=>Movement::Right,

            Movement::Up=>Movement::UpRight,
            Movement::Down=>Movement::DownRight,
            Movement::Left=>Movement::None,

            Movement::UpLeft=>Movement::Up,
            Movement::DownLeft=>Movement::Down,

            _=>self.movement
        }
    }
}