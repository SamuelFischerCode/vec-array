#![warn(clippy::all)]

use crate::error::ArrTooSmall;
use std::fmt::{Debug, Formatter};
use std::ops::Index;
use std::vec::IntoIter;

#[cfg(test)]
mod test;

/// A Vec but entirely on the stack
#[derive(Clone)]
pub struct VecArray<T, const CAP: usize> {
    arr: [T; CAP],
    len: usize,
}

impl<T, const CAP: usize> Default for VecArray<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const CAP: usize> VecArray<T, CAP> {
    pub fn new() -> Self {
        Self {
            arr: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), ArrTooSmall> {
        if self.len < CAP {
            self.arr[self.len] = value;
            self.len += 1;
            Ok(())
        } else {
            Err(ArrTooSmall)
        }
    }

    #[allow(clippy::uninit_assumed_init)]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                Some(std::ptr::read(&self.arr[self.len] as *const T))
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        let len = self.len;
        if index >= len {
            panic!("Removal index (is {index}) should be < len (is {len})");
        }
        unsafe {
            // infallible
            let ret;
            {
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
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            Some(&self.arr[index])
        }
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
    pub fn get_arr(self) -> [T; CAP] {
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

    /// Can panic.
    /// Use .get instead
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("Index too big");
        } else {
            &self.arr[index]
        }
    }
}

impl<T, const CAP: usize> From<Vec<T>> for VecArray<T, CAP> {
    /// Can panic
    /// Can have a failed unwrap but is highly unlikely unless ::new is broken
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
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(self).into_iter()
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
            for i in 0..self.len {
                if self.arr[i] != other.arr[i] {
                    return false;
                }
            }
            true
        }
    }
}

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
                temp_vec.push($x).expect(&format!("VecArray to small, (used in macro vec_arr! at line {})", line!()));
			}
			temp_vec
		}
	}
}
