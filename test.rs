// TODO: Move all of these to automated grammar specs in spec/rust-spec.coffee

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

// Lifetimes in associated type definitions (#55)
trait Foo {
    type B: A + 'static;
}
