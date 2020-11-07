use cat_engine::graphics::{DependentObject, TexturedVertex2D};
use cat_engine::glium::index::{NoIndices, PrimitiveType};
use cat_engine::Colour;

pub struct ImageObject{
    x1:f32,
    y1:f32,
    x2:f32,
    y2:f32,

    ux1:f32,
    uy1:f32,
    ux2:f32,
    uy2:f32,

    colour_filter:[f32;4],
}

impl ImageObject{
    // rect - [x,y,width,height]
    pub fn new(rect:[f32;4],rect_image:[f32;4],colour_filter:[f32;4])->ImageObject{
        Self{
            x1:rect[0],
            y1:rect[1],
            x2:rect[0]+rect[2],
            y2:rect[1]+rect[3],

            ux1:rect_image[0],
            uy1:rect_image[1],
            ux2:rect_image[0]+rect_image[2],
            uy2:rect_image[1]+rect_image[3],

            colour_filter,
        }
    }
}

impl<'o> DependentObject<'o,TexturedVertex2D,u8> for ImageObject{
    type Vertices=[TexturedVertex2D;4];
    type Indices=[u8;1];

    /// Цвет объекта.

    ///

    /// Object's colour.

    fn colour(&self)->Colour{
        self.colour_filter
    }

    /// Вершины объекта.

    ///

    /// Object's vertices.

    fn vertices(&self)->Self::Vertices{
        [
            TexturedVertex2D::new([self.x1,self.y1],[self.ux1,self.uy2]),
            TexturedVertex2D::new([self.x1,self.y2],[self.ux1,self.uy1]),
            TexturedVertex2D::new([self.x2,self.y1],[self.ux2,self.uy2]),
            TexturedVertex2D::new([self.x2,self.y2],[self.ux2,self.uy1])
        ]
    }

    /// Индексы для построения объекта.

    ///

    /// Indices to build the object.

    fn indices(&self)->Option<Self::Indices>{
        None
    }

    fn primitive_type(&self)->PrimitiveType{
        PrimitiveType::TriangleStrip
    }
}