
#[derive(Debug)]
enum List{
    Cons(i32,Box<List>),
    Nil
}

use std::clone::Clone as OtherClone;
use List::{Cons, Nil};

#[derive(PartialEq,Debug)]
struct Node<T>{
    data:Option<T>,
    node:Option<Box<Node<T>>>,
}

struct Vector2d {
    x:f64,
    y:f64
}

trait Copy{
    fn copy(&self)->Self;
}

impl Copy for Vector2d{
    fn copy(&self)-> Self{
        Self{
            x:self.x.clone(),
            y:self.y.clone(),
        }
    }
}

// trait Clone<T>{
//     fn clone(&self);
// }





impl<T> Node<T> {
    fn new(data:T) -> Node<T> {
        Node{
            data:Some(data),
            node: None
        }
    }
}

pub fn _box_smart_pointer(){
    let l1 = Cons(12,Box::new(Cons(11,Box::new(Cons(23,Box::new(Nil))))));
   dbg!(l1);
}

pub fn add_and_view_data_in_linked_list<T>()
where T:PartialEq
{
    let mut node = Node::new(10);
    // let mut last_node = node;





    // _view_linked_list_data::<T>(node)

}

fn _view_linked_list_data<T>(list: Node<i32>)
where T:PartialEq{




    let mut l1 = &(list.node);

    if l1.as_ref() == None{
        return
    }

    while l1.as_ref() != None{
        println!("{:?}", l1.as_ref().unwrap().data);

        l1 = & l1.as_ref().unwrap().node;
    };


}