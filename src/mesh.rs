
use cgmath::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub offset: usize,
    pub size: usize
}

pub struct AttributeSpec(pub String, pub usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    pub attributes: Vec<Attribute>,
    pub stride: usize
}

impl Layout {
    pub fn position() -> Layout {
        Layout::new(vec![AttributeSpec("position".to_string(), 3)])
    }
    pub fn position_texcoord() -> Layout {
        Layout::new(vec![AttributeSpec("position".to_string(), 3), AttributeSpec("texcoord".to_string(), 2)])
    }
    pub fn position_texcoord_normal() -> Layout {
        Layout::new(vec![AttributeSpec("position".to_string(), 3), AttributeSpec("texcoord".to_string(), 2), AttributeSpec("normal".to_string(), 3)])
    }
    pub fn new(layout: Vec<AttributeSpec>) -> Layout {
        let mut stride = 0;
        let mut attributes = vec![];
        for AttributeSpec(name, size) in layout {
            attributes.push(Attribute {
                name: name,
                offset: stride.clone(),
                size: size.clone()
            });
            stride += size;
        }
        Layout {
            attributes: attributes,
            stride: stride
        }
    }
    pub fn get_attribute(&self, name: &str) -> Option<&Attribute> {
        self.attributes.iter().find(|x| x.name == name)
    }
}

#[derive(Debug, PartialEq)]
pub struct Mesh {
    pub layout: Layout,
    pub vertex_data: Vec<f32>,
    pub element_data: Vec<u32>
}

impl Mesh {
    pub fn new(layout: Layout, n_vertices: usize, n_faces: usize) -> Mesh {
        let vertices_size = n_vertices * layout.stride;
        let elements_size = (n_faces * 3) as usize;
        let mut vertex_data = Vec::with_capacity(vertices_size);
        for _ in 0..vertices_size {
            vertex_data.push(0.0);
        }
        let mut element_data = Vec::with_capacity(elements_size);
        for _ in 0..elements_size {
            element_data.push(0);
        }
        Mesh {
            vertex_data: vertex_data,
            element_data: element_data,
            layout: layout,
        }
    }
    pub fn write_to_attribute(&mut self, attribute: &Attribute, index: usize, data: Vec<f32>) {
        let i = index * self.layout.stride + attribute.offset;
        for l in 0..attribute.size {
            self.vertex_data[i + l] = data[l];
        }
    }
}
