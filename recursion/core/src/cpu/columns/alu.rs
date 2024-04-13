use sp1_derive::AlignedBorrow;

#[derive(AlignedBorrow, Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct AluCols<T> {
    _phantom: std::marker::PhantomData<T>,
}
