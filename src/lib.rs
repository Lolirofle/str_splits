extern crate core;

use core::iter::{DoubleEndedIterator,Iterator};
use core::{slice,str};

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct Splits<Seq>{
	begin: Seq,
	end  : Seq
}

impl<'l> Splits<&'l str>{
	#[inline]
	pub fn from_str(str: &'l str) -> Self{
		Splits{
			begin: unsafe{str.slice_unchecked(0,0)},
			end  : str
		}
	}

	#[inline]
	pub fn as_str(self) -> &'l str{
		unsafe{self.begin.slice_unchecked(0,self.begin.len()+self.end.len())}
	}

	/*
	pub fn prev_forward(&mut self) -> Option<<Self as Iterator>::Item>{
		self.begin.chars().next().map(|last|{
			let last_len = last.len_utf8();

			self.begin = unsafe{self.begin.slice_unchecked(last_len,self.begin.len())};

			(self.begin,self.end)
		})
	}

	pub fn next_back(&mut self) -> Option<<Self as Iterator>::Item>{
		self.end.chars().next_back().map(|last|{
			let last_len = last.len_utf8();

			self.end = unsafe{self.end.slice_unchecked(0,self.end.len()-last_len)};

			(self.begin,self.end)
		})
	}
	*/

	pub fn prev(&mut self) -> Option<<Self as Iterator>::Item>{
		self.begin.chars().next_back().map(|last|{
			let last_len = last.len_utf8();

			self.begin = unsafe{self.begin.slice_unchecked(0,self.begin.len()-last_len)};
			self.end   = unsafe{str::from_utf8_unchecked(slice::from_raw_parts(self.end.as_ptr().offset(-(last_len as isize)),self.end.len()+last_len))};

			(self.begin,self.end)
		})
	}

	pub fn first(&mut self) -> Option<<Self as Iterator>::Item>{
		if self.begin==self.end{
			None
		}else{
			self.begin = unsafe{self.begin.slice_unchecked(0,0)};
			self.end   = unsafe{self.begin.slice_unchecked(0,self.begin.len()+self.end.len())};

			Some((self.begin,self.end))
		}
	}

	pub fn last(&mut self) -> Option<<Self as Iterator>::Item>{
		self.end.chars().next_back().map(|_|{
			let len = self.end.len();

			self.begin = unsafe{self.begin.slice_unchecked(0,self.begin.len() + len)};
			self.end   = unsafe{self.end.slice_unchecked(len,len)};

			(self.begin,self.end)
		})
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

	#[inline]
	fn size_hint(&self) -> (usize,Option<usize>){
		let len = self.end.len();
		(len,Some(len))
	}

	#[inline(always)]
	fn last(mut self) -> Option<<Self as Iterator>::Item>{
		(&mut self).last()
	}
}

impl<'l> ExactSizeIterator for Splits<&'l str>{
	#[inline(always)]
	fn len(&self) -> usize{
		self.end.len()
	}
}

impl<'l,T> Splits<&'l [T]>{
	#[inline]
	pub fn from_slice(slice: &'l [T]) -> Self{
		Splits{
			begin: unsafe{slice::from_raw_parts(slice.as_ptr() , 0)},
			end  : slice
		}
	}

	#[inline]
	pub fn as_slice(self) -> &'l [T]{
		unsafe{slice::from_raw_parts(self.begin.as_ptr() , self.begin.len()+self.end.len())}
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

	#[inline]
	fn size_hint(&self) -> (usize,Option<usize>){
		let len = self.end.len();
		(len,Some(len))
	}
}

impl<'l,T> ExactSizeIterator for Splits<&'l [T]>{
	#[inline(always)]
	fn len(&self) -> usize{
		self.end.len()
	}
}



#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct FocusedSplits<Seq>{
	begin: Seq,
	end  : Seq
}

impl<'l> FocusedSplits<&'l str>{
	#[inline]
	pub fn from_str(str: &'l str) -> Self{
		FocusedSplits{
			begin: unsafe{str.slice_unchecked(0,0)},
			end  : str
		}
	}
}

impl<'l,T> FocusedSplits<&'l [T]>{
	#[inline]
	pub fn from_slice(slice: &'l [T]) -> Self{
		FocusedSplits{
			begin: unsafe{slice::from_raw_parts(slice.as_ptr() , 0)},
			end  : slice
		}
	}
}

