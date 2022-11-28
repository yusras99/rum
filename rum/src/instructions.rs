use crate::seg_operations;
use crate::structures::{Field, Opcode, Machine};
/*We are going to parse the instruction, figure out the opcodes,
values and registers and then we pass this stuff in our functions in segOperations */
type Umi = u32;
static RA: Field = Field { width: 3, lsb: 6 };
static RB: Field = Field { width: 3, lsb: 3 };
static RC: Field = Field { width: 3, lsb: 0 };
static RL: Field = Field { width: 3, lsb: 25 };
static VL: Field = Field { width: 25, lsb: 0 };
static OP: Field = Field { width: 4, lsb: 28 };
 
fn mask(bits: u32) -> u32 {
   (1 << bits) - 1
}
//fn to extract the field from the instruction and return a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
   (instruction >> field.lsb) & mask(field.width)
}
//function to get the opcode and return that opcode
pub fn op(instruction: Umi) -> u32 {
   (instruction >> OP.lsb) & mask(OP.width)
}

//Find the opcode in the instruction, using get functions to get the opcodes and registers and call the respective functions in segOperations
pub fn parse_opcode( um : &mut Machine,inst: Umi) {
    let reg_a  = get(&RA, inst);
    let reg_b = get(&RB, inst);
    let reg_c = get(&RC, inst);
    match get(&OP, inst) {
       o if o == Opcode::CMov as u32 => {
           seg_operations::c_mov(um,reg_a, reg_b, reg_c);
       }
       o if o == Opcode::Load as u32 => {
           seg_operations::load(um,reg_a, reg_b, reg_c);
       }
       o if o == Opcode::Store as u32 => {
           seg_operations::store(um,reg_a, reg_b, reg_c);
       }
       o if o == Opcode::Add as u32 => {
           seg_operations::add(um,reg_a, reg_b, reg_c);
       }
       o if o == Opcode::Mul as u32 => {
           seg_operations::mul(um,reg_a, reg_b, reg_c);
       }
       o if o == Opcode::Div as u32 => {
           seg_operations::div(um,reg_a, reg_b, reg_c);
       }
       o if o == Opcode::Nand as u32 => {
           seg_operations::nand(um,reg_a, reg_b, reg_c);
       }
       o if o == Opcode::Halt as u32 => {
           seg_operations::halt( um);
       }
       o if o == Opcode::MapSegment as u32 => {
           seg_operations::map_seg(um,reg_b, reg_c);
       }
       o if o == Opcode::UnmapSegment as u32 => {
           seg_operations::unmap_seg( um,reg_c);
       }
       o if o == Opcode::Output as u32 => {
           seg_operations::output( um,reg_c);
       }
       o if o == Opcode::Input as u32 => {
           seg_operations::input( um,reg_c);
       }
       o if o == Opcode::LoadProgram as u32 => {
           seg_operations::load_prog( um,reg_b, reg_c);
       }
       o if o == Opcode::LoadValue as u32 => {
           seg_operations::load_value( um,get(&RL, inst), get(&VL, inst));
       }
       _ => {}
   }

}