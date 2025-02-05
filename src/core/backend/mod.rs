use std::fmt::Debug;

pub use cpu::CPUBackend;

use super::fields::m31::BaseField;
use super::fields::qm31::SecureField;
use super::fields::FieldOps;
use super::poly::circle::PolyOps;

pub mod avx512;
pub mod cpu;

pub trait Backend:
    Copy + Clone + Debug + FieldOps<BaseField> + FieldOps<SecureField> + PolyOps
{
}

pub trait ColumnOps<T> {
    type Column: Column<T>;
    fn bit_reverse_column(column: &mut Self::Column);
}

pub type Col<B, T> = <B as ColumnOps<T>>::Column;

// TODO(spapini): Consider removing the generic parameter and only support BaseField.
pub trait Column<T>: Clone + Debug + FromIterator<T> {
    /// Creates a new column of zeros with the given length.
    fn zeros(len: usize) -> Self;
    /// Returns a cpu vector of the column.
    fn to_vec(&self) -> Vec<T>;
    /// Returns the length of the column.
    fn len(&self) -> usize;
    /// Returns true if the column is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Retrieves the element at the given index.
    fn at(&self, index: usize) -> T;
}
