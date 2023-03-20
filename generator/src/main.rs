use generator::{
    call_routine, deselect_io, initialise_common_code, render_string, reset_linex,
    select_display_adapter, update_pen_position, Instructions, Label, Number, Register,
    SafeInstruction, Symbol, ADD, CLF, CMP, DATA, DEFLABEL, JMP, JMPF, JR, LOAD, NOT, OUT, SHL,
    STORE,
};

use clap::Parser;
use std::rc::Rc;

// important RAM areas
// 0x0000 - 0x03FF ASCII table
// 0x0400 - 0x0400 pen position
// 0x0401 - 0x0401 keycode register
// 0x0500 - 0xFEFD user code + memory
// 0xFEFE - 0xFEFF used to jump back to user code
// 0xFF00 - 0xFFFF temporary variables

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    program: String,
}

fn main() {
    let mut instructions = Instructions::new();
    instructions.add_blocks(vec![initialise_common_code()]);

    let args: Args = Args::parse();
    match args.program.as_str() {
        "ascii" => ascii_table(&mut instructions),
        "brush" => brush(&mut instructions),
        "text_writer" => text_writer(&mut instructions),
        "me" => me(&mut instructions),
        _ => panic!("Unknown program: {}", args.program),
    }
}

fn ascii_table(instructions: &mut Instructions) {
    // MAIN FUNCTION
    instructions.add(vec![
        Rc::new(DEFLABEL::new("main")),
        Rc::new(DATA::new(Register::REG0, Number::new(0x0020))),
        Rc::new(DATA::new(Register::REG2, Number::new(0xFF23))),
        Rc::new(STORE::new(Register::REG2, Register::REG0)),
        Rc::new(DATA::new(Register::REG2, Symbol::new("LINEX"))),
        Rc::new(DATA::new(Register::REG1, Number::new(0x0000))),
        Rc::new(STORE::new(Register::REG2, Register::REG1)),
    ]);

    instructions.add_blocks(vec![update_pen_position(0x00F0)]);

    instructions.add(vec![Rc::new(DEFLABEL::new("main-loop"))]);

    instructions.add(vec![
        Rc::new(DATA::new(Register::REG2, Number::new(0xFF23))),
        Rc::new(LOAD::new(Register::REG2, Register::REG0)),
        Rc::new(DATA::new(Register::REG1, Symbol::new("ONE"))),
        Rc::new(ADD::new(Register::REG1, Register::REG0)),
        Rc::new(DATA::new(Register::REG2, Symbol::new("KEYCODE-REGISTER"))),
        Rc::new(STORE::new(Register::REG2, Register::REG0)),
        Rc::new(DATA::new(Register::REG2, Number::new(0xFF23))),
        Rc::new(STORE::new(Register::REG2, Register::REG0)),
    ]);

    instructions.add_blocks(vec![call_routine("ROUTINE-io-drawFontCharacter")]);

    instructions.add(vec![
        Rc::new(DATA::new(Register::REG2, Number::new(0xFF23))),
        Rc::new(LOAD::new(Register::REG2, Register::REG0)),
        Rc::new(DATA::new(Register::REG2, Number::new(0x007E))),
        Rc::new(CMP::new(Register::REG0, Register::REG2)),
        Rc::new(JMPF::new(vec!["E".to_string()], Label::new("main"))),
    ]);

    instructions.add(vec![Rc::new(JMP::new(Label::new("main-loop")))]);

    println!("{}", instructions.to_string());
}

fn brush(instructions: &mut Instructions) {
    // MAIN FUNCTION
    instructions.add(vec![Rc::new(DEFLABEL::new("main"))]);
    instructions.add_blocks(vec![update_pen_position(0x00F0)]);

    instructions.add(vec![Rc::new(DEFLABEL::new("main-getInput"))]);
    instructions.add_blocks(vec![
        call_routine("drawBrush"),
        call_routine("ROUTINE-io-pollKeyboard"),
        call_routine("drawBrush"),
    ]);

    instructions.add(vec![Rc::new(JMP::new(Label::new("main-getInput")))]);

    instructions.add_blocks(vec![routine_draw_brush("drawBrush")]);

    println!("{}", instructions.to_string());
}

