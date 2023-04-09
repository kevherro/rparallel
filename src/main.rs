/*
 * Copyright (c) 2023 Kevin Herro.
 * Licensed under the MIT license.
 *
 * This file implements a basic shell that accepts and executes shell commands.
 * Users can type commands at the prompt, with output displayed in the terminal.
 * The program handles input using the rustyline crate and supports quitting
 * the shell by typing "q" or "quit".
 *
 * Author: Kevin Herro <kvherro@gmail.com>
 */

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    // Initialize the readline editor.
    let mut rl = DefaultEditor::new()?;

    // Display a welcome message.
    println!("Entering interactive mode (type \"help\" for commands, \"o\" for options)");

    loop {
        // Read a line of input from the user.
        let readline = rl.readline("(parallel) ");

        match readline {
            Ok(line) => match line.trim() {
                "q" | "quit" => break,
                "help" => print_help(),
                "o" => print_current_options(),
                cmd => execute_command(cmd),
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn print_current_options() {
    println!("current options...")
}

fn print_help() {
    println!("help!")
}

fn execute_command(cmd: &str) {
    if cmd.is_empty() {
        return;
    }

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr);
    }
}
