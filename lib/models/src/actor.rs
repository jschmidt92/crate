pub struct Actor {
    pub uid: String,
    pub position: Option<(f32, f32, f32)>,
    pub direction: Option<f32>,
    pub stance: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub bank: Option<f64>,
    pub cash: Option<f64>,
    pub state: Option<String>
}