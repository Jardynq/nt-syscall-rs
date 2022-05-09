#[cfg(feature = "windows_11_21h2")]
pub use super::invalid as windows_11_21h2;

#[cfg(feature = "windows_10_21h2")]
pub use super::invalid as windows_10_21h2;

#[cfg(feature = "windows_10_21h1")]
pub use super::invalid as windows_10_21h1;

#[cfg(feature = "windows_10_20h2")]
pub mod windows_10_20h2;

#[cfg(feature = "windows_10_2004")]
pub mod windows_10_2004;

#[cfg(feature = "windows_10_1909")]
pub mod windows_10_1909;

#[cfg(feature = "windows_10_1903")]
pub mod windows_10_1903;

#[cfg(feature = "windows_10_1809")]
pub mod windows_10_1809;

#[cfg(feature = "windows_10_1803")]
pub mod windows_10_1803;

#[cfg(feature = "windows_10_1709")]
pub mod windows_10_1709;

#[cfg(feature = "windows_10_1703")]
pub mod windows_10_1703;

#[cfg(feature = "windows_10_1607")]
pub mod windows_10_1607;

#[cfg(feature = "windows_10_1511")]
pub mod windows_10_1511;

#[cfg(feature = "windows_10_1507")]
pub mod windows_10_1507;

#[cfg(feature = "windows_8_1")]
pub mod windows_8_1;

#[cfg(feature = "windows_8_0")]
pub mod windows_8_0;

#[cfg(feature = "windows_server_2012_r2")]
pub use super::invalid as windows_server_2012_r2;


#[cfg(feature = "windows_server_2012_sp0")]
pub use super::invalid as windows_server_2012_sp0;

#[cfg(feature = "windows_7_sp1")]
pub mod windows_7_sp1;

#[cfg(feature = "windows_7_sp0")]
pub mod windows_7_sp0;

#[cfg(feature = "windows_server_2008_r2_sp1")]
pub use super::invalid as windows_server_2008_r2_sp1;

#[cfg(feature = "windows_server_2008_r2")]
pub use super::invalid as windows_server_2008_r2;

#[cfg(feature = "windows_server_2008_sp2")]
pub mod windows_server_2008_sp2;

#[cfg(feature = "windows_server_2008_sp0")]
pub mod windows_server_2008_sp0;

#[cfg(feature = "windows_vista_sp2")]
pub mod windows_vista_sp2;

#[cfg(feature = "windows_vista_sp1")]
pub mod windows_vista_sp1;

#[cfg(feature = "windows_vista_sp0")]
pub mod windows_vista_sp0;

#[cfg(feature = "windows_server_2003_r2_sp2")]
pub mod windows_server_2003_r2_sp2;

#[cfg(feature = "windows_server_2003_r2")]
pub mod windows_server_2003_r2;

#[cfg(feature = "windows_server_2003_sp2")]
pub mod windows_server_2003_sp2;

#[cfg(feature = "windows_server_2003_sp1")]
pub mod windows_server_2003_sp1;

#[cfg(feature = "windows_server_2003_sp0")]
pub mod windows_server_2003_sp0;

#[cfg(feature = "windows_xp_sp3")]
pub mod windows_xp_sp3;

#[cfg(feature = "windows_xp_sp2")]
pub mod windows_xp_sp2;

#[cfg(feature = "windows_xp_sp1")]
pub mod windows_xp_sp1;

#[cfg(feature = "windows_xp_sp0")]
pub mod windows_xp_sp0;
