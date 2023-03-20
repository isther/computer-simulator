use instructions::{
    IOMode, Instructions, Label, Number, Register, SafeInstruction, Symbol, ADD, AND, CALL, CLF,
    CMP, DATA, DEFLABEL, DEFSYMBOL, IN, JMP, JMPF, JR, LOAD, NOT, OUT, SHL, STORE, XOR,
};
use lazy_static::lazy_static;
use std::{collections::HashMap, rc::Rc};
lazy_static! {
    static ref CHARACTERS: HashMap<char, [u16; 8]> = {
        let mut map = HashMap::new();
        map.insert(
            ' ',
            [
                0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            ],
        );
        map.insert(
            '!',
            [0x0010, 0x0010, 0x0010, 0x0010, 0x0010, 0x000, 0x0010, 0x000],
        );
        map.insert(
            '"',
            [0x0028, 0x0028, 0x000, 0x000, 0x000, 0x000, 0x000, 0x000],
        );
        map.insert(
            '\'',
            [
                0x0020, 0x0020, 0x0020, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
            ],
        );
        map.insert(
            '#',
            [
                0x0028, 0x0028, 0x007C, 0x0028, 0x007C, 0x0028, 0x0028, 0x000,
            ],
        );
        map.insert(
            '%',
            [0x00C2, 0x00C4, 0x008, 0x0010, 0x0020, 0x004C, 0x008C, 0x000],
        );
        map.insert(
            '$',
            [
                0x0010, 0x007E, 0x0090, 0x007C, 0x0012, 0x00FC, 0x0010, 0x000,
            ],
        );
        map.insert(
            '&',
            [
                0x0038, 0x0028, 0x0038, 0x00E0, 0x0094, 0x0088, 0x00F4, 0x000,
            ],
        );
        map.insert(
            '(',
            [0x008, 0x0010, 0x0020, 0x0020, 0x0020, 0x0010, 0x008, 0x000],
        );
        map.insert(
            ')',
            [0x0020, 0x0010, 0x008, 0x008, 0x008, 0x0010, 0x0020, 0x000],
        );
        map.insert(
            '*',
            [0x000, 0x0092, 0x0054, 0x0038, 0x0038, 0x0054, 0x0092, 0x000],
        );
        map.insert(
            '+',
            [0x000, 0x0010, 0x0010, 0x007C, 0x0030, 0x0010, 0x000, 0x000],
        );
        map.insert(
            '/',
            [0x002, 0x004, 0x008, 0x0010, 0x0020, 0x0040, 0x0080, 0x000],
        );
        map.insert(
            '.',
            [0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x0010, 0x000],
        );
        map.insert(
            ',',
            [0x000, 0x000, 0x000, 0x000, 0x008, 0x008, 0x0010, 0x000],
        );
        map.insert(
            '-',
            [0x000, 0x000, 0x000, 0x007C, 0x000, 0x000, 0x000, 0x000],
        );
        map.insert(
            '=',
            [0x000, 0x000, 0x00FE, 0x000, 0x00FE, 0x000, 0x000, 0x000],
        );
        map.insert(
            '>',
            [0x0040, 0x0020, 0x0010, 0x008, 0x0010, 0x0020, 0x0040, 0x000],
        );
        map.insert(
            '<',
            [0x002, 0x004, 0x008, 0x0010, 0x008, 0x004, 0x002, 0x000],
        );
        map.insert(
            '|',
            [
                0x0010, 0x0010, 0x0010, 0x0010, 0x0010, 0x0010, 0x0010, 0x000,
            ],
        );
        map.insert(
            ']',
            [
                0x0030, 0x0010, 0x0010, 0x0010, 0x0010, 0x0010, 0x0030, 0x000,
            ],
        );
        map.insert(
            '[',
            [
                0x0030, 0x0020, 0x0020, 0x0020, 0x0020, 0x0020, 0x0030, 0x000,
            ],
        );
        map.insert(
            '\\',
            [0x0080, 0x0040, 0x0020, 0x0010, 0x008, 0x004, 0x002, 0x000],
        );
        map.insert(
            '~',
            [0x000, 0x000, 0x000, 0x0032, 0x004C, 0x000, 0x000, 0x000],
        );
        map.insert(
            ']',
            [0x0030, 0x008, 0x00C, 0x002, 0x00C, 0x008, 0x0030, 0x000],
        );
        map.insert(
            '[',
            [
                0x0010, 0x0020, 0x0060, 0x0080, 0x0060, 0x0020, 0x0010, 0x000,
            ],
        );
        map.insert(
            '_',
            [0x000, 0x000, 0x000, 0x000, 0x000, 0x000, 0x007E, 0x000],
        );
        map.insert(
            '`',
            [0x000, 0x0020, 0x0010, 0x008, 0x000, 0x000, 0x000, 0x000],
        );
        map.insert(
            '^',
            [0x0010, 0x0028, 0x0044, 0x000, 0x000, 0x000, 0x000, 0x000],
        );
        map.insert(
            ',',
            [0x000, 0x0010, 0x000, 0x000, 0x0010, 0x000, 0x000, 0x000],
        );
        map.insert(
            ';',
            [0x000, 0x0010, 0x000, 0x000, 0x0010, 0x0020, 0x000, 0x000],
        );
        map.insert(
            '?',
            [0x007C, 0x0042, 0x002, 0x004, 0x008, 0x000, 0x008, 0x000],
        );
        map.insert(
            '@',
            [
                0x007C, 0x008A, 0x009C, 0x00A8, 0x0098, 0x0084, 0x0078, 0x000,
            ],
        );
        map.insert(
            'A',
            [
                0x007C, 0x00C6, 0x0082, 0x00FE, 0x0082, 0x0082, 0x0082, 0x0000,
            ],
        );
        map.insert(
            'B',
            [
                0x00FC, 0x0086, 0x0082, 0x00FE, 0x0082, 0x0086, 0x00FC, 0x0000,
            ],
        );
        map.insert(
            'C',
            [
                0x007E, 0x00C0, 0x0080, 0x0080, 0x0080, 0x00C0, 0x007E, 0x0000,
            ],
        );
        map.insert(
            'D',
            [
                0x00F8, 0x0086, 0x0082, 0x0082, 0x0082, 0x0086, 0x00F8, 0x0000,
            ],
        );
        map.insert(
            'E',
            [
                0x007E, 0x00C0, 0x0080, 0x00FE, 0x0080, 0x00C0, 0x007E, 0x0000,
            ],
        );
        map.insert(
            'F',
            [
                0x007E, 0x0080, 0x0080, 0x00FC, 0x0080, 0x0080, 0x0080, 0x0000,
            ],
        );
        map.insert(
            'G',
            [
                0x007E, 0x0080, 0x0080, 0x009C, 0x0082, 0x0082, 0x00FE, 0x0000,
            ],
        );
        map.insert(
            'H',
            [
                0x0082, 0x0082, 0x0082, 0x00FE, 0x0082, 0x0082, 0x0082, 0x0000,
            ],
        );
        map.insert(
            'I',
            [
                0x00FE, 0x0010, 0x0010, 0x0010, 0x0010, 0x0010, 0x00FE, 0x0000,
            ],
        );
        map.insert(
            'J',
            [
                0x0002, 0x0002, 0x0002, 0x0002, 0x0002, 0x0002, 0x00FC, 0x0000,
            ],
        );
        map.insert(
            'K',
            [
                0x00C4, 0x00C8, 0x00F0, 0x00E0, 0x00D8, 0x00C4, 0x00C6, 0x000,
            ],
        );
        map.insert(
            'L',
            [
                0x0080, 0x0080, 0x0080, 0x0080, 0x0080, 0x0080, 0x007E, 0x0000,
            ],
        );
        map.insert(
            'M',
            [
                0x0066, 0x00aa, 0x0092, 0x0092, 0x0082, 0x0082, 0x0082, 0x0000,
            ],
        );
        map.insert(
            'N',
            [
                0x00C2, 0x00a2, 0x0092, 0x0092, 0x008A, 0x008A, 0x0086, 0x0000,
            ],
        );
        map.insert(
            'O',
            [
                0x007C, 0x0082, 0x0082, 0x0082, 0x0082, 0x0082, 0x007C, 0x000,
            ],
        );
        map.insert(
            'P',
            [
                0x00FC, 0x0082, 0x0082, 0x001FC, 0x0080, 0x0080, 0x0080, 0x000,
            ],
        );
        map.insert(
            'Q',
            [
                0x0078, 0x0084, 0x0084, 0x0084, 0x0094, 0x008C, 0x0076, 0x007,
            ],
        );
        map.insert(
            'R',
            [
                0x00FC, 0x0082, 0x0082, 0x00FC, 0x00A0, 0x0090, 0x008E, 0x000,
            ],
        );
        map.insert(
            'S',
            [0x007C, 0x0080, 0x0080, 0x007C, 0x004, 0x004, 0x00F8, 0x000],
        );
        map.insert(
            'T',
            [
                0x00FE, 0x0010, 0x0010, 0x0010, 0x0010, 0x0010, 0x0010, 0x000,
            ],
        );
        map.insert(
            'U',
            [
                0x00C6, 0x0042, 0x0042, 0x0042, 0x0042, 0x0042, 0x003C, 0x000,
            ],
        );
        map.insert(
            'V',
            [
                0x0082, 0x0082, 0x0082, 0x0082, 0x0044, 0x006C, 0x0010, 0x000,
            ],
        );
        map.insert(
            'W',
            [
                0x0082, 0x0082, 0x0082, 0x0092, 0x00BA, 0x00AA, 0x0044, 0x000,
            ],
        );
        map.insert(
            'X',
            [
                0x00C6, 0x0044, 0x0028, 0x0010, 0x0028, 0x0044, 0x00C6, 0x000,
            ],
        );
        map.insert(
            'Y',
            [
                0x00C6, 0x0044, 0x0028, 0x0010, 0x0010, 0x0010, 0x0038, 0x000,
            ],
        );
        map.insert(
            'Z',
            [0x00FE, 0x0082, 0x00C, 0x0038, 0x0060, 0x0082, 0x007E, 0x000],
        );
        map.insert(
            'a',
            [
                0x007c, 0x00c6, 0x0082, 0x00fe, 0x0082, 0x0082, 0x0082, 0x0000,
            ],
        );
        map.insert(
            'b',
            [
                0x00fc, 0x0086, 0x0082, 0x00fe, 0x0082, 0x0086, 0x00fc, 0x0000,
            ],
        );
        map.insert(
            'c',
            [
                0x007e, 0x00c0, 0x0080, 0x0080, 0x0080, 0x00c0, 0x007e, 0x0000,
            ],
        );
        map.insert(
            'd',
            [
                0x00f8, 0x0086, 0x0082, 0x0082, 0x0082, 0x0086, 0x00f8, 0x0000,
            ],
        );
        map.insert(
            'e',
            [
                0x007e, 0x00c0, 0x0080, 0x00fe, 0x0080, 0x00c0, 0x007e, 0x0000,
            ],
        );
        map.insert(
            'f',
            [
                0x007e, 0x0080, 0x0080, 0x00fc, 0x0080, 0x0080, 0x0080, 0x0000,
            ],
        );
        map.insert(
            'g',
            [
                0x007e, 0x0080, 0x0080, 0x009c, 0x0082, 0x0082, 0x00fe, 0x0000,
            ],
        );
        map.insert(
            'h',
            [
                0x0082, 0x0082, 0x0082, 0x00fe, 0x0082, 0x0082, 0x0082, 0x0000,
            ],
        );
        map.insert(
            'i',
            [
                0x00fe, 0x0010, 0x0010, 0x0010, 0x0010, 0x0010, 0x00fe, 0x0000,
            ],
        );
        map.insert(
            'j',
            [
                0x0002, 0x0002, 0x0002, 0x0002, 0x0002, 0x0002, 0x00fc, 0x0000,
            ],
        );
        map.insert(
            'k',
            [
                0x00c4, 0x00c8, 0x00f0, 0x00e0, 0x00d8, 0x00c4, 0x00c6, 0x000,
            ],
        );
        map.insert(
            'l',
            [
                0x0080, 0x0080, 0x0080, 0x0080, 0x0080, 0x0080, 0x007e, 0x0000,
            ],
        );
        map.insert(
            'm',
            [
                0x0066, 0x00aa, 0x0092, 0x0092, 0x0082, 0x0082, 0x0082, 0x0000,
            ],
        );
        map.insert(
            'n',
            [
                0x00c2, 0x00a2, 0x0092, 0x0092, 0x008a, 0x008a, 0x0086, 0x0000,
            ],
        );
        map.insert(
            'o',
            [
                0x007c, 0x0082, 0x0082, 0x0082, 0x0082, 0x0082, 0x007c, 0x000,
            ],
        );
        map.insert(
            'p',
            [
                0x00fc, 0x0082, 0x0082, 0x001fc, 0x0080, 0x0080, 0x0080, 0x000,
            ],
        );
        map.insert(
            'q',
            [
                0x0078, 0x0084, 0x0084, 0x0084, 0x0094, 0x008c, 0x0076, 0x007,
            ],
        );
        map.insert(
            'r',
            [
                0x00fc, 0x0082, 0x0082, 0x00fc, 0x00a0, 0x0090, 0x008e, 0x000,
            ],
        );
        map.insert(
            's',
            [0x007c, 0x0080, 0x0080, 0x007c, 0x004, 0x004, 0x00f8, 0x000],
        );
        map.insert(
            't',
            [
                0x00fe, 0x0010, 0x0010, 0x0010, 0x0010, 0x0010, 0x0010, 0x000,
            ],
        );
        map.insert(
            'u',
            [
                0x00c6, 0x0042, 0x0042, 0x0042, 0x0042, 0x0042, 0x003c, 0x000,
            ],
        );
        map.insert(
            'v',
            [
                0x0082, 0x0082, 0x0082, 0x0082, 0x0044, 0x006c, 0x0010, 0x000,
            ],
        );
        map.insert(
            'w',
            [
                0x0082, 0x0082, 0x0082, 0x0092, 0x00ba, 0x00aa, 0x0044, 0x000,
            ],
        );
        map.insert(
            'x',
            [
                0x00c6, 0x0044, 0x0028, 0x0010, 0x0028, 0x0044, 0x00c6, 0x000,
            ],
        );
        map.insert(
            'y',
            [
                0x00c6, 0x0044, 0x0028, 0x0010, 0x0010, 0x0010, 0x0038, 0x000,
            ],
        );
        map.insert(
            'z',
            [0x00fe, 0x0082, 0x00c, 0x0038, 0x0060, 0x0082, 0x007e, 0x000],
        );
        map.insert(
            '0',
            [
                0x007C, 0x00E2, 0x00A2, 0x0092, 0x008A, 0x008E, 0x007C, 0x000,
            ],
        );
        map.insert(
            '1',
            [
                0x0038, 0x0058, 0x0018, 0x0018, 0x0018, 0x0018, 0x007E, 0x000,
            ],
        );
        map.insert(
            '2',
            [
                0x007C, 0x0082, 0x001C, 0x0020, 0x0040, 0x0080, 0x00FE, 0x000,
            ],
        );
        map.insert(
            '3',
            [0x007C, 0x002, 0x002, 0x001E, 0x002, 0x002, 0x00FC, 0x000],
        );
        map.insert(
            '4',
            [0x001C, 0x0024, 0x0044, 0x0084, 0x00FE, 0x004, 0x004, 0x000],
        );
        map.insert(
            '5',
            [0x00FE, 0x0080, 0x00F8, 0x004, 0x002, 0x006, 0x00FC, 0x000],
        );
        map.insert(
            '6',
            [
                0x003E, 0x0040, 0x00F8, 0x0084, 0x0082, 0x0086, 0x00FC, 0x000,
            ],
        );
        map.insert(
            '7',
            [0x00FE, 0x002, 0x004, 0x008, 0x0010, 0x0020, 0x0040, 0x000],
        );
        map.insert(
            '8',
            [
                0x007C, 0x0082, 0x0082, 0x007C, 0x0082, 0x0082, 0x007C, 0x000,
            ],
        );
        map.insert(
            '9',
            [0x007C, 0x0082, 0x0082, 0x007E, 0x002, 0x0082, 0x007C, 0x000],
        );
        map.insert(
            '0',
            [
                0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
            ],
        );

        map
    };
}

