#![feature(convert)]
extern crate cgmath;

use cgmath::*;

pub struct Grid {
    pub layout: Layout,
    pub top_left: Vector2<f32>,
    pub size: Vector2<f32>,
    pub n_vertices: (u32, u32),
    pub uv_min: Vector2<f32>,
    pub uv_max: Vector2<f32>,
    pub normal: Vector3<f32>,
    pub position_attribute: String,
    pub texcoord_attribute: String,
    pub normal_attribute: String,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            layout: Layout::position_texcoord_normal(),
            top_left: Vector2::zero(),
            size: Vector2::one(),
            n_vertices: (2, 2),
            uv_min: Vector2::zero(),
            uv_max: Vector2::one(),
            normal: Vector3::new(0.0, 0.0, 1.0),
            position_attribute: "position".to_string(),
            texcoord_attribute: "texcoord".to_string(),
            normal_attribute: "normal".to_string()
        }
    }
}

impl From<Grid> for Mesh {
    fn from(grid: Grid) -> Mesh {
        let n_vertices = (grid.n_vertices.0 * grid.n_vertices.1) as usize;
        let n_faces = ((grid.n_vertices.0 - 1) * (grid.n_vertices.1 - 1) * 2) as usize;
        let mut mesh = Mesh::new(grid.layout.clone(), n_vertices, n_faces);
        let position_attr = mesh.layout.get_attribute(grid.position_attribute.as_str()).cloned();
        let texcoord_attr = mesh.layout.get_attribute(grid.texcoord_attribute.as_str()).cloned();
        let normal_attr = mesh.layout.get_attribute(grid.normal_attribute.as_str()).cloned();
        for y in 0..grid.n_vertices.1 {
            for x in 0..grid.n_vertices.0 {
                let index = (x + y * grid.n_vertices.0) as usize;
                let p = Vector2::new(
                    x as f32 / (grid.n_vertices.0 as f32 - 1.0),
                    y as f32 / (grid.n_vertices.1 as f32 - 1.0));
                if let &Some(ref p_attr) = &position_attr {
                    mesh.write_to_attribute(&p_attr, index, vec![
                        grid.top_left.x + grid.size.x * p.x,
                        grid.top_left.y + grid.size.y * p.y,
                        0.0
                    ]);
                }
                if let &Some(ref t_attr) = &texcoord_attr {
                    mesh.write_to_attribute(&t_attr, index, vec![
                        grid.uv_min.x + (grid.uv_max.x - grid.uv_min.x) * p.x,
                        grid.uv_min.y + (grid.uv_max.y - grid.uv_min.y) * p.y
                    ]);
                }
                if let &Some(ref n_attr) = &normal_attr {
                    mesh.write_to_attribute(&n_attr, index, vec![
                        grid.normal.x,
                        grid.normal.y,
                        grid.normal.z
                    ]);
                }
                if y < grid.n_vertices.1 - 1 && x < grid.n_vertices.0 - 1 {
                    let index = x + y * (grid.n_vertices.0 - 1);
                    mesh.element_data[(index * 6) as usize] = index;
                    mesh.element_data[(index * 6 + 1) as usize] = index + grid.n_vertices.0;
                    mesh.element_data[(index * 6 + 2) as usize] = index + 1;

                    mesh.element_data[(index * 6 + 3) as usize] = index + 1;
                    mesh.element_data[(index * 6 + 4) as usize] = index + grid.n_vertices.0;
                    mesh.element_data[(index * 6 + 5) as usize] = index + grid.n_vertices.0 + 1;
                }
            }
        }
        mesh
    }
}

#[test]
fn test_grid() {
    let mut grid = Grid::new();
    grid.top_left = Vector2::new(1.0, 2.0);
    grid.size = Vector2::new(6.0, 6.0);
    grid.n_vertices = (3, 2);

    let mesh: Mesh = grid.into();
    assert_eq!(mesh, Mesh {
        layout: Layout::position_texcoord_normal(),
        vertex_data: vec![
            1.0, 2.0, 0.0,  0.0, 0.0,  0.0, 0.0, 1.0,
            4.0, 2.0, 0.0,  0.5, 0.0,  0.0, 0.0, 1.0,
            7.0, 2.0, 0.0,  1.0, 0.0,  0.0, 0.0, 1.0,
            1.0, 8.0, 0.0,  0.0, 1.0,  0.0, 0.0, 1.0,
            4.0, 8.0, 0.0,  0.5, 1.0,  0.0, 0.0, 1.0,
            7.0, 8.0, 0.0,  1.0, 1.0,  0.0, 0.0, 1.0,
        ],
        element_data: vec![
            0, 3, 1,
            1, 3, 4,
            1, 4, 2,
            2, 4, 5]
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub offset: usize,
    pub size: usize
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    pub attributes: Vec<Attribute>,
    pub stride: usize
}

impl Layout {
    pub fn position() -> Layout {
        Layout::new(vec![("position".to_string(), 3)])
    }
    pub fn position_texcoord() -> Layout {
        Layout::new(vec![("position".to_string(), 3), ("texcoord".to_string(), 2)])
    }
    pub fn position_texcoord_normal() -> Layout {
        Layout::new(vec![("position".to_string(), 3), ("texcoord".to_string(), 2), ("normal".to_string(), 3)])
    }
    pub fn new(layout: Vec<(String, usize)>) -> Layout {
        let mut stride = 0;
        let mut attributes = vec![];
        for (name, size) in layout {
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
