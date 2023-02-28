use super::{
    ANDGate3, Bit, Bus, BusOne, Component, Decoder2x4, Enableable, FlagState, IOBus,
    InstructionDecoder3x8, ORGate3, ORGate4, ORGate5, ORGate6, Register, Settable, Stepper,
    Updatable, Wire, ALU, AND, BUS_WIDTH, NOT, OR,
};
use crate::io::Peripheral;
use crate::memory::Memory64K;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CPU {
    gp_reg0: Rc<RefCell<Register>>,
    gp_reg1: Rc<RefCell<Register>>,
    gp_reg2: Rc<RefCell<Register>>,
    gp_reg3: Rc<RefCell<Register>>,

    tmp: Option<Rc<RefCell<Register>>>,
    acc: Option<Rc<RefCell<Register>>>,
    iar: Rc<RefCell<Register>>,        // Instruction address register
    ir: Option<Rc<RefCell<Register>>>, // Instruction register
    flags: Option<Rc<RefCell<Register>>>,

    clock_state: bool,
    memory: Rc<RefCell<Memory64K>>,
    alu: Option<Rc<RefCell<ALU>>>,
    stepper: Stepper,
    busone: Option<Rc<RefCell<BusOne>>>,

    main_bus: Rc<RefCell<Bus>>,
    tmp_bus: Option<Rc<RefCell<Bus>>>,
    busone_output: Option<Rc<RefCell<Bus>>>,
    control_bus: Rc<RefCell<Bus>>,
    acc_bus: Option<Rc<RefCell<Bus>>>,
    alu_to_flags_bus: Rc<RefCell<Bus>>,
    flags_bus: Rc<RefCell<Bus>>,
    io_bus: Rc<RefCell<IOBus>>,

    // CONTROL UNIT
    // inc. gates, wiring, instruction decoding etc
    step4_gates: [AND; 8],
    step4_gate3_and: ANDGate3,
    step5_gates: [AND; 6],
    step5_gate3_and: ANDGate3,
    step6_gates: [ANDGate3; 2],
    step6_gates2_and: AND,

    instr_decoder3x8: InstructionDecoder3x8,
    instruction_decoder_enables2x4: [Decoder2x4; 2],
    instruction_decoder_set2x4: Decoder2x4,

    ir_instruction_and_gate: ANDGate3,
    ir_instruction_not_gate: NOT,

    io_bus_enable_gate: AND,
    register_a_enable_or_gate: ORGate3,
    register_b_enable_or_gate: ORGate4,
    register_b_set_or_gate: ORGate4,
    register_a_enable: Wire,
    register_b_enable: Wire,
    acc_enable_or_gate: ORGate4,
    acc_enable_and_gate: AND,
    bus_one_enable_or_gate: ORGate4,
    iar_enable_or_gate: ORGate4,
    iar_enable_and_gate: AND,
    ram_enable_or_gate: ORGate5,
    ram_enable_and_gate: AND,
    gp_reg_enable_and_gates: [ANDGate3; 8],
    gp_reg_enable_or_gates: [OR; 4],
    gp_reg_set_and_gates: [ANDGate3; 4],

    io_bus_set_gate: AND,

    // IR
    ir_set_and_gate: AND,
    ir_bit4_not_gate: NOT,

    // MAR
    mar_set_and_gate: AND,
    mar_set_or_gate: ORGate6,

    // IAR
    iar_set_and_gate: AND,
    iar_set_or_gate: ORGate6,

    // ACC
    acc_set_and_gate: AND,
    acc_set_or_gate: ORGate4,

    // RAM
    ram_set_and_gate: AND,

    // TMP
    tmp_set_and_gate: AND,

    // FLAGS
    flags_set_and_gate: AND,
    flags_set_or_gate: OR,

    register_b_set: Wire,

    flag_state_gates: [AND; 4],
    flag_state_or_gate: ORGate4,

    alu_op_and_gates: [ANDGate3; 3],

    carry_temp: Bit,
    carry_and_gate: AND,

    peripherals: Vec<Rc<RefCell<dyn Peripheral>>>,
}

