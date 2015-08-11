use std::marker::PhantomData;
use std::mem::transmute;

pub struct CArray<T: Sized> (PhantomData<T>);

impl<T: Sized> CArray<T> {
	#[inline]
	pub unsafe fn get_unchecked(&self, index: usize) -> &T {
		transmute(transmute::<&Self, *const T>(self).offset(index as isize))
	}

	#[inline]
	pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
		transmute(transmute::<&mut Self, *mut T>(self).offset(index as isize))
	}
}
