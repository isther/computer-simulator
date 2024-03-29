use super::{FlagState, InstructionDecoder3x8, ALU};
use crate::computer::{
    components::{
        ANDGate3, Bit, Bus, BusOne, Component, Decoder2x4, Enableable, IOBus, ORGate3, ORGate4,
        ORGate5, ORGate6, Register, Settable, Stepper, Updatable, BUS_WIDTH,
    },
    gates::{Wire, AND, NOT, OR},
    io::Peripheral,
    memory::Memory64K,
};
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct CPU {
    gp_reg0: Register,
    gp_reg1: Register,
    gp_reg2: Register,
    gp_reg3: Register,

    tmp: Register,
    acc: Register,
    iar: Register, // Instruction address register
    ir: Register,  // Instruction register
    pub flags: Register,

    pub clock_state: bool,
    memory: Arc<Mutex<Memory64K>>,
    alu: ALU,
    stepper: Stepper,
    pub busone: BusOne,

    // Bus
    pub main_bus: Arc<Mutex<Bus>>,
    pub tmp_bus: Arc<Mutex<Bus>>,
    pub busone_output: Arc<Mutex<Bus>>,
    pub control_bus: Arc<Mutex<Bus>>,
    pub acc_bus: Arc<Mutex<Bus>>,
    pub alu_to_flags_bus: Arc<Mutex<Bus>>,
    pub flags_bus: Arc<Mutex<Bus>>,
    pub io_bus: Arc<Mutex<IOBus>>,

    // CONTROL UNIT
    // inc. gates, wiring, instruction decoding etc
    pub step4_gates: [AND; 8],
    pub step4_gate3_and: ANDGate3,
    pub step5_gates: [AND; 6],
    pub step5_gate3_and: ANDGate3,
    pub step6_gates: [ANDGate3; 2],
    pub step6_gates2_and: AND,

    pub instr_decoder3x8: InstructionDecoder3x8,
    pub instruction_decoder_enables2x4: [Decoder2x4; 2],
    pub instruction_decoder_set2x4: Decoder2x4,

    pub ir_instruction_and_gate: ANDGate3,
    pub ir_instruction_not_gate: NOT,

    pub io_bus_enable_gate: AND,
    pub register_a_enable_or_gate: ORGate3,
    pub register_b_enable_or_gate: ORGate4,
    pub register_b_set_or_gate: ORGate4,
    pub register_a_enable: Wire,
    pub register_b_enable: Wire,
    pub acc_enable_or_gate: ORGate4,
    pub acc_enable_and_gate: AND,
    pub bus_one_enable_or_gate: ORGate4,
    pub iar_enable_or_gate: ORGate4,
    pub iar_enable_and_gate: AND,
    pub ram_enable_or_gate: ORGate5,
    pub ram_enable_and_gate: AND,
    pub gp_reg_enable_and_gates: [ANDGate3; 8],
    pub gp_reg_enable_or_gates: [OR; 4],
    pub gp_reg_set_and_gates: [ANDGate3; 4],

    pub io_bus_set_gate: AND,

    // IR
    pub ir_set_and_gate: AND,
    pub ir_bit4_not_gate: NOT,

    // MAR
    pub mar_set_and_gate: AND,
    pub mar_set_or_gate: ORGate6,

    // IAR
    pub iar_set_and_gate: AND,
    pub iar_set_or_gate: ORGate6,

    // ACC
    pub acc_set_and_gate: AND,
    pub acc_set_or_gate: ORGate4,

    // RAM
    pub ram_set_and_gate: AND,

    // TMP
    pub tmp_set_and_gate: AND,

    // FLAGS
    pub flags_set_and_gate: AND,
    pub flags_set_or_gate: OR,

    pub register_b_set: Wire,

    pub flag_state_gates: [AND; 4],
    pub flag_state_or_gate: ORGate4,

    pub alu_op_and_gates: [ANDGate3; 3],

    pub carry_temp: Bit,
    pub carry_and_gate: AND,

    pub peripherals: Vec<Arc<Mutex<dyn Peripheral>>>,
}

impl CPU {
    pub fn new(main_bus: Arc<Mutex<Bus>>, memory: Arc<Mutex<Memory64K>>) -> Self {
        // TMP
        let tmp_bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let mut tmp = Register::new("TMP", main_bus.clone(), tmp_bus.clone());

        // tmp register is always enabled, and we initialise it with value 0
        CPU::update_enable_status(&mut tmp, true);
        CPU::update_set_status(&mut tmp, true);
        CPU::update_on(&mut tmp);
        CPU::update_set_status(&mut tmp, false);

        // ACC
        let acc_bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let acc = Register::new("ACC", acc_bus.clone(), main_bus.clone());

        // IR
        let control_bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let mut ir = Register::new("IR", main_bus.clone(), control_bus.clone());
        ir.disable();

        // FLAGS
        let alu_to_flags_bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let flags_bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let mut flags = Register::new("FLAGS", alu_to_flags_bus.clone(), flags_bus.clone());

        CPU::update_enable_status(&mut flags, true);
        CPU::update_set_status(&mut flags, true);
        CPU::update_on(&mut flags);
        CPU::update_set_status(&mut flags, false);

        // BUS one
        let busone_output = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let busone = BusOne::new(tmp_bus.clone(), busone_output.clone());

        // ALU
        let alu = ALU::new(
            main_bus.clone(),
            busone_output.clone(),
            acc_bus.clone(),
            alu_to_flags_bus.clone(),
        );

        Self {
            gp_reg0: Register::new("R0", main_bus.clone(), main_bus.clone()),
            gp_reg1: Register::new("R1", main_bus.clone(), main_bus.clone()),
            gp_reg2: Register::new("R2", main_bus.clone(), main_bus.clone()),
            gp_reg3: Register::new("R3", main_bus.clone(), main_bus.clone()),
            tmp,
            acc,
            ir,
            iar: Register::new("IAR", main_bus.clone(), main_bus.clone()),
            flags,
            clock_state: false,
            memory,
            alu,
            stepper: Stepper::new(),
            busone,
            main_bus,
            tmp_bus,
            busone_output,
            control_bus,
            acc_bus,
            alu_to_flags_bus,
            flags_bus,
            io_bus: Arc::new(Mutex::new(IOBus::new())),
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
        }
    }

    fn update_enable_status<T>(enableable: &mut T, state: bool)
    where
        T: Enableable,
    {
        match state {
            true => enableable.enable(),
            false => enableable.disable(),
        }
    }

    fn update_enable_status_arc_mutex<T>(enableable: Arc<Mutex<T>>, state: bool)
    where
        T: Enableable,
    {
        match state {
            true => enableable.lock().unwrap().enable(),
            false => enableable.lock().unwrap().disable(),
        }
    }

    fn update_set_status<T>(set: &mut T, state: bool)
    where
        T: Settable,
    {
        match state {
            true => set.set(),
            false => set.unset(),
        }
    }

    fn update_set_status_arc_mutex<T>(set: Arc<Mutex<T>>, state: bool)
    where
        T: Settable,
    {
        match state {
            true => set.lock().unwrap().set(),
            false => set.lock().unwrap().unset(),
        }
    }

    fn update_on<T>(u: &mut T)
    where
        T: Updatable,
    {
        u.update()
    }

    fn update_on_arc_mutex<T>(u: Arc<Mutex<T>>)
    where
        T: Updatable,
    {
        u.lock().unwrap().update()
    }

    pub fn connect_peripheral<T>(&mut self, p: Arc<Mutex<T>>)
    where
        T: Peripheral + 'static,
    {
        p.lock()
            .unwrap()
            .connect(self.io_bus.clone(), self.main_bus.clone());
        self.peripherals.push(p);
    }

    pub fn set_iar(&mut self, address: u16) {
        self.main_bus.lock().unwrap().set_value(address);

        Self::update_set_status(&mut self.iar, true);
        Self::update_on(&mut self.iar);
        Self::update_set_status(&mut self.iar, false);
        Self::update_on(&mut self.iar);

        self.clear_main_bus()
    }

    pub fn step(&mut self) {
        for _ in 0..2 {
            self.clock_state = match self.clock_state {
                true => false,
                false => true,
            };
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
        self.step4_gates[0].update(self.stepper.get_output_wire(3), self.ir.bit(8));

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
            self.ir.bit(12),
        );

        self.ir_bit4_not_gate.update(self.ir.bit(12));
    }

