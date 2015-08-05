use std::marker::PhantomData;

struct CArray<T> {
	_: PhantomData<T>
}

impl<T> CArray<T> {
	pub unsafe fn get_unchecked(&self, index: usize) -> &T {
		unsafe { (self as *const T).offset(index).as_ref().unwrap() }
	}

	pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
		unsafe { (self as *mut T).offset(index).as_mut().unwrap() }
	}
}
