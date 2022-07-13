mod tsfn_fixed;
use napi::{
    threadsafe_function::{ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode},
    Env, JsFunction,
};
use napi_derive::napi;
use tsfn_fixed::TsfnFixed;

#[napi]
pub fn leaking_func(env: Env, func: JsFunction) -> napi::Result<()> {
    let mut tsfn: ThreadsafeFunction<String> =
        func.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<String>| {
            ctx.env
                .create_string_from_std(ctx.value)
                .map(|js_string| vec![js_string])
        })?;

    tsfn.clone();
    tsfn.unref(&env)?;
    tsfn.call(Ok("foo".into()), ThreadsafeFunctionCallMode::Blocking);

    Ok(())
}

#[napi]
pub fn fixed_func(env: Env, func: JsFunction) -> napi::Result<()> {
    let mut tsfn = TsfnFixed::new(env, func)?;

    tsfn.unref(&env)?;

    tsfn.call("foo".into())?;

    Ok(())
}
