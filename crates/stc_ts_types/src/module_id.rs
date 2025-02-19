use std::sync::Arc;

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use stc_visit::Visit;
use swc_common::{collections::AHashMap, EqIgnoreSpan, FileName, TypeEq};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EqIgnoreSpan, TypeEq, Visit, Serialize, Deserialize)]
pub struct ModuleId(u32);

impl ModuleId {
    pub const fn builtin() -> Self {
        ModuleId(0)
    }

    pub fn is_builtin(self) -> bool {
        self.0 == 0
    }
}

#[derive(Default)]
pub struct ModuleIdGenerator {
    cache: Mutex<Data>,
}

#[derive(Default)]
struct Data {
    cur: u32,
    modules: AHashMap<Arc<FileName>, ModuleId>,
    paths: AHashMap<ModuleId, Arc<FileName>>,
}

impl ModuleIdGenerator {
    pub fn generate(&self, path: &Arc<FileName>) -> ModuleId {
        let mut data = self.cache.lock();
        if let Some(v) = data.modules.get(path) {
            return *v;
        }

        data.cur += 1;

        let module_id = ModuleId(data.cur);
        let res = data.modules.insert(path.clone(), module_id);
        data.paths.insert(module_id, path.clone());

        debug_assert_eq!(res, None, "Found multiple module id for one file");

        module_id
    }

    pub fn path(&self, module_id: ModuleId) -> Arc<FileName> {
        self.cache.lock().paths.get(&module_id).cloned().unwrap()
    }
}
