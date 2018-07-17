use std::path::PathBuf;
use std::ffi::OsString;

pub fn home_dir()       -> Option<PathBuf> { None }
pub fn cache_dir()      -> Option<PathBuf> { None }
pub fn config_dir()     -> Option<PathBuf> { None }
pub fn data_dir()       -> Option<PathBuf> { None }
pub fn data_local_dir() -> Option<PathBuf> { None }
pub fn runtime_dir()    -> Option<PathBuf> { None }
pub fn executable_dir() -> Option<PathBuf> { None }
pub fn audio_dir()      -> Option<PathBuf> { None }
pub fn desktop_dir()    -> Option<PathBuf> { None }
pub fn document_dir()   -> Option<PathBuf> { None }
pub fn download_dir()   -> Option<PathBuf> { None }
pub fn font_dir()       -> Option<PathBuf> { None }
pub fn picture_dir()    -> Option<PathBuf> { None }
pub fn public_dir()     -> Option<PathBuf> { None }
pub fn template_dir()   -> Option<PathBuf> { None }
pub fn video_dir()      -> Option<PathBuf> { None }

// we don't need to explicitly handle empty strings in the code above,
// because an empty string is not considered to be a absolute path here.
fn is_absolute_path(_path: OsString) -> Option<PathBuf> {
    None
}

fn run_xdg_user_dir_command(arg: &str) -> Option<PathBuf> {
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn home_dir_should_return_none() {
        assert!(::home_dir().is_none());
    }
}
