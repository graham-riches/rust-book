use std::{fmt::{self, Write}, io::{self}, iter::{self, FromIterator}, path::Path};

use string_utils::string_utils;

use nom::{
    bytes::complete::take_until,
    character::complete::{anychar, multispace0, one_of},
    combinator::recognize,
    error::ErrorKind,
    multi::many1,
    sequence::tuple,
    IResult,
};

pub struct SymbolTable(Vec<SymbolTableEntry>);

impl SymbolTable {

    /// Creates a new SymbolTable object
    /// 
    /// # Examples
    /// ```ignore
    /// use arm_binary_utils::symbol_table::SymbolTable;
    /// let symbol_table = SymbolTable::new();
    /// ```
    fn new() -> Self {
        SymbolTable(Vec::new())
    }

    /// Creates a symbol table from a file
    /// 
    /// # Arguments
    /// * 'filename' - Path/filename to read 
    /// 
    /// # Examples
    /// ```ignore
    /// let symbol_table = symbol_table::SymbolTable::from_file("binary.txt")?;
    /// ```
    pub fn from_file(filename: impl AsRef<Path>) -> io::Result<Self> {
        let input = string_utils::read_lines_from_file(filename)?;
        let result = input.iter()
            .map(|x| parse_symbol_table_entry(x))
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .map(|(_, result)| result)
            .collect::<SymbolTable>();
        Ok(result)
    }

    /// Gets the maximum length of all names contained in the symbol table
    fn get_max_name_length(&self) -> usize {
        self.iter().map(|x| x.name.len()).max().unwrap()
    }

    /// Gets the maximum length of all symbol location names
    fn get_max_section_name_length(&self) -> usize {
        let min = self.iter().map(|x| x.section.len()).max().unwrap();
        if min < "Section".len() {
            "Section".len()
        } else {
            min
        }
    }    
}

impl std::ops::Deref for SymbolTable {
    type Target = Vec<SymbolTableEntry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SymbolTable {    
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<SymbolTableEntry> for SymbolTable {
    fn from_iter<T: IntoIterator<Item = SymbolTableEntry>>(iter: T) -> Self {
        let mut collection = SymbolTable::new();
        for i in iter {
            collection.push(i);
        }
        collection
    }
}

impl IntoIterator for SymbolTable {
    type Item = SymbolTableEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Debug for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_name_width = self.get_max_name_length();
        let max_section_width = self.get_max_section_name_length();
        let max_width = max_name_width + max_section_width + 19; // Hack for now for formatting, fix later
        let banner_break: String = iter::repeat("-").take(max_width).collect();
        f.write_fmt(format_args!("{}\r\n", banner_break)).unwrap();
        f.write_fmt(format_args!("{:0n_width$} {:0s_width$} {:8} {:8}\r\n", "Name", "Section", "Address", "Size", n_width = max_name_width, s_width = max_section_width)).unwrap();
        f.write_fmt(format_args!("{}\r\n", banner_break)).unwrap();

        for i in &self.0 {
            let result = f.write_str(&i.to_string(max_name_width, max_section_width));
            if result.is_err() {
                return Err(std::fmt::Error);
            }
        }
        Ok(())
    }
}




#[derive(Debug, PartialEq)]
pub struct SymbolTableEntry {
    pub address: u32,
    pub flags: SymbolTableFlags,
    pub section: String,
    pub alignment_or_size: u32,
    pub name: String,
}

impl SymbolTableEntry {
    fn to_string(&self, name_width: usize, section_width: usize) -> String {
        format!("{:0n_width$} {:0s_width$} {:08x} {:08x}\r\n", 
            self.name, 
            self.section, 
            self.address, 
            self.alignment_or_size, 
            n_width = name_width, 
            s_width = section_width)
    }
}


#[derive(Debug, PartialEq)]
pub struct SymbolTableFlags {
    pub scope: SymbolScope,
    pub weakness: SymbolWeakness,
    pub constructor: SymbolConstructor,
    pub warning: SymbolWarning,
    pub reference: SymbolReference,
    pub debugging: SymbolDebugging,
    pub symbol_type: SymbolType,
}

#[derive(Debug, PartialEq)]
pub enum SymbolScope {
    Local,
    Global,
    Neither,
    Both,
}

#[derive(Debug, PartialEq)]
pub enum SymbolWeakness {
    Weak,
    Strong,
}

#[derive(Debug, PartialEq)]
pub enum SymbolConstructor {
    Constructor,
    Regular,
}

#[derive(Debug, PartialEq)]
pub enum SymbolWarning {
    Warning,
    Regular,
}

#[derive(Debug, PartialEq)]
pub enum SymbolReference {
    Reference,
    Regular,
}

#[derive(Debug, PartialEq)]
pub enum SymbolDebugging {
    Debug,
    Dynamic,
    Regular,
}

#[derive(Debug, PartialEq)]
pub enum SymbolType {
    Function,
    File,
    Object,
    Regular,
}

/// Parses an unsigned decimal value from a string
fn parse_u32(input: &str) -> IResult<&str, u32> {
    recognize(many1(one_of("0123456789abcdefABCDEF")))(input)
        .map(|(i, o)| (i, u32::from_str_radix(o, 16).unwrap()))
}

/// Parses the symbol table scope bit out of the bit flags sequence
fn parse_flag_bit_scope(input: &str) -> IResult<&str, SymbolScope> {
    let result = one_of("lg !")(input)?;
    match result.1 {
        'l' => Ok((result.0, SymbolScope::Local)),
        'g' => Ok((result.0, SymbolScope::Global)),
        ' ' => Ok((result.0, SymbolScope::Neither)),
        '!' => Ok((result.0, SymbolScope::Both)),
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: ErrorKind::Char,
        })),
    }
}

