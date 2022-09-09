use std::alloc::{self, Layout};

#[test]
fn vm() {
    let mem = create_virtual_memory(100000000);
    let mut cpu = create_virtual_machine(vec!(), mem);
    run_virtual_machine(&mut cpu)
}

#[derive(Copy, Clone, PartialEq)]
pub enum Instruction {
    None,
    R(usize),
    RVAL(f64),
    F(usize),
    FVAL(f64),
    CLF,
    MOV, MOVF,
    STI, STF, LDI, LDF,
    LII, LIF,
    PSH, POP,
    PSHF, POPF,
    ADD, SUB, MUL, DIV,
    FADD, FSUB, FMUL, FDIV,
    JNZ, JEZ, JGZ, JLZ,
    SHL, SHR,
    BAND, BOR, BNOT, BXOR,
    LAND, LOR, LNOT,
    HLT
}

pub type InstructionMemory = Vec<Instruction>;

pub struct VirtualMemory {
    base: *mut u8,
    size: usize,
    layout: Layout
}

fn create_virtual_memory(size: usize) -> VirtualMemory {
    unsafe {
        let layout = Layout::array::<u64>(size).unwrap();
        
        VirtualMemory {
            base: alloc::alloc(layout),
            size,
            layout
        }
    }
}

pub struct VirtualMachine {
    instruction_stream: InstructionMemory,
    
    instruction_pointer: Option<usize>,
    stack_pointer: usize,
    registers: [i64; 8],
    float_registers: [f64; 8],

    instruction: Instruction,
    dest: Instruction,
    source: Instruction,

    zero: bool,
    lower_then_zero: bool,
    greater_than_zero: bool
}

fn create_virtual_machine(instruction_stream: InstructionMemory, memory: VirtualMemory) -> VirtualMachine {
    VirtualMachine { 
        instruction_stream,
        instruction_pointer: None,
        stack_pointer: memory.size - 1,
        registers: [0; 8],
        float_registers: [0.0; 8],
        instruction: Instruction::None,
        dest: Instruction::None,
        source: Instruction::None,
        zero: false,
        lower_then_zero: false,
        greater_than_zero: false
    }
}

fn run_virtual_machine(cpu: &mut VirtualMachine) {
    while cpu.instruction != Instruction::HLT {
        fetch(cpu);
        execute(cpu);
    }
}


fn fetch(cpu: &mut VirtualMachine) {
    increment_instruction_pointer(cpu, 1);
    set_instruction_at_pointer(cpu);
}

fn set_instruction_at_pointer(cpu: &mut VirtualMachine) {
    if let Some(pointer) = cpu.instruction_pointer {
        cpu.instruction = cpu.instruction_stream[pointer];
    }
}

fn increment_instruction_pointer(cpu: &mut VirtualMachine, offset: usize) {
    cpu.instruction_pointer = Some(get_instruction_pointer_offset(cpu, offset));
}


fn get_instruction_pointer_offset(cpu: &VirtualMachine, offset: usize) -> usize {
    match cpu.instruction_pointer {
        Some(pointer) => pointer + offset,
        None => offset,
    }
}

fn execute(cpu: &mut VirtualMachine) {
    match cpu.instruction {
        Instruction::CLF => clear_flags(cpu),
        Instruction::MOV => mov(cpu),
        Instruction::MOVF => movf(cpu),
        Instruction::STI => sti(cpu),
        Instruction::STF => stf(cpu),
        Instruction::LDI => ldi(cpu),
        Instruction::LDF => ldf(cpu),
        Instruction::LII => lii(cpu),
        Instruction::LIF => lif(cpu),
        Instruction::PSH => psh(cpu),
        Instruction::POP => pop(cpu),
        Instruction::PSHF => pshf(cpu),
        Instruction::POPF => popf(cpu),
        Instruction::ADD => add(cpu),
        Instruction::SUB => sub(cpu),
        Instruction::MUL => mul(cpu),
        Instruction::DIV => div(cpu),
        Instruction::FADD => fadd(cpu),
        Instruction::FSUB => fsub(cpu),
        Instruction::FMUL => fmul(cpu),
        Instruction::FDIV => fdiv(cpu),
        Instruction::JNZ => jnz(cpu),
        Instruction::JEZ => jez(cpu),
        Instruction::JGZ => jgz(cpu),
        Instruction::JLZ => jlz(cpu),
        Instruction::SHL => shl(cpu),
        Instruction::SHR => shr(cpu),
        Instruction::BAND => band(cpu),
        Instruction::BOR => bor(cpu),
        Instruction::BNOT => bnot(cpu),
        Instruction::BXOR => bxor(cpu),
        Instruction::LAND => land(cpu),
        Instruction::LOR => lor(cpu),
        Instruction::LNOT => lnot(cpu),
        _ => {} 
    }

}

