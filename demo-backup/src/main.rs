use cmdr::*;

struct Application {}

#[cmdr(help="This is an application that greets people")]
impl Application {
    fn prompt(&self) -> String {
        "%% ".to_string()
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

    fn default(&mut self, command: &Line) -> CommandResult {
        println!("{} ??", command.command);

        CommandResult::Ok
    }
}

fn main() {
    cmd_loop(&mut Application {});
}
