use std::alloc::{self, Layout};
use crate::intermediate_representation::*;

#[derive(Debug)]
pub struct VirtualMemory {
    base: *mut u8,
    size: usize
}

fn create_virtual_memory(size: usize) -> VirtualMemory {
    unsafe {
        VirtualMemory {
            base: alloc::alloc(Layout::array::<u64>(size).unwrap()),
            size
        }
    }
}

fn set_stack_memory(memory: &mut VirtualMemory, memory_offset: i64, value: i64) {
    unsafe {
        *get_stack_memory_offset_address(memory, memory_offset) = value;
    }
}

fn set_float_stack_memory(memory: &mut VirtualMemory, memory_offset: i64, value:f64) {
    unsafe {
        *get_float_stack_memory_offset_address(memory, memory_offset) = value;
    }
}

fn load_stack_memory(memory: &mut VirtualMemory, memory_offset: i64) -> i64 {
    unsafe {
        *get_stack_memory_offset_address(memory, memory_offset)
    }
}

fn load_float_stack_memory(memory: &mut VirtualMemory, memory_offset: i64) -> f64 {
    unsafe {
        *get_float_stack_memory_offset_address(memory, memory_offset)
    }
}

fn get_stack_memory_offset_address(memory: &mut VirtualMemory, memory_offset: i64) -> *mut i64 {
    unsafe {
        let address = memory.base.add(memory_offset as usize) as *mut i64;
        address
    }
}

fn get_float_stack_memory_offset_address(memory: &mut VirtualMemory, memory_offset: i64) -> *mut f64 {
    unsafe {
        let address = memory.base.add(memory_offset as usize) as *mut f64;
        address
    }
}

fn set_memory(address: i64, value: i64) {
    unsafe {
        *(address as *mut i64) = value
    }
}

fn set_float_memory(address: i64, value:f64) {
    unsafe {
        *(address as *mut f64) = value
    }
}

fn load_memory(address: i64) -> i64 {
    unsafe {
        *(address as *mut i64)
    }
}

fn load_float_memory(address: i64) -> f64 {
    unsafe {
        *(address as *mut f64)
    }
}

#[derive(Debug)]
pub struct VirtualMachine {
    instruction_stream: ByteCodeInstructionStream,
    stack_memory: VirtualMemory,
    
    instruction_pointer: Option<usize>,
    stack_pointer: usize,
    registers: [i64; 8],
    float_registers: [f64; 8],

    instruction: ByteCodeInstruction,

    equal_to_zero: bool,
    lower_then_zero: bool,
    greater_than_zero: bool
}

fn create_virtual_machine(instruction_stream: ByteCodeInstructionStream) -> VirtualMachine {
    let stack_memory = create_virtual_memory(100000000);
    let stack_memory_size = stack_memory.size;
    VirtualMachine { 
        instruction_stream,
        stack_memory,
        instruction_pointer: None,
        stack_pointer: stack_memory_size - 1,
        registers: [0; 8],
        float_registers: [0.0; 8],
        instruction: ByteCodeInstruction::None,
        equal_to_zero: false,
        lower_then_zero: false,
        greater_than_zero: false
    }
}

fn run_virtual_machine(cpu: &mut VirtualMachine) {
    while cpu.instruction != ByteCodeInstruction::HLT {
        fetch(cpu);
        execute(cpu);
    }
    dbg!(cpu);
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
        None => 0,
    }
}

fn get_register_at(cpu: &VirtualMachine, pointer: usize) -> usize {
    match cpu.instruction_stream[pointer] {
        ByteCodeInstruction::R(register) => return register,
        _ => panic!("requested register, but not register"),
    }
}

fn get_register_at_instruction_pointer_offset(cpu: &VirtualMachine, offset: usize) -> usize {
    get_register_at(cpu, get_instruction_pointer_offset(cpu, offset))
}

fn get_float_register_at(cpu: &VirtualMachine, pointer: usize) -> usize {
    match cpu.instruction_stream[pointer] {
        ByteCodeInstruction::F(register) => return register,
        _ => panic!("requested float register, but not float register"),
    }
}

fn get_float_register_at_instruction_pointer_offset(cpu: &VirtualMachine, offset: usize) -> usize {
    get_float_register_at(cpu, get_instruction_pointer_offset(cpu, offset))
}