/// Parses the symbol table weak/strong symbol bit out of the bit flags sequence
fn parse_flag_bit_weakness(input: &str) -> IResult<&str, SymbolWeakness> {
    let result = one_of("w ")(input)?;
    match result.1 {
        'w' => Ok((result.0, SymbolWeakness::Weak)),
        ' ' => Ok((result.0, SymbolWeakness::Strong)),
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: ErrorKind::Char,
        })),
    }
}

/// Parses the symbol table constructor flag bit
fn parse_flag_bit_constructor(input: &str) -> IResult<&str, SymbolConstructor> {
    let result = one_of("C ")(input)?;
    match result.1 {
        'C' => Ok((result.0, SymbolConstructor::Constructor)),
        ' ' => Ok((result.0, SymbolConstructor::Regular)),
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: ErrorKind::Char,
        })),
    }
}

/// Parses if the symbol is a warning symbol or not
fn parse_flag_bit_warning(input: &str) -> IResult<&str, SymbolWarning> {
    let result = one_of("W ")(input)?;
    match result.1 {
        'W' => Ok((result.0, SymbolWarning::Warning)),
        ' ' => Ok((result.0, SymbolWarning::Regular)),
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: ErrorKind::Char,
        })),
    }
}

/// Parses the symbol flags reference bit
fn parse_flag_bit_reference(input: &str) -> IResult<&str, SymbolReference> {
    let result = one_of("l ")(input)?;
    match result.1 {
        'l' => Ok((result.0, SymbolReference::Reference)),
        ' ' => Ok((result.0, SymbolReference::Regular)),
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: ErrorKind::Char,
        })),
    }
}

/// Parses debug symbols from the flag bits
fn parse_flag_bit_debugging(input: &str) -> IResult<&str, SymbolDebugging> {
    let result = one_of("dD ")(input)?;
    match result.1 {
        'd' => Ok((result.0, SymbolDebugging::Debug)),
        'D' => Ok((result.0, SymbolDebugging::Dynamic)),
        ' ' => Ok((result.0, SymbolDebugging::Regular)),
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: ErrorKind::Char,
        })),
    }
}

/// Parses the symbol type from the flag bits
fn parse_flag_bit_type(input: &str) -> IResult<&str, SymbolType> {
    let result = one_of("FfO ")(input)?;
    match result.1 {
        'F' => Ok((result.0, SymbolType::Function)),
        'f' => Ok((result.0, SymbolType::File)),
        'O' => Ok((result.0, SymbolType::Object)),
        ' ' => Ok((result.0, SymbolType::Regular)),
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: ErrorKind::Char,
        })),
    }
}

