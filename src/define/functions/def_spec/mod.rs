mod mclass;
mod symbols_ord;

use super::public::FunctionDefSpec;
use std::sync::Mutex;

lazy_static! {
    pub static ref FUNCS: Mutex<Vec<FunctionDefSpec>> = Mutex::new({
        let x = mclass::MCLASS.lock().unwrap();
        let y = symbols_ord::MATHORD.lock().unwrap();
        let res = vec![x.clone(),y.clone()];
        res
    });
}
