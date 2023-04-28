use crate::error::ArrTooSmall;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};
use std::slice::IterMut;

#[cfg(test)]
mod test;

/// A Vec but entirely on the stack.
///
/// # Example
/// ```
/// use vector_array::vec::VecArray;
///
/// let mut vec: VecArray<_, 10> = VecArray::new();
/// vec.push(9).unwrap();
/// assert_eq!(vec[0], 9);
/// ```
#[derive(Clone)]
pub struct VecArray<T, const CAP: usize> {
    arr: [T; CAP],
    len: usize,
}

pub struct IntoIter<T, const CAP: usize> {
    arr: [T; CAP],
    len: usize,
    itr: usize,
}

#[derive(Clone)]
pub struct Iter<'a, T> {
    arr: &'a [T],
    itr: usize,
}

/// Does the same as ::new
impl<T, const CAP: usize> Default for VecArray<T, CAP>
where
    T: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const CAP: usize> VecArray<T, CAP>
where
    T: Default,
{
    /// Initializes all elements with defaults (does not increment length)
    ///
    /// # Example
    /// ```
    /// use vector_array::vec::VecArray;
    ///
    /// let mut vec: VecArray<_, 10> = VecArray::new();
    /// vec.push(9).unwrap();
    /// assert_eq!(vec[0], 9);
    /// ```
    ///
    /// Use ::new_no_default if type doesn't implement default
    ///
    pub fn new() -> Self {
        let mut slf = Self::new_no_default();
        slf.arr
            .iter_mut()
            .for_each(|x| unsafe { std::ptr::write(x as *mut T, Default::default()) });
        slf
    }
}

impl<T, const CAP: usize> VecArray<T, CAP> {
    /// Creates a new VecArray. Use ::new if type has default especially if type contains pointers/references (think String, Box, etc)
    ///
    /// # Example
    /// ```
    /// use vector_array::vec::VecArray;
    ///
    /// let mut vec: VecArray<_, 10> = VecArray::new_no_default();
    /// vec.push(9).unwrap();
    /// assert_eq!(vec[0], 9);
    /// ```
    ///
    /// # Safety
    /// There may be problems if you try to index in to parts of the array which are no yet initialized but this is nearly impossible.
    ///
    #[allow(clippy::uninit_assumed_init)]
    pub fn new_no_default() -> Self {
        Self {
            arr: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    pub fn new_arr(arr: [T; CAP], len: usize) -> Self {
        Self { arr, len }
    }

    /// Pushes an element.
    ///
    /// # Example
    /// ```
    /// use vector_array::vec::VecArray;
    ///
    /// let mut vec: VecArray<_, 10> = VecArray::new();
    /// vec.push(9).unwrap();
    /// assert_eq!(vec[0], 9);
    /// ```
    pub fn push(&mut self, value: T) -> Result<(), ArrTooSmall> {
        if self.len < CAP {
            unsafe {
                std::ptr::write(&mut self.arr[self.len] as *mut T, value);
            }
            self.len += 1;
            Ok(())
        } else {
            Err(ArrTooSmall)
        }
    }

    /// Removes the last element
    ///
    /// # Example
    /// ```
    /// use vector_array::vec::VecArray;
    ///
    /// let mut vec: VecArray<_, 10> = VecArray::new();
    /// vec.push(9).unwrap();
    /// assert_eq!(vec.pop(), Some(9));
    /// ```
    ///
    /// # Safety
    /// Returns memory which will realistically wont be used anymore
    ///
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(unsafe { std::ptr::read(&self.arr[self.len] as *const T) })
        }
    }