impl CPU {
    pub fn new(main_bus: Rc<RefCell<Bus>>, memory: Rc<RefCell<Memory64K>>) -> Self {
        let mut cpu = Self {
            gp_reg0: Rc::new(RefCell::new(Register::new(
                "R0",
                main_bus.clone(),
                main_bus.clone(),
            ))),
            gp_reg1: Rc::new(RefCell::new(Register::new(
                "R1",
                main_bus.clone(),
                main_bus.clone(),
            ))),
            gp_reg2: Rc::new(RefCell::new(Register::new(
                "R2",
                main_bus.clone(),
                main_bus.clone(),
            ))),
            gp_reg3: Rc::new(RefCell::new(Register::new(
                "R3",
                main_bus.clone(),
                main_bus.clone(),
            ))),
            tmp: None,
            acc: None,
            ir: None,
            iar: Rc::new(RefCell::new(Register::new(
                "IAR",
                main_bus.clone(),
                main_bus.clone(),
            ))),
            flags: None,
            clock_state: false,
            memory,
            alu: None,
            stepper: Stepper::new(),
            busone: None,
            main_bus: main_bus.clone(),
            tmp_bus: None,
            busone_output: None,
            control_bus: Rc::new(RefCell::new(Bus::new(BUS_WIDTH))),
            acc_bus: None,
            alu_to_flags_bus: Rc::new(RefCell::new(Bus::new(BUS_WIDTH))),
            flags_bus: Rc::new(RefCell::new(Bus::new(BUS_WIDTH))),
            io_bus: Rc::new(RefCell::new(IOBus::new())),
            step4_gates: (0..8)
                .map(|_| AND::new())
                .collect::<Vec<AND>>()
                .try_into()
                .unwrap(),
            step4_gate3_and: ANDGate3::new(),
            step5_gates: (0..6)
                .map(|_| AND::new())
                .collect::<Vec<AND>>()
                .try_into()
                .unwrap(),
            step5_gate3_and: ANDGate3::new(),
            step6_gates: (0..2)
                .map(|_| ANDGate3::new())
                .collect::<Vec<ANDGate3>>()
                .try_into()
                .unwrap(),
            step6_gates2_and: AND::new(),
            instr_decoder3x8: InstructionDecoder3x8::new(),
            instruction_decoder_enables2x4: (0..2)
                .map(|_| Decoder2x4::new())
                .collect::<Vec<Decoder2x4>>()
                .try_into()
                .unwrap(),
            instruction_decoder_set2x4: Decoder2x4::new(),
            ir_instruction_and_gate: ANDGate3::new(),
            ir_instruction_not_gate: NOT::new(),
            io_bus_enable_gate: AND::new(),
            register_a_enable_or_gate: ORGate3::new(),
            register_b_enable_or_gate: ORGate4::new(),
            register_b_set_or_gate: ORGate4::new(),
            register_a_enable: Wire::new("Z".to_string(), false),
            register_b_enable: Wire::new("Z".to_string(), false),
            acc_enable_or_gate: ORGate4::new(),
            acc_enable_and_gate: AND::new(),
            bus_one_enable_or_gate: ORGate4::new(),
            iar_enable_or_gate: ORGate4::new(),
            iar_enable_and_gate: AND::new(),
            ram_enable_or_gate: ORGate5::new(),
            ram_enable_and_gate: AND::new(),
            gp_reg_enable_and_gates: (0..8)
                .map(|_| ANDGate3::new())
                .collect::<Vec<ANDGate3>>()
                .try_into()
                .unwrap(),
            gp_reg_enable_or_gates: (0..4)
                .map(|_| OR::new())
                .collect::<Vec<OR>>()
                .try_into()
                .unwrap(),
            gp_reg_set_and_gates: (0..4)
                .map(|_| ANDGate3::new())
                .collect::<Vec<ANDGate3>>()
                .try_into()
                .unwrap(),
            io_bus_set_gate: AND::new(),
            ir_bit4_not_gate: NOT::new(),
            ir_set_and_gate: AND::new(),
            mar_set_or_gate: ORGate6::new(),
            mar_set_and_gate: AND::new(),
            iar_set_or_gate: ORGate6::new(),
            iar_set_and_gate: AND::new(),
            acc_set_or_gate: ORGate4::new(),
            acc_set_and_gate: AND::new(),
            ram_set_and_gate: AND::new(),
            tmp_set_and_gate: AND::new(),
            flags_set_or_gate: OR::new(),
            flags_set_and_gate: AND::new(),
            register_b_set: Wire::new("Z".to_string(), false),
            flag_state_gates: (0..4)
                .map(|_| AND::new())
                .collect::<Vec<AND>>()
                .try_into()
                .unwrap(),
            flag_state_or_gate: ORGate4::new(),
            alu_op_and_gates: (0..3)
                .map(|_| ANDGate3::new())
                .collect::<Vec<ANDGate3>>()
                .try_into()
                .unwrap(),
            carry_temp: Bit::new(),
            carry_and_gate: AND::new(),
            peripherals: Vec::new(),
        };

        // IR
        cpu.ir = Some(Rc::new(RefCell::new(Register::new(
            "IR",
            cpu.main_bus.clone(),
            cpu.control_bus.clone(),
        ))));
        cpu.ir.as_ref().unwrap().borrow_mut().disable();

        cpu.flags = Some(Rc::new(RefCell::new(Register::new(
            "FLAGS",
            cpu.alu_to_flags_bus.clone(),
            cpu.flags_bus.clone(),
        ))));

        CPU::update_enable_status(cpu.flags.as_ref().unwrap().clone(), true);
        CPU::update_set_status(cpu.flags.as_ref().unwrap().clone(), true);
        CPU::update_on(cpu.flags.as_ref().unwrap().clone());
        CPU::update_set_status(cpu.flags.as_ref().unwrap().clone(), false);

        // TMP
        cpu.tmp_bus = Some(Rc::new(RefCell::new(Bus::new(BUS_WIDTH))));
        cpu.tmp = Some(Rc::new(RefCell::new(Register::new(
            "TMP",
            cpu.main_bus.clone(),
            cpu.tmp_bus.as_ref().unwrap().clone(),
        ))));

        // tmp register is always enabled, and we initialise it with value 0
        CPU::update_enable_status(cpu.tmp.as_ref().unwrap().clone(), true);
        CPU::update_set_status(cpu.tmp.as_ref().unwrap().clone(), true);
        CPU::update_on(cpu.tmp.as_ref().unwrap().clone());
        CPU::update_set_status(cpu.tmp.as_ref().unwrap().clone(), false);

        cpu.busone_output = Some(Rc::new(RefCell::new(Bus::new(BUS_WIDTH))));
        cpu.busone = Some(Rc::new(RefCell::new(BusOne::new(
            cpu.tmp_bus.as_ref().unwrap().clone(),
            cpu.busone_output.as_ref().unwrap().clone(),
        ))));

        // ACC
        cpu.acc_bus = Some(Rc::new(RefCell::new(Bus::new(BUS_WIDTH))));
        cpu.acc = Some(Rc::new(RefCell::new(Register::new(
            "ACC",
            cpu.acc_bus.as_ref().unwrap().clone(),
            cpu.main_bus.clone(),
        ))));

        // ALU
        cpu.alu = Some(Rc::new(RefCell::new(ALU::new(
            cpu.main_bus.clone(),
            cpu.busone_output.as_ref().unwrap().clone(),
            cpu.acc_bus.as_ref().unwrap().clone(),
            cpu.alu_to_flags_bus.clone(),
        ))));

        cpu
    }

