use std::ops::Deref;

pub struct MyBox<T>{
    pub data:T
}

// impl<T> MyBox<T>{
//     pub fn new(&self)->MyBox<T>{
//         self.clone()
//     }
// }

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

pub fn d_ref_trait_fun(){
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

}