#[cfg(all(feature = "tokio-runtime", feature = "async-std"))]
compile_error!(
    "feature \"tokio-runtime\" and feature \"async-std\" cannot be enabled at the same time"
);

mod cacache_serde;
mod cacache_serde_tidy;
mod error;

pub use cacache_serde::CacacheSerde;
pub use cacache_serde_tidy::SerdeCacacheTidy;
pub use error::Error;
