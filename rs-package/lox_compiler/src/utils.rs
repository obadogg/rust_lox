pub mod utils {
    use std::rc::Rc;

    pub fn get_rc_ref_address<T>(rc: Rc<T>) -> usize {
        Rc::into_raw(rc.clone()) as usize
    }
}
