use arm_binary_tools::parser::symbol_table;
use string_utils::string_utils;

#[test]
fn test_read_symbol_table_from_file() -> Result<(), std::io::Error> {
    let input = string_utils::read_lines_from_file("tests/symbol_table.txt")?;
    let symbol_table = input.iter()
        .map(|x| symbol_table::parse_symbol_table_entry(x))
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .map(|(_, result)| result)
        .collect::<symbol_table::SymbolTable>();   
    assert_eq!(2283, symbol_table.len());
    Ok(())
}

#[test]
fn test_symbol_table_from_iterable_of_string() -> Result<(), std::io::Error> {
    let symbol_table = symbol_table::SymbolTable::from_file("tests/symbol_table.txt")?;    
    assert_eq!(2283, symbol_table.len());
    Ok(())
}

