use std::collections::HashMap;

use hawk_common::object::Object;

/// Runs Hawk code from a file given by `filename`, returning the global scope after execution
pub fn run_script(
    filename: String,
    global_state: HashMap<String, Object>,
    filein_fn: fn(String) -> Result<String, String>,
    fileout_fn: fn(String, String) -> Result<(), ()>,
    warn_fn: fn(String, usize) -> (),
    err_fn: fn(String, usize) -> (),
    output_fn: fn(String) -> (),
) -> HashMap<String, Object> {
    let source = filein_fn(filename.clone());

    match source {
        Result::Ok(source) => match run(
            source,
            global_state,
            false,
            filein_fn,
            fileout_fn,
            warn_fn,
            err_fn,
            output_fn,
        ) {
            Ok(globals) => globals,
            Err((message, line)) => {
                err_fn(message, line);
                HashMap::new()
            }
        },
        Result::Err(_) => {
            err_fn(format!("Couldn't read file {filename}"), 0);
            HashMap::new()
        }
    }
}

pub fn run(
    source: String,
    global_state: HashMap<String, Object>,
    in_repl: bool,
    filein_fn: fn(String) -> Result<String, String>,
    fileout_fn: fn(String, String) -> Result<(), ()>,
    warn_fn: fn(String, usize) -> (),
    err_fn: fn(String, usize) -> (),
    output_fn: fn(String) -> (),
) -> Result<HashMap<String, Object>, (String, usize)> {
    let tokens = hawk_lib::lexer::Lexer::lex(&source);

    //println!("{}", Tokens(tokens.clone()));

    let statements = hawk_lib::parser::Parser::parse(&tokens, warn_fn)?;

    //println!("{:?}\n", statements);

    let result = crate::Interpreter::interpret(
        statements,
        global_state,
        in_repl,
        filein_fn,
        fileout_fn,
        warn_fn,
        err_fn,
        output_fn,
    )?;

    Ok(result)
}
