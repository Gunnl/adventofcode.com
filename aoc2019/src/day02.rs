use std::io::BufRead;

use crate::error::Error;

pub fn run<R>(input: R) -> Result<(), Error>
where
R: BufRead,
{
    let mut computer = Computer::new(input)?;
    let answer = computer.execute(12, 2)?;
    println!("{}", answer);

    for noun in 0..=99 {
        for verb in 0..=99 {
            if computer.execute(noun, verb)? == 19690720 {
                println!("{}", 100*noun+verb);
            }
        }
    }

    Ok(())
}

struct Computer {
    rom: Vec<usize>,
    ram: Vec<usize>,
    pc: usize,
}

impl Computer {
    fn new<R>(mut rom_reader: R) -> Result<Self, Error>
    where
        R: BufRead,
    {
        let mut buffer = String::new();
        rom_reader.read_to_string(&mut buffer)?;
        let rom = buffer
            .trim()
            .split(",")
            .map(|s| Ok(s.parse::<usize>()?))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(
            Self {
                rom,
                ram: Vec::new(),
                pc: 0,
            }
        )
    }
    
    fn execute(&mut self, noun: usize, verb: usize) -> Result<usize, Error> {
        self.ram = self.rom.clone();
        self.ram[1] = noun;
        self.ram[2] = verb;
        self.pc = 0;

        loop {
            let opcode = self.ram[self.pc];
            // opcode 1 = add
            // opcode 2 = mul
            // opcode 99 = end
            match opcode {
                1 | 2 => {
                    let a_ptr = self.ram[self.pc+1];
                    let b_ptr = self.ram[self.pc+2];
                    let w_ptr = self.ram[self.pc+3];
                    let a = self.ram[a_ptr];
                    let b = self.ram[b_ptr];
                    self.ram[w_ptr] = if opcode == 1 { a + b } else { a * b };
                    self.pc += 4;
                },
                99 => break,
                _ => unimplemented!("Invalid opcode {}", opcode),
            }
        }
        Ok(self.ram[0])
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_02() {
        let test_cases = &[
            // input, expected1, expected2
            ("1,0,0,0,99", "2,0,0,0,99", 2),
            ("2,3,0,3,99", "2,3,0,6,99", 2),
            ("2,4,4,5,99,0", "1,0,0,0,99,9801", 966),
            ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99", 50346),
        ];

        for (input, expected1, expected2) in test_cases {
            let reader = BufReader::new(input.as_bytes());
            let mut computer = Computer::new(reader)?;
            let answer = computer.execute(0, 0)?;
            //let (actual1, actual2) = run(reader).unwrap();
            assert_eq!(answer, *expected1);
            //assert_eq!(actual2, *expected2);
        }
    }
}