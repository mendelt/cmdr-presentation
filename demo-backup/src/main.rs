use cmdr::*;

struct Application {}

#[cmdr(help="This is an application that greets people")]
impl Application {
    fn prompt(&self) -> String {
        "%%".to_string()
    }

    #[cmd(hello)]
    fn say_hello(&self, args: &[String]) -> CommandResult {
        //! Also help text
        println!("Hello {}", args[0]);
        CommandResult::Ok
    }

    #[cmd(quit, help="help text\nquitter!", alias(exit, q, x))]
    fn quit_method(&self, _args: &[String]) -> CommandResult {
        CommandResult::Quit
    }

    fn default(&mut self, command: &Line) -> CommandResult {
        println!("{} ??", command.command);

        CommandResult::Ok
    }

    #[cmd]
    fn cd(&self, args: &[String]) -> CommandResult {
        CommandResult::sub_scope(SubDir { name: args[0].clone()})
    }
}

struct SubDir {
    name: String
}

#[cmdr]
impl SubDir {
    fn prompt(&self) -> String {
        self.name.clone()
    }

    #[cmd]
    fn up(&self, _args: &[String]) -> CommandResult {
        CommandResult::Exit
    }
}

fn main() {
    cmd_loop(&mut Application {});
}
