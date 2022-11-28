
use rum::instructions;
use rum::rumload;
use std::env;
use rum::structures;
fn main() {
   let input = env::args().nth(1);
   let insts = rumload::load(input.as_deref());
   //create a new instance of machine and initialize it
   let mut segments:Vec<Vec<u32>> = Vec::new();
   segments.push(insts.clone());
   let mut um = structures::Machine{
       registers : [0_u32;8],
       segment: segments,
       pc : 0,
       ids : Vec::new(),
       seg_id : 1
   };
   let mut count_inst = 0;
   loop{
       //take out each instruction and parse it, on every iteration increment the program counter
       let inst = um.segment[0][um.pc as usize];
           instructions::parse_opcode(&mut um,inst);
       }
       /*
    Invariants:
    We have to make sure that the first segment always has to be 0
    because it needs to run the program while the other segments have to be unmapped.
    If the first segment is not zero the program will fail to run.
    Hence when declaring the vector for segments we have initialized it with 0 to run the program.

    Each segment in the memory has to be reachable by the program counter, because if it is not reachable then the program will not execute.
    Also, this program counter needs to only execute the instructions which are present and should not go out of bounds. If the counter goes out of bounds, the program will fail.
    Hence we have our program counter be a 32 bit number so that it is large enough to handle all the possible address spaces.

    The program counter should always point towards a valid instruction, and if it points to something that is not valid, the machine should fail.
    We can handle it by using the match arm which checks if the instruction is valid or not.
    We have this match arm in our instructions module

    We can not be executing the next instruction in the instruction vector,
    because load program requires us to look at a different instruction rather than next
    We need to make sure that we are always executing the instruction that is pointed by the program counter.
    We do this by running an infinte for loop that terminates when program executes halt instruction.
    In this loop we parse the instruction at program counter and then update the counter.

    We also have to make sure that we are saving IDs when unmapping segments, so we don't run out of address space.
    So we have created a pool of unused IDs which indicate that the segment at that ID is unmapped.
    Everytime we map a segment we pop that ID from that ID pool.
    */
    }