    fn update_enable_status<T>(enableable: Rc<RefCell<T>>, state: bool)
    where
        T: Enableable,
    {
        match state {
            true => enableable.borrow_mut().enable(),
            false => enableable.borrow_mut().disable(),
        }
    }

    fn update_set_status<T>(set: Rc<RefCell<T>>, state: bool)
    where
        T: Settable,
    {
        match state {
            true => set.borrow_mut().set(),
            false => set.borrow_mut().unset(),
        }
    }

    fn update_on<T>(u: Rc<RefCell<T>>)
    where
        T: Updatable,
    {
        u.borrow_mut().update()
    }

    pub fn connect_peripheral<T>(&mut self, p: Rc<RefCell<T>>)
    where
        T: Peripheral + 'static,
    {
        p.borrow_mut()
            .connect(self.io_bus.clone(), self.main_bus.clone());
        self.peripherals.push(p);
    }

    pub fn set_iar(&mut self, address: u16) {
        self.main_bus.borrow_mut().set_value(address);

        Self::update_set_status(self.iar.clone(), true);
        Self::update_on(self.iar.clone());
        Self::update_set_status(self.iar.clone(), false);
        Self::update_on(self.iar.clone());

        self.clear_main_bus()
    }

    pub fn step(&mut self) {
        for _ in 0..2 {
            match self.clock_state {
                true => self.clock_state = false,
                false => self.clock_state = true,
            }
            self.to_step(self.clock_state);
        }
    }

    fn to_step(&mut self, clock_state: bool) {
        self.stepper.update(clock_state);
        self.run_step_4_gates();
        self.run_step_5_gates();
        self.run_step_6_gates();

        self.run_enable(clock_state);
        self.update_states();
        if clock_state {
            self.run_enable(false);
            self.update_states();
        }

        self.run_set(clock_state);
        self.update_states();
        if clock_state {
            self.run_set(false);
            self.update_states();
        }

        self.clear_main_bus();
    }

    fn run_step_4_gates(&mut self) {
        self.step4_gates[0].update(
            self.stepper.get_output_wire(3),
            self.ir.as_ref().unwrap().borrow().bit(8),
        );

        let mut gate = 1;
        for selector in 0..7 {
            self.step4_gates[gate].update(
                self.stepper.get_output_wire(3),
                self.instr_decoder3x8.selector_gates[selector].get(),
            );
            gate += 1;
        }

        self.step4_gate3_and.update(
            self.stepper.get_output_wire(3),
            self.instr_decoder3x8.selector_gates[7].get(),
            self.ir.as_ref().unwrap().borrow().bit(12),
        );

        self.ir_bit4_not_gate
            .update(self.ir.as_ref().unwrap().borrow().bit(12));
    }

    fn run_step_5_gates(&mut self) {
        self.step5_gates[0].update(
            self.stepper.get_output_wire(4),
            self.ir.as_ref().unwrap().borrow().bit(8),
        );

        self.step5_gates[1].update(
            self.stepper.get_output_wire(4),
            self.instr_decoder3x8.selector_gates[0].get(),
        );
        self.step5_gates[2].update(
            self.stepper.get_output_wire(4),
            self.instr_decoder3x8.selector_gates[1].get(),
        );
        self.step5_gates[3].update(
            self.stepper.get_output_wire(4),
            self.instr_decoder3x8.selector_gates[2].get(),
        );

        self.step5_gates[4].update(
            self.stepper.get_output_wire(4),
            self.instr_decoder3x8.selector_gates[4].get(),
        );
        self.step5_gates[5].update(
            self.stepper.get_output_wire(4),
            self.instr_decoder3x8.selector_gates[5].get(),
        );

        self.step5_gate3_and.update(
            self.stepper.get_output_wire(4),
            self.instr_decoder3x8.selector_gates[7].get(),
            self.ir_bit4_not_gate.get(),
        );
    }

