

// use std::env;
// use std::process;
// use minigrep;
mod smart_pointer;
mod dref_trait;

fn main() {
    // dref trait call
    dref_trait::d_ref_trait_fun();
    let m_box = dref_trait::MyBox{data:12};
    println!("data {}",*m_box);
    // // smart_pointer::box_smart_pointer();
    // smart_pointer::add_and_view_data_in_linked_list::<i32>();
    //
    // minigrep::extra_learn();
    //
    // let args: Vec<String> = env::args().collect();
    //
    // let config = minigrep::Config::new(&args).unwrap_or_else(|_err|{
    //     panic!("{}",_err)
    // });
    //
    //
    // if let Err(e) = minigrep::run(config){
    //     println!("Application Error {e}");
    //     process::exit(1);
    //
    // };

}