fn get_value_at(cpu: &VirtualMachine, pointer: usize) -> i64 {
    match cpu.instruction_stream[pointer] {
        ByteCodeInstruction::RVAL(value) => return value,
        _ => panic!("requested value, but not value"),
    }
}

fn get_value_at_instruction_pointer_offset(cpu: &VirtualMachine, offset: usize) -> i64 {
    get_value_at(cpu, get_instruction_pointer_offset(cpu, offset))
}

fn get_float_value_at(cpu: &VirtualMachine, pointer: usize) -> f64 {
    match cpu.instruction_stream[pointer] {
        ByteCodeInstruction::FVAL(value) => return value,
        _ => panic!("requested value, but not value"),
    }
}

fn get_float_value_at_instruction_pointer_offset(cpu: &VirtualMachine, offset: usize) -> f64 {
    get_float_value_at(cpu, get_instruction_pointer_offset(cpu, offset))
}

fn execute(cpu: &mut VirtualMachine) {
    match cpu.instruction {
        ByteCodeInstruction::CLF => clear_flags(cpu),
        ByteCodeInstruction::MOV => mov(cpu),
        ByteCodeInstruction::MOVF => movf(cpu),
        ByteCodeInstruction::STI => sti(cpu),
        ByteCodeInstruction::STF => stf(cpu),
        ByteCodeInstruction::LDI => ldi(cpu),
        ByteCodeInstruction::LDF => ldf(cpu),
        ByteCodeInstruction::LII => lii(cpu),
        ByteCodeInstruction::LIF => lif(cpu),
        ByteCodeInstruction::PSH => psh(cpu),
        ByteCodeInstruction::POP => pop(cpu),
        ByteCodeInstruction::PSHF => pshf(cpu),
        ByteCodeInstruction::POPF => popf(cpu),
        ByteCodeInstruction::ADD => add(cpu),
        ByteCodeInstruction::SUB => sub(cpu),
        ByteCodeInstruction::MUL => mul(cpu),
        ByteCodeInstruction::DIV => div(cpu),
        ByteCodeInstruction::FADD => fadd(cpu),
        ByteCodeInstruction::FSUB => fsub(cpu),
        ByteCodeInstruction::FMUL => fmul(cpu),
        ByteCodeInstruction::FDIV => fdiv(cpu),
        ByteCodeInstruction::JNZ => jnz(cpu),
        ByteCodeInstruction::JEZ => jez(cpu),
        ByteCodeInstruction::JGZ => jgz(cpu),
        ByteCodeInstruction::JLZ => jlz(cpu),
        ByteCodeInstruction::JMP => jmp(cpu),
        ByteCodeInstruction::SHL => shl(cpu),
        ByteCodeInstruction::SHR => shr(cpu),
        ByteCodeInstruction::BAND => band(cpu),
        ByteCodeInstruction::BOR => bor(cpu),
        ByteCodeInstruction::BNOT => bnot(cpu),
        ByteCodeInstruction::BXOR => bxor(cpu),
        ByteCodeInstruction::LAND => land(cpu),
        ByteCodeInstruction::LOR => lor(cpu),
        ByteCodeInstruction::LNOT => lnot(cpu),
        _ => {} 
    }

}

fn mov(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] = cpu.registers[register_2];
    println!("MOV register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn movf(cpu: &mut VirtualMachine) {
    let register_1 = get_float_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_float_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.float_registers[register_1] = cpu.float_registers[register_2];
    println!("MOVF register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn sti(cpu: &mut VirtualMachine) {
    let address = get_value_at_instruction_pointer_offset(cpu, 1);
    let register = get_register_at_instruction_pointer_offset(cpu, 2);
    
    set_memory(address, cpu.registers[register]);
    println!("STI address: {}, register: {}", address, register);

    increment_instruction_pointer(cpu, 2);
}

fn stf(cpu: &mut VirtualMachine) {
    let address = get_value_at_instruction_pointer_offset(cpu, 1);
    let register = get_float_register_at_instruction_pointer_offset(cpu, 2);
    
    set_float_memory(address, cpu.float_registers[register]);
    println!("STF address: {}, register: {}", address, register);

    increment_instruction_pointer(cpu, 2);
}

fn ldi(cpu: &mut VirtualMachine) {
    let register = get_register_at_instruction_pointer_offset(cpu, 1);
    let address = get_value_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register] = load_memory(address);
    println!("LDI register: {}, address: {}", register, address);

    increment_instruction_pointer(cpu, 2);
}

