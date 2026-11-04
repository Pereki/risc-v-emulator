pub fn decode_operation(operation: u32) {
    let opcode = operation & 0x7F;

    println!("Opcode is: {}", opcode);
    match opcode {
        0x33 => {
            println!("R-Type OpCode of ADD/SUB/XOR/...");
        }
        _ => todo!("Other Op Codes may follow"),
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Brings everything from the outer module into scope

    #[test]
    fn test_add_decode() {
        decode_operation(0x33);
    }
}
