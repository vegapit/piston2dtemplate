#[derive(Clone)]
pub struct Screen{
    pub width: f64,
    pub height: f64
}

impl Default for Screen {
    
    fn default() -> Self {
        Self {
            height: 600.0,
            width: 800.0
        }
    }
    
}

impl Screen {

    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width: width,
            height: height
        }
    }

    pub fn as_array(&self) -> [ f64; 2 ] {
        [self.width, self.height]
    }

}