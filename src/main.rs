use rustpython_parser::{Mode, parse, ast, ParseError};

use std::fs;


fn main() {

    let parsed_content = parse_python_file("script.py").unwrap();
    // iterate over the parsed python AST nodes
    match parsed_content {
        ast::Mod::Module(mod_module) => {
            for stmt in mod_module.body {
                match stmt {
                    ast::Stmt::ClassDef(class_def) => {
                        println!("Class: {}", class_def.name);
                        for class_stmt in class_def.body {
                            match class_stmt {
                                ast::Stmt::FunctionDef(func_def) => {
                                    println!("  Function: {}", func_def.name);
                                }
                                _ => {}
                            }
                        }
                    },
                    ast::Stmt::FunctionDef(func_def) => {
                        println!("Function: {}", func_def.name);
                    },
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn parse_python_file(filename: &str) -> Result<ast::Mod, ParseError> {
    let script = fs::read_to_string(filename).unwrap();
    let program = parse(&script, Mode::Module, filename);
    program
}