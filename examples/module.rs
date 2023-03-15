

mod core {
    use std::fmt::Display;

    pub trait Todo {
        fn add_task(&mut self, task: Task);
        fn remove_task(&mut self, task_id: u64);
        fn get_all_tasks(&self) -> Vec<Task>;
    }

    impl Display for dyn Todo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln! {
                f,
                "{}",
                self
            }
        }
    }
    pub trait Reminder {
        fn add_reminder(&mut self, reminder: ReminderData);
        fn remove_reminder(&mut self, reminder_id: u64);
        fn get_all_reminders(&self) -> Vec<ReminderData>;
    }

    impl Display for dyn Reminder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln! {
                f,
                "{}",
                self
            }
        }
    }

    #[derive(Clone)]
    pub struct Task {
        pub id: u64,
        pub title: String,
        pub completed: bool,
    }

    impl Display for Task {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln! {
                f,
                "{}",
                self
            }
        }
    }

    #[derive(Clone)]
    pub struct ReminderData {
        pub id: u64,
        pub title: String,
        pub date: String,
        pub completed: bool,
    }

    impl Display for ReminderData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln! {
                    f,
                    "{}",
                    self
            }
        }
    }
}

core!{
    Component : Todo1 {
        Interface :  core::Task,
        Method : [ 
            fn add_task_1(&mut self, task: core::Task);
            // fn remove_task_1(self, task_id:u64);
            // fn get_all_tasks_1(self) return Vec<core::Task>;
            ],
        Display : Todo1
    },
}


// core!{
//     Component : Todo1 {
//         Interface : Task1,
//         Method : [ 
//             fn add_task(&mut self, task:core::Task), 
            // remove_task(u64), 
            // fn get_all_tasks() -> Vec<core::Task>,
//             ],

//     },
// }

#[macro_export]
macro_rules! core {
    ($(
        Component : $component_name:ident {
            $(
                Interface : $Interface:ty,
                $(
                    Method : [ 
                        
                        $(
                            fn $Method:ident($($arg:tt)*) 
                            $(return $($return:ty)*)?;
                        )*
                    
                    ]$(,)?
                    $(Display : $display:tt)?
                )?
            )*

        }$(,)?
),*) => {

    $(
        #[allow(non_snake_case,dead_code,unused_variables)]
        pub trait $component_name {
            $(
                $(
                    $(
                           fn $Method(
                            $($arg)*
                           ) $(-> $($return)*)?
                           ;
                    )*
                )*
            )*
        }
        $(
            $(
                #[allow(non_snake_case,dead_code,unused_variables)]
                impl $component_name for $Interface{
                    $(
                        fn $Method(
                         $($arg)*
                        ) $(-> $($return)*)?
                        {
                            unimplemented!()
                        }
                 )*
                }
            )*
        )*
        
    )*
      
    };
    }


mod service {
    use crate::core::{Reminder, ReminderData, Task, Todo};

    #[derive(Clone)]
    pub struct TodoService<T: Todo> {
        todo_repo: T,
    }

    impl<T: Todo> TodoService<T> {
        pub fn new(todo_repo: T) -> Self {
            TodoService { todo_repo }
        }

        pub fn add_task(&mut self, task: Task) {
            self.todo_repo.add_task(task);
        }

        pub fn remove_task(&mut self, task_id: u64) {
            self.todo_repo.remove_task(task_id);
        }

        pub fn get_all_tasks(&self) -> Vec<Task> {
            self.todo_repo.get_all_tasks()
        }
    }

    #[derive(Clone)]
    pub struct ReminderService<T: Reminder> {
        reminder_repo: T,
    }

    impl<T: Reminder> ReminderService<T> {
        pub fn new(reminder_repo: T) -> Self {
            ReminderService { reminder_repo }
        }

        pub fn add_reminder(&mut self, reminder: ReminderData) {
            self.reminder_repo.add_reminder(reminder);
        }

        pub fn remove_reminder(&mut self, reminder_id: u64) {
            self.reminder_repo.remove_reminder(reminder_id);
        }

