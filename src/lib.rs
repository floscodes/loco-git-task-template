use loco_rs::prelude::*;

pub struct LocoGitTask;
#[async_trait]
impl Task for LocoGitTask {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "loco-git-task".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Task LocoGitTask generated");
        Ok(())
    }
}