pub fn initialise_common_code() -> Vec<SafeInstruction> {
    let mut instructions = Instructions::new();

    instructions.add(vec![
        Rc::new(DEFSYMBOL::new("LINE-WIDTH", 0x001E)),
        Rc::new(DEFSYMBOL::new("CALL-RETURN-ADDRESS", 0xFF33)),
        Rc::new(DEFSYMBOL::new("ONE", 0x0001)),
        Rc::new(DEFSYMBOL::new("LINEX", 0xFF01)),
        Rc::new(DEFSYMBOL::new("PEN-POSITION-ADDR", 0x0400)),
        Rc::new(DEFSYMBOL::new("KEYCODE-REGISTER", 0x0401)),
        Rc::new(DEFSYMBOL::new("DISPLAY-ADAPTER-ADDR", 0x0007)),
        Rc::new(DEFSYMBOL::new("KEY-ADAPTER-ADDR", 0x000F)),
    ]);

    instructions.add(vec![
        Rc::new(DATA::new(Register::REG0, Symbol::new("LINEX"))),
        Rc::new(DATA::new(Register::REG1, Number::new(0x0000))),
        Rc::new(STORE::new(Register::REG0, Register::REG1)),
    ]);

    // jump to main
    instructions.add(vec![Rc::new(DEFLABEL::new("start"))]);
    instructions.add_blocks(vec![call_routine("ROUTINE-init-fontDescriptions")]);
    instructions.add(vec![Rc::new(JMP::new(Label::new("main")))]);

    instructions.add(vec![Rc::new(DEFLABEL::new("ROUTINES"))]);
    instructions.add_blocks(vec![routine_load_font_descriptions(
        "ROUTINE-init-fontDescriptions",
    )]);
    instructions.add_blocks(vec![routine_draw_font_character(
        "ROUTINE-io-drawFontCharacter",
    )]);
    instructions.add_blocks(vec![routine_poll_keyboard("ROUTINE-io-pollKeyboard")]);

    instructions.get()
}

