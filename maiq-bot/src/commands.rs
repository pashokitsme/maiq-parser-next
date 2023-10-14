use crate::Result;
use teloxide::macros::BotCommands;

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
      fn executor(&self) -> String;
      $(async fn $fn_name(self, $($($arg: $tt),*)?) -> Result<()>;)*
    }

    pub trait DeveloperCommands {
      fn executor(&self) -> String;
      $(async fn $dev_fn_name(self, $($($dev_arg: $dev_tt),*)?) -> Result<()>;)*
    }

    impl Command {
      pub async fn execute<C: Commands>(self, executor: C) -> Result<()> {
        info!(target: "command", "executing: {:?}; executor: {}", self, executor.executor());
        match self {
          $(Command::$name$(($($arg),*))? => executor.$fn_name($($($arg),*)?).await?),*
        }
        Ok(())
      }
    }

    impl DeveloperCommand {
      pub async fn execute<D: DeveloperCommands>(self, executor: D) -> Result<()> {
        info!(target: "dev-command", "executing: {:?}; executor: {}", self, executor.executor());
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
    Start[desc: "start"] => start,
    Today[desc: "today"] => today,
    Next[desc: "next"] => next,
    About[desc: "about"] => about,
    Config[desc: "config"] => show_config,
    AddGroup[desc: "add-group", args: (name: String)] => add_group,
    RemoveGroup[desc: "remove-group", args: (name: String)] => remove_group
  },
  dev: {
    Test => test
  }
}
