pub use curve25519_dalek::{
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar,
    scalar::Scalar,
};

pub use crate::spoint::ByteBuffer;

pub type Bytes = Vec<ByteBuffer>;
pub type TPoint = RistrettoPoint;
pub type TScalar = Scalar;