pub fn call_routine(routine: &str) -> Vec<SafeInstruction> {
    vec![Rc::new(CALL::new(Label::new(routine)))]
}

fn routine_load_font_descriptions(lable: &str) -> Vec<SafeInstruction> {
    let mut instructions = Instructions::new();

    instructions.add(vec![Rc::new(DEFLABEL::new(lable))]);

    for i in CHARACTERS.keys() {
        instructions.add_blocks(vec![load_font_character_into_font_region(i)]);
    }

    instructions.add(vec![Rc::new(JR::new(Register::REG3))]);
    instructions.get()
}

fn load_font_character_into_font_region(c: &char) -> Vec<SafeInstruction> {
    let font_description = CHARACTERS.get(c).unwrap();
    let mut instructions = Instructions::new();

    for i in 0..8 {
        let line = font_description[i];
        instructions.add(vec![Rc::new(DATA::new(
            Register::REG0,
            Number::new((*c as u16) << 3 + i as u16),
        ))]);
        instructions.add(vec![Rc::new(DATA::new(Register::REG1, Number::new(line)))]);
        instructions.add(vec![Rc::new(STORE::new(Register::REG0, Register::REG1))]);
    }

    instructions.get()
}

fn routine_draw_font_character(label_prefix: &str) -> Vec<SafeInstruction> {
    let font_y_addr = 0xFF00;
    let mut instructions = Instructions::new();

    instructions.add(vec![Rc::new(DEFLABEL::new(label_prefix))]);

    instructions.add(vec![
        Rc::new(DATA::new(
            Register::REG2,
            Symbol::new("CALL-RETURN-ADDRESS"),
        )),
        Rc::new(STORE::new(Register::REG2, Register::REG3)),
        Rc::new(DATA::new(Register::REG2, Symbol::new("PEN-POSITION-ADDR"))),
        Rc::new(LOAD::new(Register::REG2, Register::REG2)),
    ]);

    // we can keep this value in reg2 to track where in display RAM we are writing
    let pen_position_register = Register::REG2;

    // counter for what line of the font are we rendering
    instructions.add(vec![
        Rc::new(DATA::new(Register::REG0, Number::new(font_y_addr))),
        Rc::new(DATA::new(Register::REG1, Number::new(0x0000))),
        Rc::new(STORE::new(Register::REG0, Register::REG1)),
    ]);
    instructions.add(vec![
        Rc::new(DATA::new(Register::REG3, Symbol::new("KEYCODE-REGISTER"))),
        Rc::new(LOAD::new(Register::REG3, Register::REG3)),
        Rc::new(DATA::new(Register::REG1, Number::new(0x0101))),
        Rc::new(CMP::new(Register::REG3, Register::REG1)),
        Rc::new(JMPF::new(
            vec!["E".to_string()],
            Label::new(&(label_prefix.to_owned() + "-carriage-return")),
        )),
    ]);
    instructions.add_blocks(vec![select_display_adapter(Register::REG3)]);

    // calculate memory position of font line
    // start of loop:
    instructions.add(vec![
        Rc::new(DEFLABEL::new(&(label_prefix.to_owned() + "-STARTLOOP"))),
        Rc::new(DATA::new(Register::REG3, Symbol::new("KEYCODE-REGISTER"))), // load keycode
        Rc::new(LOAD::new(Register::REG3, Register::REG3)),
        Rc::new(SHL::new(Register::REG3)),
        Rc::new(SHL::new(Register::REG3)),
        Rc::new(SHL::new(Register::REG3)), // memory address in RAM for start of font
        Rc::new(DATA::new(Register::REG0, Number::new(font_y_addr))), // fontY address
        Rc::new(LOAD::new(Register::REG0, Register::REG0)), // load fontY
        Rc::new(ADD::new(Register::REG0, Register::REG3)), // calculate memory position of fontstart+fontYinstructions = append(instructions, ADD{asm.REG0, asm.REG3})    calculate memory position of fontstart+fontY
        //increment fontY by 1
        Rc::new(DATA::new(Register::REG1, Symbol::new("ONE"))), // one
        Rc::new(ADD::new(Register::REG1, Register::REG0)),      // increment fontY by 1
        Rc::new(DATA::new(Register::REG1, Number::new(font_y_addr))), // fontY address
        Rc::new(STORE::new(Register::REG1, Register::REG0)), // store new value of fontY in memory
        // load font line from memory
        Rc::new(LOAD::new(Register::REG3, Register::REG0)), // load value from memory into reg0
        // write to display ram
        Rc::new(OUT::new(IOMode::DataMode, pen_position_register)), // display RAM address
        Rc::new(OUT::new(IOMode::DataMode, Register::REG0)),        // display RAM value
        Rc::new(DATA::new(Register::REG1, Symbol::new("LINE-WIDTH"))),
        Rc::new(ADD::new(Register::REG1, pen_position_register)), // move pen down by 1 line
        // check if we have rendered all 8 lines
        Rc::new(DATA::new(Register::REG0, Number::new(font_y_addr))), // fontY addr
        Rc::new(LOAD::new(Register::REG0, Register::REG0)),           //load fontY into reg0
        Rc::new(DATA::new(Register::REG1, Number::new(0x0007))),
        Rc::new(CMP::new(Register::REG0, Register::REG1)), // if fontY == 0x0007 then we have rendered the last line
        // if all 8 lines rendered, jump out of loop, we're done
        Rc::new(JMPF::new(
            vec!["E".to_string()],
            Label::new(&(label_prefix.to_owned() + "-ENDLOOP")),
        )),
        // otherwise jump back to start of loop and render next line of font
        Rc::new(JMP::new(Label::new(
            &(label_prefix.to_owned() + "-STARTLOOP"),
        ))),
    ]);

    //update pen position we are moving to the next character
    instructions.add(vec![
        Rc::new(DEFLABEL::new(&(label_prefix.to_owned() + "-ENDLOOP"))),
        // increment line x
        Rc::new(DATA::new(Register::REG1, Symbol::new("LINEX"))),
        Rc::new(LOAD::new(Register::REG1, Register::REG1)),
        Rc::new(DATA::new(Register::REG2, Symbol::new("ONE"))),
        Rc::new(ADD::new(Register::REG2, Register::REG1)), //increment line X
        Rc::new(DATA::new(Register::REG2, Symbol::new("LINEX"))),
        Rc::new(STORE::new(Register::REG2, Register::REG1)),
        Rc::new(DATA::new(Register::REG0, Symbol::new("PEN-POSITION-ADDR"))),
        Rc::new(LOAD::new(Register::REG0, Register::REG0)),
        // test if pen needs to be moved down
        Rc::new(DATA::new(Register::REG3, Number::new(0x001E))), // have we reached the end of the line?
        Rc::new(CMP::new(Register::REG1, Register::REG3)),
        Rc::new(JMPF::new(
            vec!["E".to_string()],
            Label::new(&(label_prefix.to_owned() + "-carriage-return")),
        )),
        Rc::new(JMP::new(Label::new(
            &(label_prefix.to_owned() + "-increment-cursor"),
        ))),
        Rc::new(DEFLABEL::new(
            &(label_prefix.to_owned() + "-increment-cursor"),
        )),
        Rc::new(DATA::new(Register::REG1, Symbol::new("ONE"))), // one
        Rc::new(ADD::new(Register::REG1, Register::REG0)),      // increment pen position by 1
        Rc::new(DATA::new(Register::REG1, Symbol::new("PEN-POSITION-ADDR"))),
        Rc::new(STORE::new(Register::REG1, Register::REG0)), // store new value of pen position in memory
        Rc::new(JMP::new(Label::new(
            &(label_prefix.to_owned() + "-deselectIO"),
        ))),
        // used for when the enter key is hit and you need to return
        // this is absolutely monstrous
        Rc::new(DEFLABEL::new(
            &(label_prefix.to_owned() + "-carriage-return"),
        )),
        Rc::new(DATA::new(Register::REG1, Symbol::new("LINEX"))),
        Rc::new(LOAD::new(Register::REG1, Register::REG1)), // retrieve linex
        // if linex == 0
        Rc::new(DATA::new(Register::REG2, Number::new(0x0000))),
        Rc::new(DATA::new(Register::REG3, Number::new(0x00F0))),
        Rc::new(CMP::new(Register::REG1, Register::REG2)),
        Rc::new(JMPF::new(
            vec!["E".to_string()],
            Label::new(&(label_prefix.to_owned() + "-reposition-pen")),
        )),
        // if linex == 1...
        Rc::new(DATA::new(Register::REG2, Symbol::new("ONE"))),
        Rc::new(DATA::new(Register::REG3, Number::new(0x00EF))),
        Rc::new(CMP::new(Register::REG1, Register::REG2)),
        Rc::new(JMPF::new(
            vec!["E".to_string()],
            Label::new(&(label_prefix.to_owned() + "-reposition-pen")),
        )),
        // if linex == end of line
        Rc::new(DATA::new(Register::REG2, Symbol::new("LINE-WIDTH"))),
        Rc::new(DATA::new(Register::REG3, Number::new(0x00F1))),
        Rc::new(CMP::new(Register::REG1, Register::REG2)),
        Rc::new(JMPF::new(
            vec!["E".to_string()],
            Label::new(&(label_prefix.to_owned() + "-reposition-pen")),
        )),
        // otherwise calculate difference and move pen down
        Rc::new(DEFLABEL::new(
            &(label_prefix.to_owned() + "-reposition-pen-when-midline"),
        )),
        Rc::new(DATA::new(Register::REG2, Symbol::new("ONE"))),
        Rc::new(DATA::new(Register::REG0, Number::new(0x00EF))), // 239 pixels (next line)
        // subtract linex - 239
        Rc::new(NOT::new(Register::REG1)),
        Rc::new(ADD::new(Register::REG2, Register::REG1)),
        Rc::new(CLF::new()),
        Rc::new(ADD::new(Register::REG0, Register::REG1)),
        // add subtraction to pen position
        Rc::new(DATA::new(Register::REG0, Symbol::new("PEN-POSITION-ADDR"))),
        Rc::new(LOAD::new(Register::REG0, Register::REG0)),
        Rc::new(ADD::new(Register::REG1, Register::REG0)),
        Rc::new(DATA::new(Register::REG1, Symbol::new("PEN-POSITION-ADDR"))),
        Rc::new(STORE::new(Register::REG1, Register::REG0)), // store new value of pen position in memory
        Rc::new(JMP::new(Label::new(
            &(label_prefix.to_owned() + "-resetlinex"),
        ))),
        // needs value in reg3
        Rc::new(DEFLABEL::new(
            &(label_prefix.to_owned() + "-reposition-pen"),
        )),
        // add subtraction to pen position
        Rc::new(DATA::new(Register::REG0, Symbol::new("PEN-POSITION-ADDR"))),
        Rc::new(LOAD::new(Register::REG0, Register::REG0)),
        Rc::new(ADD::new(Register::REG3, Register::REG0)),
        Rc::new(DATA::new(Register::REG1, Symbol::new("PEN-POSITION-ADDR"))),
        Rc::new(STORE::new(Register::REG1, Register::REG0)), // store new value of pen position in memory
        Rc::new(JMP::new(Label::new(
            &(label_prefix.to_owned() + "-resetlinex"),
        ))),
        // reset linex
        Rc::new(DEFLABEL::new(&(label_prefix.to_owned() + "-resetlinex"))),
        Rc::new(DATA::new(Register::REG2, Symbol::new("LINEX"))), //reset linex
        Rc::new(DATA::new(Register::REG3, Number::new(0x0000))),
        Rc::new(STORE::new(Register::REG2, Register::REG3)),
        Rc::new(JMP::new(Label::new(
            &(label_prefix.to_owned() + "-deselectIO"),
        ))),
    ]);
    instructions.add(vec![Rc::new(DEFLABEL::new(
        &(label_prefix.to_owned() + "-deselectIO"),
    ))]);

    // deselect IO adapter
    instructions.add_blocks(vec![deselect_io(Register::REG3)]);

    // return to callee
    instructions.add(vec![
        Rc::new(CLF::new()),
        Rc::new(DATA::new(
            Register::REG3,
            Symbol::new("CALL-RETURN-ADDRESS"),
        )),
        Rc::new(LOAD::new(Register::REG3, Register::REG3)),
        Rc::new(JR::new(Register::REG3)),
    ]);
    instructions.get()
}

