use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone, PartialEq, Debug)]
#[command(rename_rule = "snake_case")]
pub enum Command {
  #[command(description = "start")]
  Start,

  #[command(description = "about")]
  About,

  #[command(description = "config")]
  Config,

  #[command(description = "today")]
  Today,

  #[command(description = "next")]
  Next,
}

#[derive(BotCommands, Clone, PartialEq, Debug)]
#[command(rename_rule = "snake_case")]
pub enum DeveloperCommand {
  Test,
}
