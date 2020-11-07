pub struct Box{
    x1:f32,
    y1:f32,
    x2:f32,
    y2:f32
}

pub struct Map{
    current_height:f32,
    character_box:Box,
    next_box:usize,
    last_box:usize,
    boxes:Vec<Box>,
    boxes_drawable:Vec<usize>,
}

// impl Map{
//     pub fn new(char_box:[f32;4])->Map{
//         Self{
//             character_box:Box{

//             },
//             next_box:0,
//             last_box:0
//         }
//     }
// }