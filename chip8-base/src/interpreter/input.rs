use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;
use VirtualKeyCode::*;
// 1 2 3 C --> 1 2 3 4
// 4 5 6 D --> Q W E R
// 7 8 9 E --> A S D F
// A 0 B F --> Z X C V

const KEYMAP: [VirtualKeyCode; 16] = [
    X,    //0
    Key1, //1
    Key2, //2
    Key3, //3
    Q,    //4
    W,    //5
    E,    //6
    A,    //7
    S,    //8
    D,    //9
    Z,    //A
    C,    //B
    Key4, //C
    R,    //D
    F,    //E
    V,    //F
];

pub fn key_state(input: &WinitInputHelper) -> [bool; 16] {
    KEYMAP.map(|k| input.key_held(k))
}
