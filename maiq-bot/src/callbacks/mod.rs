mod handler;
pub use handler::*;

use serde::Deserialize;
use serde::Serialize;

use crate::Result;


macro_rules! callbacks {
  {$($name: ident$([args: ($($arg:ident: $tt:ty),*)])? => $fn_name: ident),*} => {
    #[derive(Serialize, Deserialize, Debug)]
    pub enum Callback {
      $(
        $name$(($($tt),*))?
      ),*
    }

    #[derive(Debug)]
    pub struct CallbackPayload<T: Into<String>> {
      text: T,
      kind: Callback,
    }

    impl Callback {
      pub fn into<T: Into<String>>(self, text: T) -> CallbackPayload<T> {
        CallbackPayload { text, kind: self }
      }
    }

    pub trait Callbacks {
      $(async fn $fn_name(self, $($($arg: $tt),*)?) -> Result<()>;)*
    }

    impl Callback {
      pub async fn execute<C: Callbacks>(self, executor: C) -> Result<()> {
        // info!(target: "callback", "executing: {:?}; executor: {}", self, executor.executor());
        // match self {
          // $(Callback::$name$(($($arg),*))? => executor.$fn_name($($($arg),*)?).await?),*
        // }
        Ok(())
      }
    }

  };
}

callbacks! {
  Test => test
}
