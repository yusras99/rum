//type Umi = u32;
//use crate::Opcode;

//use std::mem;
use std::io::{stdin, Read, stdout, Write};
use std::char::from_u32;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Opcode{
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

pub struct Field { 
    width: u32,
    lsb: u32, 
}
static RA: Field = Field {width: 3, lsb: 6}; 
static RB: Field = Field {width: 3, lsb: 3}; 
static RC: Field = Field {width: 3, lsb: 0}; 
static RL: Field = Field {width: 3, lsb: 25}; 
static VL: Field = Field {width: 25, lsb: 0}; 
static OP: Field = Field {width: 4, lsb: 28};

#[inline]
fn mask(bits: u32) -> u32 { 
    (1 << bits) - 1 
}

#[inline]
pub fn get(field: &Field, instruction: u32) -> u32 { 
    (instruction >> field.lsb) & mask(field.width)
}


// pub fn op(instruction: u32) -> u32 { 
//     (instruction >> OP.lsb) & mask(OP.width)
// }
#[inline]
pub fn run(instruction: Vec<u32>){
    // 8 elemetns in an array each representing a register 
    let mut register: [u32; 8] = [0,0,0,0,0,0,0,0];
    // Memory is a 2D vector
    // The first element of memeory is a vecotor of all instructions 
    let mut memory: Vec<Vec<u32>> = vec![vec![0]];
    // Id map 
    let mut id_map: Vec<u32> = Vec::new();
    // Prorgam counter
    let mut pcounter: u32 = 0;
    memory[0] = instruction;
    let mut max_id: u32 = 0;
    
    // Loop for opcode instructionset, will continue too loop until Halt is reached or error code 1 
    loop {
        match get(&OP, memory[0][pcounter as usize]) {
            // Opcode = 0
            o if o == Opcode::CMov as u32 => {
                cmov(memory[0][pcounter as usize], &mut register);
                pcounter += 1;
            },
            // Opcode = 1
            o if o == Opcode::Load as u32 => {
                load(memory[0][pcounter as usize], &mut register, &mut memory);
                pcounter += 1;
            },
            // Opcode = 2
            o if o == Opcode::Store as u32 => {
                store(memory[0][pcounter as usize], &mut register, &mut memory);
                pcounter += 1;
            },
            // Opcode = 3
            o if o == Opcode::Add as u32 => {
                add(memory[0][pcounter as usize], &mut register);
                pcounter += 1;
            },
            // Opcode = 4
            o if o == Opcode::Mul as u32 => {
                mul(memory[0][pcounter as usize], &mut register);
                pcounter += 1;
            },
            // Opcode = 5
            o if o == Opcode::Div as u32 => {
                div(memory[0][pcounter as usize], &mut register);
                pcounter += 1;
            },
            // Opcode = 6
            o if o == Opcode::Halt as u32 => {
                std::process::exit(0);
                //pcounter += 1;
                // format!("r{} := Halt();", get(&RA, inst))
            },
            // Opcode = 7
            o if o == Opcode::Nand as u32 => {
                nand(memory[0][pcounter as usize], &mut register);
                pcounter += 1;
            },
            // Opcode = 8
            o if o == Opcode::MapSegment as u32 => {
                map_segment(memory[0][pcounter as usize], &mut register, &mut id_map, &mut memory, &mut max_id);
                pcounter += 1;
                // format!("map r{} := Segment();", get(&RC, inst))
            },
            // Opcode = 9
            o if o == Opcode::UnmapSegment as u32 => {
                unmap_segment(memory[0][pcounter as usize], &mut register, &mut id_map);
                pcounter += 1;
                // format!("unmap r{};", get(&RC, inst))
            },
            // Opcode = 10
            o if o == Opcode::Output as u32 => {
                output(memory[0][pcounter as usize], &mut register);
                pcounter += 1;
            },
            // Opcode = 11
            o if o == Opcode::Input as u32 => {
                input(memory[0][pcounter as usize], &mut register);
                pcounter += 1;
            },
            // Opcode = 12
            o if o == Opcode::LoadProgram as u32 => {
                //println!("Opcode 12 = Load Program");
                load_program(memory[0][pcounter as usize],
                     &mut register, 
                     &mut memory,
                     &mut pcounter
                );
                //pcounter += 1;
                // format!("goto r{} in program m[r{}];", get(&RC, inst), get(&RB, inst))
            },
            // Opcode = 13
            o if o == Opcode::LoadValue as u32 => {
                //println!("Opcode 13 = LoadValue");
                load_value(memory[0][pcounter as usize], &mut register);
                pcounter += 1;
                // format!("r{} := {};", get(&RL, inst), get(&VL, inst))
            },
            _ => {
                //println!("ERR {} OpCode not found", o);
                eprintln!("Invalid OPCODE Input");
                std::process::exit(1);
                //pcounter += 1;
                // format!(".data 0x{:x}", inst)
            }
            //o if o == Opcode::Load as u32
        }
    }  
}


// If opcode == 0
    // Conditional Move
pub fn cmov(inst: u32, registers: &mut [u32; 8]){
    //println!("Opcode = 0 CMov");
    // Get registers a,b,c
    let ra = get(&RA, inst);
    let rb = get(&RB, inst);
    let rc = get(&RC, inst);

    // validate that regiceter rc does not equal 0 
    if registers[rc as usize] != 0 {
        
        // set the value of register a to the value of register b
        registers[ra as usize] = registers[rb as usize];    
    }
}

// If opcode == 1
    // Segmented Load 
pub fn load(inst: u32, registers: &mut [u32; 8], memory: &mut Vec<Vec<u32>>){   
    
    // Get registers a,b,c
    //println!("Opcode = 1 Load");
    let ra = get(&RA, inst);
    let rb = get(&RB, inst);
    let rc = get(&RC, inst);
    
    

    // let reg_b = registers[rb as usize] as usize;
    // let reg_c = registers[rc as usize] as usize;
    // println!("{:?}", reg_b);
    // println!("{:?}", reg_c);
    // println!("{:?}", memory[reg_b][reg_c]);
    
    //registers[ra as usize] = memory[reg_b][reg_c];
    
    // Assign the value at memory location mem[b][c] to register a 
    // $r[A] := $m[$r[B]][$r[C]]
    registers[ra as usize] = memory[registers[rb as usize] as usize][registers[rc as usize] as usize];

}

// If opcode == 2
// Seegmented Store 
pub fn store(inst: u32, registers: &mut [u32; 8], memory: &mut Vec<Vec<u32>>){
    // println!("Opcode = 2 Store");
    // Get registers a,b,c
    let ra = get(&RA, inst);
    let rb = get(&RB, inst);
    let rc = get(&RC, inst);
        
    // println!("{:?}", registers[ra as usize] as usize);
    // println!("{:?}", registers[rb as usize] as usize);
    // println!("{:?}", rc);
    // println!("{:?}", registers[rc as usize] as usize);
    
    // store the value of register c in memory loaction mem[a][b] 
    // $m[$r[A]][$r[B]] := $r[C]
    memory[registers[ra as usize] as usize][registers[rb as usize] as usize] = registers[rc as usize];

}

// if  opcode == 3 
pub fn add(inst: u32, registers: &mut [u32; 8]){
    //println!("Opcode = 3 Add");
    // Get registers a,b,c
    let ra = get(&RA, inst);
    let rb = get(&RB, inst);
    let rc = get(&RC, inst);

    // using wrapping_add handles the modulator operator of the equation below  
    // $r[A] := ($r[B] + $r[C]) mod 2^32
    registers[ra as usize] = registers[rb as usize].wrapping_add(registers[rc as usize]);
}

// if opcode == 4
pub fn mul(inst: u32, registers: &mut [u32; 8]){
    //println!("Opcode = 4 Mul");
    // Get registers a,b,c
    let ra = get(&RA, inst);
    let rb = get(&RB, inst);
    let rc = get(&RC, inst);

    //println!("{:?}", registers[rb as usize]);
    //println!("{:?}", registers[rc as usize]);

    // using wrapping_mul handles the modulator operator of the equation below  
    // $r[A] := ($r[B] × $r[C]) mod 2^32
    registers[ra as usize] = registers[rb as usize].wrapping_mul(registers[rc as usize]);
    
    // todo!()
}

// if opcode == 5 
pub fn div(inst: u32, registers: &mut [u32; 8]){
    // println!("Opcode = 5 Div");
    // Get registers a,b,c
    let ra = get(&RA, inst);
    let rb = get(&RB, inst);
    let rc = get(&RC, inst);

    // $r[A] := ($r[B] ÷ $r[C]) (integer division)
    let temp_val =  registers[rb as usize] / registers[rc as usize];
    registers[ra as usize] = temp_val; 
    
    // todo!()
}

// if opcode == 6 
pub fn nand(inst: u32, registers: &mut [u32; 8]){
    // println!("Opcode = 6 Nand");
    // Get registers a,b,c
    let ra = get(&RA, inst);
    let rb = get(&RB, inst);
    let rc = get(&RC, inst);
    // $r[A] :=¬($r[B]∧$r[C])

    registers[ra as usize] = !(registers[rb as usize] & registers[rc as usize]);

    // todo!()
}

// if opcode == 8 
pub fn map_segment(inst: u32, registers: &mut [u32; 8], id_map: &mut Vec<u32>, memory: &mut Vec<Vec<u32>>, max_id: &mut u32){
    // println!("Opcode 8 = Map");
    // Get rc + rb
    let rb = get(&RB, inst);
    let rc = get(&RC, inst);

    // temp new address / segment
    let mut new_segment = 0 as u32;
    // new segment created with number of words == value in $r[c]
    // Check if id_map is empty
    if id_map.is_empty(){
        // Add one to the max id
        *max_id += 1;
        // Each word in segment is initalized with 0
        
        new_segment = *max_id;
        // Push memory with vector of 0's
        memory.push(vec![0; registers[rc as usize] as usize]);
    }
    else{
        // remove value from id_map
        new_segment = id_map.pop().unwrap();
        
        // assign memory wiht a vector of 0's
        memory[new_segment as usize] = vec![0; registers[rc as usize] as usize];
    }
    // assign register rb with new_segment
    registers[rb as usize] = new_segment;
        
    // A bit pattern that is not all 0's 
        // placed in $r[b]
    
    // new segment mapped as $m[$r[b]]

    // println!("Opcode 8 = map");
    
    //todo!()
}

// if opcode == 9 
    // Unmap segment from memory
pub fn unmap_segment(inst: u32, registers: &mut [u32; 8], id_map: &mut Vec<u32>){
    // println!("Opcode 9 = unmap");
    // get register 
    let rc = get(&RC, inst);

    // Push register value to id map
    id_map.push(registers[rc as usize]);
    
}

// if opcode == 10 
    // returns output to standard output
pub fn output(inst: u32, registers: &mut [u32; 8]){
    //println!("Opcode = 10 Output");
    let rc = get(&RC, inst);
    let val = registers[rc as usize];

    // If val less than 256
    if val < 256{ 
        //dispaly the value of the register
        //println!("Opcode 10 = Output");
        //println!("{}", registers[rc]);
        
        // convert u32 to a char
        let temp = from_u32(val).unwrap() as u8;
        // output value to the standard output
        stdout().write(&[temp]).unwrap();
    }
    else{
        // value was larger then 255 
        eprintln!("Invalid output must be less than 255");
        std::process::exit(1);
    }
}

// if opcode == 11
    // Reads input from standard input
pub fn input(inst: u32, registers: &mut [u32; 8]){
    //println!("Opcode = 11 input");
    let rc = get(&RC, inst);
    
    // Take in standard input
    match stdin().bytes().next() {
        // if input is valid and less than 255
        Some(value)=> {
            let x = value.unwrap();
            
            registers[rc as usize] = x as u32; 
                
        }
        None => {
            // Return error code if invalid input
            eprintln!("Invalid Input must be between 0 and 255");
            std::process::exit(1);
        }
    }


}

// if opcode == 12
    // Loads program from instruction
pub fn load_program(inst: u32, registers: &mut [u32; 8], memory: &mut Vec<Vec<u32>>, pcounter: &mut u32){
    // println!("Opcode = 12 Load Program");
    let rb = get(&RB, inst);
    let rc = get(&RC, inst);

    // Check if register rb is 0 
    if registers[rb as usize] != 0{
        // Clone memory 0 (instructions)
        memory[0] = memory[registers[rb as usize] as usize].clone();
    }
    
    // Assign p counter to registers rc
    *pcounter = registers[rc as usize];

}

// if opcode == 13 
    // Loads value in from instruction
pub fn load_value(inst: u32, registers: &mut [u32; 8]){
    // println!("Opcode = 13 Load Value");
    let rl = get(&RL, inst);
    let vl = get(&VL, inst);

    registers[rl as usize] = vl;
    
}

