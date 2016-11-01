extern crate core;

use core::iter::Iterator;
use core::slice;

//TODO: RSplits or specialize?

pub struct Splits<Seq>{
	begin: Seq,
	end  : Seq
}

impl<'l> Splits<&'l str>{
	pub fn from_str(str: &'l str) -> Self{
		Splits{
			begin: unsafe{str.slice_unchecked(0,0)},
			end  : str
		}
	}
}

impl<'l> Iterator for Splits<&'l str>{
	type Item = (&'l str,&'l str);

	fn next(&mut self) -> Option<<Self as Iterator>::Item>{
		self.end.chars().next().map(|head|{
			let head_len = head.len_utf8();

			self.begin = unsafe{self.begin.slice_unchecked(0,self.begin.len() + head_len)};
			self.end   = unsafe{self.end.slice_unchecked(head_len,self.end.len())};

			(self.begin,self.end)
		})
	}
}

impl<'l,T> Splits<&'l [T]>{
	pub fn from_slice(slice: &'l [T]) -> Self{
		Splits{
			begin: unsafe{slice::from_raw_parts(slice.as_ptr() , 0)},
			end  : slice
		}
	}
}

impl<'l,T> Iterator for Splits<&'l [T]>{
	type Item = (&'l [T],&'l [T]);

	fn next(&mut self) -> Option<<Self as Iterator>::Item>{
		self.end.split_first().map(|(_,end)|{
			self.begin = unsafe{slice::from_raw_parts(self.begin.as_ptr() , self.begin.len()+1)};
			self.end   = end;

			(self.begin,self.end)
		})
	}
}



pub struct SplitsChar<'l>{
	begin: &'l str,
	end  : &'l str
}

impl<'l> SplitsChar<'l>{
	pub fn from_str(str: &'l str) -> Self{
		SplitsChar{
			begin: unsafe{str.slice_unchecked(0,0)},
			end  : str
		}
	}
}

impl<'l> Iterator for SplitsChar<'l>{
	type Item = (&'l str,char,&'l str);

