use soroban_sdk::{contractimpl, env, Address, Bytes, BytesN, Env, IntoVal, Symbol};

#[derive(Clone)]
struct Task {
    description: String,
    completed: bool,
}

pub struct TodoContract;

#[contractimpl]
impl TodoContract {
    pub fn add_task(e: Env, owner: Address, description: String) {
        let task_id = get_next_task_id(&e, &owner);

        e.data().set(task_id, Task {
            description,
            completed: false
        });

        increment_task_count(&e, &owner);
    }

    pub fn toggle_task(e: Env, task_id: Bytes) {
        let mut task = e.data().get::<Task>(task_id).unwrap().unwrap();
        task.completed = !task.completed;
        e.data().set(task_id, task);
    }

    pub fn delete_task(e: Env, task_id: Bytes) {
        e.data().remove(task_id);
        decrement_task_count(&e, &e.contract_data().get_owner().unwrap());
    }

    pub fn list_tasks(e: Env, owner: Address) -> Vec<(Bytes, Task)> {
        let mut tasks = Vec::new();
        let prefix = get_task_prefix(&owner);

        for i in 0.. {
            let key = BytesN::from_array(&e, &[prefix, i]);
            if let Ok(Some(value)) = e.data().get::<Task>(key.clone()) {
                tasks.push((key, value));
            } else {
            
                break;
            }
        }

        tasks
    }
}


fn get_next_task_id(e: &Env, owner: &Address) -> Bytes {
    let prefix = get_task_prefix(&owner);
    let task_count = e.data().get(prefix).unwrap_or(Ok(0)).unwrap();
    BytesN::from_array(&e, &[prefix, task_count])
}

fn increment_task_count(e: &Env, owner: &Address) {
    let prefix = get_task_prefix(&owner);
    let count = e.data().get::<i32>(prefix).unwrap_or(Ok(0)).unwrap() + 1;
    e.data().set(prefix, &count);
}

fn decrement_task_count(e: &Env, owner: &Address) {
    let prefix = get_task_prefix(&owner);
    let count = e.data().get::<i32>(prefix).unwrap_or(Ok(0)).unwrap() - 1;
    e.data().set(prefix, &count);
}

fn get_task_prefix(owner: &Address) -> Symbol {
    Symbol::from_str(&format!("tasks:{}", owner))
}
