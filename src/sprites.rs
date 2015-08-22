use helpers::*;

pub trait Sprite {
    fn draw(&self, frame: &Vec<String>, size: usize) -> Vec<String>;
}

pub struct Jumper {
    pub area: FallArea,
}

impl Sprite for Jumper {
    fn draw(&self, frame: &Vec<String>, pos: usize) -> Vec<String> {
        let idx = (pos - 1) % self.area.width.0;
        let body = ["  \\\\   //  ",
                    "   \\\\ //   ",
                    "====[o]====",
                    "    (O)    "]
                    .iter()
                    .map(|&string| string.to_owned())
                    .collect::<Vec<String>>();
        new_draw(self.area.clone(), body, idx)
    }
}

// pub struct Smokes {
//     jumper: Jumper,
// }
//
// impl Sprite for Smokes {
//     fn draw(&self, frame: &Vec<String>, size: usize) -> Vec<String> {
//         let body: Vec<String> = (1..5).map(|part| {
//             match part {
//                 1 => fill_up(" ", " ", "_", size),     //      __
//                 2 => fill_up("/", "\\", "O", size),    //     /OO\
//                 3 => fill_up("\\", "/", "O", size),    //     \OO/
//                 4 => fill_up(" ", " ", "-", size),     //      --
//                 _ => panic!("Unexpected value!"),
//             }
//         }).collect();
//         merge_draw(frame, body, 1)
//     }
// }
