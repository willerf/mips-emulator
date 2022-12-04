
use crate::memory::Memory;

pub mod memory;


/*
use crate::rv32i_state::RV32IState;

pub mod rv32i_state;
pub mod rv32i_cpu;

fn main() {
    let mem: Memory = Memory::new(128);
    let mut state: RV32IState = RV32IState::new(mem);

    state.write_mem(0, 0b10000000000000000000111110000000);
    rv32i_cpu::step(&mut state);
    
    state.write_mem(4, 0b01111111111100000000000000000000);
    rv32i_cpu::step(&mut state);

    state.write_mem(8, 0b01111110000000000000111110000000);
    rv32i_cpu::step(&mut state);

}
*/



use crate::mips_state::MIPSState;

pub mod mips_state;
pub mod mips_cpu;

fn main() {
    env_logger::init();
    mips_example();
    
}

fn mips_example() {
    let mut mem: Memory = Memory::from_binary("data/mips_output.bin");
    let heap_addr = mem.size();
    //println!("Heap Address Start: {}", heap_addr);
    mem.resize(16777216);

    let mut state: MIPSState = MIPSState::new(mem);

    let input1 = 5;
    let input2 = 3;
    state.write_reg(1, input1 as u32);
    state.write_reg(2, input2 as u32);
    state.write_reg(30, 16777216);

    mips_cpu::run(&mut state);
    println!("\n{}", state.read_reg(3) as i32);
}

fn mips_arr_example() {
    let mut mem: Memory = Memory::from_binary("data/mips_output.bin");
    
    //let arr_vec: Vec<i32> = vec![1, 2, 3, 4];
    let arr_vec: Vec<i32> = vec![77,3,6,22,-1,-1,-8,9,12,-36,-1,-1,999,-1,-1];

    let arr_addr: u32 = mem.size();

    mem.resize(2048);
    let mut state: MIPSState = MIPSState::new(mem);

    for i in 0..arr_vec.len() {
        state.write_mem((arr_addr + (i as u32)*4) as u32, arr_vec[i as usize] as u32);
        println!("{}", state.read_mem((arr_addr + (i as u32)*4) as u32) as i32);
    }

    // state.print_words_binary();

    let input1 = arr_addr;
    let input2 = arr_vec.len();
    state.write_reg(1, input1 as u32);
    state.write_reg(2, input2 as u32);
    state.write_reg(30, 2048);

    mips_cpu::run(&mut state);
    println!("\n{}", state.read_reg(3) as i32);
}