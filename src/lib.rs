extern crate bytes;
extern crate prost;
extern crate prost_types;
#[macro_use]
extern crate prost_derive;

pub mod google {
  pub mod spanner {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/google.spanner.v1.rs"));
    }
  }
}
