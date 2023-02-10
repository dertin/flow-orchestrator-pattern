struct ReturnCallback {
    status: bool,
    return_to_flow: String,
    return_to_task: u64,
}

fn main() {

    /*
    let context = 0;
    flow_a(context);
    std::process::exit(0);
    */

    const MAX_ITERATION: u64 = 5;
    
    let mut i = 0;
    
    let mut active_flow = "A".to_string();
    let mut active_task = 1;

    let mut context = "999";

    println!("Init context: {context}");

    let mut callback: ahash::AHashMap<String, Vec<ReturnCallback>> = ahash::AHashMap::new();

    loop {
        i += 1;
        println!("{i}");
        if active_flow == "A" {
            
            if active_task == 1 {
                
                // Functional code for this task
                println!("  Flow A - Task 01!");
                if i > MAX_ITERATION {
                    context = "200"; // this will cause flow B to exit the loop and be able to respond with its callback to flow A
                    
                }

                // Arrows with conditions for next task
                if context == "300" {
                    active_flow = "A".to_string();
                    active_task = 3;
                } else {
                    active_flow = "A".to_string();
                    active_task = 2;
                }

                // End of task with arrows to continue
                continue;

            } else if active_task == 2 {

                // Functional code for this task
                println!("  Flow A - Task 02!");
                
                // Check if the invoked flow has already returned a finished status
                // If I have a status of finished I can continue with the evaluation of the next task
                let next = wait_callback(&mut callback, "B".to_string());
                if next {
                    println!("  Flow A - wait_callback()");

                    // Arrows with conditions for next task
                    if true {
                        active_flow = "A".to_string();
                        active_task = 3;
                    }

                    // End of task with arrows to continue
                    continue;
                    
                }
                println!("  Flow A - register_callback()");
                // First callback register to return when the invoked flow finishes
                register_callback(&mut callback, "B".to_owned(), "A".to_string(), 2);
                // Called the flow
                active_flow = "B".to_string();
                active_task = 1;

                // End of task with call to flow to continue
                continue;

            } else if active_task == 3 {

                // Functional code for this task
                println!("  Flow A - Task 03!");
                context = "ok";
                
                // No more condition arrows to continue                
                // before the end of the program. check if there are pending callbacks to return.
                if let Some(last_callback) = execute_callback(&mut callback, "A".to_string()) {
                    println!("  Flow A - execute_callback()");

                    active_flow = last_callback.0;
                    active_task = last_callback.1;

                    // End of task with pending callback to continue
                    continue;
                }
                // End of flow.
                break;
            }

        } else if active_flow == "B" {

            if active_task == 1 {
                
                // Functional code for this task
                println!("  Flow B - Task 01!");

                // Arrows with conditions for next task
                if context == "200" {
                    active_flow = "B".to_string();
                    active_task = 3;
                } else {
                    active_flow = "B".to_string();
                    active_task = 2;
                }
                
                // End of task with arrows to continue
                continue;

            } else if active_task == 2 {

                // Functional code for this task
                println!("  Flow B - Task 02!");
            
                // Check if the invoked flow has already returned a finished status
                // If I have a status of finished I can continue with the evaluation of the next task
                let next = wait_callback(&mut callback, "A".to_string());
                if next {
                    println!("  Flow B - wait_callback()");

                    // Arrows with conditions for next task
                    if context == "400" {
                        active_flow = "B".to_string();
                        active_task = 3;
                    } else {
                        active_flow = "B".to_string();
                        active_task = 4;
                    }
                    
                    // End of task with arrows to continue
                    continue;
                }
                println!("  Flow B - register_callback()");
                // First callback register to return when the invoked flow finishes
                register_callback(&mut callback, "A".to_owned(), "B".to_string(), 2);
                // Called the flow
                active_flow = "A".to_string();
                active_task = 1;

                // End of task with call to flow to continue
                continue;
            } else if active_task == 3 {
                
                // Functional code for this task
                println!("  Flow B - Task 03!");
                context = "kkkk";

                // No more condition arrows to continue                
                // before the end of the program. check if there are pending callbacks to return.
                if let Some(last_callback) = execute_callback(&mut callback, "B".to_string()) {
                    println!("  Flow B - execute_callback()");

                    active_flow = last_callback.0;
                    active_task = last_callback.1;

                    // End of task with pending callback to continue
                    continue;
                }
                // End of flow.
                break;
            } else if active_task == 4 {
                
                // Functional code for this task
                println!("  Flow B - Task 04!");
                context = "kkkk";

                // Arrows with conditions for next task
                if true {
                    active_flow = "B".to_string();
                    active_task = 5;
                }
                
                // End of task with arrows to continue
                continue;

            } else if active_task == 5 {
                
                // Functional code for this task
                println!("  Flow B - Task 05!");
                context = "kkkk";

                // No more condition arrows to continue                
                // before the end of the program. check if there are pending callbacks to return.
                if let Some(last_callback) = execute_callback(&mut callback, "B".to_string()) {
                    println!("  Flow B - execute_callback()");

                    active_flow = last_callback.0;
                    active_task = last_callback.1;

                    // End of task with pending callback to continue
                    continue;
                }
                // End of flow.
                break;
            }
        }

        break; // end
    }

    println!("End context: {context}!");
}