        pub fn get_all_reminders(&self) -> Vec<ReminderData> {
            self.reminder_repo.get_all_reminders()
        }
    }
}

mod model {
    #[derive(Clone)]
    pub struct TaskModel {
        pub title: String,
    }

    #[derive(Clone)]
    pub struct ReminderModel {
        pub title: String,
        pub date: String,
    }
}

mod repository {
    use crate::core::{Reminder, ReminderData, Task, Todo};
    use crate::model::{ReminderModel, TaskModel};

    #[derive(Clone)]
    pub struct TodoRepository {
        tasks: Vec<Task>,
    }

    impl TodoRepository {
        pub fn new() -> Self {
            TodoRepository { tasks: Vec::new() }
        }
    }

    impl Todo for TodoRepository {
        fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
        }

        fn remove_task(&mut self, task_id: u64) {
            self.tasks.retain(|task| task.id != task_id);
        }

        fn get_all_tasks(&self) -> Vec<Task> {
            self.tasks.clone()
        }
    }

    #[derive(Clone)]
    pub struct ReminderRepository {
        reminders: Vec<ReminderData>,
    }

    impl ReminderRepository {
        pub fn new() -> Self {
            ReminderRepository {
                reminders: Vec::new(),
            }
        }
    }

    impl Reminder for ReminderRepository {
        fn add_reminder(&mut self, reminder: ReminderData) {
            self.reminders.push(reminder);
        }

        fn remove_reminder(&mut self, reminder_id: u64) {
            self.reminders.retain(|reminder| reminder.id != reminder_id);
        }

        fn get_all_reminders(&self) -> Vec<ReminderData> {
            self.reminders.clone()
        }
    }
}

mod controller {
    use crate::core::{ReminderData, Task};
    use crate::model::{ReminderModel, TaskModel};
    use crate::repository::{ReminderRepository, TodoRepository};
    use crate::service::{ReminderService, TodoService};

    #[derive(Clone)]
    pub struct TodoController {
        todo_service: TodoService<TodoRepository>,
    }

    impl TodoController {
        pub fn new() -> Self {
            TodoController {
                todo_service: TodoService::new(TodoRepository::new()),
            }
        }

        pub fn add_task(&mut self, task: TaskModel) {
            self.todo_service.add_task(Task {
                id: 1,
                title: task.title,
                completed: false,
            });
        }

        pub fn remove_task(&mut self, task_id: u64) {
            self.todo_service.remove_task(task_id);
        }

        pub fn get_all_tasks(&self) -> Vec<Task> {
            self.todo_service.get_all_tasks()
        }
    }

    #[derive(Clone)]
    pub struct ReminderController {
        reminder_service: ReminderService<ReminderRepository>,
    }

    impl ReminderController {
        pub fn new() -> Self {
            ReminderController {
                reminder_service: ReminderService::new(ReminderRepository::new()),
            }
        }

        pub fn add_reminder(&mut self, reminder: ReminderModel) {
            self.reminder_service.add_reminder(ReminderData {
                id: 1,
                title: reminder.title,
                date: reminder.date,
                completed: false,
            });
        }

        pub fn remove_reminder(&mut self, reminder_id: u64) {
            self.reminder_service.remove_reminder(reminder_id);
        }

        pub fn get_all_reminders(&self) -> Vec<ReminderData> {
            self.reminder_service.get_all_reminders()
        }
    }
}

mod evals {
    pub fn is_valid_title(title: &str) -> bool {
        !title.trim().is_empty()
    }

    pub fn is_valid_date(date: &str) -> bool {
        chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok()
    }
}

mod data {
    use crate::core::{ReminderData, Task};

    pub trait TodoRepository {
        fn add_task(&mut self, task: Task);
        fn remove_task(&mut self, task_id: u64);
        fn get_all_tasks(&self) -> Vec<Task>;
    }

    #[derive(Clone)]
    pub struct InMemoryTodoRepository {
        tasks: Vec<Task>,
    }

    impl InMemoryTodoRepository {
        pub fn new() -> Self {
            InMemoryTodoRepository { tasks: vec![] }
        }
    }