    fn run_step_6_gates(&mut self) {
        self.step6_gates[0].update(
            self.stepper.get_output_wire(5),
            self.ir.as_ref().unwrap().borrow().bit(8),
            self.ir_instruction_not_gate.get(),
        );

        self.step6_gates2_and.update(
            self.stepper.get_output_wire(5),
            self.instr_decoder3x8.selector_gates[2].get(),
        );

        self.step6_gates[1].update(
            self.stepper.get_output_wire(5),
            self.instr_decoder3x8.selector_gates[5].get(),
            self.flag_state_or_gate.get(),
        );
    }

    fn update_states(&mut self) {
        // IAR
        Self::update_on(self.iar.clone());

        // MAR
        Self::update_on(self.memory.borrow().address_register.clone());

        // IR
        Self::update_on(self.ir.as_ref().unwrap().clone());

        // RAM
        Self::update_on(self.memory.clone());

        // TMP
        Self::update_on(self.tmp.as_ref().unwrap().clone());

        // FLAGS
        Self::update_on(self.flags.as_ref().unwrap().clone());

        // BUS1
        Self::update_on(self.busone.as_ref().unwrap().clone());

        // ALU
        self.update_alu();

        // ACC
        Self::update_on(self.acc.as_ref().unwrap().clone());

        // R0
        Self::update_on(self.gp_reg0.clone());

        // R1
        Self::update_on(self.gp_reg1.clone());

        // R2
        Self::update_on(self.gp_reg2.clone());

        // R3
        Self::update_on(self.gp_reg3.clone());

        self.update_instruction_decoder3x8();
        self.update_io_bus();
        self.update_peripherals();
    }

    fn clear_main_bus(&mut self) {
        for i in 0..BUS_WIDTH {
            self.main_bus.borrow_mut().set_input_wire(i, false);
        }
    }

    fn update_instruction_decoder3x8(&mut self) {
        self.instr_decoder3x8
            .bit0_not_gate
            .update(self.ir.as_ref().unwrap().borrow().bit(8));

        self.instr_decoder3x8.decoder.update(
            self.ir.as_ref().unwrap().borrow().bit(9),
            self.ir.as_ref().unwrap().borrow().bit(10),
            self.ir.as_ref().unwrap().borrow().bit(11),
        );

        for i in 0..8 {
            self.instr_decoder3x8.selector_gates[i].update(
                self.instr_decoder3x8.decoder.get_output_wire(i as i32),
                self.instr_decoder3x8.bit0_not_gate.get(),
            );
        }
    }

    fn update_io_bus(&mut self) {
        self.io_bus.borrow_mut().update(
            self.ir.as_ref().unwrap().borrow().bit(12),
            self.ir.as_ref().unwrap().borrow().bit(13),
        )
    }

    fn update_peripherals(&mut self) {
        for i in self.peripherals.iter() {
            i.borrow_mut().update()
        }
    }

    fn update_alu(&mut self) {
        // update ALU operation based on instruction register
        self.alu_op_and_gates[2].update(
            self.ir.as_ref().unwrap().borrow().bit(9),
            self.ir.as_ref().unwrap().borrow().bit(8),
            self.stepper.get_output_wire(4),
        );

        self.alu_op_and_gates[1].update(
            self.ir.as_ref().unwrap().borrow().bit(10),
            self.ir.as_ref().unwrap().borrow().bit(8),
            self.stepper.get_output_wire(4),
        );
        self.alu_op_and_gates[0].update(
            self.ir.as_ref().unwrap().borrow().bit(11),
            self.ir.as_ref().unwrap().borrow().bit(8),
            self.stepper.get_output_wire(4),
        );

        self.alu.as_ref().unwrap().borrow_mut().op[2].update(self.alu_op_and_gates[2].get());
        self.alu.as_ref().unwrap().borrow_mut().op[1].update(self.alu_op_and_gates[1].get());
        self.alu.as_ref().unwrap().borrow_mut().op[0].update(self.alu_op_and_gates[0].get());

        self.alu
            .as_ref()
            .unwrap()
            .borrow_mut()
            .carry_in
            .update(self.carry_and_gate.get());

        self.alu.as_ref().unwrap().borrow_mut().update();
    }
}

// Run enable
impl CPU {
    fn run_enable(&mut self, state: bool) {
        self.run_enable_on_io(state);
        self.run_enable_on_iar(state);
        self.run_enable_on_bus_one(state);
        self.run_enable_on_acc(state);
        self.run_enable_on_ram(state);
        self.run_enable_on_register_b();
        self.run_enable_on_register_a();
        self.run_enable_general_purpose_registers(state);
    }

