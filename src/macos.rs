use std::env;
use std::path::PathBuf;
use std::mem;
use std::ptr;
use std::ffi::CStr;
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;

extern crate libc;

pub fn home_dir() -> Option<PathBuf> {
    return env::var_os("HOME").and_then(|h| { if h.is_empty() { None } else { Some(h) } } ).or_else(|| unsafe {
        fallback()
    }).map(PathBuf::from);

    unsafe fn fallback() -> Option<OsString> {
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512 as usize,
            n => n as usize,
        };
        let mut buf = Vec::with_capacity(amt);
        let mut passwd: libc::passwd = mem::zeroed();
        let mut result = ptr::null_mut();
        match libc::getpwuid_r(libc::getuid(), &mut passwd, buf.as_mut_ptr(),
                               buf.capacity(), &mut result) {
            0 if !result.is_null() => {
                let ptr = passwd.pw_dir as *const _;
                let bytes = CStr::from_ptr(ptr).to_bytes();
                if bytes.is_empty() {
                    None
                } else {
                    Some(OsStringExt::from_vec(bytes.to_vec()))
                }
            },
            _ => None,
        }
    }
}

pub fn cache_dir()      -> Option<PathBuf> { home_dir().map(|h| h.join("Library/Caches")) }
pub fn config_dir()     -> Option<PathBuf> { home_dir().map(|h| h.join("Library/Preferences")) }
pub fn data_dir()       -> Option<PathBuf> { home_dir().map(|h| h.join("Library/Application Support")) }
pub fn data_local_dir() -> Option<PathBuf> { data_dir() }
pub fn executable_dir() -> Option<PathBuf> { None }
pub fn runtime_dir()    -> Option<PathBuf> { None }
pub fn audio_dir()      -> Option<PathBuf> { home_dir().map(|h| h.join("Music")) }
pub fn desktop_dir()    -> Option<PathBuf> { home_dir().map(|h| h.join("Desktop")) }
pub fn document_dir()   -> Option<PathBuf> { home_dir().map(|h| h.join("Documents")) }
pub fn download_dir()   -> Option<PathBuf> { home_dir().map(|h| h.join("Downloads")) }
pub fn font_dir()       -> Option<PathBuf> { home_dir().map(|h| h.join("Library/Fonts")) }
pub fn picture_dir()    -> Option<PathBuf> { home_dir().map(|h| h.join("Pictures")) }
pub fn public_dir()     -> Option<PathBuf> { home_dir().map(|h| h.join("Public")) }
pub fn template_dir()   -> Option<PathBuf> { None }
pub fn video_dir()      -> Option<PathBuf> { home_dir().map(|h| h.join("Movies")) }