    fn run_step_5_gates(&mut self) {
        self.step5_gates[0].update(self.stepper.get_output_wire(4), self.ir.bit(8));

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
            self.ir.bit(8),
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
        Self::update_on(&mut self.iar);

        // MAR
        Self::update_on(&mut self.memory.lock().unwrap().address_register);

        // IR
        Self::update_on(&mut self.ir);

        // RAM
        Self::update_on_arc_mutex(self.memory.clone());

        // TMP
        Self::update_on(&mut self.tmp);

        // FLAGS
        Self::update_on(&mut self.flags);

        // BUS1
        Self::update_on(&mut self.busone);

        // ALU
        self.update_alu();

        // ACC
        Self::update_on(&mut self.acc);

        // R0
        Self::update_on(&mut self.gp_reg0);

        // R1
        Self::update_on(&mut self.gp_reg1);

        // R2
        Self::update_on(&mut self.gp_reg2);

        // R3
        Self::update_on(&mut self.gp_reg3);

        self.update_instruction_decoder3x8();
        self.update_io_bus();
        self.update_peripherals();
    }

    fn clear_main_bus(&mut self) {
        self.main_bus.lock().unwrap().set_value(0);
    }

    fn update_instruction_decoder3x8(&mut self) {
        self.instr_decoder3x8.bit0_not_gate.update(self.ir.bit(8));

        self.instr_decoder3x8
            .decoder
            .update(self.ir.bit(9), self.ir.bit(10), self.ir.bit(11));

        for i in 0..8 {
            self.instr_decoder3x8.selector_gates[i].update(
                self.instr_decoder3x8.decoder.get_output_wire(i as i32),
                self.instr_decoder3x8.bit0_not_gate.get(),
            );
        }
    }

    fn update_io_bus(&mut self) {
        self.io_bus
            .lock()
            .unwrap()
            .update(self.ir.bit(12), self.ir.bit(13))
    }

    fn update_peripherals(&mut self) {
        for i in self.peripherals.iter() {
            i.lock().unwrap().update()
        }
    }

    fn update_alu(&mut self) {
        // update ALU operation based on instruction register
        self.alu_op_and_gates[2].update(
            self.ir.bit(9),
            self.ir.bit(8),
            self.stepper.get_output_wire(4),
        );

        self.alu_op_and_gates[1].update(
            self.ir.bit(10),
            self.ir.bit(8),
            self.stepper.get_output_wire(4),
        );
        self.alu_op_and_gates[0].update(
            self.ir.bit(11),
            self.ir.bit(8),
            self.stepper.get_output_wire(4),
        );

        self.alu.op[2].update(self.alu_op_and_gates[2].get());
        self.alu.op[1].update(self.alu_op_and_gates[1].get());
        self.alu.op[0].update(self.alu_op_and_gates[0].get());

        self.alu.carry_in.update(self.carry_and_gate.get());

        self.alu.update();
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

        Self::update_enable_status_arc_mutex(self.io_bus.clone(), self.io_bus_enable_gate.get());
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

        Self::update_enable_status(&mut self.iar, self.iar_enable_and_gate.get());
    }

    fn run_enable_on_bus_one(&mut self, _: bool) {
        self.bus_one_enable_or_gate.update(
            self.stepper.get_output_wire(0),
            self.step4_gates[7].get(),
            self.step4_gates[6].get(),
            self.step4_gates[3].get(),
        );
        Self::update_enable_status(&mut self.busone, self.bus_one_enable_or_gate.get());
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

        Self::update_enable_status(&mut self.acc, self.acc_enable_and_gate.get());
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
        Self::update_enable_status_arc_mutex(self.memory.clone(), self.ram_enable_and_gate.get());
    }

