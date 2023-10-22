mod handler;
pub use handler::*;

use serde::Deserialize;
use serde::Serialize;
use teloxide::types::CallbackQuery;

use crate::Caller;
use crate::Result;

use teloxide::types::InlineKeyboardButton;

macro_rules! callbacks {
  {$($name: ident$(($($arg_name: ident: $tt: ty),*))? => $fn_name: ident),*} => {
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

pub fn filter_callback(query: CallbackQuery) -> Option<Callback> {
  query
    .data
    .as_ref()
    .and_then(|data| bincode::deserialize(data.as_bytes()).ok())
}

callbacks! {
  Test(arg: i32) => test,
  SetMyGroups => show_my_groups,
  SetGroup(name: String) => set_group
}
