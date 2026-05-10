use napi::bindgen_prelude::JsValuesTupleIntoVec;
use napi::threadsafe_function::ThreadsafeFunction;
use napi::{Error, Status};
use std::sync::Arc;

pub type JsCallback<Args = ()> = Arc<ThreadsafeFunction<Args, (), Args, Status, false, true>>;

pub trait JsCallbackExt<Args> {
  fn invoke(&self, args: Args) -> Result<(), Error>;
}

impl<Args> JsCallbackExt<Args> for JsCallback<Args>
where
  Args: 'static + Send + JsValuesTupleIntoVec,
{
  fn invoke(&self, args: Args) -> Result<(), Error> {
    let status = self.call(args, napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking);
    if status != Status::Ok {
      return Err(Error::from_status(status));
    }
    Ok(())
  }
}