fn text_writer(instructions: &mut Instructions) {
    // MAIN FUNCTION
    instructions.add(vec![Rc::new(DEFLABEL::new("main"))]);
    instructions.add_blocks(vec![update_pen_position(0x00F0)]);

    instructions.add(vec![Rc::new(DEFLABEL::new("main-getInput"))]);
    instructions.add_blocks(vec![
        call_routine("ROUTINE-io-pollKeyboard"),
        call_routine("ROUTINE-io-drawFontCharacter"),
    ]);

    instructions.add(vec![Rc::new(JMP::new(Label::new("main-getInput")))]);

    println!("{}", instructions.to_string());
}

fn me(instructions: &mut Instructions) {
    // MAIN FUNCTION
    instructions.add(vec![Rc::new(DEFLABEL::new("main"))]);

    instructions.add_blocks(vec![
        update_pen_position(0x00F7),
        render_string("Daniel Harper"),
        reset_linex(),
        update_pen_position(0x01E0),
        render_string(&"-".repeat(30)),
        reset_linex(),
        update_pen_position(0x03C1),
        render_string("www.ther.rs"),
        reset_linex(),
        update_pen_position(0x05A1),
        render_string("@ther"),
        reset_linex(),
        update_pen_position(0x087C),
        render_string(":^)"),
        reset_linex(),
    ]);

    instructions.add(vec![
        Rc::new(DEFLABEL::new("noop")),
        Rc::new(CLF::new()),
        Rc::new(JMP::new(Label::new("noop"))),
    ]);

    println!("{}", instructions.to_string());
}

