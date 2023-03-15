// #![no_std]
// - `core` module chứa các cấu trúc dữ liệu và phương thức liên quan đến super::core::Todo và Todo.
// - `service` module chứa các phương thức cung cấp dịch vụ Todo.
// - `model` module chứa các phương thức liên quan đến TodoModel, một bản sao của Todo.
// - `data` module chứa các phương thức liên quan đến truy cập dữ liệu, một bản sao của Todo.
// - `utility` module chứa các phương thức tiện ích liên quan đến super::core::Todo.
// - `ui` module chứa các phương thức liên quan đến giao diện người dùng ConsoleUI.

use std::any::{self, type_name, Any};

pub struct Module<Fn> {
    pub components: Vec<Box<dyn Any>>,
    pub module: Fn,
}

impl<T> Module<T> {
    pub fn new(module: T) -> Self {
        Module {
            components: Vec::new(),
            module,
        }
    }

    pub fn register_component<C>(&mut self, component: C)
    where
        C: 'static,
    {
        self.components.push(Box::new(component));
    }

    pub fn get_component<C>(&self) -> Option<&C>
    where
        C: 'static,
    {
        for component in &self.components {
            if let Some(component) = component.downcast_ref::<C>() {
                return Some(component);
            }
        }

        None
    }

    pub fn get_component_mut<C>(&mut self) -> Option<&mut C>
    where
        C: 'static,
    {
        for component in &mut self.components {
            if let Some(component) = component.downcast_mut::<C>() {
                return Some(component);
            }
        }

        None
    }

    pub fn get_component_by_name<C>(&self, name: &str) -> Option<&C>
    where
        C: 'static + std::fmt::Debug,
    {
        for component in &self.components {
            if let Some(component) = component.downcast_ref::<C>() {
                let component_type_name = std::any::type_name::<C>();
                println!("component_type_name: {:?}", component_type_name);
                if component_type_name == name {
                    return Some(component);
                } else {
                    return Some(component);
                }
            }
        }

        // Some(component)
        None
    }
}



mod application {
    // core.rs
    mod core {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Todo {
            pub id: u32,
            pub title: String,
            pub description: String,
            pub completed: bool,
        }
    }
    mod service {
        use super::*;
        pub trait Todo {
            fn add_todo_item(&mut self, title: String, description: String) -> core::Todo;
            fn get_todo_item(&self, id: u32) -> Option<super::core::Todo>;
            fn get_all_todo_items(&self) -> Vec<super::core::Todo>;
            fn update_todo_item(&mut self, id: u32, title: String, description: String) -> bool;
            fn remove_todo_item(&mut self, id: u32) -> bool;
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct DefaultTodoService {
            todo_items: Vec<super::core::Todo>,
        }

        impl DefaultTodoService {
            pub fn new() -> DefaultTodoService {
                DefaultTodoService {
                    todo_items: Vec::new(),
                }
            }

            fn generate_todo_item_id(&self) -> u32 {
                if let Some(todo_item) = self.todo_items.last() {
                    todo_item.id + 1
                } else {
                    1
                }
            }
        }

        impl Todo for DefaultTodoService {
            fn add_todo_item(&mut self, title: String, description: String) -> super::core::Todo {
                let new_todo_item = super::core::Todo {
                    id: self.generate_todo_item_id(),
                    title,
                    description,
                    completed: false,
                };

                self.todo_items.push(new_todo_item.clone());

                new_todo_item
            }

            fn get_todo_item(&self, id: u32) -> Option<super::core::Todo> {
                self.todo_items
                    .iter()
                    .find(|&todo_item| todo_item.id == id)
                    .cloned()
            }

            fn get_all_todo_items(&self) -> Vec<super::core::Todo> {
                self.todo_items.clone()
            }

            fn update_todo_item(&mut self, id: u32, title: String, description: String) -> bool {
                if let Some(todo_item) = self.todo_items.iter_mut().find(|x| x.id == id) {
                    todo_item.title = title;
                    todo_item.description = description;
                    true
                } else {
                    false
                }
            }

            fn remove_todo_item(&mut self, id: u32) -> bool {
                if let Some(index) = self.todo_items.iter().position(|x| x.id == id) {
                    self.todo_items.remove(index);
                    true
                } else {
                    false
                }
            }
        }
    }
    // model.rs
    mod model {
        // use crate::core::{super::core::Todo, Todo};

        pub struct TodoModel {
            todo_service: Box<dyn super::service::Todo>,
        }

        impl TodoModel {
            pub fn new(todo_service: Box<dyn super::service::Todo>) -> TodoModel {
                TodoModel { todo_service }
            }

            pub fn add_todo_item(
                &mut self,
                title: String,
                description: String,
            ) -> super::core::Todo {
                self.todo_service.add_todo_item(title, description)
            }

            pub fn get_todo_item(&self, id: u32) -> Option<super::core::Todo> {
                self.todo_service.get_todo_item(id)
            }

            pub fn get_all_todo_items(&self) -> Vec<super::core::Todo> {
                self.todo_service.get_all_todo_items()
            }

            pub fn update_todo_item(
                &mut self,
                id: u32,
                title: String,
                description: String,
            ) -> bool {
                self.todo_service.update_todo_item(id, title, description)
            }

