use crate::parameters::ParameterMode;

/// Encodes a parameter as well as it's calling type
#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub mode: ParameterMode,
    pub value: i64
}

/// Encodes an intcode instruction and it's parameter passing modes
#[derive(Debug, PartialEq)]
pub struct InstructionType {
    pub op_code: i64,
    pub a: Parameter,
    pub b: Parameter,
    pub c: Parameter,
}

/// Parses an integer value into an instruction type with parameter modes
/// 
/// ## Note
/// Always parses three arguments regardless of the operation, each operation
/// must specifically reduce this to what it requires as necessary
/// 
/// # Examples
/// ```ignore
/// let instruction = match parse_instruction_type(1002);
/// // instruction = Some(2, Position, Immediate, Position)
/// ```
pub fn parse_instruction_type(instruction: i64) -> Option<InstructionType> {
    let mut chars: Vec<char> = instruction.to_string().chars().collect();
    while chars.len() < 5 {
        chars.insert(0, '0');
    }
    let a = parse_parameter_mode(chars[0])?;
    let b = parse_parameter_mode(chars[1])?;
    let c = parse_parameter_mode(chars[2])?;
    let op_code = chars.iter().skip(3).collect::<String>().parse::<i64>().unwrap();
    Some(InstructionType{ op_code, 
                          a: Parameter{mode: a, value: 0},
                          b: Parameter{mode: b, value: 0},
                          c: Parameter{mode: c, value: 0}})    
}

/// Gets the actual value of a parameter depending on the parameter mode
/// 
/// # Examples
/// ```ignore
/// assert_eq!(get_parameter_value( Parameter{ mode: ParameterMode::Position, value: 3}, &vec![10, 20, 30, 40]), 40);
/// assert_eq!(get_parameter_value( Parameter{ mode: ParameterMode::Immediate, value: 3}, &vec![10, 20, 30, 40]), 3);
/// ```
pub fn get_parameter_value(param: &Parameter, program: &[i64]) -> i64 {
    match param.mode {
        ParameterMode::Position => program[param.value as usize],
        ParameterMode::Immediate => param.value
    }
}


/// Helper function to match a char and convert it to a ParameterMode
fn parse_parameter_mode(c: char) -> Option<ParameterMode> {
    match c {
        '0' => Some(ParameterMode::Position),
        '1' => Some(ParameterMode::Immediate),
        _   => None
    }
}

#[test]
fn test_parse_instruction_type() {
    assert_eq!(parse_instruction_type(1002), Some(InstructionType{ op_code: 2, 
        a: Parameter{ mode: ParameterMode::Position, value: 0}, 
        b: Parameter{ mode: ParameterMode::Immediate, value: 0},
        c: Parameter{ mode: ParameterMode::Position, value: 0}}));

    assert_eq!(parse_instruction_type(1008), Some(InstructionType{ op_code: 8, 
            a: Parameter{ mode: ParameterMode::Position, value: 0}, 
            b: Parameter{ mode: ParameterMode::Immediate, value: 0},
            c: Parameter{ mode: ParameterMode::Position, value: 0}}));
}

#[test]
fn test_parse_parameter_mode() {
    assert_eq!(parse_parameter_mode('0'), Some(ParameterMode::Position));
    assert_eq!(parse_parameter_mode('1'), Some(ParameterMode::Immediate));
    assert_eq!(parse_parameter_mode('2'), None);
}

#[test]
fn test_get_parameter_value() {
    assert_eq!(get_parameter_value( &Parameter{ mode: ParameterMode::Position, value: 3}, &vec![10, 20, 30, 40]), 40);
    assert_eq!(get_parameter_value( &Parameter{ mode: ParameterMode::Immediate, value: 3}, &vec![10, 20, 30, 40]), 3);
}