impl<'l> Iterator for FocusedSplits<&'l str>{
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

impl<'l,T> Iterator for FocusedSplits<&'l [T]>{
	type Item = (&'l [T],&'l T,&'l [T]);

	fn next(&mut self) -> Option<<Self as Iterator>::Item>{
		self.end.split_first().map(|(elem,end)|{
			self.end   = end;
			let begin = self.begin;
			self.begin = unsafe{slice::from_raw_parts(self.begin.as_ptr() , self.begin.len()+1)};

			(begin,elem,self.end)
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
		/*12*/assert_eq!(splits.next(),None);
	}

	/*#[test]
	fn Splits_str_back(){
		let mut splits = Splits::from_str("0123456789");
		/*01<*/assert_eq!(splits.next_back(),Some(("","012345678")));
		/*01>*/assert_eq!(splits.next()     ,Some(("0","12345678")));
		/*02<*/assert_eq!(splits.next_back(),Some(("0","1234567")));
		/*02>*/assert_eq!(splits.next()     ,Some(("01","234567")));
		/*03<*/assert_eq!(splits.next_back(),Some(("01","23456")));
		/*03>*/assert_eq!(splits.next()     ,Some(("012","3456")));
		/*04<*/assert_eq!(splits.next_back(),Some(("012","345")));
		/*04>*/assert_eq!(splits.next()     ,Some(("0123","45")));
		/*05<*/assert_eq!(splits.next_back(),Some(("0123","4")));
		/*05>*/assert_eq!(splits.next()     ,Some(("01234","")));
		/*06<*/assert_eq!(splits.next_back(),None);
		/*06>*/assert_eq!(splits.next()     ,None);
		/*07<*/assert_eq!(splits.next_back(),None);
		/*07>*/assert_eq!(splits.next()     ,None);
	}

	#[test]
	fn Splits_str_prev_forward(){
		let mut splits = Splits::from_str("0123456789");
		/*00 */assert_eq!((&mut splits).last() ,Some(("0123456789","")));
		/*01<*/assert_eq!(splits.prev_forward(),Some(("123456789","")));
		/*01>*/assert_eq!(splits.prev()        ,Some(("12345678","9")));
		/*02<*/assert_eq!(splits.prev_forward(),Some(("2345678","9")));
		/*02>*/assert_eq!(splits.prev()        ,Some(("234567","89")));
		/*03<*/assert_eq!(splits.prev_forward(),Some(("34567","89")));
		/*03>*/assert_eq!(splits.prev()        ,Some(("3456","789")));
		/*04<*/assert_eq!(splits.prev_forward(),Some(("456","789")));
		/*04>*/assert_eq!(splits.prev()        ,Some(("45","6789")));
		/*05<*/assert_eq!(splits.prev_forward(),Some(("5","6789")));
		/*05>*/assert_eq!(splits.prev()        ,Some(("","56789")));
		/*06<*/assert_eq!(splits.prev_forward(),None);
		/*06>*/assert_eq!(splits.prev()        ,None);
		/*07<*/assert_eq!(splits.prev_forward(),None);
		/*07>*/assert_eq!(splits.prev()        ,None);
	}*/

	#[test]
	fn Splits_str_last(){
		assert_eq!(Some(("0123456789","")),Splits::from_str("0123456789").last());
		assert_eq!(Some(("012345678","")),Splits::from_str("012345678").last());
		assert_eq!(Some(("01234567","")),Splits::from_str("01234567").last());
		assert_eq!(Some(("0123456","")),Splits::from_str("0123456").last());
		assert_eq!(Some(("012345","")),Splits::from_str("012345").last());
		assert_eq!(Some(("01234","")),Splits::from_str("01234").last());
		assert_eq!(Some(("0123","")),Splits::from_str("0123").last());
		assert_eq!(Some(("012","")),Splits::from_str("012").last());
		assert_eq!(Some(("01","")),Splits::from_str("01").last());
		assert_eq!(Some(("0","")),Splits::from_str("0").last());
		assert_eq!(None,Splits::from_str("").last());
	}

	#[test]
	fn Splits_str_len(){
		let mut iter = Splits::from_str("0123");
		assert_eq!(4,iter.len());iter.next();
		assert_eq!(3,iter.len());iter.next();
		assert_eq!(2,iter.len());iter.next();
		assert_eq!(1,iter.len());iter.next();
		assert_eq!(0,iter.len());iter.next();
		assert_eq!(0,iter.len());iter.next();

		/*let mut iter = Splits::from_str("0123");
		assert_eq!(4,iter.len());iter.next_back();
		assert_eq!(3,iter.len());iter.next_back();
		assert_eq!(2,iter.len());iter.next_back();
		assert_eq!(1,iter.len());iter.next_back();
		assert_eq!(0,iter.len());iter.next_back();
		assert_eq!(0,iter.len());iter.next_back();*/
	}

	#[test]
	fn Splits_str_empty(){
		let mut splits = Splits::from_str("");
		assert!(splits.next().is_none());
		assert!(splits.next().is_none());
	}


	#[test]
	fn Splits_str_single(){
		let mut splits = Splits::from_str("a");
		/*01*/assert_eq!(splits.next().unwrap(),("a",""));
		/*02*/assert!(splits.next().is_none());
		/*03*/assert!(splits.next().is_none());
	}

	#[test]
	fn Splits_str_original(){
		let mut splits = Splits::from_str("0123456789");
		/*00*/               assert_eq!(splits.clone().as_str(),"0123456789");
		/*01*/splits.next(); assert_eq!(splits.clone().as_str(),"0123456789");
		/*02*/splits.next(); assert_eq!(splits.clone().as_str(),"0123456789");
		/*03*/splits.next(); assert_eq!(splits.clone().as_str(),"0123456789");
		/*04*/splits.next(); assert_eq!(splits.clone().as_str(),"0123456789");
		/*05*/splits.next(); assert_eq!(splits.clone().as_str(),"0123456789");
		/*06*/splits.next(); assert_eq!(splits.clone().as_str(),"0123456789");
		/*07*/splits.next(); assert_eq!(splits.clone().as_str(),"0123456789");
		/*08*/splits.next(); assert_eq!(splits.clone().as_str(),"0123456789");
		/*09*/splits.next(); assert_eq!(splits.clone().as_str(),"0123456789");
		/*10*/splits.next(); assert_eq!(splits.clone().as_str(),"0123456789");
		/*11*/splits.next(); assert_eq!(splits.clone().as_str(),"0123456789");
		/*12*/splits.next(); assert_eq!(splits.clone().as_str(),"0123456789");
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
		/*12*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn Splits_slice_empty(){
		let mut splits = Splits::from_slice(&[] as &[u8]);
		assert!(splits.next().is_none());
		assert!(splits.next().is_none());
	}


	#[test]
	fn Splits_slice_single(){
		let slice = [1u8];
		let mut splits = Splits::from_slice(&slice as &[u8]);
		/*01*/assert_eq!(splits.next().unwrap(),(&[1u8] as &[u8],&[] as &[u8]));
		/*02*/assert!(splits.next().is_none());
		/*03*/assert!(splits.next().is_none());
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
		/*07*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn FocusedSplits_str(){
		let mut splits = FocusedSplits::from_str("0123456789");
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
		/*12*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn FocusedSplits_str_empty(){
		let mut splits = FocusedSplits::from_str("");
		assert_eq!(splits.next(),None);
		assert_eq!(splits.next(),None);
	}

	#[test]
	fn FocusedSplits_str_single(){
		let mut splits = FocusedSplits::from_str("a");
		/*01*/assert_eq!(splits.next(),Some(("",'a',"")));
		/*02*/assert_eq!(splits.next(),None);
		/*03*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn FocusedSplits_str_two(){
		let mut splits = FocusedSplits::from_str("ab");
		/*01*/assert_eq!(splits.next(),Some(("",'a',"b")));
		/*02*/assert_eq!(splits.next(),Some(("a",'b',"")));
		/*03*/assert_eq!(splits.next(),None);
		/*04*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn FocusedSplits_str_three(){
		let mut splits = FocusedSplits::from_str("abc");
		/*01*/assert_eq!(splits.next(),Some(("",'a',"bc")));
		/*02*/assert_eq!(splits.next(),Some(("a",'b',"c")));
		/*03*/assert_eq!(splits.next(),Some(("ab",'c',"")));
		/*04*/assert_eq!(splits.next(),None);
		/*05*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn FocusedSplits_slice(){
		let mut splits = FocusedSplits::from_str("0123456789");
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
		/*12*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn FocusedSplits_slice_empty(){
		let mut splits = FocusedSplits::from_slice(&[] as &[u8]);
		assert_eq!(splits.next(),None);
		assert_eq!(splits.next(),None);
	}

	#[test]
	fn FocusedSplits_slice_single(){
		let slice = [1u8];
		let mut splits = FocusedSplits::from_slice(&slice as &[u8]);
		/*01*/assert_eq!(splits.next(),Some((&[] as &[_],&1,&[] as &[_])));
		/*02*/assert_eq!(splits.next(),None);
		/*03*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn FocusedSplits_slice_two(){
		let slice = [1u8,2];
		let mut splits = FocusedSplits::from_slice(&slice as &[u8]);
		/*01*/assert_eq!(splits.next(),Some((&[] as &[_],&1,&[2u8] as &[_])));
		/*02*/assert_eq!(splits.next(),Some((&[1u8] as &[_],&2,&[] as &[_])));
		/*03*/assert_eq!(splits.next(),None);
		/*04*/assert_eq!(splits.next(),None);
	}

	#[test]
	fn FocusedSplits_slice_three(){
		let slice = [1u8,2,3];
		let mut splits = FocusedSplits::from_slice(&slice as &[u8]);
		/*01*/assert_eq!(splits.next(),Some((&[] as &[_],&1,&[2,3u8] as &[_])));
		/*02*/assert_eq!(splits.next(),Some((&[1u8] as &[_],&2,&[3u8] as &[_])));
		/*03*/assert_eq!(splits.next(),Some((&[1,2u8] as &[_],&3,&[] as &[_])));
		/*04*/assert_eq!(splits.next(),None);
		/*05*/assert_eq!(splits.next(),None);
	}
}
