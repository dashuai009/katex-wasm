mod accent;
mod accentunder;
mod arrow;
mod cr;
mod enclose;
mod genfrac;
mod mclass;
mod ordgroup;
mod supsub;
mod symbols_op;
mod symbols_ord;
mod horiz_brace;
mod href;
mod text;
mod hbox;

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
        let h1 = enclose::COLOR_BOX.lock().unwrap();
        let h2 = enclose::FCOLOR_BOX.lock().unwrap();
        let h3 = enclose::FBOX.lock().unwrap();
        let h4 = enclose::CANCEL.lock().unwrap();
        let h5 = enclose::ANGL.lock().unwrap();
        let i1 = genfrac::FRAC.lock().unwrap();
        let j = horiz_brace::HORIZBRACE.lock().unwrap();
        let k1 = href::HREF.lock().unwrap();
        let k2 = href::URL.lock().unwrap();
        let l = text::TEXT.lock().unwrap();
        let m = hbox::HBOX.lock().unwrap();
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
            g.clone(),
            h1.clone(),
            h2.clone(),
            h3.clone(),
            h4.clone(),
            h5.clone(),
            i1.clone(),
            j.clone(),
            k1.clone(),
            k2.clone(),
            l.clone(),
            m.clone()
        ];
        res
    });
}
