use sentience::SentienceAgent;
use tokio::task;

use crate::api::handlers::{LONG_MEM, SHORT_MEM};

pub struct SentienceWrapper {
    pub inner: SentienceAgent,
}

impl SentienceWrapper {
    pub fn new() -> Self {
        SentienceWrapper {
            inner: SentienceAgent::new(),
        }
    }

    pub fn load_program(&mut self, program: &str) -> Result<(), String> {
        self.inner.run_sentience(program)?;
        Ok(())
    }

    fn seed_short(&mut self) {
        let global = SHORT_MEM.get().unwrap();
        if let Some(map) = global.all() {
            for (k, v) in map.into_iter() {
                self.inner.set_short(&k, &v);
            }
        }
    }

    fn seed_long(&mut self) {
        let global = LONG_MEM.get().unwrap();

        let all_vec: Vec<(String, String)> = task::block_in_place(|| {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(global.all())
        });

        for (k, v) in all_vec.into_iter() {
            self.inner.set_long(&k, &v);
        }
    }

    fn flush_short(&self) {
        let global = SHORT_MEM.get().unwrap();
        let all_map = self.inner.all_short();
        for (k, v) in all_map.into_iter() {
            global.set(k, v);
        }
    }

    fn flush_long(&self) {
        let global = LONG_MEM.get().unwrap();
        let all_map = self.inner.all_long();
        for (k, v) in all_map.into_iter() {
            task::block_in_place(|| {
                let rt = tokio::runtime::Handle::current();
                rt.block_on(global.set(&k, &v))
            });
        }
    }

    pub fn handle_code(&mut self, code: &str) -> Result<String, String> {
        self.seed_short();
        self.seed_long();

        // run_sentience is synchronous, so call directly
        let res = self.inner.run_sentience(code);

        self.flush_short();
        self.flush_long();

        res
    }
}