fn ldf(cpu: &mut VirtualMachine) {
    let register = get_float_register_at_instruction_pointer_offset(cpu, 1);
    let address = get_value_at_instruction_pointer_offset(cpu, 2);
    
    cpu.float_registers[register] = load_float_memory(address);
    println!("LDF register: {}, address: {}", register, address);

    increment_instruction_pointer(cpu, 2);
}

fn lii(cpu: &mut VirtualMachine) {
    let register = get_register_at_instruction_pointer_offset(cpu, 1);
    let literal_value = get_value_at_instruction_pointer_offset(cpu, 2);

    cpu.registers[register] = literal_value;
    println!("LII register: {}, literal: {}", register, literal_value);
    
    increment_instruction_pointer(cpu, 2);
}

fn lif(cpu: &mut VirtualMachine) {
    let register = get_float_register_at_instruction_pointer_offset(cpu, 1);
    let literal_value = get_float_value_at_instruction_pointer_offset(cpu, 2);

    cpu.float_registers[register] = literal_value;
    println!("LIF register: {}, literal: {}", register, literal_value);
    
    increment_instruction_pointer(cpu, 2);
}

fn psh(cpu: &mut VirtualMachine) {
    let register = get_register_at_instruction_pointer_offset(cpu, 1);
    cpu.stack_pointer -= 8;

    set_stack_memory(&mut cpu.stack_memory, cpu.stack_pointer as i64, cpu.registers[register]);
    println!("PSH register: {}", register);
    
    increment_instruction_pointer(cpu, 1);
}

fn pshf(cpu: &mut VirtualMachine) {
    let register = get_float_register_at_instruction_pointer_offset(cpu, 1);
    
    cpu.stack_pointer -= 8;

    set_float_stack_memory(&mut cpu.stack_memory, cpu.stack_pointer as i64, cpu.float_registers[register]);
    println!("PSHF register: {}", register);
    
    increment_instruction_pointer(cpu, 1);
}

fn pop(cpu: &mut VirtualMachine) {
    let register = get_register_at_instruction_pointer_offset(cpu, 1);

    cpu.registers[register] = load_stack_memory(&mut cpu.stack_memory, cpu.stack_pointer as i64);
    println!("POP register: {}", register);

    cpu.stack_pointer += 8;

    increment_instruction_pointer(cpu, 1);
}

fn popf(cpu: &mut VirtualMachine) {
    let register = get_float_register_at_instruction_pointer_offset(cpu, 1);

    cpu.float_registers[register] = load_float_stack_memory(&mut cpu.stack_memory, cpu.stack_pointer as i64);
    println!("POPF register: {}", register);

    cpu.stack_pointer += 8;

    increment_instruction_pointer(cpu, 1);
}

