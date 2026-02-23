use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use snc_core::distill::distill_chat_turn as core_distill_chat_turn;
use snc_core::orchestration::run_with_hgo_and_humanshell as core_run_with_hgo_and_humanshell;

fn json_in(ptr: *const c_char) -> Result<String, String> {
    if ptr.is_null() {
        return Err("null pointer for JSON input".into());
    }
    unsafe {
        CStr::from_ptr(ptr)
            .to_str()
            .map(|s| s.to_owned())
            .map_err(|e| format!("invalid UTF-8: {e}"))
    }
}

fn json_out(s: String) -> *mut c_char {
    CString::new(s).expect("CString::new failed").into_raw()
}

#[no_mangle]
pub extern "C" fn distill_chat_turn(json_input: *const c_char) -> *mut c_char {
    let result = (|| -> Result<String, String> {
        let input = json_in(json_input)?;
        core_distill_chat_turn(&input)
    })();

    let wrapped = match result {
        Ok(ok) => format!(r#"{{"ok":true,"data":{ok}}}"#),
        Err(err) => format!(r#"{{"ok":false,"error":{:?}}}"#, err),
    };
    json_out(wrapped)
}

#[no_mangle]
pub extern "C" fn orchestrate_with_hgo_and_shell(
    hgo_json: *const c_char,
    context_json: *const c_char,
    sovereignty_state_json: *const c_char,
    discipline_signals_json: *const c_char,
    fate_deck_seed_json: *const c_char,
) -> *mut c_char {
    let result = (|| -> Result<String, String> {
        let hgo = json_in(hgo_json)?;
        let ctx = json_in(context_json)?;
        let sov = json_in(sovereignty_state_json)?;
        let sig = json_in(discipline_signals_json)?;
        let seed = json_in(fate_deck_seed_json)?;
        core_run_with_hgo_and_humanshell(&hgo, &ctx, &sov, &sig, &seed)
    })();

    let wrapped = match result {
        Ok(ok) => format!(r#"{{"ok":true,"data":{ok}}}"#),
        Err(err) => format!(r#"{{"ok":false,"error":{:?}}}"#, err),
    };
    json_out(wrapped)
}

/// Must be called by Kotlin once it is done with a string from Rust.
#[no_mangle]
pub extern "C" fn snc_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        CString::from_raw(ptr); // drops and frees
    }
}
