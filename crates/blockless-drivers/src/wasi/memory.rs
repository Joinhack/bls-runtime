#![allow(non_upper_case_globals)]
use wasi_common::WasiCtx;
use wiggle::GuestPtr;
use crate::{memory_driver, BlocklessMemoryErrorKind};

wiggle::from_witx!({
    witx: ["$BLOCKLESS_DRIVERS_ROOT/witx/blockless_memory.witx"],
    errors: { blockless_memory_error => BlocklessMemoryErrorKind },
    async: *,
    wasmtime: false,
});

  impl From<BlocklessMemoryErrorKind> for types::BlocklessMemoryError {
    fn from(e: BlocklessMemoryErrorKind) -> types::BlocklessMemoryError {
          use types::BlocklessMemoryError;
          match e {
            BlocklessMemoryErrorKind::InvalidHandle => BlocklessMemoryError::InvalidHandle,
            BlocklessMemoryErrorKind::RuntimeError => BlocklessMemoryError::RuntimeError,
            BlocklessMemoryErrorKind::InvalidParameter => BlocklessMemoryError::InvalidParameter,
          }
      }
  }

  impl types::UserErrorConversion for WasiCtx {
    fn blockless_memory_error_from_blockless_memory_error_kind(
        &mut self,
        e: BlocklessMemoryErrorKind,
    ) -> Result<types::BlocklessMemoryError, wiggle::Trap> {
        e.try_into()
            .map_err(|e| wiggle::Trap::String(format!("{:?}", e)))
    }
  }


  impl wiggle::GuestErrorType for types::BlocklessMemoryError {
      fn success() -> Self {
          Self::Success
      }
  }

  #[wiggle::async_trait]
  impl blockless_memory::BlocklessMemory for WasiCtx {
    async fn memory_read<'a>(
      &mut self,
      buf: &GuestPtr<'a, u8>,
      buf_len: u32,
  ) -> Result<u32, BlocklessMemoryErrorKind> {
      let stdin = self.blockless_config.as_ref().unwrap().stdin_ref();
      let mut dest_buf = vec![0; buf_len as _];
      let rs = memory_driver::read(&mut dest_buf, stdin.to_string()).await?;
      if rs > 0 {
        buf.as_array(rs).copy_from_slice(&dest_buf[0..rs as _]).map_err(|_| BlocklessMemoryErrorKind::RuntimeError)?;
      }
      Ok(rs)
  }
}