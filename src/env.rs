use super::*;

use once_cell::sync::Lazy;
static ECHO_PREFIX: Lazy<String> = Lazy::new(|| echo::prefix("env"));

pub fn set_current_dir(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    echo!(
        &*ECHO_PREFIX,
        "set_current_dir".bold().cyan(),
        path.to_string_lossy().bold().underline(),
    );
    #[cfg(feature = "tracing")]
    tracing::info!(?path, "set_current_dir");
    Ok(std::env::set_current_dir(path).map_err(echo::error)?)
}

pub fn set_var(key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) {
    let key = key.as_ref();
    let value = value.as_ref();
    echo!(
        &*ECHO_PREFIX,
        "set_var".bold().cyan(),
        key.to_string_lossy().bold().underline(),
        "=".bright_black(),
        value.to_string_lossy().bold().underline(),
    );
    #[cfg(feature = "tracing")]
    tracing::info!(?key, ?value, "set_var");
    std::env::set_var(key, value);
}
