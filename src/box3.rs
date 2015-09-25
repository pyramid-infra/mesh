
use cgmath::*;
use mesh::*;

pub struct Box3 {
    pub layout: Layout,
    pub position: Vector3<f32>,
    pub size: Vector3<f32>,
    pub position_attribute: String,
    pub texcoord_attribute: String,
    pub normal_attribute: String,
}

impl Box3 {
    pub fn new() -> Box3 {
        Box3 {
            layout: Layout::position_texcoord_normal(),
            position: Vector3::zero(),
            size: Vector3::one(),
            position_attribute: "position".to_string(),
            texcoord_attribute: "texcoord".to_string(),
            normal_attribute: "normal".to_string()
        }
    }
}

impl From<Box3> for Mesh {
    fn from(box3: Box3) -> Mesh {
        let n_vertices = 8*3 as usize;
        let n_faces = 6*2 as usize;
        let mut mesh = Mesh::new(box3.layout.clone(), n_vertices, n_faces);
        let position_attr = mesh.layout.get_attribute(box3.position_attribute.as_str()).cloned();
        let texcoord_attr = mesh.layout.get_attribute(box3.texcoord_attribute.as_str()).cloned();
        let normal_attr = mesh.layout.get_attribute(box3.normal_attribute.as_str()).cloned();
        let min = box3.position;
        let max = box3.position + box3.size;
        if let &Some(ref p_attr) = &position_attr {
            //-Z
            mesh.write_to_attribute(&p_attr, 0, vec![min.x, min.y, min.z]);
            mesh.write_to_attribute(&p_attr, 1, vec![max.x, min.y, min.z]);
            mesh.write_to_attribute(&p_attr, 2, vec![min.x, max.y, min.z]);
            mesh.write_to_attribute(&p_attr, 3, vec![max.x, max.y, min.z]);

            //+Z
            mesh.write_to_attribute(&p_attr, 4, vec![min.x, min.y, max.z]);
            mesh.write_to_attribute(&p_attr, 5, vec![min.x, max.y, max.z]);
            mesh.write_to_attribute(&p_attr, 6, vec![max.x, min.y, max.z]);
            mesh.write_to_attribute(&p_attr, 7, vec![max.x, max.y, max.z]);

            //-X
            mesh.write_to_attribute(&p_attr, 8, vec![min.x, min.y, min.z]);
            mesh.write_to_attribute(&p_attr, 9, vec![min.x, max.y, min.z]);
            mesh.write_to_attribute(&p_attr, 10, vec![min.x, min.y, max.z]);
            mesh.write_to_attribute(&p_attr, 11, vec![min.x, max.y, max.z]);

            //+X
            mesh.write_to_attribute(&p_attr, 12, vec![max.x, max.y, min.z]);
            mesh.write_to_attribute(&p_attr, 13, vec![max.x, min.y, min.z]);
            mesh.write_to_attribute(&p_attr, 14, vec![max.x, max.y, max.z]);
            mesh.write_to_attribute(&p_attr, 15, vec![max.x, min.y, max.z]);

            //-Y
            mesh.write_to_attribute(&p_attr, 16, vec![max.x, min.y, min.z]);
            mesh.write_to_attribute(&p_attr, 17, vec![min.x, min.y, min.z]);
            mesh.write_to_attribute(&p_attr, 18, vec![max.x, min.y, max.z]);
            mesh.write_to_attribute(&p_attr, 19, vec![min.x, min.y, max.z]);

            //+Y
            mesh.write_to_attribute(&p_attr, 20, vec![min.x, max.y, min.z]);
            mesh.write_to_attribute(&p_attr, 21, vec![max.x, max.y, min.z]);
            mesh.write_to_attribute(&p_attr, 22, vec![min.x, max.y, max.z]);
            mesh.write_to_attribute(&p_attr, 23, vec![max.x, max.y, max.z]);
        }
        if let &Some(ref t_attr) = &texcoord_attr {
            //-Z
            mesh.write_to_attribute(&t_attr, 0, vec![0.0, 0.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 1, vec![0.0, 1.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 2, vec![1.0, 0.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 3, vec![1.0, 1.0, 0.0]);

            //+Z
            mesh.write_to_attribute(&t_attr, 4, vec![0.0, 1.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 5, vec![1.0, 1.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 6, vec![0.0, 0.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 7, vec![1.0, 0.0, 0.0]);

            //-X
            mesh.write_to_attribute(&t_attr, 8, vec![0.0, 1.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 9, vec![1.0, 1.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 10, vec![0.0, 0.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 11, vec![1.0, 0.0, 0.0]);

            //+X
            mesh.write_to_attribute(&t_attr, 12, vec![0.0, 1.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 13, vec![1.0, 1.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 14, vec![0.0, 0.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 15, vec![1.0, 0.0, 0.0]);

            //-Y
            mesh.write_to_attribute(&t_attr, 16, vec![0.0, 1.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 17, vec![1.0, 1.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 18, vec![0.0, 0.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 19, vec![1.0, 0.0, 0.0]);

            //+Y
            mesh.write_to_attribute(&t_attr, 20, vec![0.0, 1.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 21, vec![1.0, 1.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 22, vec![0.0, 0.0, 0.0]);
            mesh.write_to_attribute(&t_attr, 23, vec![1.0, 0.0, 0.0]);
        }
        if let &Some(ref n_attr) = &normal_attr {
            //-Z
            mesh.write_to_attribute(&n_attr, 0, vec![0.0, 0.0, -1.0]);
            mesh.write_to_attribute(&n_attr, 1, vec![0.0, 0.0, -1.0]);
            mesh.write_to_attribute(&n_attr, 2, vec![0.0, 0.0, -1.0]);
            mesh.write_to_attribute(&n_attr, 3, vec![0.0, 0.0, -1.0]);

            //+Z
            mesh.write_to_attribute(&n_attr, 4, vec![0.0, 0.0, 1.0]);
            mesh.write_to_attribute(&n_attr, 5, vec![0.0, 0.0, 1.0]);
            mesh.write_to_attribute(&n_attr, 6, vec![0.0, 0.0, 1.0]);
            mesh.write_to_attribute(&n_attr, 7, vec![0.0, 0.0, 1.0]);

            //-X
            mesh.write_to_attribute(&n_attr, 8, vec![-1.0, 0.0, 0.0]);
            mesh.write_to_attribute(&n_attr, 9, vec![-1.0, 0.0, 0.0]);
            mesh.write_to_attribute(&n_attr, 10, vec![-1.0, 0.0, 0.0]);
            mesh.write_to_attribute(&n_attr, 11, vec![-1.0, 0.0, 0.0]);

            //+X
            mesh.write_to_attribute(&n_attr, 12, vec![1.0, 0.0, 0.0]);
            mesh.write_to_attribute(&n_attr, 13, vec![1.0, 0.0, 0.0]);
            mesh.write_to_attribute(&n_attr, 14, vec![1.0, 0.0, 0.0]);
            mesh.write_to_attribute(&n_attr, 15, vec![1.0, 0.0, 0.0]);

            //-Y
            mesh.write_to_attribute(&n_attr, 16, vec![0.0, -1.0, 0.0]);
            mesh.write_to_attribute(&n_attr, 17, vec![0.0, -1.0, 0.0]);
            mesh.write_to_attribute(&n_attr, 18, vec![0.0, -1.0, 0.0]);
            mesh.write_to_attribute(&n_attr, 19, vec![0.0, -1.0, 0.0]);

            //+Y
            mesh.write_to_attribute(&n_attr, 20, vec![0.0, 1.0, 0.0]);
            mesh.write_to_attribute(&n_attr, 21, vec![0.0, 1.0, 0.0]);
            mesh.write_to_attribute(&n_attr, 22, vec![0.0, 1.0, 0.0]);
            mesh.write_to_attribute(&n_attr, 23, vec![0.0, 1.0, 0.0]);
        }
        for i in 0..6 {
            mesh.element_data[(i * 6 + 0) as usize] = i * 4 + 0;
            mesh.element_data[(i * 6 + 1) as usize] = i * 4 + 2;
            mesh.element_data[(i * 6 + 2) as usize] = i * 4 + 1;

            mesh.element_data[(i * 6 + 3) as usize] = i * 4 + 1;
            mesh.element_data[(i * 6 + 4) as usize] = i * 4 + 2;
            mesh.element_data[(i * 6 + 5) as usize] = i * 4 + 3;
        }

        mesh
    }
}

#[test]
fn test_box() {
    let mut box_mesh = Box3::new();
    box_mesh.position = Vector3::new(1.0, 2.0, 0.0);
    box_mesh.size = Vector3::new(6.0, 6.0, 1.0);
    box_mesh.layout = Layout::position();

    let mesh: Mesh = box_mesh.into();
    assert_eq!(mesh, Mesh {
        layout: Layout::position(),
        vertex_data: vec![
            1.0, 2.0, 0.0,
            7.0, 2.0, 0.0,
            1.0, 8.0, 0.0,
            7.0, 8.0, 0.0,
            1.0, 2.0, 1.0,
            1.0, 8.0, 1.0,
            7.0, 2.0, 1.0,
            7.0, 8.0, 1.0,
            1.0, 2.0, 0.0,
            1.0, 8.0, 0.0,
            1.0, 2.0, 1.0,
            1.0, 8.0, 1.0,
            7.0, 8.0, 0.0,
            7.0, 2.0, 0.0,
            7.0, 8.0, 1.0,
            7.0, 2.0, 1.0,
            7.0, 2.0, 0.0,
            1.0, 2.0, 0.0,
            7.0, 2.0, 1.0,
            1.0, 2.0, 1.0,
            1.0, 8.0, 0.0,
            7.0, 8.0, 0.0,
            1.0, 8.0, 1.0,
            7.0, 8.0, 1.0
        ],
        element_data: vec![
            0, 2, 1,
            1, 2, 3,
            4, 6, 5,
            5, 6, 7,
            8, 10, 9,
            9, 10, 11,
            12, 14, 13,
            13, 14, 15,
            16, 18, 17,
            17, 18, 19,
            20, 22, 21,
            21, 22, 23
        ]
    })
}
