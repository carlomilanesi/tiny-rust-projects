use std::env::args;
use std::process::{abort, exit, ExitCode, Termination};

enum MyExitCode {
    Ok = 0,
    InvalidArguments,
    TooMuchData = 42,
}

impl Termination for MyExitCode {
    fn report(self) -> ExitCode {
        println!(
            "{}",
            match &self {
                MyExitCode::Ok => "Good!",
                MyExitCode::InvalidArguments => "Error: invalid arguments.",
                MyExitCode::TooMuchData => "Error: too much input data.",
            }
        );
        ExitCode::from(self as u8)
    }
}

fn handle_second_argument(call_exit: bool) -> MyExitCode {
    if let Some(string_code) = args().nth(2) {
        let my_exit_code = match string_code.as_str() {
            "ok" => MyExitCode::Ok,
            "invalid_argument" => MyExitCode::InvalidArguments,
            "too_much_data" => MyExitCode::TooMuchData,
            _ => abort(),
        };
        if call_exit {
            exit(my_exit_code as i32)
        } else {
            return my_exit_code;
        }
    }
    abort()
}

fn main() -> MyExitCode {
    if let Some(option) = args().nth(1) {
        match option.as_str() {
            "success" => MyExitCode::Ok,
            "abort" => abort(),
            "panic" => {
                if let Some(arg) = args().nth(2) {
                    panic!("{}", arg);
                } else {
                    panic!()
                }
            }
            "exit" => handle_second_argument(true),
            "return" => handle_second_argument(false),
            _ => abort(),
        }
    } else {
        abort()
    }
}