    impl TodoRepository for InMemoryTodoRepository {
        fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
        }

        fn remove_task(&mut self, task_id: u64) {
            self.tasks.retain(|task| task.id != task_id);
        }

        fn get_all_tasks(&self) -> Vec<Task> {
            self.tasks.clone()
        }
    }

    pub trait ReminderRepository {
        fn add_reminder(&mut self, reminder: ReminderData);
        fn remove_reminder(&mut self, reminder_id: u64);
        fn get_all_reminders(&self) -> Vec<ReminderData>;
    }

    #[derive(Clone)]
    pub struct InMemoryReminderRepository {
        reminders: Vec<ReminderData>,
    }

    impl InMemoryReminderRepository {
        pub fn new() -> Self {
            InMemoryReminderRepository { reminders: vec![] }
        }
    }

    impl ReminderRepository for InMemoryReminderRepository {
        fn add_reminder(&mut self, reminder: ReminderData) {
            self.reminders.push(reminder);
        }

        fn remove_reminder(&mut self, reminder_id: u64) {
            self.reminders.retain(|reminder| reminder.id != reminder_id);
        }

        fn get_all_reminders(&self) -> Vec<ReminderData> {
            self.reminders.clone()
        }
    }
}

mod utility {
    pub fn generate_id() -> u64 {
        rand::random()
    }

    pub fn format_date(date: &str) -> String {
        let date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
        date.format("%B %e, %Y").to_string()
    }
}

mod view {
    use crate::{
        core::{ReminderData, Task},
        utility::format_date,
    };

    pub struct ConsoleView;

    impl ConsoleView {
        pub fn display_tasks(tasks: &[Task]) {
            println!("Tasks:");
            for task in tasks {
                let status = if task.completed { "✅" } else { "❌" };
                println!("{} [{}] {}", task.id, status, task.title);
            }
        }

        pub fn display_reminders(reminders: &[ReminderData]) {
            println!("Reminders:");
            for reminder in reminders {
                let status = if reminder.completed { "✅" } else { "❌" };
                let formatted_date = format_date(&reminder.date);
                println!(
                    "{} [{}] {} - {}",
                    reminder.id, status, reminder.title, formatted_date
                );
            }
        }
    }
}

mod run {
    use crate::controller::{ReminderController, TodoController};
    use crate::evals::{is_valid_date, is_valid_title};
    use crate::model::{ReminderModel, TaskModel};
    use crate::view;

    pub(crate) fn main() {
        let mut todo_controller = TodoController::new();
        todo_controller.add_task(TaskModel {
            title: "Buy milk".to_string(),
        });
        todo_controller.add_task(TaskModel {
            title: "Buy eggs".to_string(),
        });
        todo_controller.add_task(TaskModel {
            title: "Buy bread".to_string(),
        });
        todo_controller.remove_task(2);
        let tasks = todo_controller.get_all_tasks();
        view::ConsoleView::display_tasks(&tasks);
        // println!("Tasks: {}", tasks);

        let mut reminder_controller = ReminderController::new();
        reminder_controller.add_reminder(ReminderModel {
            title: "Buy milk".to_string(),
            date: "2020-01-01".to_string(),
        });
        reminder_controller.add_reminder(ReminderModel {
            title: "Buy eggs".to_string(),
            date: "2020-01-02".to_string(),
        });
        reminder_controller.add_reminder(ReminderModel {
            title: "Buy bread".to_string(),
            date: "2020-01-03".to_string(),
        });
        reminder_controller.remove_reminder(2);
        let reminders = reminder_controller.get_all_reminders();
        view::ConsoleView::display_reminders(&reminders);
        // println!("Reminders: {}", reminders);
    }
}

fn main() {
   run::main();
}



#[macro_export]
macro_rules! impl_display {
    ($($app:ident : $model:ty),*) => {
        $(
            impl std::fmt::Display for $model {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", &self)
                }
            }
        )*
    };
}

impl_display! {
    TodoController: model::TaskModel
}

impl_display! {
    ReminderController: model::ReminderModel
}

// impl_display! {
//     ReminderController: dyn Todo1
// }

