#[macro_use]
extern crate napi_derive;

use napi::*;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("interpret_lox", interpret_lox)?;

    Ok(())
}

#[js_function(1)]
fn interpret_lox(ctx: CallContext) -> Result<JsUndefined> {
    let code = ctx.get::<JsString>(0)?.into_utf8()?;
    let code = code.as_str()?.to_string();
    lox_compiler::interpret(&code, None);
    ctx.env.get_undefined()
}
