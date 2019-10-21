use cmdr::*;

struct Application {}

#[cmdr(help="This is an application that can greet people")]
impl Application {
    fn prompt(&self) -> String {
        "%% ".to_string()
    }

    fn before_command(&mut self, _line: Line) -> Line {
        println!("Do something before\n");
        _line
    }

    #[cmd(hello)]
    fn say_hello(&self, args: &[String]) -> CommandResult {
        //! Also help text
        println!("Hello {}", args[0]);
        CommandResult::Ok
    }

    #[cmd(quit, help="help text\nquitter!")]
    fn quit_method(&self, _args: &[String]) -> CommandResult {
        CommandResult::Quit
    }
}

fn main() {
    cmd_loop(&mut Application {});
}