fn routine_draw_brush(label_prefix: &str) -> Vec<SafeInstruction> {
    let font_y_adrr = 0xFF00;
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
        Rc::new(DATA::new(Register::REG0, Number::new(font_y_adrr))),
        Rc::new(DATA::new(Register::REG1, Number::new(0x0000))),
        Rc::new(STORE::new(Register::REG0, Register::REG1)),
    ]);

    instructions.add(vec![
        Rc::new(DATA::new(Register::REG3, Symbol::new("KEYCODE-REGISTER"))), // load keycode
        Rc::new(LOAD::new(Register::REG3, Register::REG3)),
        Rc::new(DATA::new(Register::REG1, Number::new(0x0107))), // load keycode
        Rc::new(CMP::new(Register::REG3, Register::REG1)),       // load keycode
        Rc::new(JMPF::new(
            vec!["E".to_string()],
            Label::new(&(label_prefix.to_owned() + "-left")),
        )),
        Rc::new(DATA::new(Register::REG1, Number::new(0x0106))), // load keycode
        Rc::new(CMP::new(Register::REG3, Register::REG1)),       // load keycode
        Rc::new(JMPF::new(
            vec!["E".to_string()],
            Label::new(&(label_prefix.to_owned() + "-right")),
        )),
        Rc::new(DATA::new(Register::REG1, Number::new(0x0108))), // load keycode
        Rc::new(CMP::new(Register::REG3, Register::REG1)),       // load keycode
        Rc::new(JMPF::new(
            vec!["E".to_string()],
            Label::new(&(label_prefix.to_owned() + "down")),
        )),
        Rc::new(DATA::new(Register::REG1, Number::new(0x0109))), // load keycode
        Rc::new(CMP::new(Register::REG3, Register::REG1)),       // load keycode
        Rc::new(JMPF::new(
            vec!["E".to_string()],
            Label::new(&(label_prefix.to_owned() + "-up")),
        )),
        Rc::new(JMP::new(Label::new(&(label_prefix.to_owned() + "-start")))),
    ]);

    instructions.add(vec![
        Rc::new(DEFLABEL::new(&(label_prefix.to_owned() + "-right"))),
        Rc::new(DATA::new(Register::REG1, Symbol::new("ONE"))), // load keycode
        Rc::new(ADD::new(Register::REG1, Register::REG2)),      // load keycode
        Rc::new(DATA::new(Register::REG3, Symbol::new("PEN-POSITION-ADDR"))), // load keycode
        Rc::new(STORE::new(Register::REG3, Register::REG2)),    // load keycode
        Rc::new(JMP::new(Label::new(&(label_prefix.to_owned() + "-start")))),
    ]);

    instructions.add(vec![
        Rc::new(DEFLABEL::new(&(label_prefix.to_owned() + "-down"))),
        Rc::new(DATA::new(Register::REG1, Number::new(0x00F0))), // load keycode
        Rc::new(ADD::new(Register::REG1, Register::REG2)),       // load keycode
        Rc::new(DATA::new(Register::REG3, Symbol::new("PEN-POSITION-ADDR"))), // load keycode
        Rc::new(STORE::new(Register::REG3, Register::REG2)),     // load keycode
        Rc::new(JMP::new(Label::new(&(label_prefix.to_owned() + "-start")))),
    ]);

    instructions.add(vec![
        Rc::new(DEFLABEL::new(&(label_prefix.to_owned() + "-up"))),
        Rc::new(DATA::new(Register::REG1, Symbol::new("ONE"))), // load keycode
        Rc::new(DATA::new(Register::REG1, Number::new(0x00F0))), // load keycode
        Rc::new(NOT::new(Register::REG1)),
        Rc::new(ADD::new(Register::REG0, Register::REG1)),
        Rc::new(CLF::new()),
        Rc::new(ADD::new(Register::REG1, Register::REG2)),
        Rc::new(DATA::new(Register::REG3, Symbol::new("PEN-POSITION-ADDR"))), // load keycode
        Rc::new(STORE::new(Register::REG3, Register::REG2)),                  // load keycode
        Rc::new(JMP::new(Label::new(&(label_prefix.to_owned() + "-start")))),
    ]);

    instructions.add(vec![
        Rc::new(DEFLABEL::new(&(label_prefix.to_owned() + "-left"))),
        Rc::new(DATA::new(Register::REG0, Symbol::new("ONE"))), // load keycode
        Rc::new(DATA::new(Register::REG1, Symbol::new("ONE"))), // load keycode
        Rc::new(NOT::new(Register::REG1)),
        Rc::new(ADD::new(Register::REG0, Register::REG1)),
        Rc::new(CLF::new()),
        Rc::new(ADD::new(Register::REG1, Register::REG2)),
        Rc::new(DATA::new(Register::REG3, Symbol::new("PEN-POSITION-ADDR"))), // load keycode
        Rc::new(STORE::new(Register::REG3, Register::REG2)),                  // load keycode
        Rc::new(JMP::new(Label::new(&(label_prefix.to_owned() + "-start")))),
    ]);

    instructions.add(vec![Rc::new(DEFLABEL::new(
        &(label_prefix.to_owned() + "-start"),
    ))]);

    instructions.add_blocks(vec![select_display_adapter(Register::REG3)]);

    // calculate memory position of font line
    // start of loop:
    instructions.add(vec![
        Rc::new(DEFLABEL::new(&(label_prefix.to_owned() + "-STARTLOOP"))),
        Rc::new(DATA::new(Register::REG3, Number::new(0x0000))), // load keycode
        Rc::new(SHL::new(Register::REG3)),
        Rc::new(SHL::new(Register::REG3)),
        Rc::new(SHL::new(Register::REG3)), // memory address in RAM for start of font
        Rc::new(DATA::new(Register::REG0, Number::new(font_y_adrr))), // fontY address
        Rc::new(LOAD::new(Register::REG0, Register::REG0)), // load fontY
        Rc::new(ADD::new(Register::REG0, Register::REG3)), // calculate memory position of fontstart+fontYinstructions = append(instructions, ADD{asm.REG0, asm.REG3})       // calculate memory position of fontstart+fontY
        //increment fontY by 1
        Rc::new(DATA::new(Register::REG1, Symbol::new("ONE"))), // one
        Rc::new(ADD::new(Register::REG1, Register::REG0)),      // increment fontY by 1
        Rc::new(DATA::new(Register::REG1, Number::new(font_y_adrr))), // fontY address
        Rc::new(STORE::new(Register::REG1, Register::REG0)), // store new value of fontY in memory
        // load font line from memory
        Rc::new(LOAD::new(Register::REG3, Register::REG0)), // load value from memory into reg0
        // write to display ram
        Rc::new(OUT::new(assembler::IOMode::DataMode, pen_position_register)), // display RAM address
        Rc::new(OUT::new(assembler::IOMode::DataMode, Register::REG0)),        // display RAM value
        Rc::new(DATA::new(Register::REG1, Symbol::new("LINE-WIDTH"))),
        Rc::new(ADD::new(Register::REG1, pen_position_register)), // move pen down by 1 line
        // check if we have rendered all 8 lines
        Rc::new(DATA::new(Register::REG0, Number::new(font_y_adrr))), // fontY addr
        Rc::new(LOAD::new(Register::REG0, Register::REG0)),           //load fontY into reg0
        Rc::new(DATA::new(Register::REG1, Number::new(0x0008))),
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
    instructions.add(vec![Rc::new(DEFLABEL::new(
        &(label_prefix.to_owned() + "-ENDLOOP"),
    ))]);

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
