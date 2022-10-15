mod accent;
mod mclass;
mod ordgroup;
mod supsub;
mod symbols_op;
mod symbols_ord;
mod accentunder;
mod arrow;
mod cr;

use super::public::FunctionDefSpec;
use std::sync::Mutex;

lazy_static! {
    pub static ref FUNCS: Mutex<Vec<FunctionDefSpec>> = Mutex::new({
        let x = mclass::MCLASS.lock().unwrap();
        let y = symbols_ord::MATHORD.lock().unwrap();
        let b = symbols_ord::TEXTORD.lock().unwrap();
        let z = symbols_op::ATOM.lock().unwrap();
        let a = supsub::SUPSUB.lock().unwrap();
        let c = accent::ACCENT.lock().unwrap();
        let c2 = accent::ACCENT2.lock().unwrap();
        let d = ordgroup::ORDGROUP.lock().unwrap();
        let e = accentunder::ACCENTUNDER.lock().unwrap();
        let f = arrow::XARROW.lock().unwrap();
        let g = cr::CR.lock().unwrap();
        let res = vec![
            x.clone(),
            y.clone(),
            z.clone(),
            a.clone(),
            b.clone(),
            c.clone(),
            c2.clone(),
            d.clone(),
            e.clone(),
            f.clone(),
            g.clone()
        ];
        res
    });
}