fn add(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] -= cpu.registers[register_2];
    set_flags(cpu, cpu.registers[register_1]);
    println!("ADD register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn sub(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] -= cpu.registers[register_2];
    set_flags(cpu, cpu.registers[register_1]);
    println!("SUB register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn mul(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] *= cpu.registers[register_2];
    set_flags(cpu, cpu.registers[register_1]);
    println!("MUL register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn div(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] /= cpu.registers[register_2];
    set_flags(cpu, cpu.registers[register_1]);
    println!("DIV register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn fadd(cpu: &mut VirtualMachine) {
    let register_1 = get_float_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_float_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.float_registers[register_1] -= cpu.float_registers[register_2];
    set_float_flags(cpu, cpu.float_registers[register_1]);
    println!("FADD register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn fsub(cpu: &mut VirtualMachine) {
    let register_1 = get_float_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_float_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.float_registers[register_1] -= cpu.float_registers[register_2];
    set_float_flags(cpu, cpu.float_registers[register_1]);
    println!("FSUB register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn fmul(cpu: &mut VirtualMachine) {
    let register_1 = get_float_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_float_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.float_registers[register_1] *= cpu.float_registers[register_2];
    set_float_flags(cpu, cpu.float_registers[register_1]);
    println!("FMUL register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn fdiv(cpu: &mut VirtualMachine) {
    let register_1 = get_float_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_float_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.float_registers[register_1] /= cpu.float_registers[register_2];
    set_float_flags(cpu, cpu.float_registers[register_1]);
    println!("FDIV register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn jnz(cpu: &mut VirtualMachine) {
    let jump_to = get_value_at_instruction_pointer_offset(cpu, 1);

    if !cpu.equal_to_zero {
        cpu.instruction_pointer = Some(jump_to as usize);
    } else {
        increment_instruction_pointer(cpu, 1);
    }
    println!("JNZ jump to: {}", jump_to);
}

fn jez(cpu: &mut VirtualMachine) {
    let jump_to = get_value_at_instruction_pointer_offset(cpu, 1);

    if cpu.equal_to_zero {
        cpu.instruction_pointer = Some(jump_to as usize);
    } else {
        increment_instruction_pointer(cpu, 1);
    }
    println!("JEZ jump to: {}", jump_to);
}

fn jgz(cpu: &mut VirtualMachine) {
    let jump_to = get_value_at_instruction_pointer_offset(cpu, 1);

    if cpu.greater_than_zero {
        cpu.instruction_pointer = Some(jump_to as usize);
    } else {
        increment_instruction_pointer(cpu, 1);
    }
    println!("JGZ jump to: {}", jump_to);
}

fn jlz(cpu: &mut VirtualMachine) {
    let jump_to = get_value_at_instruction_pointer_offset(cpu, 1);

    if cpu.lower_then_zero {
        cpu.instruction_pointer = Some(jump_to as usize);
    } else {
        increment_instruction_pointer(cpu, 1);
    }    
    println!("JLZ jump to: {}", jump_to);
}

fn jmp(cpu: &mut VirtualMachine) {
    let jump_to = get_value_at_instruction_pointer_offset(cpu, 1);

    cpu.instruction_pointer = Some(jump_to as usize);

    println!("JMP jump to: {}", jump_to);
}

fn shl(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] <<= cpu.registers[register_2];
    set_flags(cpu, cpu.registers[register_1]);
    println!("SHL register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn shr(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] >>= cpu.registers[register_2];
    set_flags(cpu, cpu.registers[register_1]);
    println!("SHR register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn band(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] &= cpu.registers[register_2];
    set_flags(cpu, cpu.registers[register_1]);
    println!("BAND register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn bnot(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] = !cpu.registers[register_2];
    set_flags(cpu, cpu.registers[register_1]);
    println!("BNOT register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn bor(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] |= cpu.registers[register_2];
    set_flags(cpu, cpu.registers[register_1]);
    println!("BOR register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn bxor(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] ^= cpu.registers[register_2];
    set_flags(cpu, cpu.registers[register_1]);
    println!("BXOR register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn land(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] = (cpu.registers[register_1] != 0 && cpu.registers[register_2] != 0) as i64;
    set_flags(cpu, cpu.registers[register_1]);
    println!("LAND register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn lor(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] = (cpu.registers[register_1] != 0 || cpu.registers[register_2] != 0) as i64;
    set_flags(cpu, cpu.registers[register_1]);
    println!("LOR register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn lnot(cpu: &mut VirtualMachine) {
    let register_1 = get_register_at_instruction_pointer_offset(cpu, 1);
    let register_2 = get_register_at_instruction_pointer_offset(cpu, 2);
    
    cpu.registers[register_1] = !(cpu.registers[register_2] != 0) as i64;
    set_flags(cpu, cpu.registers[register_1]);
    println!("LNOT register: {}, register: {}", register_1, register_2);
     
    increment_instruction_pointer(cpu, 2);
}

fn clear_flags(cpu: &mut VirtualMachine) {
    cpu.equal_to_zero = false;
    cpu.lower_then_zero = false;
    cpu.greater_than_zero = false;
}

fn set_flags(cpu: &mut VirtualMachine, dest: i64) {
    cpu.equal_to_zero = dest == 0;
    cpu.lower_then_zero = dest < 0;
    cpu.greater_than_zero = dest > 0;
}

fn set_float_flags(cpu: &mut VirtualMachine, dest: f64) {
    cpu.equal_to_zero = dest == 0.0;
    cpu.lower_then_zero = dest < 0.0;
    cpu.greater_than_zero = dest > 0.0;
}
