
use std::{process, io::{Read, stdin}};
use crate::structures::Machine;
use std::char;
 
/*We make functions to do the operations
Every function has to increment program counter to get to the next instruction*/
pub fn c_mov(machine:&mut Machine,reg_a: u32, reg_b: u32, reg_c: u32) {
   if machine.registers[reg_c as usize] != 0 {
       machine.registers[reg_a as usize] = machine.registers[reg_b as usize];
   }
   machine.pc +=1;
}
pub fn load(machine:&mut Machine,reg_a: u32, reg_b: u32, reg_c: u32) {
   //Get the value at segment register b and offset reg_c and store it in reg_a.
   let load_val = machine.segment[machine.registers[reg_b as usize] as usize][machine.registers[reg_c as usize]as usize];
   machine.registers[reg_a as usize] = load_val;
   machine.pc +=1;
}
pub fn store(machine:&mut Machine,reg_a: u32, reg_b: u32, reg_c: u32) {
   //Store val in reg_c which is in segment a offset b.
   machine.segment[machine.registers[reg_a as usize] as usize][machine.registers[reg_b as usize] as usize] = machine.registers[reg_c as usize];
   machine.pc +=1;
}
pub fn add(machine:&mut Machine,reg_a: u32, reg_b: u32, reg_c: u32) {
   //add values in reg_b and reg_c and wrap it in reg_a so that it doesn't overflow.
   machine.registers[reg_a as usize] = machine.registers[reg_b as usize] + machine.registers[reg_c as usize];
   machine.pc +=1;
}
pub fn mul(machine:&mut Machine,reg_a: u32, reg_b: u32, reg_c: u32) {
   //multiply values in reg_b and reg_c and wrap it in reg_a so that it doesn't overflow.
   machine.registers[reg_a as usize] = machine.registers[reg_b as usize] * machine.registers[reg_c as usize];
   machine.pc +=1;
}
pub fn div(machine:&mut Machine,reg_a: u32, reg_b: u32, reg_c: u32) {
   //divide values in reg_b and reg_c and store it in reg_a
   machine.registers[reg_a as usize] = machine.registers[reg_b as usize] / machine.registers[reg_c as usize];
   machine.pc +=1;
}
pub fn nand(machine:&mut Machine,reg_a: u32, reg_b: u32, reg_c: u32) {
   //Nand the values in reg_b and reg_c and store it in reg_a.
   machine.registers[reg_a as usize] = !(machine.registers[reg_b as usize] & machine.registers[reg_c as usize]);
   machine.pc +=1;
}
pub fn halt(machine:&mut Machine){
   //exit the program when halt instruction shows up.
   machine.pc +=1;
   process::exit(0);
}
 
pub fn map_seg(machine:&mut Machine,reg_b: u32, reg_c: u32) {
   //if there is no unused id in the machine.ids pool,
    //then pop one and map a new segment of zeros of size reg_c at the id. Otherwise no need to pop just insert the segment.
   let num_words = machine.registers[reg_c as usize];
   if machine.ids.len()>0{
       machine.ids.pop();
       machine.segment.insert(machine.seg_id as usize,vec![0;num_words.try_into().unwrap()]);
       machine.registers[reg_b as usize] = machine.seg_id;
       machine.seg_id+=1;
   }
   else{
       machine.segment.insert(machine.seg_id as usize,vec![0;num_words.try_into().unwrap()]);
       machine.registers[reg_b as usize] = machine.seg_id;
       machine.seg_id+=1;
   }
   machine.pc +=1;
}
 
pub fn unmap_seg(machine:&mut Machine,reg_c: u32) {
  //in order to unmap the segment free up the segment at reg_c and store its id in the machine.ids pool to reuse it
   machine.segment[machine.registers[reg_c as usize] as usize] = Vec::new();
   machine.ids.push(machine.registers[reg_c as usize]);
   machine.pc +=1;
}
 
pub fn input(machine:&mut Machine,reg_c: u32){
   //Take the value from standard input and store it in reg_c
   match stdin().bytes().next(){
       Some(value) => {
           machine.registers[reg_c as usize] = value.unwrap() as u32
       }
       None => {
           machine.registers[reg_c as usize] = !0 as u32
       }
   }
   machine.pc +=1;
}
 
pub fn output(machine:&mut Machine,reg_c: u32) {
   //whatever is in reg_c output it's ascii val on std out
   print!("{}", char::from_u32(machine.registers[reg_c as usize]).unwrap());
   machine.pc +=1;
}
 
pub fn load_prog(machine:&mut Machine,reg_b: u32, reg_c: u32) {
   //if reg_b val is not 0, then make a duplicate of segment at reg_b, and store it in seg 0 to run it.
   //set the program counter to reg_c, and then increment it to go to the next instruction.
   if machine.registers[reg_b as usize]!=0{
      machine.segment[0]  = machine.segment[machine.registers[reg_b as usize] as usize].clone();
   }
   machine.pc = (machine.registers[reg_c as usize])as u32;
}
pub fn load_value(machine:&mut Machine,reg_rl: u32, val: u32) {
   //load the value in reg_rl which is the reg_a of opcode 13
   machine.registers[reg_rl as usize] = val;
   machine.pc +=1;
}