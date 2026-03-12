pub(crate) mod array;

use std::sync::Mutex;
use crate::define::functions::public::FunctionDefSpec;
use std::collections::HashMap;
use crate::define::functions::public::FunctionSpec;

lazy_static! {
    pub static ref ENVS: Mutex<Vec<FunctionDefSpec>> = Mutex::new({
        let a = array::ARRAY.lock().unwrap();
        let b = array::ARRAY_ALIGN.lock().unwrap();
        let c = array::ARRAY_ALIGN_AT.lock().unwrap();
        let d = array::MATRIX_ENVS.lock().unwrap();
        let e = array::SMALLMATRIX.lock().unwrap();
        let f = array::SUBARRAY.lock().unwrap();
        let g = array::GATHER.lock().unwrap();
        let h = array::EQUATION.lock().unwrap();
        let i = array::CASES.lock().unwrap();
        let j = array::CD.lock().unwrap();
        vec![
            a.clone(),
            b.clone(),
            c.clone(),
            d.clone(),
            e.clone(),
            f.clone(),
            g.clone(),
            h.clone(),
            i.clone(),
            j.clone()
        ]
    });
    pub static ref _environments: std::sync::RwLock<HashMap<String, FunctionSpec>> =
        std::sync::RwLock::new({
            let mut res = HashMap::new();
            for data in ENVS.lock().unwrap().iter() {
                for name in data.names.iter() {
                    res.insert(name.clone(), (data.props.clone(), data.handler));
                }
            }
            res
        });
}
