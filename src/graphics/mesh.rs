
#[allow(dead_code)]
pub struct Mesh {
    vertices: Vec<(f32, f32)>,
    uv: Vec<(f32, f32)>,
    indices: Vec<u32>,
    has_uv: bool
}

#[allow(dead_code)]
impl Mesh {
    pub fn new() -> Mesh {
        return Mesh {
            vertices: Vec::new(),
            uv: Vec::new(),
            indices: Vec::new(),
            has_uv: false
        };
    }

    pub fn rect(width: f32, height: f32) -> Mesh {
        let mut mesh = Mesh::new();

        mesh.add_vert(((-width / 2.0), (-height / 2.0))); //TL
        mesh.add_vert(((-width / 2.0), (height / 2.0))); //BL
        mesh.add_vert((width / 2.0, height / 2.0)); //BR
        mesh.add_vert((width / 2.0, -height / 2.0)); //TR

        mesh.add_uv((0., 0.));
        mesh.add_uv((0., 1.));
        mesh.add_uv((1., 1.));
        mesh.add_uv((1., 0.));

        mesh.set_indices(vec![0, 1, 2, 0, 2, 3]);

        return mesh;
    }

    pub fn has_uvs(&self) -> &bool {
        return &self.has_uv;
    }

    pub fn add_vert(&mut self, vert: (f32, f32)) {
        self.has_uv = true;
        self.vertices.push(vert);
    }

    pub fn add_uv(&mut self, uv: (f32, f32)) {
        self.uv.push(uv);
    }

    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = indices;
    }

    pub fn get_indices(&self) -> &Vec<u32> {
        return &self.indices;
    }

    pub fn get_verts(&self) -> &Vec<(f32, f32)> {
        return &self.vertices;
    }

    pub fn get_uvs(&self) -> &Vec<(f32, f32)> {
        return &self.uv;
    }
}