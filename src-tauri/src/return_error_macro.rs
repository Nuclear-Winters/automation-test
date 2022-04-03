///A very simple macro that will continue the loop if a function fails
///
/// Thanks to this stackoverflow question for the macro https://stackoverflow.com/questions/49785136/is-there-a-shortcut-to-unwrap-or-continue-in-a-loop
///This macro has been modified for this project.
#[macro_export]
macro_rules! return_error {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                return Err(e.to_string());
            }
        }
    };
}

///A modification of the above macro, it generates code that puts errors into the finished tasks list
#[macro_export]
macro_rules! return_task_error {
    ($function:expr,$state:expr,$name:expr) => {
        match $function {
            Ok(val) => val,
            Err(e) => {
                let name = $name.clone();
                let mut task_lock = $state.completed_tasks.write().unwrap();
                if task_lock.contains_key(&name) {
                    task_lock.get_mut(&name).unwrap().push(e.to_string());
                }
                else {
                    task_lock.insert(name,vec![e.to_string()]);
                }
                return;
            }
        }
    };
}
