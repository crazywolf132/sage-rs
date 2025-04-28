use clap::Args;
use sage_stack::CreateChildOpts;
use sage_core::port::git::GitExecutor;
use crate::cmd::Runtime;

#[derive(Args)]
pub struct Child {
    pub name: String,
}

impl Child {
    pub fn run(self, rt: &mut Runtime) -> anyhow::Result<()> {
        let opts = CreateChildOpts { name: self.name.parse()? };
        
        // Use the Runtime's create_child method
        let acts = rt.create_child(opts)?;
        
        rt.git.run_actions(&acts)?;
        println!("Child branch created");
        Ok(())
    }
}
