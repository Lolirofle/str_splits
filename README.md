splits_iter
-----------
Library for the Rust programming language. Provides iterators for all 2-splits. A 2-split is an operation that splits one sequence `s` into two so that when they are concatenated creates the original sequence (`split_at(s,_)=(s1,s2)` so that `concat(s1,s2)=s`).

## Examples

```rust
let mut i = Splits::from_str("0123456789");
/*01*/assert_eq!(i.next(),Some(("0","123456789")));
/*02*/assert_eq!(i.next(),Some(("01","23456789")));
/*03*/assert_eq!(i.next(),Some(("012","3456789")));
/*04*/assert_eq!(i.next(),Some(("0123","456789")));
/*05*/assert_eq!(i.next(),Some(("01234","56789")));
/*06*/assert_eq!(i.next(),Some(("012345","6789")));
/*07*/assert_eq!(i.next(),Some(("0123456","789")));
/*08*/assert_eq!(i.next(),Some(("01234567","89")));
/*09*/assert_eq!(i.next(),Some(("012345678","9")));
/*10*/assert_eq!(i.next(),Some(("0123456789","")));
/*11*/assert_eq!(i.next(),None);
```

```rust
let mut i = FocusedSplits::from_str("0123456789");
/*01*/assert_eq!(i.next(),Some(("",'0',"123456789")));
/*02*/assert_eq!(i.next(),Some(("0",'1',"23456789")));
/*03*/assert_eq!(i.next(),Some(("01",'2',"3456789")));
/*04*/assert_eq!(i.next(),Some(("012",'3',"456789")));
/*05*/assert_eq!(i.next(),Some(("0123",'4',"56789")));
/*06*/assert_eq!(i.next(),Some(("01234",'5',"6789")));
/*07*/assert_eq!(i.next(),Some(("012345",'6',"789")));
/*08*/assert_eq!(i.next(),Some(("0123456",'7',"89")));
/*09*/assert_eq!(i.next(),Some(("01234567",'8',"9")));
/*10*/assert_eq!(i.next(),Some(("012345678",'9',"")));`
```