            pub fn remove_todo_item(&mut self, id: u32) -> bool {
                self.todo_service.remove_todo_item(id)
            }
        }
    }
    // data.rs
    mod data {
        // use crate::core::{super::core::Todo, Todo};

        pub struct TodoData {
            todo_service: Box<dyn super::service::Todo>,
        }

        impl TodoData {
            pub fn new(todo_service: Box<dyn super::service::Todo>) -> TodoData {
                TodoData { todo_service }
            }

            pub fn get_all_todo_items(&self) -> Vec<super::core::Todo> {
                self.todo_service.get_all_todo_items()
            }
        }
    }
    // utility.rs
    mod utility {
        pub fn format_todo_item(todo_item: &super::core::Todo) -> String {
            format!(
                "ID: {}\nTitle: {}\nDescription: {}\nCompleted: {}",
                todo_item.id, todo_item.title, todo_item.description, todo_item.completed
            )
        }
    }
    // ui.rs
    mod ui {
        // use crate::model::TodoModel;
        // use crate::utility::format_todo_item;
        use std::io::{self, Write};

        use crate::application::utility::format_todo_item;

        pub struct ConsoleUI {
            todo_model: super::model::TodoModel,
        }

        impl ConsoleUI {
            pub fn new(todo_model: super::model::TodoModel) -> ConsoleUI {
                ConsoleUI { todo_model }
            }

            pub fn start(&mut self) {
                println!("Welcome to Todo!");

                loop {
                    println!("What would you like to do?");
                    println!("1. Add a todo item");
                    println!("2. View a todo item");
                    println!("3. View all todo items");
                    println!("4. Update a todo item");
                    println!("5. Remove a todo item");
                    println!("6. Exit");

                    let choice = read_int_input("Choice: ");

                    match choice {
                        1 => {
                            let title = read_string_input("Title: ");
                            let description = read_string_input("Description: ");
                            let todo_item = self.todo_model.add_todo_item(title, description);
                            println!("Todo item added:\n{}", format_todo_item(&todo_item));
                        }
                        2 => {
                            let id = read_int_input("ID: ");
                            if let Some(todo_item) = self.todo_model.get_todo_item(id) {
                                println!("{}", format_todo_item(&todo_item));
                            } else {
                                println!("No todo item found with ID {}", id);
                            }
                        }
                        3 => {
                            let todo_items = self.todo_model.get_all_todo_items();
                            if todo_items.is_empty() {
                                println!("No todo items found.");
                            } else {
                                for todo_item in todo_items {
                                    println!("{}", format_todo_item(&todo_item));
                                    println!();
                                }
                            }
                        }
                        4 => {
                            let id = read_int_input("ID: ");
                            let title = read_string_input("Title: ");
                            let description = read_string_input("Description: ");
                            if self.todo_model.update_todo_item(id, title, description) {
                                println!("Todo item with ID {} updated.", id);
                            } else {
                                println!("No todo item found with ID {}", id);
                            }
                        }
                        5 => {
                            let id = read_int_input("ID: ");
                            if self.todo_model.remove_todo_item(id) {
                                println!("Todo item with ID {} removed.", id);
                            } else {
                                println!("No todo item found with ID {}", id);
                            }
                        }
                        6 => {
                            println!("Goodbye!");
                            break;
                        }
                        _ => println!("Invalid choice. Please choose a number between 1 and 6."),
                    }
                }
            }
        }

        fn read_int_input(prompt: &str) -> u32 {
            print!("{}", prompt);
            io::stdout().flush().expect("Failed to flush stdout");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input.trim().parse().expect("Invalid input")
        }

        fn read_string_input(prompt: &str) -> String {
            print!("{}", prompt);
            io::stdout().flush().expect("Failed to flush stdout");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input.trim().to_string()
        }
    }
}
// let console_ui = app::module::<app::UI>().unwrap().component::<ConsoleUI>();
    // console_ui.start();
    struct Todo {
        pub name: String,
    }
    struct TestModule1 {
        pub name: String,
    }

    #[derive(Debug, PartialEq)]
    struct TestComponent {
        pub value: u32,
    }
fn main() {
    

    let mut module0 = Module::new(Todo {
        name: "Test Module".to_string(),
    });
    let mut module1 = Module::new(TestModule1 {
        name: "Test Module".to_string(),
    });

    module0.register_component(TestComponent { value: 10 });

    let component = module0.get_component_by_name("TestComponent");

    let value: &TestComponent  = component.unwrap();

    // assert_eq!(
    //     component,
    //     Some(&TestComponent { value : 10})
    // );
}

// // Đăng ký các module và component của ứng dụng
// module_registry! {
//     modules {
//         Core: core::CoreModule,
//         Service: service::ServiceModule,
//         Model: model::ModelModule,
//         Data: data::DataModule,
//         Utility: utility::UtilityModule,
//         UI: ui::UIModule,
//     }

//     components {
//         Core(super::core::Todo, super::service::Todo),
//         Service(super::service::Todo),
//         Model(TodoModel),
//         Data(TodoData),
//         Utility(super::core::Todo),
//         UI(ConsoleUI),
//     }
// }

// // viết nội dung cho macro module_registry
// #[macro_export]