    fn run_enable_on_io(&mut self, state: bool) {
        self.io_bus_enable_gate
            .update(state, self.step5_gate3_and.get());

        Self::update_enable_status(self.io_bus.clone(), self.io_bus_enable_gate.get());
    }

    fn run_enable_on_iar(&mut self, state: bool) {
        self.iar_enable_or_gate.update(
            self.stepper.get_output_wire(0),
            self.step4_gates[3].get(),
            self.step4_gates[5].get(),
            self.step4_gates[6].get(),
        );

        self.iar_enable_and_gate
            .update(state, self.iar_enable_or_gate.get());

        Self::update_enable_status(self.iar.clone(), self.iar_enable_and_gate.get());
    }

    fn run_enable_on_bus_one(&mut self, state: bool) {
        self.bus_one_enable_or_gate.update(
            self.stepper.get_output_wire(0),
            self.step4_gates[7].get(),
            self.step4_gates[6].get(),
            self.step4_gates[3].get(),
        );
        Self::update_enable_status(
            self.busone.as_ref().unwrap().clone(),
            self.bus_one_enable_or_gate.get(),
        );
    }

    fn run_enable_on_acc(&mut self, state: bool) {
        self.acc_enable_or_gate.update(
            self.stepper.get_output_wire(2),
            self.step5_gates[5].get(),
            self.step6_gates2_and.get(),
            self.step6_gates[0].get(),
        );
        self.acc_enable_and_gate
            .update(state, self.acc_enable_or_gate.get());

        Self::update_enable_status(
            self.acc.as_ref().unwrap().clone(),
            self.acc_enable_and_gate.get(),
        );
    }

    fn run_enable_on_ram(&mut self, state: bool) {
        self.ram_enable_or_gate.update(
            self.stepper.get_output_wire(1),
            self.step6_gates[1].get(),
            self.step5_gates[4].get(),
            self.step5_gates[3].get(),
            self.step5_gates[1].get(),
        );
        self.ram_enable_and_gate
            .update(state, self.ram_enable_or_gate.get());
        Self::update_enable_status(self.memory.clone(), self.ram_enable_and_gate.get());
    }

    fn run_enable_on_register_b(&mut self) {
        self.register_b_enable_or_gate.update(
            self.stepper.get_output_wire(0),
            self.step5_gates[2].get(),
            self.step4_gates[4].get(),
            self.step4_gate3_and.get(),
        );
        self.register_b_enable
            .update(self.register_b_enable_or_gate.get());
    }

    fn run_enable_on_register_a(&mut self) {
        self.register_a_enable_or_gate.update(
            self.step4_gates[1].get(),
            self.step4_gates[2].get(),
            self.step5_gates[0].get(),
        );
        self.register_a_enable
            .update(self.register_a_enable_or_gate.get());
    }

    fn run_enable_general_purpose_registers(&mut self, state: bool) {
        self.instruction_decoder_enables2x4[0].update(
            self.ir.as_ref().unwrap().borrow().bit(14),
            self.ir.as_ref().unwrap().borrow().bit(15),
        );
        self.instruction_decoder_enables2x4[1].update(
            self.ir.as_ref().unwrap().borrow().bit(12),
            self.ir.as_ref().unwrap().borrow().bit(13),
        );

        // R0
        self.gp_reg_enable_and_gates[0].update(
            state,
            self.register_b_enable.get(),
            self.instruction_decoder_enables2x4[0].get_output_wire(0),
        );
        self.gp_reg_enable_and_gates[4].update(
            state,
            self.register_a_enable.get(),
            self.instruction_decoder_enables2x4[1].get_output_wire(0),
        );
        self.gp_reg_enable_or_gates[0].update(
            self.gp_reg_enable_and_gates[4].get(),
            self.gp_reg_enable_and_gates[0].get(),
        );
        Self::update_enable_status(self.gp_reg0.clone(), self.gp_reg_enable_or_gates[0].get());

        // R1
        self.gp_reg_enable_and_gates[1].update(
            state,
            self.register_b_enable.get(),
            self.instruction_decoder_enables2x4[0].get_output_wire(1),
        );
        self.gp_reg_enable_and_gates[5].update(
            state,
            self.register_a_enable.get(),
            self.instruction_decoder_enables2x4[1].get_output_wire(1),
        );
        self.gp_reg_enable_or_gates[1].update(
            self.gp_reg_enable_and_gates[5].get(),
            self.gp_reg_enable_and_gates[1].get(),
        );
        Self::update_enable_status(self.gp_reg1.clone(), self.gp_reg_enable_or_gates[1].get());

        // R2
        self.gp_reg_enable_and_gates[2].update(
            state,
            self.register_b_enable.get(),
            self.instruction_decoder_enables2x4[0].get_output_wire(2),
        );
        self.gp_reg_enable_and_gates[6].update(
            state,
            self.register_a_enable.get(),
            self.instruction_decoder_enables2x4[1].get_output_wire(2),
        );
        self.gp_reg_enable_or_gates[2].update(
            self.gp_reg_enable_and_gates[6].get(),
            self.gp_reg_enable_and_gates[2].get(),
        );
        Self::update_enable_status(self.gp_reg2.clone(), self.gp_reg_enable_or_gates[2].get());

        // R3
        self.gp_reg_enable_and_gates[3].update(
            state,
            self.register_b_enable.get(),
            self.instruction_decoder_enables2x4[0].get_output_wire(3),
        );
        self.gp_reg_enable_and_gates[7].update(
            state,
            self.register_a_enable.get(),
            self.instruction_decoder_enables2x4[1].get_output_wire(3),
        );
        self.gp_reg_enable_or_gates[3].update(
            self.gp_reg_enable_and_gates[7].get(),
            self.gp_reg_enable_and_gates[3].get(),
        );
        Self::update_enable_status(self.gp_reg3.clone(), self.gp_reg_enable_or_gates[3].get());
    }
}

