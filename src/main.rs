mod lexer;
mod parser;
mod code_generator;
mod symbol_table;
mod semantic_analyzer;
mod error;
use std::{env};
use crate::parser::program::Program;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let program = Program::new(args[1].clone());
        let result = program.unwrap().start();
        println!("Assembler status: {:?}", result);
    } else {
        eprintln!("Error, usage: {} <file.asm>", args[0]);
    }
}



/*

    let mut code_gen : Vec<u8> = Vec::new();

    if 1 == 1 { //esempio caso push32
        code_gen.extend((0x0010 as u16).to_be_bytes());//inserimento opcode
        //TODO: inserimento altri valori
    }

    if 2 == 2 {
        //esempio jmp label
        /*
            code_gen.extend([0x00, 0x02].into_iter()); // jmp opcode
            if labelmap.present(label) {
                label = labelmap.get(label);
                code_gen.push(label.address);
            } else {
                let unresolved_label = UnresolvedLabel::new(label.name, address_to_be_resolve); //address_to_be_resolve sarebbe l'indirizzo di memoria del codice generato dove inizia l'indirizzo del salto
                unresolved_label.add(unresolved_label);
                code_gen.push(0x0000);                  //empty 4 byte address, will be resolved after
            }


         */
    }

    if 3 == 3 {
        //label declaration
        /*
            labelmap.add(label, address);
            unresolved_label.resolve(label);//risolve gli indirizzi per tutte le istruzioni prima che sia stata dichiarata la label
         */
    }

    //println!("test {:?} {:?}", code_gen.get(0), code_gen.get(1));





    Ok(())
}*/