fn routine_poll_keyboard(label_prefix: &str) -> Vec<SafeInstruction> {
    let mut instructions = Instructions::new();
    instructions.add(vec![Rc::new(DEFLABEL::new(label_prefix))]);

    // push retun address
    instructions.add(vec![
        Rc::new(DATA::new(
            Register::REG2,
            Symbol::new("CALL-RETURN-ADDRESS"),
        )),
        Rc::new(STORE::new(Register::REG2, Register::REG3)),
    ]);

    instructions.add(vec![
        Rc::new(DATA::new(Register::REG2, Number::new(0x000F))), //select keyboard keyboard
        Rc::new(OUT::new(IOMode::AddressMode, Register::REG2)),
    ]);

    instructions.add(vec![
        Rc::new(DEFLABEL::new(&(label_prefix.to_owned() + "-STARTLOOP"))),
        Rc::new(IN::new(IOMode::DataMode, Register::REG3)), // request key from keyboard adapter
        Rc::new(AND::new(Register::REG3, Register::REG3)),  // check if value is zero
        Rc::new(JMPF::new(
            vec!["Z".to_string()],
            Label::new(&(label_prefix.to_owned() + "-STARTLOOP")),
        )), // if it is - keep polling
        Rc::new(DEFLABEL::new(&(label_prefix.to_owned() + "-ENDLOOP"))), //otherwise
        Rc::new(DATA::new(Register::REG0, Symbol::new("KEYCODE-REGISTER"))),
        Rc::new(STORE::new(Register::REG0, Register::REG3)), //store key in here
        // deselect keyboard
        Rc::new(XOR::new(Register::REG2, Register::REG2)),
        Rc::new(OUT::new(IOMode::AddressMode, Register::REG2)),
    ]);

    // return to callee
    instructions.add(vec![
        Rc::new(CLF::new()),
        Rc::new(DATA::new(
            Register::REG3,
            Symbol::new("CALL-RETURN-ADDRESS"),
        )),
        Rc::new(LOAD::new(Register::REG3, Register::REG3)),
        Rc::new(JR::new(Register::REG3)),
    ]);

    instructions.get()
}

