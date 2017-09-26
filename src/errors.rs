use clap;

error_chain! {
  types {
    Error, ErrorKind, ResultExt;
  }

  foreign_links {
    Clap(clap::Error);
  }

  errors {
    // Internal{message: String} {
    //   description("internal error")
    // }
  }
}

impl Error {
  pub fn code(&self) -> i32 {
    use exit_code::*;
    match *self.kind() {
      ErrorKind::Clap(ref clap_error) => {
        match clap_error.kind {
          clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed => SUCCESS,
          _ => USAGE_ERROR,
        }
      },
      _ => FAILURE,
    }
  }
}

/*
/// Construct internal error
pub fn internal_error<S>(message: S) -> Error 
  where S: Display
{
  ErrorKind::Internal{message: internal_error_message(message)}.into()
}
*/
