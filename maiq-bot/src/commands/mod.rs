mod handler;

use teloxide::macros::BotCommands;

use crate::Caller;
use crate::Result;

macro_rules! cmds {
  {
    pub: { $($name: ident[desc: $desc: literal $(, args: ($($arg:ident: $tt:ty),*) )?] => $fn_name: ident),* },
    dev: { $($dev_name: ident$([args: ($($dev_arg:ident: $dev_tt:ty),*)])? => $dev_fn_name: ident),* }
  } => {
    #[derive(BotCommands, Clone, PartialEq, Debug)]
    #[command(rename_rule = "snake_case")]
    pub enum Command {
      $(
        #[command(description = $desc)]
        $name$(($($tt),*))?
      ),*
    }

    #[derive(BotCommands, Clone, PartialEq, Debug)]
    #[command(rename_rule = "snake_case")]
    pub enum DeveloperCommand {
      $($dev_name$(($($dev_tt),*))?),*
    }

    pub trait Commands {
      $(async fn $fn_name(self, $($($arg: $tt),*)?) -> Result<()>;)*
    }

    pub trait DeveloperCommands {
      $(async fn $dev_fn_name(self, $($($dev_arg: $dev_tt),*)?) -> Result<()>;)*
    }

    impl Command {
      pub async fn execute<C: Commands + Caller>(self, executor: C) -> Result<()> {
        info!(target: "command", "executing: {:?}; caller: {}", self, executor.caller_name());
        match self {
          $(Command::$name$(($($arg),*))? => executor.$fn_name($($($arg),*)?).await?),*
        }
        Ok(())
      }
    }

    impl DeveloperCommand {
      pub async fn execute<D: DeveloperCommands + Caller>(self, executor: D) -> Result<()> {
        info!(target: "dev-command", "executing: {:?}; caller: {}", self, executor.caller_name());
        match self {
          $(DeveloperCommand::$dev_name$(($($dev_arg),*))? => executor.$dev_fn_name($($($dev_arg),*)?).await?),*
        }
        Ok(())
      }
    }

  }
}

cmds! {
  pub: {
    Start[desc: "Стартовая команда", args: (group_indexes: String)] => start,
    Today[desc: "Сегодня"] => today,
    Next[desc: "Завтра"] => next,
    About[desc: "Информация"] => about,
    Config[desc: "Настройки"] => show_config,
    Version[desc: "Версия"] => version
  },
  dev: {
    Test => test
  }
}