fn wait_callback(callback: &mut ahash::AHashMap<String, Vec<ReturnCallback>>, flow: String) -> bool {
    // Check if the invoked flow has already returned a finished status.
    // If I have a status of finished I can continue with the evaluation of the next task.
    if let Some(callback_flow) = callback.get_mut(&flow) {
        if let Some(last_callback) = callback_flow.last() {
            if last_callback.status {
                callback_flow.pop();
                return true;
            }
        }
    };

    false
}

fn register_callback(callback: &mut ahash::AHashMap<String, Vec<ReturnCallback>>, flow: String, return_to_flow: String, return_to_task: u64) {
    // Called the flow
    // callback register to return when the invoked flow finishes.
    match callback.get_mut(&flow) {
        Some(callback_flow) => {
            callback_flow.push(ReturnCallback {
                status: false,
                return_to_flow,
                return_to_task,
            });
        }
        None => {
            callback.insert(
                flow,
                vec![ReturnCallback {
                    status: false,
                    return_to_flow,
                    return_to_task,
                }],
            );
        }
    }
}

fn execute_callback(callback: &mut ahash::AHashMap<String, Vec<ReturnCallback>>, flow: String) -> Option<(String, u64)> {
    
    if let Some(callback_flow) = callback.get_mut(&flow) {
        if let Some(last_callback) = callback_flow.last_mut() {
            last_callback.status = true;
            return Some((last_callback.return_to_flow.to_string(), last_callback.return_to_task));
        }
    }

    None    
}

/* 
fn flow_a_task_1(){
    println!("  Flow A - Task 01!");
}
fn flow_a_task_2(context: u64){
    println!("  Flow A - Task 02!");
    flow_b(context);

}
fn flow_a_task_3(){
    println!("  Flow A - Task 03!");
}
fn flow_b_task_1(){
    println!("  Flow B - Task 01!");
    
}
fn flow_b_task_2(context: u64){
    println!("  Flow B - Task 02!");
    flow_a(context);
}
fn flow_b_task_3(){
    println!("  Flow B - Task 03!");
}
fn flow_b_task_4(){
    println!("  Flow B - Task 04!");
}
fn flow_b_task_5(){
    println!("  Flow B - Task 05!");
}

fn flow_a(mut context: u64){
    flow_a_task_1();
    
    context += 1;

    if context == 500000 {
        flow_a_task_3();
    }else{
        flow_a_task_2(context);
        flow_a_task_3();
    }
}
fn flow_b(context: u64){

    flow_b_task_1();

    if context == 3 {
        flow_b_task_3();
    }else{
        flow_b_task_2(context);
        if context == 500000 {
            flow_b_task_3();
        }else{
            flow_b_task_4();
            flow_b_task_5();
        }
    }
}
*/