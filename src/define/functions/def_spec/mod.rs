mod mclass;
mod supsub;
mod symbols_op;
mod symbols_ord;

use super::public::FunctionDefSpec;
use std::sync::Mutex;

lazy_static! {
    pub static ref FUNCS: Mutex<Vec<FunctionDefSpec>> = Mutex::new({
        let x = mclass::MCLASS.lock().unwrap();
        let y = symbols_ord::MATHORD.lock().unwrap();
        let b = symbols_ord::TEXTORD.lock().unwrap();
        let z = symbols_op::ATOM.lock().unwrap();
        let a = supsub::SUPSUB.lock().unwrap();
        let res = vec![x.clone(), y.clone(), z.clone(), a.clone(), b.clone()];
        res
    });
}