// Run set
impl CPU {
    fn run_set(&mut self, state: bool) {
        self.ir_instruction_and_gate.update(
            self.ir.as_ref().unwrap().borrow().bit(11),
            self.ir.as_ref().unwrap().borrow().bit(10),
            self.ir.as_ref().unwrap().borrow().bit(9),
        );
        self.ir_instruction_not_gate
            .update(self.ir_instruction_and_gate.get());

        self.refresh_flag_state_gates();

        self.run_set_on_io(state);
        self.run_set_on_mar(state);
        self.run_set_on_iar(state);
        self.run_set_on_ir(state);
        self.run_set_on_acc(state);
        self.run_set_on_ram(state);
        self.run_set_on_tmp(state);
        self.run_set_on_flags(state);
        self.run_set_on_register_b();
        self.run_set_general_purpose_registers(state);
    }

    fn refresh_flag_state_gates(&mut self) {
        // C
        self.flag_state_gates[0].update(
            self.ir.as_ref().unwrap().borrow().bit(12),
            self.flags_bus
                .borrow()
                .get_output_wire(FlagState::Carry as i32),
        );

        // A
        self.flag_state_gates[1].update(
            self.ir.as_ref().unwrap().borrow().bit(13),
            self.flags_bus
                .borrow()
                .get_output_wire(FlagState::ALarger as i32),
        );

        // E
        self.flag_state_gates[2].update(
            self.ir.as_ref().unwrap().borrow().bit(14),
            self.flags_bus
                .borrow()
                .get_output_wire(FlagState::Equal as i32),
        );

        // Z
        self.flag_state_gates[3].update(
            self.ir.as_ref().unwrap().borrow().bit(15),
            self.flags_bus
                .borrow()
                .get_output_wire(FlagState::Zero as i32),
        );

        self.flag_state_or_gate.update(
            self.flag_state_gates[0].get(),
            self.flag_state_gates[1].get(),
            self.flag_state_gates[2].get(),
            self.flag_state_gates[3].get(),
        );
    }

    fn run_set_on_io(&mut self, state: bool) {
        self.io_bus_set_gate
            .update(state, self.step4_gate3_and.get());
        Self::update_set_status(self.io_bus.clone(), self.io_bus_set_gate.get());
    }

    fn run_set_on_mar(&mut self, state: bool) {
        self.mar_set_or_gate.update(
            self.stepper.get_output_wire(0),
            self.step4_gates[3].get(),
            self.step4_gates[6].get(),
            self.step4_gates[1].get(),
            self.step4_gates[2].get(),
            self.step4_gates[5].get(),
        );
        self.mar_set_and_gate
            .update(state, self.mar_set_or_gate.get());
        Self::update_set_status(
            self.memory.borrow().address_register.clone(),
            self.mar_set_and_gate.get(),
        );
    }

    fn run_set_on_iar(&mut self, state: bool) {
        self.iar_set_or_gate.update(
            self.stepper.get_output_wire(2),
            self.step4_gates[4].get(),
            self.step5_gates[4].get(),
            self.step5_gates[5].get(),
            self.step6_gates2_and.get(),
            self.step6_gates[1].get(),
        );
        self.iar_set_and_gate
            .update(state, self.iar_set_or_gate.get());
        Self::update_set_status(self.iar.clone(), self.iar_set_and_gate.get());
    }

    fn run_set_on_ir(&mut self, state: bool) {
        self.ir_set_and_gate
            .update(state, self.stepper.get_output_wire(1));
        Self::update_set_status(
            self.ir.as_ref().unwrap().clone(),
            self.ir_set_and_gate.get(),
        );
    }

    fn run_set_on_acc(&mut self, state: bool) {
        self.acc_set_or_gate.update(
            self.stepper.get_output_wire(0),
            self.step4_gates[3].get(),
            self.step4_gates[6].get(),
            self.step5_gates[0].get(),
        );
        self.acc_set_and_gate
            .update(state, self.acc_set_or_gate.get());
        Self::update_set_status(
            self.acc.as_ref().unwrap().clone(),
            self.acc_set_and_gate.get(),
        );
    }

