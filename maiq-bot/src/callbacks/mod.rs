mod handler;
pub use handler::*;

use serde::Deserialize;
use serde::Serialize;

use crate::Caller;
use crate::Result;

use teloxide::types::InlineKeyboardButton;

macro_rules! callbacks {
  {$($name: ident$([args: ($($arg:ident: $tt:ty),*)])? => $fn_name: ident),*} => {
    #[derive(Serialize, Deserialize, Debug)]
    pub enum Callback {
      $(
        $name$(($($tt),*))?
      ),*
    }

    pub trait Callbacks {
      $(async fn $fn_name(self, $($($arg: $tt),*)?) -> Result<()>;)*
    }

    impl Callback {
      pub async fn execute<C: Callbacks + Caller>(self, executor: C) -> Result<()> {
        info!(target: "callback", "executing: {:?}; caller: {}", self, executor.caller());
        match self {
          $(Callback::$name$(($($arg),*))? => executor.$fn_name($($($arg),*)?).await?),*
        }
        Ok(())
      }
    }

  };
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

impl<T: Into<String>> From<CallbackPayload<T>> for InlineKeyboardButton {
  fn from(val: CallbackPayload<T>) -> Self {
    let data = String::from_utf8(bincode::serialize(&val.kind).unwrap()).unwrap();
    InlineKeyboardButton::callback(val.text, data)
  }
}

callbacks! {
  Test => test
}