fn mov(cpu: &mut VirtualMachine) {
    cpu.registers[get_instruction_pointer_offset(cpu, 1)] =
        cpu.registers[get_instruction_pointer_offset(cpu, 2)];
    
    increment_instruction_pointer(cpu, 2);
}

fn movf(cpu: &mut VirtualMachine) {
    cpu.float_registers[get_instruction_pointer_offset(cpu, 1)] = 
        cpu.float_registers[get_instruction_pointer_offset(cpu, 2)];
    
    increment_instruction_pointer(cpu, 2);
}

fn sti(cpu: &mut VirtualMachine) {
    
}

fn stf(cpu: &mut VirtualMachine) {
    todo!()
}

fn ldi(cpu: &mut VirtualMachine) {
    todo!()
}

fn ldf(cpu: &mut VirtualMachine) {
    todo!()
}

fn lii(cpu: &mut VirtualMachine) {
    todo!()
}

fn lif(cpu: &mut VirtualMachine) {
    todo!()
}

fn pop(cpu: &mut VirtualMachine) {
    todo!()
}

fn psh(cpu: &mut VirtualMachine) {
    todo!()
}

fn sub(cpu: &mut VirtualMachine) {
    todo!()
}

fn pshf(cpu: &mut VirtualMachine) {
    todo!()
}

fn popf(cpu: &mut VirtualMachine) {
    todo!()
}

fn add(cpu: &mut VirtualMachine) {
    todo!()
}

fn mul(cpu: &mut VirtualMachine) {
    todo!()
}

fn div(cpu: &mut VirtualMachine) {
    todo!()
}

fn fadd(cpu: &mut VirtualMachine) {
    todo!()
}

fn fsub(cpu: &mut VirtualMachine) {
    todo!()
}

fn fmul(cpu: &mut VirtualMachine) {
    todo!()
}

fn fdiv(cpu: &mut VirtualMachine) {
    todo!()
}

fn jnz(cpu: &mut VirtualMachine) {
    todo!()
}

fn jez(cpu: &mut VirtualMachine) {
    todo!()
}

fn jgz(cpu: &mut VirtualMachine) {
    todo!()
}

fn jlz(cpu: &mut VirtualMachine) {
    todo!()
}

fn shl(cpu: &mut VirtualMachine) {
    todo!()
}

fn shr(cpu: &mut VirtualMachine) {
    todo!()
}

fn band(cpu: &mut VirtualMachine) {
    todo!()
}

fn bnot(cpu: &mut VirtualMachine) {
    todo!()
}

fn bor(cpu: &mut VirtualMachine) {
    todo!()
}

fn bxor(cpu: &mut VirtualMachine) {
    todo!()
}

fn land(cpu: &mut VirtualMachine) {
    todo!()
}

fn lor(cpu: &mut VirtualMachine) {
    todo!()
}

fn lnot(cpu: &mut VirtualMachine) {
    todo!()
}

fn clear_flags(cpu: &mut VirtualMachine) {
    cpu.zero = false;
    cpu.lower_then_zero = false;
    cpu.greater_than_zero = false;
}

fn set_flags(cpu: &mut VirtualMachine, dest: i64) {
    cpu.zero = dest == 0;
    cpu.lower_then_zero = dest < 0;
    cpu.greater_than_zero = dest > 0;
}

fn f_set_flags(cpu: &mut VirtualMachine, dest: f64) {
    cpu.zero = dest == 0.0;
    cpu.lower_then_zero = dest < 0.0;
    cpu.greater_than_zero = dest > 0.0;
}