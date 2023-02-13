pub(crate) mod array;

use std::sync::Mutex;
use crate::define::functions::public::FunctionDefSpec;
use std::collections::HashMap;
use crate::define::functions::public::FunctionSpec;

lazy_static! {
    pub static ref ENVS: Mutex<Vec<FunctionDefSpec>> = Mutex::new({
        let a = array::ARRAY.lock().unwrap();
        vec![
            a.clone()
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
