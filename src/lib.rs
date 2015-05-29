#![feature(core,str_char)]

extern crate core;

use core::iter::Iterator;

pub struct Splits<'l>{
	str  : &'l str,
	begin: &'l str,
	end  : &'l str
}

impl<'l> Splits<'l>{
	pub fn from_str(str: &'l str) -> Self{
		Splits{
			str  : str,
			begin: "",
			end  : str
		}
	}
}

impl<'l> Iterator for Splits<'l>{
	type Item = (&'l str,&'l str);

	fn next(&mut self) -> Option<<Self as Iterator>::Item>{
		if self.end.is_empty(){
			None
		}else{
			let head_len = self.end.char_at(0).len_utf8();
			self.begin = unsafe{self.str.slice_unchecked(0,self.begin.len() + head_len)};
			self.end = unsafe{self.end.slice_unchecked(head_len,self.end.len())};

			Some((self.begin,self.end))
		}
	}
}



pub struct SplitsChar<'l>{
	str  : &'l str,
	begin: &'l str,
	end  : &'l str
}

impl<'l> SplitsChar<'l>{
	pub fn from_str(str: &'l str) -> Self{
		SplitsChar{
			str  : str,
			begin: "",
			end  :  str
		}
	}
}

impl<'l> Iterator for SplitsChar<'l>{
	type Item = (&'l str,char,&'l str);

	fn next(&mut self) -> Option<<Self as Iterator>::Item>{
		if self.end.is_empty(){
			None
		}else{
			let c = self.end.char_at(0);
			let c_len = c.len_utf8();

			self.end = unsafe{self.end.slice_unchecked(c_len,self.end.len())};
			let begin = self.begin;
			self.begin = unsafe{self.str.slice_unchecked(0,self.begin.len() + c_len)};

			Some((begin,c,self.end))
		}
	}
}

#[test]
fn test_splits(){
	let mut splits = Splits::from_str("0123456789");
	/*01*/assert_eq!(splits.next().unwrap(),("0","123456789"));
	/*02*/assert_eq!(splits.next().unwrap(),("01","23456789"));
	/*03*/assert_eq!(splits.next().unwrap(),("012","3456789"));
	/*04*/assert_eq!(splits.next().unwrap(),("0123","456789"));
	/*05*/assert_eq!(splits.next().unwrap(),("01234","56789"));
	/*06*/assert_eq!(splits.next().unwrap(),("012345","6789"));
	/*07*/assert_eq!(splits.next().unwrap(),("0123456","789"));
	/*08*/assert_eq!(splits.next().unwrap(),("01234567","89"));
	/*09*/assert_eq!(splits.next().unwrap(),("012345678","9"));
	/*10*/assert_eq!(splits.next().unwrap(),("0123456789",""));
	/*11*/assert!(splits.next().is_none());
}

#[test]
fn test_splits_char(){
	let mut splits = SplitsChar::from_str("0123456789");
	/*01*/assert_eq!(splits.next().unwrap(),("",'0',"123456789"));
	/*02*/assert_eq!(splits.next().unwrap(),("0",'1',"23456789"));
	/*03*/assert_eq!(splits.next().unwrap(),("01",'2',"3456789"));
	/*04*/assert_eq!(splits.next().unwrap(),("012",'3',"456789"));
	/*05*/assert_eq!(splits.next().unwrap(),("0123",'4',"56789"));
	/*06*/assert_eq!(splits.next().unwrap(),("01234",'5',"6789"));
	/*07*/assert_eq!(splits.next().unwrap(),("012345",'6',"789"));
	/*08*/assert_eq!(splits.next().unwrap(),("0123456",'7',"89"));
	/*09*/assert_eq!(splits.next().unwrap(),("01234567",'8',"9"));
	/*10*/assert_eq!(splits.next().unwrap(),("012345678",'9',""));
	/*11*/assert!(splits.next().is_none());
}
