use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, Nil};

#[derive(Debug)]
pub enum List<T>{
    Cons(T, RefCell<Rc<List<T>>>),
    Nil,
}

impl <T>List<T>{
    pub fn tail(&self)->Option<&RefCell<Rc<List<T>>>> {
        match self {
            Cons(_, item) => Some(&item),
            Nil => None,
        }
    }
}