	fn next(&mut self) -> Option<<Self as Iterator>::Item>{
		self.end.chars().next().map(|c|{
			let c_len = c.len_utf8();

			self.end   = unsafe{self.end.slice_unchecked(c_len,self.end.len())};
			let begin  = self.begin;
			self.begin = unsafe{self.begin.slice_unchecked(0,self.begin.len() + c_len)};

			(begin,c,self.end)
		})
	}
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test{
	use super::*;

	#[test]
	fn Splits_str(){
		let mut splits = Splits::from_str("0123456789");
		/*01*/assert_eq!(splits.next(),Some(("0","123456789")));
		/*02*/assert_eq!(splits.next(),Some(("01","23456789")));
		/*03*/assert_eq!(splits.next(),Some(("012","3456789")));
		/*04*/assert_eq!(splits.next(),Some(("0123","456789")));
		/*05*/assert_eq!(splits.next(),Some(("01234","56789")));
		/*06*/assert_eq!(splits.next(),Some(("012345","6789")));
		/*07*/assert_eq!(splits.next(),Some(("0123456","789")));
		/*08*/assert_eq!(splits.next(),Some(("01234567","89")));
		/*09*/assert_eq!(splits.next(),Some(("012345678","9")));
		/*10*/assert_eq!(splits.next(),Some(("0123456789","")));
		/*11*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn Splits_str_empty(){
		let mut splits = Splits::from_str("");
		assert!(splits.next().is_none());
	}


	#[test]
	fn Splits_str_single(){
		let mut splits = Splits::from_str("a");
		/*01*/assert_eq!(splits.next().unwrap(),("a",""));
		/*02*/assert!(splits.next().is_none());
	}

	#[test]
	fn Splits_slice(){
		let slice = [0u8,1,2,3,4,5,6,7,8,9];
		let mut splits = Splits::from_slice(&slice as &[u8]);
		/*01*/assert_eq!(splits.next(),Some((&[0u8] as &[u8],&[1,2,3,4,5,6,7,8,9u8] as &[u8])));
		/*02*/assert_eq!(splits.next(),Some((&[0u8,1] as &[u8],&[2,3,4,5,6,7,8,9u8] as &[u8])));
		/*03*/assert_eq!(splits.next(),Some((&[0u8,1,2] as &[u8],&[3,4,5,6,7,8,9u8] as &[u8])));
		/*04*/assert_eq!(splits.next(),Some((&[0u8,1,2,3] as &[u8],&[4,5,6,7,8,9u8] as &[u8])));
		/*05*/assert_eq!(splits.next(),Some((&[0u8,1,2,3,4] as &[u8],&[5,6,7,8,9u8] as &[u8])));
		/*06*/assert_eq!(splits.next(),Some((&[0u8,1,2,3,4,5] as &[u8],&[6,7,8,9u8] as &[u8])));
		/*07*/assert_eq!(splits.next(),Some((&[0u8,1,2,3,4,5,6] as &[u8],&[7,8,9u8] as &[u8])));
		/*08*/assert_eq!(splits.next(),Some((&[0u8,1,2,3,4,5,6,7] as &[u8],&[8,9u8] as &[u8])));
		/*09*/assert_eq!(splits.next(),Some((&[0u8,1,2,3,4,5,6,7,8] as &[u8],&[9u8] as &[u8])));
		/*10*/assert_eq!(splits.next(),Some((&[0u8,1,2,3,4,5,6,7,8,9] as &[u8],&[] as &[u8])));
		/*11*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn Splits_slice_empty(){
		let mut splits = Splits::from_slice(&[] as &[u8]);
		assert!(splits.next().is_none());
	}


	#[test]
	fn Splits_slice_single(){
		let slice = [1u8];
		let mut splits = Splits::from_slice(&slice as &[u8]);
		/*01*/assert_eq!(splits.next().unwrap(),(&[1u8] as &[u8],&[] as &[u8]));
		/*02*/assert!(splits.next().is_none());
	}

	#[test]
	fn Splits_units(){
		let slice = [(),(),(),(),()];
		let mut splits = Splits::from_slice(&slice as &[()]);
		/*01*/assert_eq!(splits.next(),Some((&[()] as &[()],&[(),(),(),()] as &[()])));
		/*02*/assert_eq!(splits.next(),Some((&[(),()] as &[()],&[(),(),()] as &[()])));
		/*03*/assert_eq!(splits.next(),Some((&[(),(),()] as &[()],&[(),()] as &[()])));
		/*04*/assert_eq!(splits.next(),Some((&[(),(),(),()] as &[()],&[()] as &[()])));
		/*05*/assert_eq!(splits.next(),Some((&[(),(),(),(),()] as &[()],&[] as &[()])));
		/*06*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn SplitsChar(){
		let mut splits = SplitsChar::from_str("0123456789");
		/*01*/assert_eq!(splits.next(),Some(("",'0',"123456789")));
		/*02*/assert_eq!(splits.next(),Some(("0",'1',"23456789")));
		/*03*/assert_eq!(splits.next(),Some(("01",'2',"3456789")));
		/*04*/assert_eq!(splits.next(),Some(("012",'3',"456789")));
		/*05*/assert_eq!(splits.next(),Some(("0123",'4',"56789")));
		/*06*/assert_eq!(splits.next(),Some(("01234",'5',"6789")));
		/*07*/assert_eq!(splits.next(),Some(("012345",'6',"789")));
		/*08*/assert_eq!(splits.next(),Some(("0123456",'7',"89")));
		/*09*/assert_eq!(splits.next(),Some(("01234567",'8',"9")));
		/*10*/assert_eq!(splits.next(),Some(("012345678",'9',"")));
		/*11*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn SplitsChar_empty(){
		let mut splits = SplitsChar::from_str("");
		assert_eq!(splits.next(),None);
	}

	#[test]
	fn SplitsChar_single(){
		let mut splits = SplitsChar::from_str("a");
		/*01*/assert_eq!(splits.next(),Some(("",'a',"")));
		/*02*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn SplitsChar_two(){
		let mut splits = SplitsChar::from_str("ab");
		/*01*/assert_eq!(splits.next(),Some(("",'a',"b")));
		/*02*/assert_eq!(splits.next(),Some(("a",'b',"")));
		/*03*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn SplitsChar_three(){
		let mut splits = SplitsChar::from_str("abc");
		/*01*/assert_eq!(splits.next(),Some(("",'a',"bc")));
		/*02*/assert_eq!(splits.next(),Some(("a",'b',"c")));
		/*03*/assert_eq!(splits.next(),Some(("ab",'c',"")));
		/*04*/assert_eq!(splits.next(),None);
	}

}