    /// Removes an element.
    ///
    /// # Panics
    /// If index is greater than or equal to length
    ///
    /// # Example
    /// ```
    /// use vector_array::vec::VecArray;
    /// let mut vec: VecArray<_, 10> = VecArray::new();
    /// vec.push(9).unwrap();
    /// vec.remove(0);
    /// assert!(vec.is_empty());
    /// ```
    ///
    /// # Safety
    /// Copied from Vec source code
    ///
    pub fn remove(&mut self, index: usize) -> T {
        let len = self.len;
        if index >= len {
            panic!("Removal index (is {index}) should be < len (is {len})");
        }

        // infallible
        let ret;
        unsafe {
            // the place we are taking from.
            let ptr = self.arr.as_mut_ptr().add(index);
            // copy it out, unsafely having a copy of the value on
            // the stack and in the vector at the same time.
            ret = std::ptr::read(ptr);

            // Shift everything down to fill in that spot.
            std::ptr::copy(ptr.add(1), ptr, len - index - 1);
        }
        self.len -= 1;
        ret
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            Some(&self.arr[index])
        }
    }

    pub fn set(&mut self, index: usize, value: T) -> Result<(), ArrTooSmall> {
        if index >= self.len {
            Err(ArrTooSmall)
        } else {
            self.arr[index] = value;
            Ok(())
        }
    }

    pub fn truncate(&mut self, len: usize) {
        if len > self.len {
            return;
        }
        self.len = len;
    }

    pub fn last(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            Some(&self.arr[self.len - 1])
        }
    }

    pub fn first(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            Some(&self.arr[0])
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            arr: &self.arr[..self.len],
            itr: 0,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.arr[..self.len].iter_mut()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.arr.as_mut_ptr()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.arr.as_ptr()
    }

    #[inline]
    /// Returns the entire array
    ///
    /// # Safety
    /// Can point to uninitialized memory, causes a segfault if memory is not properly initialized
    ///
    pub unsafe fn get_arr(self) -> [T; CAP] {
        self.arr
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.arr[..self.len]
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.arr[..self.len]
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        CAP
    }
}

impl<T, const CAP: usize> From<VecArray<T, CAP>> for Vec<T> {
    fn from(val: VecArray<T, CAP>) -> Self {
        let mut vec = Vec::from(val.arr);
        vec.truncate(val.len);
        vec
    }
}

impl<T, const CAP: usize> Index<usize> for VecArray<T, CAP> {
    type Output = T;

    /// # Panics
    /// If index is greater than or equal to length
    ///
    /// Use .get instead
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("Index too big");
        } else {
            &self.arr[index]
        }
    }
}

impl<T, const CAP: usize> IndexMut<usize> for VecArray<T, CAP> {
    /// # Panics
    /// If index is greater than or equal to length
    ///
    /// Use .set instead
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("Index too big");
        } else {
            &mut self.arr[index]
        }
    }
}

impl<T, const CAP: usize> From<Vec<T>> for VecArray<T, CAP>
where
    T: Default,
{
    /// # Panics
    /// If inputs length is greater than CAP
    ///
    fn from(value: Vec<T>) -> Self {
        if value.len() > CAP {
            panic!("Vector too long");
        } else {
            let mut slf = Self::new();
            for x in value {
                slf.push(x).unwrap();
            }
            slf
        }
    }
}

impl<T, const CAP: usize> IntoIterator for VecArray<T, CAP> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item, CAP>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            arr: self.arr,
            len: self.len,
            itr: 0,
        }
    }
}

impl<T, const CAP: usize> Iterator for IntoIter<T, CAP> {
    type Item = T;

    /// # Safety
    /// Is not unsafe because value wont be visited again
    ///
    fn next(&mut self) -> Option<Self::Item> {
        if self.itr >= self.len {
            None
        } else {
            let ret = Some(unsafe { std::ptr::read(&self.arr[self.itr] as *const T) });
            self.itr += 1;
            ret
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.itr >= self.arr.len() {
            None
        } else {
            let ret = Some(&self.arr[self.itr]);
            self.itr += 1;
            ret
        }
    }
}

impl<T, const CAP: usize> Debug for VecArray<T, CAP>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vec = (0..self.len).map(|i| &self.arr[i]).collect::<Vec<_>>();
        if f.alternate() {
            write!(f, "{vec:#?}")
        } else {
            write!(f, "{vec:?}")
        }
    }
}

impl<T, const CAP: usize> PartialEq for VecArray<T, CAP>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            false
        } else {
            self.arr[..self.len] == other.arr[..other.len]
        }
    }
}

/// Creates a VecArray just like the vec! macro
#[macro_export()]
macro_rules! vec_arr {
    () => { VecArray::new() };
    ($($x:expr),+ $(,)?) => {
        {
            let mut temp_vec = VecArray::new();
            $(
                temp_vec.push($x).expect(&format!("VecArray to small, (used in macro vec_arr! at line {})", line!()));
            )*
            temp_vec
        }
    };
	($x:expr; $n:literal) => {
		{
            let mut temp_vec = VecArray::new();
			for i in 0..$n => {
                temp_vec.push($x.clone()).expect(&format!("VecArray to small, (used in macro vec_arr! at line {})", line!()));
			}
			temp_vec
		}
	}
}
