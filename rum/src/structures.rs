
use std::collections::HashMap;
 
#[derive(Debug, Clone)]
pub enum Opcode {
   CMov,
   Load,
   Store,
   Add,
   Mul,
   Div,
   Nand,
   Halt,
   MapSegment,
   UnmapSegment,
   Output,
   Input,
   LoadProgram,
   LoadValue,
}
#[derive(Debug, Clone)]
pub struct Field {
   pub width: u32,
   pub lsb: u32,
}
#[derive(Debug, Clone)]
pub struct Machine{
   pub registers : [u32;8],
   //the key is the identifier about what we have looked at last, vec is the instruction set
   pub segment : Vec<Vec<u32>>,
   pub pc : u32,
   pub ids : Vec<u32>,
   pub seg_id : u32,
}
impl Machine {
   pub fn new(registers:[u32;8],segment:Vec<Vec<u32>>,pc : u32,ids:Vec<u32>, seg_id: u32)->Self{
       Machine{
           registers,segment,pc,ids,seg_id
       }
   }
   }