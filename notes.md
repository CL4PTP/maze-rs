# Notes

*
	is there any way to alias the
	```
	ToPrimitive + FromPrimitive + Integer + Copy
	```
	part of
	```
	pub unsafe fn get<T>(&self, index: T) -> u8
		where T: ToPrimitive + FromPrimitive + Integer + Copy
	```
	so that I don't have to copy it everywhere?

* lots of `as usize`
 