/// Parses a string into symbol table flags
fn parse_symbol_flags(input: &str) -> IResult<&str, SymbolTableFlags> {
    let result = tuple((
        parse_flag_bit_scope,
        parse_flag_bit_weakness,
        parse_flag_bit_constructor,
        parse_flag_bit_warning,
        parse_flag_bit_reference,
        parse_flag_bit_debugging,
        parse_flag_bit_type,
    ))(input)?;
    let symbol_flags = result.1;
    Ok((
        result.0,
        SymbolTableFlags {
            scope: symbol_flags.0,
            weakness: symbol_flags.1,
            constructor: symbol_flags.2,
            warning: symbol_flags.3,
            reference: symbol_flags.4,
            debugging: symbol_flags.5,
            symbol_type: symbol_flags.6,
        },
    ))
}

/// Parses a symbol table entry from a string. Returns a results type containing the parsed result if successful
///
/// # Arguments
/// * 'input' - The input string to parse the symbol table entry from
pub fn parse_symbol_table_entry(input: &str) -> IResult<&str, SymbolTableEntry> {
    let mut parser = tuple((
        parse_u32,
        one_of(" "),
        parse_symbol_flags,
        multispace0,
        take_until("\t"),
        multispace0,
        parse_u32,
        multispace0,
        many1(anychar),
    ));
    let result = parser(input)?;
    let (address, _, flags, _, section, _, alignment_or_size, _, name) = result.1;
    Ok((
        "",
        SymbolTableEntry {
            address,
            flags,
            section: section.to_string(),
            alignment_or_size,
            name: name.iter().collect::<String>(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_decimal_from_string() {
        assert_eq!(parse_u32("12345"), Ok(("", 74565)));
        assert!(parse_u32("gabcdeg").is_err());
    }

    #[test]
    fn test_parse_flag_bit_scope() {
        assert_eq!(parse_flag_bit_scope("l"), Ok(("", SymbolScope::Local)));
        assert_eq!(parse_flag_bit_scope("g"), Ok(("", SymbolScope::Global)));
        assert_eq!(parse_flag_bit_scope(" "), Ok(("", SymbolScope::Neither)));
        assert_eq!(parse_flag_bit_scope("!"), Ok(("", SymbolScope::Both)));
        assert_eq!(
            parse_flag_bit_scope("x"),
            Err(nom::Err::Error(nom::error::Error {
                input: "x",
                code: ErrorKind::OneOf
            }))
        );
    }

    #[test]
    fn test_parse_flag_bit_weakness() {
        assert_eq!(parse_flag_bit_weakness("w"), Ok(("", SymbolWeakness::Weak)));
        assert_eq!(
            parse_flag_bit_weakness(" "),
            Ok(("", SymbolWeakness::Strong))
        );
        assert_eq!(
            parse_flag_bit_weakness("x"),
            Err(nom::Err::Error(nom::error::Error {
                input: "x",
                code: ErrorKind::OneOf
            }))
        );
    }

    #[test]
    fn test_parse_flag_bit_constructor() {
        assert_eq!(
            parse_flag_bit_constructor("C"),
            Ok(("", SymbolConstructor::Constructor))
        );
        assert_eq!(
            parse_flag_bit_constructor(" "),
            Ok(("", SymbolConstructor::Regular))
        );
        assert_eq!(
            parse_flag_bit_constructor("x"),
            Err(nom::Err::Error(nom::error::Error {
                input: "x",
                code: ErrorKind::OneOf
            }))
        );
    }

    #[test]
    fn test_parse_flag_bit_warning() {
        assert_eq!(
            parse_flag_bit_warning("W"),
            Ok(("", SymbolWarning::Warning))
        );
        assert_eq!(
            parse_flag_bit_warning(" "),
            Ok(("", SymbolWarning::Regular))
        );
        assert_eq!(
            parse_flag_bit_warning("x"),
            Err(nom::Err::Error(nom::error::Error {
                input: "x",
                code: ErrorKind::OneOf
            }))
        );
    }

    #[test]
    fn test_parse_flag_bit_reference() {
        assert_eq!(
            parse_flag_bit_reference("l"),
            Ok(("", SymbolReference::Reference))
        );
        assert_eq!(
            parse_flag_bit_reference(" "),
            Ok(("", SymbolReference::Regular))
        );
        assert_eq!(
            parse_flag_bit_reference("x"),
            Err(nom::Err::Error(nom::error::Error {
                input: "x",
                code: ErrorKind::OneOf
            }))
        );
    }

    #[test]
    fn test_parse_flag_bit_debugging() {
        assert_eq!(
            parse_flag_bit_debugging("d"),
            Ok(("", SymbolDebugging::Debug))
        );
        assert_eq!(
            parse_flag_bit_debugging("D"),
            Ok(("", SymbolDebugging::Dynamic))
        );
        assert_eq!(
            parse_flag_bit_debugging(" "),
            Ok(("", SymbolDebugging::Regular))
        );
        assert_eq!(
            parse_flag_bit_debugging("x"),
            Err(nom::Err::Error(nom::error::Error {
                input: "x",
                code: ErrorKind::OneOf
            }))
        );
    }

    #[test]
    fn test_parse_flag_bit_type() {
        assert_eq!(parse_flag_bit_type("F"), Ok(("", SymbolType::Function)));
        assert_eq!(parse_flag_bit_type("f"), Ok(("", SymbolType::File)));
        assert_eq!(parse_flag_bit_type("O"), Ok(("", SymbolType::Object)));
        assert_eq!(parse_flag_bit_type(" "), Ok(("", SymbolType::Regular)));
        assert_eq!(
            parse_flag_bit_type("x"),
            Err(nom::Err::Error(nom::error::Error {
                input: "x",
                code: ErrorKind::OneOf
            }))
        );
    }

    #[test]
    fn test_parse_symbol_table_flags() {
        assert_eq!(
            parse_symbol_flags("l    df"),
            Ok((
                "",
                SymbolTableFlags {
                    scope: SymbolScope::Local,
                    weakness: SymbolWeakness::Strong,
                    constructor: SymbolConstructor::Regular,
                    warning: SymbolWarning::Regular,
                    reference: SymbolReference::Regular,
                    debugging: SymbolDebugging::Debug,
                    symbol_type: SymbolType::File
                }
            ))
        );
    }

    #[test]
    fn test_parse_symbol_table_flags_with_empty_start() {
        assert_eq!(
            parse_symbol_flags(" w   df"),
            Ok((
                "",
                SymbolTableFlags {
                    scope: SymbolScope::Neither,
                    weakness: SymbolWeakness::Weak,
                    constructor: SymbolConstructor::Regular,
                    warning: SymbolWarning::Regular,
                    reference: SymbolReference::Regular,
                    debugging: SymbolDebugging::Debug,
                    symbol_type: SymbolType::File
                }
            ))
        );
    }

    #[test]
    fn test_parse_symbol_table_entry() {
        let result =
            parse_symbol_table_entry("08020000 l    d  .vectors	00000000 .vectors").unwrap();
        let entry = result.1;
        assert_eq!(entry.address, 134348800);
        assert_eq!(entry.name, ".vectors");
        assert_eq!(entry.alignment_or_size, 0);
        assert_eq!(entry.section, ".vectors");
        assert_eq!(
            entry.flags,
            SymbolTableFlags {
                scope: SymbolScope::Local,
                weakness: SymbolWeakness::Strong,
                constructor: SymbolConstructor::Regular,
                warning: SymbolWarning::Regular,
                reference: SymbolReference::Regular,
                debugging: SymbolDebugging::Debug,
                symbol_type: SymbolType::Regular
            }
        );
    }

    #[test]
    fn test_parse_symbol_table_entry_with_empty_flag_start() {
        let result =
            parse_symbol_table_entry("0803f76a  w    F .text	00000002 __printf_unlock").unwrap();
        let entry = result.1;
        assert_eq!(entry.address, 134477674);
        assert_eq!(entry.section, ".text");
        assert_eq!(entry.alignment_or_size, 2);
        assert_eq!(entry.name, "__printf_unlock");
        assert_eq!(
            entry.flags,
            SymbolTableFlags {
                scope: SymbolScope::Neither,
                weakness: SymbolWeakness::Weak,
                constructor: SymbolConstructor::Regular,
                warning: SymbolWarning::Regular,
                reference: SymbolReference::Regular,
                debugging: SymbolDebugging::Regular,
                symbol_type: SymbolType::Function
            }
        );
    }
}