    fn run_set_on_ram(&mut self, state: bool) {
        self.ram_set_and_gate
            .update(state, self.step5_gates[2].get());
        Self::update_set_status(self.memory.clone(), self.ram_set_and_gate.get());
    }

    fn run_set_on_tmp(&mut self, state: bool) {
        self.tmp_set_and_gate
            .update(state, self.step4_gates[0].get());

        Self::update_set_status(
            self.tmp.as_ref().unwrap().clone(),
            self.tmp_set_and_gate.get(),
        );

        self.carry_temp.update(
            self.flags_bus
                .borrow()
                .get_output_wire(FlagState::Carry as i32),
            self.tmp_set_and_gate.get(),
        );
        self.carry_and_gate
            .update(self.carry_temp.get(), self.step5_gates[0].get());
    }

    fn run_set_on_flags(&mut self, state: bool) {
        self.flags_set_or_gate
            .update(self.step5_gates[0].get(), self.step4_gates[7].get());

        self.flags_set_and_gate
            .update(state, self.flags_set_or_gate.get());
        Self::update_set_status(
            self.flags.as_ref().unwrap().clone(),
            self.flags_set_and_gate.get(),
        );
    }

    fn run_set_on_register_b(&mut self) {
        self.register_b_set_or_gate.update(
            self.step5_gates[1].get(),
            self.step6_gates[0].get(),
            self.step5_gates[3].get(),
            self.step5_gate3_and.get(),
        );
        self.register_b_set
            .update(self.register_b_set_or_gate.get());
    }