pub fn select_display_adapter(use_register: Register) -> Vec<SafeInstruction> {
    vec![
        Rc::new(DATA::new(use_register, Symbol::new("DISPLAY-ADAPTER-ADDR"))),
        Rc::new(OUT::new(IOMode::AddressMode, use_register)),
    ]
}

pub fn deselect_io(use_register: Register) -> Vec<SafeInstruction> {
    vec![
        Rc::new(XOR::new(use_register, use_register)),
        Rc::new(OUT::new(IOMode::AddressMode, use_register)),
    ]
}

pub fn update_pen_position(position: u16) -> Vec<SafeInstruction> {
    vec![
        Rc::new(DATA::new(Register::REG0, Symbol::new("PEN-POSITION-ADDR"))),
        Rc::new(DATA::new(Register::REG1, Number::new(position))),
        Rc::new(STORE::new(Register::REG0, Register::REG1)),
    ]
}

pub fn render_string(str: &str) -> Vec<SafeInstruction> {
    let mut instructions = Instructions::new();

    for c in str.chars() {
        instructions.add_blocks(vec![load_char_into_keycode_register(c)]);
        instructions.add_blocks(vec![call_routine("ROUTINE-io-drawFontCharacter")]);
    }

    instructions.get()
}

fn load_char_into_keycode_register(c: char) -> Vec<SafeInstruction> {
    vec![
        Rc::new(DATA::new(Register::REG0, Symbol::new("KEYCODE-REGISTER"))),
        Rc::new(DATA::new(Register::REG1, Number::new(c as u16))),
        Rc::new(STORE::new(Register::REG0, Register::REG1)),
    ]
}

pub fn reset_linex() -> Vec<SafeInstruction> {
    let mut instructions = Instructions::new();

    instructions.add(vec![
        Rc::new(DATA::new(Register::REG2, Symbol::new("LINEX"))), //reset linex
        Rc::new(DATA::new(Register::REG3, Number::new(0x0000))),
        Rc::new(STORE::new(Register::REG2, Register::REG3)),
    ]);

    instructions.get()
}
