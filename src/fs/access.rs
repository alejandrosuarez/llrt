use rquickjs::{prelude::Opt, Ctx, Exception, Result};
use tokio::fs;

use crate::utils::result::ResultExt;

#[allow(dead_code, unused_imports)]
use super::{CONSTANT_F_OK, CONSTANT_R_OK, CONSTANT_W_OK, CONSTANT_X_OK};

pub async fn access(ctx: Ctx<'_>, path: String, mode: Opt<u32>) -> Result<()> {
    let metadata = fs::metadata(path.clone())
        .await
        .or_throw_msg(&ctx, &format!("Can't access file \"{}\"", &path))?;

    let permissions = metadata.permissions();

    let mode = mode.unwrap_or(CONSTANT_F_OK);

    if mode & CONSTANT_W_OK != 0 && permissions.readonly() {
        return Err(Exception::throw_message(
            &ctx,
            "Permission denied. File not writable",
        ));
    }

    if mode & CONSTANT_X_OK != 0 {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if permissions.mode() & 0o100 == 0 {
                return Err(Exception::throw_message(
                    &ctx,
                    "Permission denied. File not executable",
                ));
            }
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;
            // Get the file attributes
            let file_attributes = metadata.file_attributes();

            // Check if the file has execute permissions
            if file_attributes & FILE_ATTRIBUTE_DIRECTORY == 0 {
                return Err(Exception::throw_message(&ctx, "Permission denied"));
            }
        }
    }

    Ok(())
}
