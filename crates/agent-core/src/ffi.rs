//! C ABI surface consumed by the auto-generated Qt plugin shim.
//!
//! Conventions (match `wallet-ffi` style for consistency):
//! - All functions return `AgentResult` (0 = success).
//! - Handles are opaque pointers; the caller owns them and must call `agent_destroy`.
//! - Strings are NUL-terminated UTF-8; the caller frees strings returned to it
//!   via `agent_free_string`.
//! - Byte arrays come as `(uint8_t*, size_t)` pairs.

use crate::runtime::Runtime;
use std::ffi::{c_char, c_int, CStr, CString};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentResult {
    Success = 0,
    NullPointer = 1,
    InvalidUtf8 = 2,
    NotInitialized = 3,
    ConfigError = 4,
    RuntimeError = 5,
    UpstreamError = 6,
    SkillNotFound = 7,
    ApprovalRequired = 8,
    ApprovalDenied = 9,
    InternalError = 99,
}

/// Opaque handle to a running Agent instance.
#[repr(C)]
pub struct AgentHandle {
    _private: [u8; 0],
}

/// Return the library version as a NUL-terminated string. The caller MUST
/// free the returned pointer with `agent_free_string`.
#[no_mangle]
pub extern "C" fn agent_version() -> *mut c_char {
    CString::new(crate::VERSION)
        .expect("static version string never contains NUL")
        .into_raw()
}

/// Initialize the Tokio runtime + tracing. Must be called once before any
/// other agent_* function (idempotent — re-init is a no-op).
#[no_mangle]
pub extern "C" fn agent_init_runtime() -> AgentResult {
    if let Err(e) = Runtime::init_global() {
        tracing::error!("init_runtime failed: {e}");
        return AgentResult::InternalError;
    }
    AgentResult::Success
}

/// Create a new agent from a TOML config path.
///
/// `config_path` — path to `~/.logos-agent/config.toml` (or equivalent).
/// `out_handle` — receives an opaque pointer; must be destroyed with `agent_destroy`.
///
/// # Safety
///
/// - `config_path` MUST be a valid NUL-terminated UTF-8 string pointer.
/// - `out_handle` MUST point to writable memory for a `*mut AgentHandle`.
/// - The returned handle MUST be freed by exactly one call to `agent_destroy`.
#[no_mangle]
pub unsafe extern "C" fn agent_create(
    config_path: *const c_char,
    out_handle: *mut *mut AgentHandle,
) -> AgentResult {
    if config_path.is_null() || out_handle.is_null() {
        return AgentResult::NullPointer;
    }
    let path = match CStr::from_ptr(config_path).to_str() {
        Ok(s) => s,
        Err(_) => return AgentResult::InvalidUtf8,
    };
    match Runtime::create_agent(path) {
        Ok(agent) => {
            *out_handle = Box::into_raw(Box::new(agent)) as *mut AgentHandle;
            AgentResult::Success
        }
        Err(e) => {
            tracing::error!("create failed: {e}");
            AgentResult::ConfigError
        }
    }
}

/// Destroy an agent and free its resources. Safe to call with a null pointer.
///
/// # Safety
///
/// - `handle` MUST have been produced by `agent_create`, or be null.
/// - After this call, `handle` MUST NOT be used again.
/// - MUST be called at most once per handle.
#[no_mangle]
pub unsafe extern "C" fn agent_destroy(handle: *mut AgentHandle) {
    if handle.is_null() {
        return;
    }
    let agent = Box::from_raw(handle as *mut crate::runtime::Agent);
    drop(agent);
}

