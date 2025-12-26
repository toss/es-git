/// Source codes are from [rolldown](https://github.com/rolldown/rolldown)
/// See: https://github.com/rolldown/rolldown/blob/fc5ec4dbb8cf7a9bc32f2cba6e0e82eba3ac888d/crates/rolldown_binding/src/types/js_callback.rs#L98
use napi::bindgen_prelude::{FromNapiValue, JsValuesTupleIntoVec};
use napi::threadsafe_function::{ThreadsafeFunction, UnknownReturnValue};
use napi::{Either, Error, Status};
use std::sync::{Arc, Condvar, Mutex};

pub type JsCallback<Args = (), Ret = ()> =
  Arc<ThreadsafeFunction<Args, Either<Ret, UnknownReturnValue>, Args, Status, false, true>>;

pub trait JsCallbackExt<Args, Ret> {
  fn invoke(&self, args: Args) -> Result<Ret, Error>;
}

impl<Args, Ret> JsCallbackExt<Args, Ret> for JsCallback<Args, Ret>
where
  Args: 'static + Send + JsValuesTupleIntoVec,
  Ret: 'static + Send + FromNapiValue,
  Either<Ret, UnknownReturnValue>: FromNapiValue,
{
  fn invoke(&self, args: Args) -> Result<Ret, Error> {
    let init_value = Ok(Either::B(UnknownReturnValue));
    let pair = Arc::new((Mutex::new(init_value), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    self.call_with_return_value(
      args,
      napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
      move |ret, _env| {
        let (lock, cvar) = &*pair;
        *lock.lock().unwrap() = ret;
        cvar.notify_one();
        Ok(())
      },
    );

    let (lock, cvar) = &*pair_clone;
    let notified = lock.lock().unwrap();
    let mut res = cvar
      .wait(notified)
      .map_err(|err| Error::new(Status::GenericFailure, format!("PoisonError: {err:?}",)))?;
    let res = res
      .as_mut()
      .map_err(|err| Error::new(Status::GenericFailure, format!("{err:?}",)))?;

    match std::mem::replace(res, Either::B(UnknownReturnValue)) {
      Either::A(ret) => Ok(ret),
      Either::B(_unknown) => unknown_return_err::<Ret>(),
    }
  }
}

fn unknown_return_err<Ret>() -> Result<Ret, Error> {
  let js_type = "unknown";
  Err(Error::new(
    Status::InvalidArg,
    format!("UNKNOWN_RETURN_VALUE. Cannot convert {js_type}"),
  ))
}
