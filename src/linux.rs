use std::env;
use std::mem;
use std::process::Command;
use std::path::PathBuf;
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
pub fn cache_dir()      -> Option<PathBuf> { env::var_os("XDG_CACHE_HOME") .and_then(is_absolute_path).or_else(|| home_dir().map(|h| h.join(".cache"))) }
pub fn config_dir()     -> Option<PathBuf> { env::var_os("XDG_CONFIG_HOME").and_then(is_absolute_path).or_else(|| home_dir().map(|h| h.join(".config"))) }
pub fn data_dir()       -> Option<PathBuf> { env::var_os("XDG_DATA_HOME")  .and_then(is_absolute_path).or_else(|| home_dir().map(|h| h.join(".local/share"))) }
pub fn data_local_dir() -> Option<PathBuf> { data_dir().clone() }
pub fn runtime_dir()    -> Option<PathBuf> { env::var_os("XDG_RUNTIME_DIR").and_then(is_absolute_path) }
pub fn executable_dir() -> Option<PathBuf> {
    env::var_os("XDG_BIN_HOME").and_then(is_absolute_path).or_else(|| {
        data_dir().map(|mut e| { e.pop(); e.push("bin"); e })
    })
}
pub fn audio_dir()      -> Option<PathBuf> { run_xdg_user_dir_command("MUSIC") }
pub fn desktop_dir()    -> Option<PathBuf> { run_xdg_user_dir_command("DESKTOP") }
pub fn document_dir()   -> Option<PathBuf> { run_xdg_user_dir_command("DOCUMENTS") }
pub fn download_dir()   -> Option<PathBuf> { run_xdg_user_dir_command("DOWNLOAD") }
pub fn font_dir()       -> Option<PathBuf> { data_dir().map(|d| d.join("fonts")) }
pub fn picture_dir()    -> Option<PathBuf> { run_xdg_user_dir_command("PICTURES") }
pub fn public_dir()     -> Option<PathBuf> { run_xdg_user_dir_command("PUBLICSHARE") }
pub fn template_dir()   -> Option<PathBuf> { run_xdg_user_dir_command("TEMPLATES") }
pub fn video_dir()      -> Option<PathBuf> { run_xdg_user_dir_command("VIDEOS") }

// we don't need to explicitly handle empty strings in the code above,
// because an empty string is not considered to be a absolute path here.
fn is_absolute_path(path: OsString) -> Option<PathBuf> {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        Some(path)
    } else {
        None
    }
}

fn run_xdg_user_dir_command(arg: &str) -> Option<PathBuf> {
    use std::os::unix::ffi::OsStringExt;
    let mut out = match Command::new("xdg-user-dir").arg(arg).output() {
    Ok(output) => output.stdout,
        Err(_) => return None,
    };
    let out_len = out.len();
    out.truncate(out_len - 1);
    Some(PathBuf::from(OsString::from_vec(out)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_file_user_dirs_exists() {
        let user_dirs_file = ::config_dir().unwrap().join("user-dirs.dirs");
        println!("{:?} exists: {:?}", user_dirs_file, user_dirs_file.exists());
    }
}
