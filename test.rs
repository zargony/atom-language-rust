text
/* this is a
/* nested
block */ comment.
And should stay a comment
even with "strings"
or 42 0x18 0b01011   // blah
or u32 as i16 if impl */
text
/** this is a
/*! nested
block */ doc comment */
text

text /**/ text
text /***/ text
text /****/ text

text
text // line comment
text /// line doc comment
text //! line doc comment
text

text #![main] text
text #![allow(great_algorithms)] text
text #![!resolve_unexported] text
text #[deny(silly_comments)] text
#[doc = "This attribute contains ] an attribute ending character"]

text r"This is a raw string" text
text r"This raw string ends in \" text
text r#"This is also valid"# text
text r##"This is ##"# also valid."## text
test b"This is a bytestring" text
test br"And a raw byte string" text
text br##"This is ##"# also valid."## text
text r##"Raw strings can
span multiple lines"## text

text "double-quote string" text
text "string\nwith\x20escaped\"characters" text
text "string with // comment /* inside" text
text "strings can
span multiple lines" text

text 'c' text
text '\n' text
text '\x20' text
text '\'' text
text b'b' text
text b'\x20' text

text 42f32 text
text 42is text
text 42i32 text
text 42e+18 text
text 42.1415 text
text 42.1415e18 text
text 42.1415e+18 text

text 42 text
text 0xf00b text
text 0o755 text
text 0b101010 text

text bool text char text usize text isize text
text u8 text u16 text u32 text u64 text
text i8 text i16 text i32 text i64 text
text str text Self text

text true text false text

text break text continue text do text else text
text if text in text for text loop text
text match text return text while text

text as text crate text extern text mod text
text let text proc text ref text

text
extern crate foo;
text
use std::slice;
text
use std::{num, str};
text
use self::foo::{bar, baz};
text

text
pub enum MyEnum {
    One,
    Two
}
text

text
pub struct MyStruct<'foo> {
    pub one: u32,
    two: Option<'a, MyEnum>,
    three: &'foo i32,
}
text

text
type MyType = u32;
text

text
static MY_CONSTANT: &str = "hello";
text

text
pub trait MyTrait {
    text
    fn create_something (param: &str, mut other_param: u32) -> Option<Self>;
    text
    fn do_whatever<T: Send+Share+Whatever, U: Freeze> (param: &T, other_param: u32) -> Option<U>;
    text
    fn do_all_the_work (&mut self, param: &str, mut other_param: u32) -> bool;
    text
    fn do_even_more<'a, T: Send+Whatever, U: Something<T>+Freeze> (&'a mut self, param: &T) -> &'a U;
    text
}
text

text
impl<'foo> MyTrait for MyStruct<'foo> {
    text
    fn create_something (param: &str, mut other_param: u32) -> Option<Self> {
        text
        return Some(cake);
        text
    }
    text
    fn do_whatever<T: Send+Share+Whatever, U: Freeze> (param: &T, other_param: u32) -> Option<U> {
        assert!(1 != 2);
        text
        self.with_something(param, |arg1, arg2| {
            text
        }, other_param);
    }
    text
    fn do_all_the_work (&mut self, param: &str, mut other_param: u32) -> bool {
        announce!("There's no cake");
        if !test_subject.under_control() {
            text
            let list: Vec<item> = some_iterator.map(|elem| elem.dosomething()).collect();
            text
            let boxed_list = box list;
            text
            self.announce_warning();
            text
            if test_subject.ignored_warnings > 3 {
                text
                test_subject.incinerate();
                text
            }
            text
        }
        text
    }
    text
    fn do_even_more<'a, T: Send+Whatever, U: Something<T>+Freeze> (&'a mut self, param: &T) -> &'a U {
        text
        let foo: Option<'a u32> = Some(18);
        text
        if self.one < 1 {
            text
            self.complain(&foo);
            text
        }
    }
    text
}
text

text
impl MyStruct<'foo> {
    text
    pub fn with_something<T: Send> (param: &T, f: |i32, &str| -> T, other_param: u32) -> T {
        text
        f(123, "hello")
        text
    }
    text
}
text

// Loop expression labels (#2)
'infinity: loop {
    do_serious_stuff();
    use_a_letter('Z');
    break 'infinity;
}

// isize/usize suffixes (#22)
let x = 123usize;

// Float literals without +- after E (#30)
let x = 1.2345e6;

// Nested generic (#33, #37)
let x: Vec<Vec<u8>> = Vec::new();

// Correct detection of == (#40)
struct Foo { x: i32 }
if x == 1 { }