    fn run_set_general_purpose_registers(&mut self, state: bool) {
        self.instruction_decoder_set2x4.update(
            self.ir.as_ref().unwrap().borrow().bit(14),
            self.ir.as_ref().unwrap().borrow().bit(15),
        );

        // R0
        self.gp_reg_set_and_gates[0].update(
            state,
            self.register_b_set.get(),
            self.instruction_decoder_set2x4.get_output_wire(0),
        );
        Self::update_set_status(self.gp_reg0.clone(), self.gp_reg_set_and_gates[0].get());

        // R1
        self.gp_reg_set_and_gates[1].update(
            state,
            self.register_b_set.get(),
            self.instruction_decoder_set2x4.get_output_wire(1),
        );
        Self::update_set_status(self.gp_reg1.clone(), self.gp_reg_set_and_gates[1].get());

        // R2
        self.gp_reg_set_and_gates[2].update(
            state,
            self.register_b_set.get(),
            self.instruction_decoder_set2x4.get_output_wire(2),
        );
        Self::update_set_status(self.gp_reg2.clone(), self.gp_reg_set_and_gates[2].get());

        // R3
        self.gp_reg_set_and_gates[3].update(
            state,
            self.register_b_set.get(),
            self.instruction_decoder_set2x4.get_output_wire(3),
        );
        Self::update_set_status(self.gp_reg3.clone(), self.gp_reg_set_and_gates[3].get());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_alu_add() {
        let main_bus = Rc::new(RefCell::new(Bus::new(BUS_WIDTH)));
        let memory = Rc::new(RefCell::new(Memory64K::new(main_bus.clone())));
        clear_memory(main_bus.clone(), memory.clone());

        let cpu = Rc::new(RefCell::new(CPU::new(main_bus.clone(), memory.clone())));

        let inputs = vec![0x0002, 0x0003, 0x0004, 0x0005];
        test_instruction(
            cpu.clone(),
            0x0081,
            inputs.to_vec(),
            vec![inputs[0] + inputs[0], inputs[1], inputs[2], inputs[3]],
        );
    }

    fn test_instruction(
        cpu: Rc<RefCell<CPU>>,
        instruction: u16,
        input_registers: Vec<u16>,
        expected_output_registers: Vec<u16>,
    ) {
        cpu.borrow_mut()
            .set_cpu_memory_location(0x0000, instruction);
        println!("0x0000: {}", cpu.borrow().memory.borrow().data[0][0]);
        assert_eq!(
            format!("{}", cpu.borrow().memory.borrow().data[0][0]),
            format!("{:016b}", instruction),
            "Instruction error: 0x0000: {} instruction: {:16b}",
            cpu.borrow().memory.borrow().data[0][0],
            instruction
        );

        for i in 0..input_registers.len() {
            cpu.borrow_mut()
                .set_cpu_register(i as u16, input_registers[i]);
        }

        cpu.borrow_mut().set_iar(0x0000);

        cpu.borrow_mut().do_fetch_decode_execute();

        cpu.borrow().check_registers(
            expected_output_registers[0],
            expected_output_registers[1],
            expected_output_registers[2],
            expected_output_registers[3],
        )
    }

    fn clear_memory(bus: Rc<RefCell<Bus>>, memory: Rc<RefCell<Memory64K>>) {
        for i in 0..=0xFFFF {
            set_memory_location(bus.clone(), memory.clone(), i, 0x0000);
        }
    }

    fn set_memory_location(
        bus: Rc<RefCell<Bus>>,
        memory: Rc<RefCell<Memory64K>>,
        address: u16,
        value: u16,
    ) {
        memory.borrow().address_register.borrow_mut().set();
        bus.borrow_mut().set_value(address);
        memory.borrow_mut().update();

        memory.borrow().address_register.borrow_mut().unset();
        memory.borrow_mut().update();

        bus.borrow_mut().set_value(value);
        memory.borrow_mut().set();
        memory.borrow_mut().update();

        memory.borrow_mut().unset();
        memory.borrow_mut().update();
    }

    impl CPU {
        fn set_cpu_memory_location(&mut self, address: u16, value: u16) {
            self.memory.borrow_mut().address_register.borrow_mut().set();
            self.main_bus.borrow_mut().set_value(address);
            self.memory.borrow_mut().update();

            self.memory
                .borrow_mut()
                .address_register
                .borrow_mut()
                .unset();
            self.memory.borrow_mut().update();

            self.main_bus.borrow_mut().set_value(value);
            self.memory.borrow_mut().set();
            self.memory.borrow_mut().update();

            self.memory.borrow_mut().unset();
            self.memory.borrow_mut().update();
        }
        fn set_cpu_register(&mut self, register: u16, value: u16) {
            match register {
                0 => {
                    self.gp_reg0.borrow_mut().set();
                    self.gp_reg0.borrow_mut().update();

                    self.main_bus.borrow_mut().set_value(value);
                    self.gp_reg0.borrow_mut().update();

                    self.gp_reg0.borrow_mut().unset();
                    self.gp_reg0.borrow_mut().update();
                }
                1 => {
                    self.gp_reg1.borrow_mut().set();
                    self.gp_reg1.borrow_mut().update();

                    self.main_bus.borrow_mut().set_value(value);
                    self.gp_reg1.borrow_mut().update();

                    self.gp_reg1.borrow_mut().unset();
                    self.gp_reg1.borrow_mut().update();
                }
                2 => {
                    self.gp_reg2.borrow_mut().set();
                    self.gp_reg2.borrow_mut().update();

                    self.main_bus.borrow_mut().set_value(value);
                    self.gp_reg2.borrow_mut().update();

                    self.gp_reg2.borrow_mut().unset();
                    self.gp_reg2.borrow_mut().update();
                }
                3 => {
                    self.gp_reg3.borrow_mut().set();
                    self.gp_reg3.borrow_mut().update();

                    self.main_bus.borrow_mut().set_value(value);
                    self.gp_reg3.borrow_mut().update();

                    self.gp_reg3.borrow_mut().unset();
                    self.gp_reg3.borrow_mut().update();
                }

                _ => panic!("Unknown register"),
            }
        }

        fn do_fetch_decode_execute(&mut self) {
            for _ in 0..6 {
                self.step();

                let get_value_of_bus = |bus: &RefCell<Bus>| -> u16 {
                    let mut x: u16 = 0;
                    let mut result: u16 = 0;
                    for i in (0..BUS_WIDTH).rev() {
                        match bus.borrow().get_output_wire(i) {
                            true => result = result | (1 << x),
                            false => result = result ^ (result & (1 << x)),
                        };
                        x += 1;
                    }
                    result
                };

                println!(
                "step: {} op: {} {} {} ir: {:#X} iar: {:#X} reg: {:#X} {:#X} {:#X} {:#X} main_bus: {:#X} acc_bus: {:#X}",
                self.stepper,
                self.alu_op_and_gates[2].get() as i32,
                self.alu_op_and_gates[1].get() as i32,
                self.alu_op_and_gates[0].get() as i32,
                self.ir.as_ref().unwrap().borrow().value(),
                self.iar.borrow().value(),
                self.gp_reg0.borrow().value(),
                self.gp_reg1.borrow().value(),
                self.gp_reg2.borrow().value(),
                self.gp_reg3.borrow().value(),
                get_value_of_bus(self.main_bus.as_ref()),
                get_value_of_bus(self.acc_bus.as_ref().unwrap()),
            );
            }
        }

        fn check_registers(&self, exp_reg0: u16, exp_reg1: u16, exp_reg2: u16, exp_reg3: u16) {
            self.check_register(0, exp_reg0);
            self.check_register(1, exp_reg1);
            self.check_register(2, exp_reg2);
            self.check_register(3, exp_reg3);
        }

        fn check_register(&self, register: u16, expected: u16) {
            let reg_value: u16 = match register {
                0 => self.gp_reg0.borrow().value(),
                1 => self.gp_reg1.borrow().value(),
                2 => self.gp_reg2.borrow().value(),
                3 => self.gp_reg3.borrow().value(),
                _ => panic!("Unknown register {}", register),
            };

            assert_eq!(
                reg_value, expected,
                "Expected register {} to have value of: {:#X} but got {:#X}",
                register, expected, reg_value
            );
        }
    }
}
