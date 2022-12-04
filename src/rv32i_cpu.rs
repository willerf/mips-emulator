
use log::{debug, info, error};

use crate::rv32i_state::RV32IState;
use crate::rv32i_state::{PC};

macro_rules! FMT_ADDR { () => { "{:04X}\t{:<20} | {}" }; }

macro_rules! bits { (val: u32, range: (u32, u32)) => { val >> range.1 }; }

/*pub fn decode_imm(instr: u32, pattern: Vec<(u32, u32)>) -> u32 {

    let mut result: u32 = ((instr as i32) >> 31) as u32;
    for i in 0..pattern.len() {
        let range: (u32, u32) = pattern[i];
        let bit_len = range.0 - range.1;
        result = result << bit_len;
        result = result | ((instr >> range.1) & ((0b1 << (bit_len + 1)) - 1));
    }

    return result;
}*/

pub fn step(state: &mut RV32IState) {
    let pc: u32 = state.read_reg(PC);           // update uses of PC to use pc
    let instr: u32 = state.read_mem(pc);
    
    state.write_reg(PC, pc+4);

    let opcode: u32 = instr & 0b1111111;
    let rd_addr: u32 = (instr >> 7) & 0b11111;
    let rs1_addr: u32 = (instr >> 15) & 0b11111;
    let rs2_addr: u32 = (instr >> 20) & 0b11111;
    
    // let rd: u32 = state.read_reg(rd_addr);
    let rs1: u32 = state.read_reg(rs1_addr);
    let rs2: u32 = state.read_reg(rs2_addr);
    
    let func: u32 = (instr >> 12) & 0b11111111111;
    let imm_i: u32 = decode_imm(instr, vec![(32, 20)]);
    let imm_s: u32 = decode_imm(instr, vec![(32, 25), (12, 7)]);
    let imm_b: u32 = decode_imm(instr, vec![(32, 31), (8, 7), (31, 25), (12, 8)]);
    let imm_u: u32 = decode_imm(instr, vec![(32, 12)]) << 12;
    let imm_j: u32 = decode_imm(instr, vec![(32, 31), (20, 12), (21, 20), (31, 21)]) << 1;

    println!("\nimm_i: {:032b}", imm_i);
    println!("imm_s: {:032b}", imm_s);
    println!("imm_b: {:032b}", imm_b);
    println!("imm_u: {:032b}", imm_u);
    println!("imm_j: {:032b}", imm_j);
}

pub fn run(state: &mut RV32IState) {
    let termination_pc: u32 = 0b11111110111000011101111010101101;

    state.write_reg(31, termination_pc);
    while state.read_reg(PC) != termination_pc {
        step(state);
    }
}