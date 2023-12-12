#[macro_export]
macro_rules! reply {
  ($path: literal$(, $($args:tt)+)?) => {
    format!($crate::reply!(const $path)$(, $($args)+)?)
  };
  (const $path: literal) => {
    include_str!(concat!(env!("OUT_DIR"), "/replies/", $path))
  }
}

#[macro_export]
macro_rules! markup {
  ($e: expr) => {
    teloxide::types::InlineKeyboardMarkup::new($e)
  };
}

#[macro_export]
macro_rules! make_commands {
  {
    pub: { $($name: ident[desc: $desc: literal $(, args: ($($arg:ident: $tt:ty),*) )?] => $fn_name: ident),* },
    dev: { $($dev_name: ident$([args: ($($dev_arg:ident: $dev_tt:ty),*)])? => $dev_fn_name: ident),* }
  } => {
    use teloxide::utils::command::BotCommands;
    use $crate::Caller;

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

#[macro_export]
macro_rules! make_callbacks {
  {$($name: ident$(($($arg_name: ident: $tt: ty),*))? => $fn_name: ident),*} => {
    use $crate::Caller;
    use serde::Serialize;
    use serde::Deserialize;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum Callback {
      $($name$($({$arg_name: $tt}),*)?),*
    }

    pub trait Callbacks {
      $(async fn $fn_name(self, $($($arg_name: $tt),*)?) -> Result<()>;)*
    }

    impl Callback {
      pub async fn execute<C: Callbacks + Caller>(self, executor: C) -> Result<()> {
        info!(target: "callback", "executing: {:?}; caller: {}", self, executor.caller_name());
        match self {
          $(Callback::$name$({$($arg_name),*})? => executor.$fn_name($($($arg_name)*,)?).await?),*
        };

        Ok(())
      }
    }

  #[derive(Debug)]
  pub struct CallbackPayload<T: Into<String>> {
    text: T,
    kind: Callback,
  }

  impl Callback {
    pub fn with_text<T: Into<String>>(self, text: T) -> CallbackPayload<T> {
      CallbackPayload { text, kind: self }
    }
  }

  impl<T: Into<String>> CallbackPayload<T> {
    pub fn into_row(self) -> [[InlineKeyboardButton; 1]; 1] {
      [[self.into()]]
    }
  }

  impl<T: Into<String>> From<CallbackPayload<T>> for InlineKeyboardButton {
    fn from(val: CallbackPayload<T>) -> Self {
      let data = String::from_utf8(bincode::serialize(&val.kind).unwrap()).unwrap();
      InlineKeyboardButton::callback(val.text, data)
    }
  }

  impl<T: Into<String>> From<CallbackPayload<T>> for InlineKeyboardMarkup {
    fn from(value: CallbackPayload<T>) -> Self {
      InlineKeyboardMarkup::new([[value.into(); 1]; 1])
    }
  }

  impl<T: Into<String>> From<CallbackPayload<T>> for ReplyMarkup {
    fn from(value: CallbackPayload<T>) -> Self {
      ReplyMarkup::InlineKeyboard(value.into())
    }
  }

  pub fn filter_callback(query: CallbackQuery) -> Option<Callback> {
    query
      .data
      .as_ref()
      .and_then(|data| bincode::deserialize(data.as_bytes()).ok())
  }
  };
}
