use rustpython_parser::{Mode, parse, ast, ParseError};
use std::fs;


pub fn run() {
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
                                    println!("  Class method: {}", func_def.name);

                                    for func_stmt in func_def.body {
                                        match func_stmt {
                                            // ast::Stmt::Expr(expr) => {
                                            //     println!("    Expr: {:?}", expr);
                                            // },
                                            ast::Stmt::Assign(assign) => {
                                                println!("    Assign: {:?}", assign.targets);

                                                // print variable names and values
                                                for target in assign.targets {
                                                    match target {
                                                        ast::Expr::Name(name) => {
                                                            println!("      Name: {:?}", name.id);
                                                        },
                                                        _ => {}
                                                    }
                                                }
                                            },
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    },
                    ast::Stmt::FunctionDef(func_def) => {
                        println!("Function: {}", func_def.name);
                    },
                    ast::Stmt::Assign(assign) => {
                        println!("Assign: {:?}", assign.targets);
                        // get variable names and values
                        for target in assign.targets {
                            match target {
                                ast::Expr::Name(name) => {
                                    if name.id.as_str() == "x" {
                                        // check length of the variable name
                                        if name.id.len() < 3 {
                                            println!("Variable name {:?} is too short!", name.id.as_str());
                                        }
                                    }
                                },
                                _ => {}
                            }
                        }
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
