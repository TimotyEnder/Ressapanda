pub struct ObjVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl ObjVertex {
    pub fn to_string(&self) -> String {
        format!("v {} {} {}", self.x, self.y, self.z)
    }
}
pub struct ObjFaceQuad {
    pub first: u32,
    pub second: u32,
    pub third: u32,
    pub fourth: u32,
}
impl ObjFaceQuad {
    pub fn to_string(&self) -> String {
        format!(
            "f {} {} {} {}",
            self.first, self.second, self.third, self.fourth
        )
    }
}
pub struct ObjExport {
    pub vertices: Vec<ObjVertex>,
    pub face_quads: Vec<ObjFaceQuad>,
    vertex_offset_counter: usize,
}
impl ObjExport {
    pub fn to_string(&self) -> String {
        let mut full_string = String::new();
        for vertex in self.vertices.iter() {
            full_string += &format!("{}\n", vertex.to_string());
        }
        for quad in self.face_quads.iter() {
            full_string += &format!("{}\n", quad.to_string());
        }
        full_string
    }
    pub fn add_vertices_and_quads(&mut self. vertices:Vec<ObjVertex>,quads:Vec<ObjFaceQuad>)
    {
        //
    }
}