/// Invoke a skill by name with a JSON-serialized params object.
///
/// `out_result_json` — on success, receives a malloc'd NUL-terminated JSON
/// string. The caller MUST free it with `agent_free_string`.
///
/// # Safety
///
/// - `handle` MUST be a live handle from `agent_create`.
/// - `skill_name` and `params_json` MUST be valid NUL-terminated UTF-8 pointers.
/// - `out_result_json` MUST point to writable memory for a `*mut c_char`.
/// - On success, the caller MUST free `*out_result_json` via `agent_free_string`.
#[no_mangle]
pub unsafe extern "C" fn agent_invoke_skill(
    handle: *mut AgentHandle,
    skill_name: *const c_char,
    params_json: *const c_char,
    out_result_json: *mut *mut c_char,
) -> AgentResult {
    if handle.is_null()
        || skill_name.is_null()
        || params_json.is_null()
        || out_result_json.is_null()
    {
        return AgentResult::NullPointer;
    }
    let name = match CStr::from_ptr(skill_name).to_str() {
        Ok(s) => s,
        Err(_) => return AgentResult::InvalidUtf8,
    };
    let params = match CStr::from_ptr(params_json).to_str() {
        Ok(s) => s,
        Err(_) => return AgentResult::InvalidUtf8,
    };
    let agent = &*(handle as *const crate::runtime::Agent);

    match agent.invoke_blocking(name, params) {
        Ok(json) => {
            *out_result_json = CString::new(json).unwrap_or_default().into_raw();
            AgentResult::Success
        }
        Err(crate::runtime::InvokeError::NotFound) => AgentResult::SkillNotFound,
        Err(crate::runtime::InvokeError::ApprovalRequired) => AgentResult::ApprovalRequired,
        Err(crate::runtime::InvokeError::ApprovalDenied) => AgentResult::ApprovalDenied,
        Err(e) => {
            tracing::error!("invoke {name} failed: {e}");
            *out_result_json = ptr::null_mut();
            AgentResult::RuntimeError
        }
    }
}

/// List all registered skills as a JSON array. Caller MUST free with
/// `agent_free_string`.
///
/// # Safety
///
/// - `handle` MUST be a live handle from `agent_create`.
/// - `out_json` MUST point to writable memory for a `*mut c_char`.
/// - Caller MUST free `*out_json` via `agent_free_string`.
#[no_mangle]
pub unsafe extern "C" fn agent_list_skills(
    handle: *mut AgentHandle,
    out_json: *mut *mut c_char,
) -> AgentResult {
    if handle.is_null() || out_json.is_null() {
        return AgentResult::NullPointer;
    }
    let agent = &*(handle as *const crate::runtime::Agent);
    let manifests = agent.list_skills();
    let json = serde_json::to_string(&manifests).unwrap_or_else(|_| "[]".into());
    *out_json = CString::new(json).unwrap_or_default().into_raw();
    AgentResult::Success
}

/// Report agent status (balance, storage usage, active tasks). JSON.
///
/// # Safety
///
/// - `handle` MUST be a live handle from `agent_create`.
/// - `out_json` MUST point to writable memory for a `*mut c_char`.
/// - Caller MUST free `*out_json` via `agent_free_string`.
#[no_mangle]
pub unsafe extern "C" fn agent_status(
    handle: *mut AgentHandle,
    out_json: *mut *mut c_char,
) -> AgentResult {
    if handle.is_null() || out_json.is_null() {
        return AgentResult::NullPointer;
    }
    let agent = &*(handle as *const crate::runtime::Agent);
    let status = agent.status_blocking();
    let json = serde_json::to_string(&status).unwrap_or_else(|_| "{}".into());
    *out_json = CString::new(json).unwrap_or_default().into_raw();
    AgentResult::Success
}

/// Free a string previously returned by an agent_* function.
///
/// # Safety
///
/// - `s` MUST have been returned by an `agent_*` function from this library,
///   or be null.
/// - MUST be called at most once per returned string.
/// - After this call, `s` MUST NOT be used again.
#[no_mangle]
pub unsafe extern "C" fn agent_free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

/// Return the count of currently pending owner-approval requests.
///
/// # Safety
///
/// - `handle` MUST be a live handle from `agent_create`, or null.
/// - Null handle returns `-1` (not `0`, which is a valid count).
#[no_mangle]
pub unsafe extern "C" fn agent_pending_approvals(handle: *mut AgentHandle) -> c_int {
    if handle.is_null() {
        return -1;
    }
    let agent = &*(handle as *const crate::runtime::Agent);
    agent.pending_approval_count() as c_int
}