    fn run_enable_on_register_b(&mut self) {
        self.register_b_enable_or_gate.update(
            self.step4_gates[0].get(),
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
        self.instruction_decoder_enables2x4[0].update(self.ir.bit(14), self.ir.bit(15));
        self.instruction_decoder_enables2x4[1].update(self.ir.bit(12), self.ir.bit(13));

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
        Self::update_enable_status(&mut self.gp_reg0, self.gp_reg_enable_or_gates[0].get());

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
        Self::update_enable_status(&mut self.gp_reg1, self.gp_reg_enable_or_gates[1].get());

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
        Self::update_enable_status(&mut self.gp_reg2, self.gp_reg_enable_or_gates[2].get());

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
        Self::update_enable_status(&mut self.gp_reg3, self.gp_reg_enable_or_gates[3].get());
    }
}

// Run set
impl CPU {
    fn run_set(&mut self, state: bool) {
        self.ir_instruction_and_gate
            .update(self.ir.bit(11), self.ir.bit(10), self.ir.bit(9));
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
        let flags_bus = self.flags_bus.lock().unwrap();
        // C
        self.flag_state_gates[0].update(
            self.ir.bit(12),
            flags_bus.get_output_wire(FlagState::Carry as i32),
        );

        // A
        self.flag_state_gates[1].update(
            self.ir.bit(13),
            flags_bus.get_output_wire(FlagState::ALarger as i32),
        );

        // E
        self.flag_state_gates[2].update(
            self.ir.bit(14),
            flags_bus.get_output_wire(FlagState::Equal as i32),
        );

        // Z
        self.flag_state_gates[3].update(
            self.ir.bit(15),
            flags_bus.get_output_wire(FlagState::Zero as i32),
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
        Self::update_set_status_arc_mutex(self.io_bus.clone(), self.io_bus_set_gate.get());
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

        match self.mar_set_and_gate.get() {
            true => self.memory.lock().unwrap().address_register.set(),
            false => self.memory.lock().unwrap().address_register.unset(),
        }
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
        Self::update_set_status(&mut self.iar, self.iar_set_and_gate.get());
    }

    fn run_set_on_ir(&mut self, state: bool) {
        self.ir_set_and_gate
            .update(state, self.stepper.get_output_wire(1));
        Self::update_set_status(&mut self.ir, self.ir_set_and_gate.get());
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
        Self::update_set_status(&mut self.acc, self.acc_set_and_gate.get());
    }

    fn run_set_on_ram(&mut self, state: bool) {
        self.ram_set_and_gate
            .update(state, self.step5_gates[2].get());
        Self::update_set_status_arc_mutex(self.memory.clone(), self.ram_set_and_gate.get());
    }

    fn run_set_on_tmp(&mut self, state: bool) {
        self.tmp_set_and_gate
            .update(state, self.step4_gates[0].get());

        Self::update_set_status(&mut self.tmp, self.tmp_set_and_gate.get());

        self.carry_temp.update(
            self.flags_bus
                .lock()
                .unwrap()
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
        Self::update_set_status(&mut self.flags, self.flags_set_and_gate.get());
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
        self.instruction_decoder_set2x4
            .update(self.ir.bit(14), self.ir.bit(15));

        // R0
        self.gp_reg_set_and_gates[0].update(
            state,
            self.register_b_set.get(),
            self.instruction_decoder_set2x4.get_output_wire(0),
        );
        Self::update_set_status(&mut self.gp_reg0, self.gp_reg_set_and_gates[0].get());

        // R1
        self.gp_reg_set_and_gates[1].update(
            state,
            self.register_b_set.get(),
            self.instruction_decoder_set2x4.get_output_wire(1),
        );
        Self::update_set_status(&mut self.gp_reg1, self.gp_reg_set_and_gates[1].get());

        // R2
        self.gp_reg_set_and_gates[2].update(
            state,
            self.register_b_set.get(),
            self.instruction_decoder_set2x4.get_output_wire(2),
        );
        Self::update_set_status(&mut self.gp_reg2, self.gp_reg_set_and_gates[2].get());

        // R3
        self.gp_reg_set_and_gates[3].update(
            state,
            self.register_b_set.get(),
            self.instruction_decoder_set2x4.get_output_wire(3),
        );
        Self::update_set_status(&mut self.gp_reg3, self.gp_reg_set_and_gates[3].get());
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let main_bus_value = self.main_bus.lock().unwrap().get_value();
        let acc_bus_value = self.acc_bus.lock().unwrap().get_value();
        write!(
            f,
            "step: {}\n{} {} {} {} {}\n{} {} {} {}\n<main_bus>: {:>#06X} <acc_bus>: {:>#06X}\n{}",
            self.stepper,
            self.iar,
            self.memory.lock().unwrap().address_register,
            self.ir,
            self.acc,
            self.tmp,
            self.gp_reg0,
            self.gp_reg1,
            self.gp_reg2,
            self.gp_reg3,
            main_bus_value,
            acc_bus_value,
            self.alu,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_cpu() -> CPU {
        let main_bus = Arc::new(Mutex::new(Bus::new(BUS_WIDTH)));
        let memory = Arc::new(Mutex::new(Memory64K::new(main_bus.clone())));
        CPU::new(main_bus.clone(), memory.clone())
    }

    #[test]
    fn test_cpu_iar_incremented_on_every_cycle() {
        let mut cpu = get_cpu();
        cpu.set_iar(0x0000);

        for i in 0..1000 {
            cpu.do_fetch_decode_execute();
            cpu.check_iar(i + 1);
        }
    }

    #[test]
    fn test_cpu_instruction_received_from_memory() {
        let mut cpu = get_cpu();
        let instructions = vec![0x008A, 0x0082, 0x0088, 0x0094, 0x00B1];

        let mut addr = 0xFF00;
        for i in instructions.iter() {
            set_memory_location(cpu.memory.clone(), addr, *i);
            addr += 1;
        }

        cpu.set_iar(0xFF00);

        for i in instructions.iter() {
            cpu.do_fetch_decode_execute();
            cpu.check_ir(*i);
        }
    }

    #[test]
    fn test_cpu_flags_register_all_false() {
        let mut cpu = get_cpu();
        set_memory_location(cpu.memory.clone(), 0x0000, 0x0081);
        cpu.set_registers(vec![0x0009, 0x00A, 0x0002, 0x0003]);
        cpu.set_iar(0x0000);

        cpu.do_fetch_decode_execute();

        cpu.check_flags_register(false, false, false, false);
    }

    #[test]
    fn test_cpu_flags_register_carry_flag_enabled() {
        let mut cpu = get_cpu();
        set_memory_location(cpu.memory.clone(), 0x0000, 0x0081);
        cpu.set_registers(vec![0x0020, 0xFFFF, 0x0002, 0x0003]);
        cpu.set_iar(0x0000);

        cpu.do_fetch_decode_execute();

        cpu.check_flags_register(true, false, false, false);
    }

    #[test]
    fn test_cpu_flags_register_is_larger_flag_enabled() {
        let mut cpu = get_cpu();
        set_memory_location(cpu.memory.clone(), 0x0000, 0x0081);
        cpu.set_registers(vec![0x0021, 0x0020, 0x0002, 0x0003]);
        cpu.set_iar(0x0000);

        cpu.do_fetch_decode_execute();

        cpu.check_flags_register(false, true, false, false);
    }

    #[test]
    fn test_cpu_flags_register_is_equal_flag_enabled() {
        let mut cpu = get_cpu();
        set_memory_location(cpu.memory.clone(), 0x0000, 0x0081);
        cpu.set_registers(vec![0x0021, 0x0021, 0x0002, 0x0003]);
        cpu.set_iar(0x0000);

        cpu.do_fetch_decode_execute();

        cpu.check_flags_register(false, false, true, false);
    }

    #[test]
    fn test_cpu_flags_register_is_zero_flag_enabled() {
        let mut cpu = get_cpu();
        set_memory_location(cpu.memory.clone(), 0x0000, 0x0081);
        cpu.set_registers(vec![0x0001, 0xFFFF, 0x0002, 0x0003]);
        cpu.set_iar(0x0000);

        cpu.do_fetch_decode_execute();

        cpu.check_flags_register(true, false, false, true);
    }

    #[test]
    fn test_cpu_flags_register_multiple_enabled() {
        let mut cpu = get_cpu();
        set_memory_location(cpu.memory.clone(), 0x0000, 0x0081);
        cpu.set_registers(vec![0xFFFF, 0x0001, 0x0002, 0x0003]);
        cpu.set_iar(0x0000);

        cpu.do_fetch_decode_execute();

        cpu.check_flags_register(true, true, false, true);
    }

    #[test]
    fn test_cpu_ld() {
        let test_ld = |instruction: u16,
                       mem_address: u16,
                       mem_value: u16,
                       input_registers: Vec<u16>,
                       expected_output_registers: Vec<u16>| {
            let mut cpu = get_cpu();
            let ins_addr = 0x0000;
            set_memory_location(cpu.memory.clone(), ins_addr, instruction);
            cpu.set_iar(ins_addr);
            set_memory_location(cpu.memory.clone(), mem_address, mem_value);
            cpu.set_registers(input_registers);

            cpu.do_fetch_decode_execute();

            cpu.check_registers(
                instruction,
                expected_output_registers[0],
                expected_output_registers[1],
                expected_output_registers[2],
                expected_output_registers[3],
            );
        };

        test_ld(
            // LD R0, R0
            0x0000,
            0x0080,
            0x0023,
            vec![0x0080, 0x0081, 0x0082, 0x0083],
            vec![0x0023, 0x0081, 0x0082, 0x0083],
        );
        test_ld(
            // LD R0, R1
            0x0001,
            0x0084,
            0x00F2,
            vec![0x0084, 0x0085, 0x0086, 0x0087],
            vec![0x0084, 0x00F2, 0x0086, 0x0087],
        );
        test_ld(
            // LD R0, R2
            0x0002,
            0x0088,
            0x0001,
            vec![0x0088, 0x0089, 0x008A, 0x008B],
            vec![0x0088, 0x0089, 0x0001, 0x008B],
        );
        test_ld(
            // LD R0, R3
            0x0003,
            0x008C,
            0x005A,
            vec![0x008C, 0x008D, 0x008E, 0x008F],
            vec![0x008C, 0x008D, 0x008E, 0x005A],
        );

        test_ld(
            // LD R1, R0
            0x0004,
            0x0091,
            0x0023,
            vec![0x0090, 0x0091, 0x0092, 0x0093],
            vec![0x0023, 0x0091, 0x0092, 0x0093],
        );
        test_ld(
            // LD R1, R1
            0x0005,
            0x0095,
            0x00F2,
            vec![0x0094, 0x0095, 0x0096, 0x0097],
            vec![0x0094, 0x00F2, 0x0096, 0x0097],
        );
        test_ld(
            // LD R1, R2
            0x0006,
            0x0099,
            0x0001,
            vec![0x0098, 0x0099, 0x009A, 0x009B],
            vec![0x0098, 0x0099, 0x0001, 0x009B],
        );
        test_ld(
            // LD R1, R3
            0x0007,
            0x009D,
            0x005A,
            vec![0x009C, 0x009D, 0x009E, 0x009F],
            vec![0x009C, 0x009D, 0x009E, 0x005A],
        );

        test_ld(
            // LD R2, R0
            0x0008,
            0x00A2,
            0x0023,
            vec![0x00A0, 0x00A1, 0x00A2, 0x00A3],
            vec![0x0023, 0x00A1, 0x00A2, 0x00A3],
        );
        test_ld(
            // LD R2, R1
            0x0009,
            0x00A6,
            0x00F2,
            vec![0x00A4, 0x00A5, 0x00A6, 0x00A7],
            vec![0x00A4, 0x00F2, 0x00A6, 0x00A7],
        );
        test_ld(
            // LD R2, R2
            0x000A,
            0x00AA,
            0x0001,
            vec![0x00A8, 0x00A9, 0x00AA, 0x00AB],
            vec![0x00A8, 0x00A9, 0x0001, 0x00AB],
        );
        test_ld(
            // LD R2, R3
            0x000B,
            0x00AE,
            0x005A,
            vec![0x00AC, 0x00AD, 0x00AE, 0x00AF],
            vec![0x00AC, 0x00AD, 0x00AE, 0x005A],
        );

        test_ld(
            // LD R3, R0
            0x000C,
            0x00B3,
            0x0023,
            vec![0x00B0, 0x00B1, 0x00B2, 0x00B3],
            vec![0x0023, 0x00B1, 0x00B2, 0x00B3],
        );
        test_ld(
            // LD R3, R1
            0x000D,
            0x00B7,
            0x00F2,
            vec![0x00B4, 0x00B5, 0x00B6, 0x00B7],
            vec![0x00B4, 0x00F2, 0x00B6, 0x00B7],
        );
        test_ld(
            // LD R3, R2
            0x000E,
            0x22BB,
            0xAB01,
            vec![0x00B8, 0x00B9, 0x00BA, 0x22BB],
            vec![0x00B8, 0x00B9, 0xAB01, 0x22BB],
        );
        test_ld(
            // LD R3, R3
            0x000F,
            0x00BF,
            0x005A,
            vec![0x00BC, 0x00BD, 0x00BE, 0x00BF],
            vec![0x00BC, 0x00BD, 0x00BE, 0x005A],
        );
    }

    #[test]
    fn test_cpu_st() {
        let test_st = |instruction: u16,
                       input_registers: Vec<u16>,
                       expected_value_addredd: u16,
                       expected_value: u16| {
            let mut cpu = get_cpu();

            // ST value into memory
            let ins_addr = 0x0000;
            set_memory_location(cpu.memory.clone(), ins_addr, instruction);
            cpu.set_iar(ins_addr);
            cpu.set_registers(input_registers.clone());

            cpu.do_fetch_decode_execute();

            //LD value into register zero
            set_memory_location(cpu.memory.clone(), ins_addr + 1, 0x0000);
            cpu.set_iar(ins_addr + 1);

            cpu.set_registers(vec![
                expected_value_addredd,
                input_registers[1].clone(),
                input_registers[2],
                input_registers[3],
            ]);

            cpu.do_fetch_decode_execute();

            assert_eq!(cpu.gp_reg0.value(), expected_value);
        };

        test_st(0x0010, vec![0x00A0, 0x0001, 0x0001, 0x0001], 0x00A0, 0x00A0); // ST R0, R0
        test_st(0x0011, vec![0x00A1, 0x0029, 0x0001, 0x0001], 0x00A1, 0x0029); // ST R0, R1
        test_st(0x0012, vec![0x00A2, 0x0001, 0x007F, 0x0001], 0x00A2, 0x007F); // ST R0, R2
        test_st(0x0013, vec![0x00A3, 0x0001, 0x0001, 0x001B], 0x00A3, 0x001B); // ST R0, R3

        test_st(0x0014, vec![0x00A0, 0x00B4, 0x0001, 0x0001], 0x00B4, 0x00A0); // ST R1, R0
        test_st(0x0015, vec![0x0001, 0x00B5, 0x0001, 0x0001], 0x00B5, 0x00B5); // ST R1, R1
        test_st(0x0016, vec![0x0001, 0x00B6, 0x007F, 0x0001], 0x00B6, 0x007F); // ST R1, R2
        test_st(0x0017, vec![0x0001, 0x00B7, 0x0001, 0x001B], 0x00B7, 0x001B); // ST R1, R3

        test_st(0x0018, vec![0x00A0, 0x0001, 0x00C8, 0x0001], 0x00C8, 0x00A0); // ST R2, R0
        test_st(0x0019, vec![0x0001, 0x0029, 0x00C9, 0x0001], 0x00C9, 0x0029); // ST R2, R1
        test_st(0x001A, vec![0x0001, 0x0001, 0x00CA, 0x0001], 0x00CA, 0x00CA); // ST R2, R2
        test_st(0x001B, vec![0x0001, 0x0001, 0x00CB, 0x001B], 0x00CB, 0x001B); // ST R2, R3

        test_st(0x001C, vec![0x00A0, 0x0001, 0x0001, 0x00DC], 0x00DC, 0x00A0); // ST R3, R0
        test_st(0x001D, vec![0x0001, 0x0029, 0x0001, 0x00DD], 0x00DD, 0x0029); // ST R3, R1
        test_st(0x001E, vec![0x0001, 0x0001, 0x1A7F, 0xFCDE], 0xFCDE, 0x1A7F); // ST R3, R2
        test_st(0x001F, vec![0x0001, 0x0001, 0x0001, 0x00DF], 0x00DF, 0x00DF); // ST R3, R3
    }

    #[test]
    fn test_cpu_data() {
        let mut cpu = get_cpu();
        let ins_addr = 0x0000;

        // DATA R0
        set_memory_location(cpu.memory.clone(), ins_addr, 0x0020);
        set_memory_location(cpu.memory.clone(), ins_addr + 1, 0xF071);

        // DATA R1
        set_memory_location(cpu.memory.clone(), ins_addr + 2, 0x0021);
        set_memory_location(cpu.memory.clone(), ins_addr + 3, 0xF172);

        // DATA R2
        set_memory_location(cpu.memory.clone(), ins_addr + 4, 0x0022);
        set_memory_location(cpu.memory.clone(), ins_addr + 5, 0xF273);

        // DATA R3
        set_memory_location(cpu.memory.clone(), ins_addr + 6, 0x0023);
        set_memory_location(cpu.memory.clone(), ins_addr + 7, 0xF374);

        cpu.set_iar(ins_addr);

        cpu.set_registers(vec![0x0001, 0x0001, 0x0001, 0x0001]);

        for _ in 0..4 {
            cpu.do_fetch_decode_execute();
        }

        cpu.check_registers(0x0020, 0xF071, 0xF172, 0xF273, 0xF374);

        // check IAR has incremented 2 each time
        cpu.check_iar(0x0008)
    }

    #[test]
    fn test_cpu_jmpr() {
        let test_jmpr = |instruction: u16, input_registers: Vec<u16>, expected_iar: u16| {
            let mut cpu = get_cpu();
            let ins_addr = 0x0000;
            // JMPR
            set_memory_location(cpu.memory.clone(), ins_addr, instruction);
            cpu.set_iar(ins_addr);
            cpu.set_registers(input_registers.clone());

            cpu.do_fetch_decode_execute();

            // registers shouldn't change
            cpu.check_registers(
                instruction,
                input_registers[0],
                input_registers[1],
                input_registers[2],
                input_registers[3],
            );

            cpu.check_iar(expected_iar);
        };
        test_jmpr(0x0030, vec![0x0083, 0x0001, 0x0001, 0x0001], 0x0083); // JR R0
        test_jmpr(0x0031, vec![0x0001, 0x00F1, 0x0001, 0x0001], 0x00F1); // JR R1
        test_jmpr(0x0032, vec![0x0001, 0x0001, 0x00BB, 0x0001], 0x00BB); // JR R2
        test_jmpr(0x0033, vec![0x0001, 0x0001, 0x0001, 0xFF19], 0xFF19); // JR R3
    }

    #[test]
    fn test_cpu_jmp() {
        let test_jmp = |expected_iar: u16| {
            let mut cpu = get_cpu();
            let ins_addr = 0x0000;

            // JMP
            set_memory_location(cpu.memory.clone(), ins_addr, 0x0040);
            set_memory_location(cpu.memory.clone(), ins_addr + 1, expected_iar);

            cpu.set_iar(ins_addr);

            let input_registers = vec![0x0001, 0x0001, 0x0001, 0x0001];
            cpu.set_registers(input_registers.clone());

            cpu.do_fetch_decode_execute();

            // registers shouldn't change
            cpu.check_registers(
                0x0040,
                input_registers[0],
                input_registers[1],
                input_registers[2],
                input_registers[3],
            );

            // check IAR has jumped to the new location
            cpu.check_iar(expected_iar);
        };

        for i in 0..0x0005 {
            test_jmp(i);
        }
    }

    #[test]
    fn test_cpu_jmpz() {
        // JMPZ
        // perform NOT on R0 (0x00FF) to trigger zero flag
        test_jmp_conditional(
            0x0051,
            0x00AE,
            0x00B0,
            vec![0xFFFF, 0x0001, 0x0001, 0x0010],
            0x00AE,
        );

        // should not jump in false case
        test_jmp_conditional(
            0x0051,
            0x00AF,
            0x00B0,
            vec![0x0000, 0x0011, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpe() {
        // JMPE
        test_jmp_conditional(
            0x0052,
            0x00AE,
            0x00F1,
            vec![0x0000, 0x0000, 0x0001, 0x0020],
            0x00AE,
        );

        // should not jump in false case
        test_jmp_conditional(
            0x0052,
            0x00AF,
            0x00F1,
            vec![0x0010, 0x0011, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpez() {
        // JMPEZ
        // Jump if A = B or zero flag
        // a = b
        test_jmp_conditional(
            0x0053,
            0x0020,
            0x00F1,
            vec![0x0002, 0x0002, 0x0001, 0x0020],
            0x0020,
        );

        // zero flag (using and)
        test_jmp_conditional(
            0x0053,
            0x0020,
            0x00C1,
            vec![0x0001, 0x00FE, 0x0002, 0x0020],
            0x0020,
        );

        // should not jump in false case
        test_jmp_conditional(
            0x0053,
            0x0021,
            0x00F1,
            vec![0x0001, 0x0003, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpa() {
        // JMPA
        test_jmp_conditional(
            0x0054,
            0x0020,
            0x00F1,
            vec![0x0002, 0x0001, 0x0001, 0x0020],
            0x0020,
        );

        // should not jump in false case
        test_jmp_conditional(
            0x0054,
            0x0021,
            0x00F1,
            vec![0x0001, 0x0003, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpaz() {
        // JMPAZ
        // Jump is A is larger or zero flag
        // a larger
        test_jmp_conditional(
            0x0055,
            0x0020,
            0x00F1,
            vec![0x0002, 0x0001, 0x0001, 0x0020],
            0x0020,
        );

        // zero flag (using and)
        test_jmp_conditional(
            0x0055,
            0x0020,
            0x00C1,
            vec![0x0001, 0x00FE, 0x0002, 0x0002],
            0x0020,
        );

        // should not jump in false case
        test_jmp_conditional(
            0x0055,
            0x0021,
            0x00F1,
            vec![0x0001, 0x0003, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpae() {
        // JMPAE
        // Jump is A is larger or A = B
        // a larger
        test_jmp_conditional(
            0x0056,
            0x0020,
            0x00F1,
            vec![0x0002, 0x0001, 0x0001, 0x0020],
            0x0020,
        );

        //a = b
        test_jmp_conditional(
            0x0056,
            0x0020,
            0x00F1,
            vec![0x0002, 0x0002, 0x0001, 0x0020],
            0x0020,
        );

        // should not jump in false case
        test_jmp_conditional(
            0x0056,
            0x0021,
            0x00F1,
            vec![0x0001, 0x0003, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpaez() {
        // JMPAEZ
        // Jump if a is larger OR a = b OR zero flag

        // a larger
        test_jmp_conditional(
            0x0057,
            0x0020,
            0x00F1,
            vec![0x0002, 0x0001, 0x0001, 0x0020],
            0x0020,
        );

        // a = b
        test_jmp_conditional(
            0x0057,
            0x0020,
            0x00F1,
            vec![0x0002, 0x0002, 0x0001, 0x0020],
            0x0020,
        );

        // zero flag (using and)
        test_jmp_conditional(
            0x0057,
            0x0020,
            0x00C1,
            vec![0x0001, 0x00FE, 0x0002, 0x0002],
            0x0020,
        );

        // should not jump in false case
        test_jmp_conditional(
            0x0057,
            0x0021,
            0x00F1,
            vec![0x0001, 0x0003, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpc() {
        // JMPC
        test_jmp_conditional(
            0x0058,
            0x0090,
            0x0081,
            vec![0x0004, 0xFFFF, 0x0001, 0x0020],
            0x0090,
        );

        // should not jump in false case
        test_jmp_conditional(
            0x0058,
            0x0091,
            0x0081,
            vec![0x0005, 0x0006, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpcz() {
        // JMPCZ
        // Jump If Carry or zero flag
        // carry condition
        test_jmp_conditional(
            0x0059,
            0x0090,
            0x0081,
            vec![0x0004, 0xFFFF, 0x0001, 0x0020],
            0x0090,
        );
        // zero flag
        test_jmp_conditional(
            0x0059,
            0x0090,
            0x00B0,
            vec![0xFFFF, 0x00FE, 0x00FE, 0x00FE],
            0x0090,
        );
        // should not jump in false case
        test_jmp_conditional(
            0x0059,
            0x0091,
            0x0081,
            vec![0x0001, 0x0002, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpce() {
        // JMPCE
        // Jump If Carry or A = B
        // carry condition
        test_jmp_conditional(
            0x005A,
            0x0090,
            0x0081,
            vec![0x0004, 0xFFFF, 0x0001, 0x0020],
            0x0090,
        );
        // a = b
        test_jmp_conditional(
            0x005A,
            0x0090,
            0x0081,
            vec![0x0002, 0x0002, 0x0001, 0x0020],
            0x0090,
        );
        // should not jump in false case
        test_jmp_conditional(
            0x005A,
            0x0091,
            0x0081,
            vec![0x0001, 0x0002, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpcez() {
        // JMPCEZ
        // Jump if Carry OR a = b OR zero flag

        // carry condition
        test_jmp_conditional(
            0x005B,
            0x0090,
            0x0081,
            vec![0x0004, 0xFFFF, 0x0001, 0x0020],
            0x0090,
        );

        // a = b
        test_jmp_conditional(
            0x005B,
            0x0020,
            0x00F1,
            vec![0x0002, 0x0002, 0x0001, 0x0020],
            0x0020,
        );

        // zero flag (using and)
        test_jmp_conditional(
            0x005B,
            0x0020,
            0x00C1,
            vec![0x0001, 0x00FE, 0x0002, 0x0002],
            0x0020,
        );

        // should not jump in false case
        test_jmp_conditional(
            0x005B,
            0x0091,
            0x0081,
            vec![0x0001, 0x0002, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpca() {
        // JMPCA
        // Jump If Carry or A larger
        // carry condition
        test_jmp_conditional(
            0x005C,
            0x0090,
            0x0081,
            vec![0x0004, 0xFFFF, 0x0001, 0x0020],
            0x0090,
        );
        // a is larger
        test_jmp_conditional(
            0x005C,
            0x0090,
            0x0081,
            vec![0x000A, 0x0001, 0x0001, 0x0020],
            0x0090,
        );
        // should not jump in false case
        test_jmp_conditional(
            0x005C,
            0x0091,
            0x0081,
            vec![0x0001, 0x0001, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpcaz() {
        // JMPCAZ
        // Jump if Carry OR A is Larger OR zero flag

        // carry condition
        test_jmp_conditional(
            0x005D,
            0x0090,
            0x0081,
            vec![0x0004, 0xFFFF, 0x0001, 0x0020],
            0x0090,
        );

        // a larger
        test_jmp_conditional(
            0x005D,
            0x0020,
            0x00F1,
            vec![0x0002, 0x0001, 0x0001, 0x0020],
            0x0020,
        );

        // zero flag (using and)
        test_jmp_conditional(
            0x005D,
            0x0020,
            0x00C1,
            vec![0x0001, 0x00FE, 0x0002, 0x0002],
            0x0020,
        );

        // should not jump in false case
        test_jmp_conditional(
            0x005D,
            0x0091,
            0x0081,
            vec![0x0001, 0x0002, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpcae() {
        // JMPCAE
        // Jump if Carry OR A is Larger OR A = B

        // carry condition
        test_jmp_conditional(
            0x005E,
            0x0090,
            0x0081,
            vec![0x0004, 0xFFFF, 0x0001, 0x0020],
            0x0090,
        );

        // a larger
        test_jmp_conditional(
            0x005E,
            0x0020,
            0x00F1,
            vec![0x0002, 0x0001, 0x0001, 0x0020],
            0x0020,
        );

        // a = b
        test_jmp_conditional(
            0x005E,
            0x0020,
            0x00F1,
            vec![0x0002, 0x0002, 0x0001, 0x0020],
            0x0020,
        );

        // should not jump in false case
        test_jmp_conditional(
            0x005E,
            0x0091,
            0x0081,
            vec![0x0001, 0x0002, 0x0001, 0x0001],
            0x0003,
        );
    }

    #[test]
    fn test_cpu_jmpcaez() {
        // JMPCAEZ
        // Jump if Carry OR A is Larger OR A = B OR zero flag

        // carry condition
        test_jmp_conditional(
            0x005F,
            0x0090,
            0x0081,
            vec![0x0004, 0xFFFF, 0x0001, 0x0020],
            0x0090,
        );

        // a larger
        test_jmp_conditional(
            0x005F,
            0x0020,
            0x00F1,
            vec![0x0002, 0x0001, 0x0001, 0x0020],
            0x0020,
        );

        // a = b
        test_jmp_conditional(
            0x005F,
            0x0020,
            0x00F1,
            vec![0x0002, 0x0002, 0x0001, 0x0020],
            0x0020,
        );

        // zero flag (using and)
        test_jmp_conditional(
            0x005F,
            0x0020,
            0x00C1,
            vec![0x0001, 0x00FE, 0x0002, 0x0002],
            0x0020,
        );

        // should not jump in false case
        test_jmp_conditional(
            0x005F,
            0x0091,
            0x0081,
            vec![0x0001, 0x0002, 0x0001, 0x0001],
            0x0003,
        );
    }

    fn test_jmp_conditional(
        jmp_condition_instr: u16,
        destination: u16,
        initial_instr: u16,
        input_registers: Vec<u16>,
        expected_iar: u16,
    ) {
        let mut cpu = get_cpu();
        let ins_addr = 0x0000;

        set_memory_location(cpu.memory.clone(), ins_addr, initial_instr);
        set_memory_location(cpu.memory.clone(), ins_addr + 1, jmp_condition_instr);
        set_memory_location(cpu.memory.clone(), ins_addr + 2, destination);

        cpu.set_iar(ins_addr);
        cpu.set_registers(input_registers);

        cpu.do_fetch_decode_execute();
        cpu.do_fetch_decode_execute();

        cpu.check_iar(expected_iar);
    }

    #[test]
    fn test_cpu_clf() {
        let test_clf = |initial_instruction: u16, initial_registers: Vec<u16>| {
            let mut cpu = get_cpu();
            let ins_addr = 0x0000;
            set_memory_location(cpu.memory.clone(), ins_addr, initial_instruction);
            set_memory_location(cpu.memory.clone(), ins_addr + 1, 0x0060);
            cpu.set_iar(ins_addr);
            cpu.set_registers(initial_registers);

            cpu.do_fetch_decode_execute();
            cpu.do_fetch_decode_execute();
            cpu.check_flags_register(false, false, false, false);
        };

        // carry + zero + greater
        test_clf(0x0081, vec![0xFFFF, 0x0001, 0x0000, 0x0000]);
        // equal flag
        test_clf(0x0081, vec![0x0001, 0x0001, 0x0000, 0x0000]);
        // all flags should be false anyway
        test_clf(0x0081, vec![0x0001, 0x0002, 0x0000, 0x0000]);
    }

    #[test]
    fn test_cpu_io_input_instruction() {}

    #[test]
    fn test_cpu_io_output_instruction() {}

    #[test]
    fn test_cpu_alu_add() {
        let mut cpu = get_cpu();
        let inputs = vec![0x0002, 0x0003, 0x0004, 0x0005];

        for i in 0..4 {
            for j in 0..4 {
                let mut res = vec![inputs[0], inputs[1], inputs[2], inputs[3]];
                match i {
                    0 => match j {
                        0 => res[0] += inputs[0], // ADD R0, R0
                        1 => res[1] += inputs[0], // ADD R1, R0
                        2 => res[2] += inputs[0], // ADD R2, R0
                        3 => res[3] += inputs[0], // ADD R3, R0
                        _ => {}
                    },
                    1 => match j {
                        0 => res[0] += inputs[1], // ADD R0, R1
                        1 => res[1] += inputs[1], // ADD R1, R1
                        2 => res[2] += inputs[1], // ADD R2, R1
                        3 => res[3] += inputs[1], // ADD R3, R1
                        _ => {}
                    },
                    2 => match j {
                        0 => res[0] += inputs[2], // ADD R0, R2
                        1 => res[1] += inputs[2], // ADD R1, R2
                        2 => res[2] += inputs[2], // ADD R2, R2
                        3 => res[3] += inputs[2], // ADD R3, R2
                        _ => {}
                    },
                    3 => match j {
                        0 => res[0] += inputs[3], // ADD R0, R3
                        1 => res[1] += inputs[3], // ADD R1, R3
                        2 => res[2] += inputs[3], // ADD R2, R3
                        3 => res[3] += inputs[3], // ADD R3, R3
                        _ => {}
                    },
                    _ => {}
                }

                cpu.test_instruction(0x0080 + i * 4 + j, inputs.to_vec(), res);
            }
        }
    }

    #[test]
    fn test_cpu_alu_add_with_array() {
        let test_alu_add_with_arrry =
            |instruction: u16, input_registers: Vec<u16>, expected_output_registers: Vec<u16>| {
                let mut cpu = get_cpu();
                cpu.set_cpu_memory_location(0x0000, instruction);
                cpu.set_cpu_memory_location(0x0001, instruction);
                cpu.set_iar(0x0000);

                cpu.set_registers(input_registers);
                cpu.do_fetch_decode_execute();

                cpu.set_registers(vec![0x0000, 0x0000, 0x0000, 0x0000]);
                cpu.do_fetch_decode_execute();

                cpu.check_registers(
                    instruction,
                    expected_output_registers[0],
                    expected_output_registers[1],
                    expected_output_registers[2],
                    expected_output_registers[3],
                );
            };

        test_alu_add_with_arrry(
            0x0080,
            vec![0xFFFE, 0x0000, 0x0000, 0x0000],
            vec![0x0001, 0x0000, 0x0000, 0x0000],
        );
        test_alu_add_with_arrry(
            0x0081,
            vec![0xFFFF, 0x0000, 0x0000, 0x0000],
            vec![0x0000, 0x0000, 0x0000, 0x0000],
        );
        test_alu_add_with_arrry(
            0x0081,
            vec![0xFFFF, 0x0001, 0x0000, 0x0000],
            vec![0x0000, 0x0001, 0x0000, 0x0000],
        );
        test_alu_add_with_arrry(
            0x0081,
            vec![0xFFFE, 0x0005, 0x0000, 0x0000],
            vec![0x0000, 0x0001, 0x0000, 0x0000],
        );
        test_alu_add_with_arrry(
            0x0082,
            vec![0xFFFE, 0x0000, 0x0005, 0x0000],
            vec![0x0000, 0x0000, 0x0001, 0x0000],
        );
        test_alu_add_with_arrry(
            0x0083,
            vec![0xFFFE, 0x0000, 0x0000, 0x0005],
            vec![0x0000, 0x0000, 0x0000, 0x0001],
        );
    }

    #[test]
    fn test_cpu_alu_shl() {
        let mut cpu = get_cpu();
        let ones = vec![0x0001, 0x0001, 0x0001, 0x0001];

        for shift in 0..16 {
            cpu.test_shift(
                // SHL R0
                0x0090,
                ones.clone(),
                vec![0x0001 << shift, 0x0001, 0x0001, 0x0001],
                shift,
            );
            cpu.test_shift(
                // SHL R1
                0x0095,
                ones.clone(),
                vec![0x0001, 0x0001 << shift, 0x0001, 0x0001],
                shift,
            );
            cpu.test_shift(
                // SHL R2
                0x009A,
                ones.clone(),
                vec![0x0001, 0x0001, 0x0001 << shift, 0x0001],
                shift,
            );
            cpu.test_shift(
                // SHL R3
                0x009F,
                ones.clone(),
                vec![0x0001, 0x0001, 0x0001, 0x0001 << shift],
                shift,
            );
        }
    }

    #[test]
    fn test_cpu_alu_shr() {
        let mut cpu = get_cpu();
        let ones = vec![0x8000, 0x8000, 0x8000, 0x8000];

        for shift in 0..16 {
            cpu.test_shift(
                // SHR R0
                0x00A0,
                ones.clone(),
                vec![0x8000 >> shift, 0x8000, 0x8000, 0x8000],
                shift,
            );
            cpu.test_shift(
                // SHR R1
                0x00A5,
                ones.clone(),
                vec![0x8000, 0x8000 >> shift, 0x8000, 0x8000],
                shift,
            );
            cpu.test_shift(
                // SHR R2
                0x00AA,
                ones.clone(),
                vec![0x8000, 0x8000, 0x8000 >> shift, 0x8000],
                shift,
            );
            cpu.test_shift(
                // SHR R3
                0x00AF,
                ones.clone(),
                vec![0x8000, 0x8000, 0x8000, 0x8000 >> shift],
                shift,
            );
        }
    }

    #[test]
    fn test_cpu_alu_not() {
        let mut cpu = get_cpu();
        cpu.test_instruction(
            // NOT R0
            0x00B0,
            vec![0x0000, 0x0000, 0x0000, 0x0000],
            vec![0xFFFF, 0x0000, 0x0000, 0x0000],
        );
        cpu.test_instruction(
            // NOT R1
            0x00B5,
            vec![0x0000, 0xFF00, 0x0000, 0x0000],
            vec![0x0000, 0x00FF, 0x0000, 0x0000],
        );
        cpu.test_instruction(
            // NOT R2
            0x00BA,
            vec![0x0000, 0x0000, 0xEEEE, 0x0000],
            vec![0x0000, 0x0000, 0x1111, 0x0000],
        );
        cpu.test_instruction(
            // NOT R3
            0x00BF,
            vec![0x0000, 0x0000, 0x0000, 0x1100],
            vec![0x0000, 0x0000, 0x0000, 0xEEFF],
        );
    }

    #[test]
    fn test_cpu_alu_and() {
        let mut cpu = get_cpu();
        let input = vec![0xF00F, 0x0F0F, 0x0FF0, 0x00F1];
        for i in 0..4 {
            for j in 0..4 {
                let mut output = vec![input[0], input[1], input[2], input[3]];
                match i {
                    0 => match j {
                        0 => output[0] = 0xF00F, // AND R0, R0
                        1 => output[1] = 0x000F, // AND R0, R1
                        2 => output[2] = 0x0000, // AND R0, R2
                        3 => output[3] = 0x0001, // AND R0, R3
                        _ => {}
                    },
                    1 => match j {
                        0 => output[0] = 0x000F, // AND R1, R0
                        1 => output[1] = 0x0F0F, // AND R1, R1
                        2 => output[2] = 0x0F00, // AND R1, R2
                        3 => output[3] = 0x0001, // AND R1, R3
                        _ => {}
                    },
                    2 => match j {
                        0 => output[0] = 0x0000, // AND R2, R0
                        1 => output[1] = 0x0F00, // AND R2, R1
                        2 => output[2] = 0x0FF0, // AND R2, R2
                        3 => output[3] = 0x00F0, // AND R2, R3
                        _ => {}
                    },
                    3 => match j {
                        0 => output[0] = 0x0001, // AND R3, R0
                        1 => output[1] = 0x0001, // AND R3, R1
                        2 => output[2] = 0x00F0, // AND R3, R2
                        3 => output[3] = 0x00F1, // AND R3, R3
                        _ => {}
                    },
                    _ => {}
                };
                cpu.test_instruction(0x00C0 + i * 4 + j, input.to_vec(), output);
            }
        }
    }

    #[test]
    fn test_cpu_alu_or() {
        let mut cpu = get_cpu();
        let input = vec![0xF00F, 0x0F0F, 0x0FF0, 0x00F1];
        for i in 0..4 {
            for j in 0..4 {
                let mut output = vec![input[0], input[1], input[2], input[3]];
                match i {
                    0 => match j {
                        0 => output[0] = 0xF00F, // OR R0, R0
                        1 => output[1] = 0xFF0F, // OR R0, R1
                        2 => output[2] = 0xFFFF, // OR R0, R2
                        3 => output[3] = 0xF0FF, // OR R0, R3
                        _ => {}
                    },
                    1 => match j {
                        0 => output[0] = 0xFF0F, // OR R1, R0
                        1 => output[1] = 0x0F0F, // OR R1, R1
                        2 => output[2] = 0x0FFF, // OR R1, R2
                        3 => output[3] = 0x0FFF, // OR R1, R3
                        _ => {}
                    },
                    2 => match j {
                        0 => output[0] = 0xFFFF, // OR R2, R0
                        1 => output[1] = 0x0FFF, // OR R2, R1
                        2 => output[2] = 0x0FF0, // OR R2, R2
                        3 => output[3] = 0x0FF1, // OR R2, R3
                        _ => {}
                    },
                    3 => match j {
                        0 => output[0] = 0xF0FF, // OR R3, R0
                        1 => output[1] = 0x0FFF, // OR R3, R1
                        2 => output[2] = 0x0FF1, // OR R3, R2
                        3 => output[3] = 0x00F1, // OR R3, R3
                        _ => {}
                    },
                    _ => {}
                };
                cpu.test_instruction(0x00D0 + i * 4 + j, input.to_vec(), output);
            }
        }
    }

    #[test]
    fn test_cpu_alu_xor() {
        let mut cpu = get_cpu();
        let input = vec![0xF00F, 0x0F0F, 0x0FF0, 0x00F1];
        for i in 0..4 {
            for j in 0..4 {
                let mut output = vec![input[0], input[1], input[2], input[3]];
                match i {
                    0 => match j {
                        0 => output[0] = 0x0000, // XOR R0, R0
                        1 => output[1] = 0xFF00, // XOR R0, R1
                        2 => output[2] = 0xFFFF, // XOR R0, R2
                        3 => output[3] = 0xF0FE, // XOR R0, R3
                        _ => {}
                    },
                    1 => match j {
                        0 => output[0] = 0xFF00, // XOR R1, R0
                        1 => output[1] = 0x0000, // XOR R1, R1
                        2 => output[2] = 0x00FF, // XOR R1, R2
                        3 => output[3] = 0x0FFE, // XOR R1, R3
                        _ => {}
                    },
                    // let input = vec![0xF00F, 0x0F0F, 0x0FF0, 0x00F1];
                    2 => match j {
                        0 => output[0] = 0xFFFF, // XOR R2, R0
                        1 => output[1] = 0x00FF, // XOR R2, R1
                        2 => output[2] = 0x0000, // XOR R2, R2
                        3 => output[3] = 0x0F01, // XOR R2, R3
                        _ => {}
                    },
                    3 => match j {
                        0 => output[0] = 0xF0FE, // XOR R3, R0
                        1 => output[1] = 0x0FFE, // XOR R3, R1
                        2 => output[2] = 0x0F01, // XOR R3, R2
                        3 => output[3] = 0x0000, // XOR R3, R3
                        _ => {}
                    },
                    _ => {}
                };
                cpu.test_instruction(0x00E0 + i * 4 + j, input.to_vec(), output);
            }
        }
    }

    #[test]
    fn test_cpu_alu_cmp() {
        let test_alu_cmp = |instruction: u16,
                            input_registers: Vec<u16>,
                            expected_output_registers: Vec<u16>,
                            compare_a: i32,
                            compare_b: i32| {
            let mut cpu = get_cpu();
            let ins_addr = 0x0000;
            set_memory_location(cpu.memory.clone(), ins_addr, instruction);
            cpu.set_registers(input_registers.clone());
            cpu.set_iar(ins_addr);

            cpu.do_fetch_decode_execute();

            cpu.check_registers(
                instruction,
                expected_output_registers[0],
                expected_output_registers[1],
                expected_output_registers[2],
                expected_output_registers[3],
            );
            cpu.check_flags_register(
                false,
                input_registers[compare_a as usize] > input_registers[compare_b as usize],
                input_registers[compare_a as usize] == input_registers[compare_b as usize],
                false,
            );
        };

        let inputs = vec![0xAB92, 0x0092, 0x0045, 0x00AF];
        let mut instruction = 0x00F0;
        for i in 0..4 {
            for j in 0..4 {
                test_alu_cmp(instruction, inputs.clone(), inputs.clone(), i, j);
                instruction += 1;
            }
        }

        let zeros = vec![0x0000, 0x0000, 0x0000, 0x0000];
        instruction = 0x00F0;
        for i in 0..4 {
            for j in 0..4 {
                test_alu_cmp(instruction, zeros.clone(), zeros.clone(), i, j);
                instruction += 1;
            }
        }
    }

    #[test]
    fn test_cpu_alu_subtract() {
        let test_alu_subtract = |input_a: u16, input_b: u16| {
            let mut cpu = get_cpu();
            cpu.set_registers(vec![input_a, input_b, 1, 0]);
            set_memory_location(cpu.memory.clone(), 0x0000, 0x00B5); // NOT
            set_memory_location(cpu.memory.clone(), 0x0001, 0x0089); // ADD R1, R2
            set_memory_location(cpu.memory.clone(), 0x0002, 0x0060); // CLF
            set_memory_location(cpu.memory.clone(), 0x0003, 0x0081); // ADD R1, R0

            cpu.set_iar(0x0000);
            cpu.do_fetch_decode_execute();
            cpu.do_fetch_decode_execute();
            cpu.do_fetch_decode_execute();
            cpu.do_fetch_decode_execute();
            cpu.check_register(0x0081, 1, input_a - input_b);
        };

        test_alu_subtract(0x0000, 0x0000);
        test_alu_subtract(0xFFFF, 0x1111);
        test_alu_subtract(0xFFFF, 0xFFFF);
    }

    #[test]
    fn test_cpu_alu_multiply() {
        let test_cpu_multiply = |test_id: u16, input_a: u16, input_b: u16| {
            let mut cpu = get_cpu();
            let ins_addr = 0x0000;

            // DATA R3, 1
            // XOR R2, R2
            // CLF
            // SHR R0
            // JC ins_addr+9
            // JMP ins_addr+11
            // CLF
            // ADD R1, R2
            // CLF
            // SHL R1
            // SHL R3
            // JC, ins_addr+18
            // JMP, ins_addr+3

            // DATA R3, 1
            set_memory_location(cpu.memory.clone(), ins_addr + 0, 0x0023);
            set_memory_location(cpu.memory.clone(), ins_addr + 1, 0x0001);
            // XOR R2, R2
            set_memory_location(cpu.memory.clone(), ins_addr + 2, 0x00EA);
            // CLF
            set_memory_location(cpu.memory.clone(), ins_addr + 3, 0x0060);
            // SHR R0
            set_memory_location(cpu.memory.clone(), ins_addr + 4, 0x00A0);
            // JC 59
            set_memory_location(cpu.memory.clone(), ins_addr + 5, 0x0058);
            set_memory_location(cpu.memory.clone(), ins_addr + 6, ins_addr + 9);
            // JMP 61
            set_memory_location(cpu.memory.clone(), ins_addr + 7, 0x0040);
            set_memory_location(cpu.memory.clone(), ins_addr + 8, ins_addr + 11);
            // CLF
            set_memory_location(cpu.memory.clone(), ins_addr + 9, 0x0060);
            // ADD R1, R2
            set_memory_location(cpu.memory.clone(), ins_addr + 10, 0x0086);
            // CLF
            set_memory_location(cpu.memory.clone(), ins_addr + 11, 0x0060);
            // SHL R1
            set_memory_location(cpu.memory.clone(), ins_addr + 12, 0x0095);
            // SHL R3
            set_memory_location(cpu.memory.clone(), ins_addr + 13, 0x009F);
            // JC 68
            set_memory_location(cpu.memory.clone(), ins_addr + 14, 0x0058);
            set_memory_location(cpu.memory.clone(), ins_addr + 15, ins_addr + 18);
            // JMP 53
            set_memory_location(cpu.memory.clone(), ins_addr + 16, 0x0040);
            set_memory_location(cpu.memory.clone(), ins_addr + 17, ins_addr + 3);

            cpu.set_registers(vec![input_a, input_b, 0, 0]);
            cpu.set_iar(ins_addr);

            loop {
                cpu.do_fetch_decode_execute();
                if cpu.iar.value() >= ins_addr + 18 {
                    break;
                }
            }

            cpu.check_register(test_id, 2, input_a * input_b);
        };

        test_cpu_multiply(0, 0x0000, 0x0000);
        test_cpu_multiply(1, 0x0001, 0x0001);
        test_cpu_multiply(2, 0x0001, 0x0002);
        test_cpu_multiply(3, 0x0002, 0x0001);
        test_cpu_multiply(4, 0x0002, 0x0002);
        test_cpu_multiply(5, 0x000F, 0x000F);
    }

    fn set_memory_location(memory: Arc<Mutex<Memory64K>>, address: u16, value: u16) {
        memory.lock().unwrap().address_register.set();
        memory
            .lock()
            .unwrap()
            .bus
            .lock()
            .unwrap()
            .set_value(address);
        memory.lock().unwrap().update();

        memory.lock().unwrap().address_register.unset();
        memory.lock().unwrap().update();

        memory.lock().unwrap().bus.lock().unwrap().set_value(value);
        memory.lock().unwrap().set();
        memory.lock().unwrap().update();

        memory.lock().unwrap().unset();
        memory.lock().unwrap().update();
    }

    impl CPU {
        fn test_instruction(
            &mut self,
            instruction: u16,
            input_registers: Vec<u16>,
            expected_output_registers: Vec<u16>,
        ) {
            self.set_cpu_memory_location(0x0000, instruction);
            self.set_iar(0x0000);
            self.set_registers(input_registers);

            self.do_fetch_decode_execute();

            self.check_registers(
                instruction,
                expected_output_registers[0],
                expected_output_registers[1],
                expected_output_registers[2],
                expected_output_registers[3],
            )
        }

        fn test_shift(
            &mut self,
            instruction: u16,
            input_registers: Vec<u16>,
            expected_output_registers: Vec<u16>,
            shifts: u16,
        ) {
            for i in 0..shifts {
                set_memory_location(self.memory.clone(), i, instruction);
            }

            self.set_registers(input_registers);
            self.set_iar(0x0000);

            for _ in 0..shifts {
                self.do_fetch_decode_execute();
            }

            self.check_registers(
                instruction,
                expected_output_registers[0],
                expected_output_registers[1],
                expected_output_registers[2],
                expected_output_registers[3],
            );
        }

        fn set_registers(&mut self, values: Vec<u16>) {
            for i in 0..values.len() {
                self.set_cpu_register(i as u16, values[i]);
            }
        }

        fn set_cpu_memory_location(&mut self, address: u16, value: u16) {
            self.memory.lock().unwrap().address_register.set();
            {
                self.main_bus.lock().unwrap().set_value(address);
            }
            self.memory.lock().unwrap().update();

            self.memory.lock().unwrap().address_register.unset();
            self.memory.lock().unwrap().update();

            {
                self.main_bus.lock().unwrap().set_value(value);
            }
            self.memory.lock().unwrap().set();
            self.memory.lock().unwrap().update();

            self.memory.lock().unwrap().unset();
            self.memory.lock().unwrap().update();
        }

        fn set_cpu_register(&mut self, register: u16, value: u16) {
            match register {
                0 => {
                    self.gp_reg0.set();
                    self.gp_reg0.update();

                    self.main_bus.lock().unwrap().set_value(value);
                    self.gp_reg0.update();

                    self.gp_reg0.unset();
                    self.gp_reg0.update();
                }
                1 => {
                    self.gp_reg1.set();
                    self.gp_reg1.update();

                    self.main_bus.lock().unwrap().set_value(value);
                    self.gp_reg1.update();

                    self.gp_reg1.unset();
                    self.gp_reg1.update();
                }
                2 => {
                    self.gp_reg2.set();
                    self.gp_reg2.update();

                    self.main_bus.lock().unwrap().set_value(value);
                    self.gp_reg2.update();

                    self.gp_reg2.unset();
                    self.gp_reg2.update();
                }
                3 => {
                    self.gp_reg3.set();
                    self.gp_reg3.update();

                    self.main_bus.lock().unwrap().set_value(value);
                    self.gp_reg3.update();

                    self.gp_reg3.unset();
                    self.gp_reg3.update();
                }

                _ => panic!("Unknown register"),
            }
        }

        fn do_fetch_decode_execute(&mut self) {
            for _ in 0..6 {
                self.step();
            }
        }

        fn check_registers(
            &self,
            instruction: u16,
            exp_reg0: u16,
            exp_reg1: u16,
            exp_reg2: u16,
            exp_reg3: u16,
        ) {
            self.check_register(instruction, 0, exp_reg0);
            self.check_register(instruction, 1, exp_reg1);
            self.check_register(instruction, 2, exp_reg2);
            self.check_register(instruction, 3, exp_reg3);
        }

        fn check_register(&self, instruction: u16, register: u16, expected: u16) {
            let reg_value: u16 = match register {
                0 => self.gp_reg0.value(),
                1 => self.gp_reg1.value(),
                2 => self.gp_reg2.value(),
                3 => self.gp_reg3.value(),
                _ => panic!("Unknown register {}", register),
            };

            assert_eq!(
                reg_value, expected,
                "Instruction: {:#X}, Expected register {} to have value of: {:#>04X} but got {:#>04X}",
           instruction,     register, expected, reg_value
            )
        }

        fn check_iar(&self, exp_value: u16) {
            assert_eq!(
                self.iar.value(),
                exp_value,
                "Expected IAR to have value of: {:#X} but got {:#X}",
                exp_value,
                self.iar.value()
            )
        }

        fn check_ir(&self, exp_value: u16) {
            assert_eq!(
                self.ir.value(),
                exp_value,
                "Expected IR to have value of: {:#X} but got {:#X}",
                exp_value,
                self.ir.value()
            )
        }

        fn check_flags_register(
            &mut self,
            expected_carry: bool,
            expected_is_larger: bool,
            expected_is_equal: bool,
            expected_is_zero: bool,
        ) {
            let flags_bus = self.flags_bus.lock().unwrap();
            assert_eq!(
                flags_bus.get_output_wire(0),
                expected_carry,
                "Expected carry flag to be: {} but got {}",
                expected_carry,
                flags_bus.get_output_wire(0)
            );
            assert_eq!(
                flags_bus.get_output_wire(1),
                expected_is_larger,
                "Expected is_larger flag to be: {} but got {}",
                expected_is_larger,
                flags_bus.get_output_wire(1)
            );
            assert_eq!(
                flags_bus.get_output_wire(2),
                expected_is_equal,
                "Expected is_equal flag to be: {} but got {}",
                expected_is_equal,
                flags_bus.get_output_wire(2)
            );
            assert_eq!(
                flags_bus.get_output_wire(3),
                expected_is_zero,
                "Expected is_zero flag to be: {} but got {}",
                expected_is_zero,
                flags_bus.get_output_wire(3)
            )
        }
    }
}
