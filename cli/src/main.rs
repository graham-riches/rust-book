extern crate clap;
use clap::{App};
use arm_binary_tools::parser::symbol_table::SymbolTable;

fn main() {
    let matches = App::new("ARM Disassembly Parser")
        .version("1.0.0")
        .author("Graham Riches")
        .about("Parses disassembled ARM binaries and provides useful output")
        .args_from_usage(
            "<INPUT>                'The disassembled binary file to parse'
            -f, --functions         'Display only functions'
            -s, --sort=[ORD]        'Sort the output. ORD='ascending', 'descending''
            -v, --verbosity=[LEVEL] 'Enables verbose output with optional levels: <0,1,2>'")
        .get_matches();

    let file = matches.value_of("INPUT").unwrap();
    let symbol_table = SymbolTable::from_file(file).expect("Could not parse input file as a symbol table");

    let mut bss = symbol_table.into_iter()
        .filter(|x| x.section == ".bss")
        .collect::<SymbolTable>();
    bss.sort_descending();
    print!("{:?}", bss);
    


}
