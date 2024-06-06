use ollama_workflows::{Executor, Model, ProgramMemory, Workflow};

#[tokio::main]
async fn main() {
    let exe = Executor::new(Model::Phi3Medium);
    let workflow = Workflow::new_from_json("/Users/kayaomers/Documents/firstbatch/ollama-workflows/my_workflows/search.json").unwrap();
    let mut memory = ProgramMemory::new();
    println!("Executing workflow");
    exe.execute(workflow, &mut memory).await;
}
