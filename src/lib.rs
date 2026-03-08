//! Kamon (家紋) — file type icon provider for Neovim with Nerd Font support
//!
//! Part of the blnvim-ng distribution — a Rust-native Neovim plugin suite.
//! Built with [`nvim-oxi`](https://github.com/noib3/nvim-oxi) for zero-cost
//! Neovim API bindings.

use nvim_oxi as oxi;

#[oxi::plugin]
fn kamon() -> oxi::Result<()> {
    Ok(())
}
