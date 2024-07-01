#[derive(Clone, Copy, Debug)]
pub enum CubeColors {
    Red,
    Blue,
    Yellow,
    Orange,
    Green,
    White,
    None,
}
#[derive(Debug)]
pub struct Face {
    face_data: [[CubeColors; 3]; 3],
}

impl Face {
    pub fn from(color: CubeColors) -> Face {
        Face {
            face_data: [[color; 3]; 3],
        }
    }
    pub fn face_data(&self) -> [String; 3] {
        let mut result: [String; 3] = [String::new(), String::new(), String::new()];

        self.write_color(&mut result, 0);

        self.write_color(&mut result, 1);

        self.write_color(&mut result, 2);

        return result;
    }
    fn write_color(&self, dest: &mut [String; 3], pos_x: usize) {
        let mut result = String::new();
        result += self.face_data[pos_x][0].color().as_str();
        result += self.face_data[pos_x][1].color().as_str();
        result += self.face_data[pos_x][2].color().as_str();
        dest[pos_x] = result;
    }
    pub fn get_up(&self) -> [CubeColors; 3] {
        self.face_data[0]
    }
    pub fn set_up(&mut self, new_up: [CubeColors; 3]) {
        self.face_data[0] = new_up;
    }
    pub fn get_down(&self) -> [CubeColors; 3] {
        self.face_data[2]
    }
    pub fn set_down(&mut self, new_down: [CubeColors; 3]) {
        self.face_data[2] = new_down;
    }
    pub fn get_right(&self) -> [CubeColors; 3] {
        [
            self.face_data[0][2],
            self.face_data[1][2],
            self.face_data[2][2],
        ]
    }
    pub fn set_right(&mut self, new_right: [CubeColors; 3]) {
        self.face_data[0][2] = new_right[0];
        self.face_data[1][2] = new_right[1];
        self.face_data[2][2] = new_right[2];
    }
    pub fn get_left(&self) -> [CubeColors; 3] {
        [
            self.face_data[0][0],
            self.face_data[1][0],
            self.face_data[2][0],
        ]
    }
    pub fn set_left(&mut self, new_left: [CubeColors; 3]) {
        self.face_data[0][0] = new_left[0];
        self.face_data[1][0] = new_left[1];
        self.face_data[2][0] = new_left[2];
    }
    pub fn get_face(&self) -> [[CubeColors; 3]; 3] {
        self.face_data
    }
    pub fn set_face(&mut self, new_face: [[CubeColors; 3]; 3]) {
        self.face_data = new_face;
    }
    pub fn rotate(&mut self, direction: i8) {
        if direction > 0 {
            let first_element = self.face_data[0][0];
            self.face_data[0][0] = self.face_data[1][0];
            self.face_data[1][0] = self.face_data[2][0];
            self.face_data[2][0] = self.face_data[2][1];
            self.face_data[2][1] = self.face_data[2][2];
            self.face_data[2][2] = self.face_data[1][2];
            self.face_data[1][2] = self.face_data[0][2];
            self.face_data[0][2] = self.face_data[0][1];
            self.face_data[0][1] = first_element;
        } else {
            let first_element = self.face_data[0][0];
            self.face_data[0][0] = self.face_data[0][1];
            self.face_data[0][1] = self.face_data[0][2];
            self.face_data[0][2] = self.face_data[1][2];
            self.face_data[1][2] = self.face_data[2][2];
            self.face_data[2][2] = self.face_data[2][1];
            self.face_data[2][1] = self.face_data[2][0];
            self.face_data[2][0] = self.face_data[1][0];
            self.face_data[1][0] = first_element;
        }
    }
}

impl CubeColors {
    fn color(&self) -> String {
        match self {
            CubeColors::None => " ".to_string(),
            _ => {
                let color = format!("{:?}", self)
                    .chars()
                    .nth(0)
                    .unwrap_or(' ')
                    .to_string();
                color
            }
        }
    }
}
