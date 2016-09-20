extern crate core;

use core::iter::Iterator;

pub struct Splits<'l>{
	begin: &'l str,
	end  : &'l str
}

impl<'l> Splits<'l>{
	pub fn from_str(str: &'l str) -> Self{
		Splits{
			begin: unsafe{str.slice_unchecked(0,0)},
			end  : str
		}
	}
}

impl<'l> Iterator for Splits<'l>{
	type Item = (&'l str,&'l str);

	fn next(&mut self) -> Option<<Self as Iterator>::Item>{
		self.end.chars().next().map(|head|{
			let head_len = head.len_utf8();

			self.begin   = unsafe{self.begin.slice_unchecked(0,self.begin.len() + head_len)};
			self.end     = unsafe{self.end.slice_unchecked(head_len,self.end.len())};

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



#[test]
fn test_splits(){
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
fn test_splits_empty(){
	let mut splits = Splits::from_str("");
	assert!(splits.next().is_none());
}


#[test]
fn test_splits_single(){
	let mut splits = Splits::from_str("a");
	/*01*/assert_eq!(splits.next().unwrap(),("a",""));
	/*02*/assert!(splits.next().is_none());
}

#[test]
fn test_splits_char(){
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
fn test_splits_char_empty(){
	let mut splits = SplitsChar::from_str("");
	assert_eq!(splits.next(),None);
}

#[test]
fn test_splits_char_single(){
	let mut splits = SplitsChar::from_str("a");
	/*01*/assert_eq!(splits.next(),Some(("",'a',"")));
	/*02*/assert_eq!(splits.next(),None);
}

#[test]
fn test_splits_char_two(){
	let mut splits = SplitsChar::from_str("ab");
	/*01*/assert_eq!(splits.next(),Some(("",'a',"b")));
	/*02*/assert_eq!(splits.next(),Some(("a",'b',"")));
	/*03*/assert_eq!(splits.next(),None);
}

#[test]
fn test_splits_char_three(){
	let mut splits = SplitsChar::from_str("abc");
	/*01*/assert_eq!(splits.next(),Some(("",'a',"bc")));
	/*02*/assert_eq!(splits.next(),Some(("a",'b',"c")));
	/*03*/assert_eq!(splits.next(),Some(("ab",'c',"")));
	/*04*/assert_eq!(splits.next(),None);
}
