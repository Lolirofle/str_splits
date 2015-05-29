#![feature(str_char)]

//TODO: Rewrite to iterators

pub fn splits<'l,T,F: FnMut(&'l str,&'l str) -> Option<T>>(s: &'l str,mut search: F) -> Option<T>{
	let mut begin = "";
	let mut end   = s;

	loop{
		if end.is_empty(){
			return None
		}

		let head_len = end.char_at(0).len_utf8();
		begin = unsafe{s.slice_unchecked(0,begin.len() + head_len)};
		end = unsafe{end.slice_unchecked(head_len,end.len())};

		if let Some(data) = search(begin,end){
			return Some(data);
		}
	}
}

pub fn splits_char<'l,T,F: FnMut(char,&'l str,&'l str) -> Option<T>>(s: &'l str,mut search: F) -> Option<T>{
	let mut begin = "";
	let mut end   = s;

	loop{
		if end.is_empty(){
			return None
		}

		let c = end.char_at(0);
		let c_len = c.len_utf8();

		end   = unsafe{end.slice_unchecked(c_len,end.len())};

		if let Some(data) = search(c,begin,end){
			return Some(data);
		}

		begin = unsafe{s.slice_unchecked(0,begin.len() + c_len)};
	}
}

#[test]
fn test_splits(){
	let mut i = 0;
	assert!(splits("0123456789",|begin,end|{
		match i{
			0 => {assert_eq!(begin,"0");assert_eq!(end,"123456789");},
			1 => {assert_eq!(begin,"01");assert_eq!(end,"23456789");},
			2 => {assert_eq!(begin,"012");assert_eq!(end,"3456789");},
			3 => {assert_eq!(begin,"0123");assert_eq!(end,"456789");},
			4 => {assert_eq!(begin,"01234");assert_eq!(end,"56789");},
			5 => {assert_eq!(begin,"012345");assert_eq!(end,"6789");},
			6 => {assert_eq!(begin,"0123456");assert_eq!(end,"789");},
			7 => {assert_eq!(begin,"01234567");assert_eq!(end,"89");},
			8 => {assert_eq!(begin,"012345678");assert_eq!(end,"9");},
			9 => {assert_eq!(begin,"0123456789");assert_eq!(end,"");},
			_ => unreachable!()
		}
		i+=1;
		Some(())
	}).is_some());
}

#[test]
fn test_splits_char(){
	let mut i = 0;
	assert!(splits_char("0123456789",|c,begin,end|{
		match i{
			0 => {assert_eq!(begin,"");assert_eq!(c,'0');assert_eq!(end,"123456789");},
			1 => {assert_eq!(begin,"0");assert_eq!(c,'1');assert_eq!(end,"23456789");},
			2 => {assert_eq!(begin,"01");assert_eq!(c,'2');assert_eq!(end,"3456789");},
			3 => {assert_eq!(begin,"012");assert_eq!(c,'3');assert_eq!(end,"456789");},
			4 => {assert_eq!(begin,"0123");assert_eq!(c,'4');assert_eq!(end,"56789");},
			5 => {assert_eq!(begin,"01234");assert_eq!(c,'5');assert_eq!(end,"6789");},
			6 => {assert_eq!(begin,"012345");assert_eq!(c,'6');assert_eq!(end,"789");},
			7 => {assert_eq!(begin,"0123456");assert_eq!(c,'7');assert_eq!(end,"89");},
			8 => {assert_eq!(begin,"01234567");assert_eq!(c,'8');assert_eq!(end,"9");},
			9 => {assert_eq!(begin,"012345678");assert_eq!(c,'9');assert_eq!(end,"");},
			_ => unreachable!()
		}
		i+=1;
		Some(())
	}).is_some());
}
