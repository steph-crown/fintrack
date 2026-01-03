use std::fs;
use tempfile::TempDir;
use fintrack::GlobalContext;

pub struct TestContext {
    #[allow(dead_code)]
    pub temp_dir: TempDir,
    pub gctx: GlobalContext,
}

impl TestContext {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let gctx = GlobalContext::new(temp_dir.path().to_path_buf());

        // Create the base directory
        fs::create_dir_all(gctx.base_path()).expect("Failed to create base directory");

        Self { temp_dir, gctx }
    }

    pub fn gctx_mut(&mut self) -> &mut GlobalContext {
        &mut self.gctx
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}
