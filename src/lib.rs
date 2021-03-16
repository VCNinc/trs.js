#![no_std]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

//-------- Testing stuff --------//

#[cfg(test)]
mod test_utils;

//-------- Modules and exports--------//

pub mod key;
mod prelude;
pub mod sig;
pub mod trace;

pub use key::*;
pub use sig::*;
pub use trace::*;
