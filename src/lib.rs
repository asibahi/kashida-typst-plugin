use wasm::*;
initiate_protocol!();

macro_rules! err {
    ($e: literal) => {
        return Err(String::from($e));
    };
}

#[wasm_func]
pub fn add_kashida(text: &[u8], count: &[u8]) -> Result<Vec<u8>, String> {
    let Ok(text) = String::from_utf8(text.to_owned()) else {
        err!("invalid utf8");
    };

    // smart type inference
    let Ok(count) = count.try_into() else {
        err!("invalid number");
    };
    let count = usize::from_le_bytes(count);

    let ret = kashida::find_kashidas(&text, kashida::Script::Arabic);
    let ret = kashida::place_kashidas(&text, &ret, count);
    let ret = ret.into_owned().into_bytes();

    Ok(ret)
}
