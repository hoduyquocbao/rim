fn main() {}

// Path: src/lib.rs
// Kiến trúc Module Component Based Architecture động trong rust. Có thể đăng ký từng component cho module. Cụ thể là Module<Component<Interface<Method<Fn Paramer>>>
// Kiến trúc Module Component Based Architecture động trong Rust là một phương pháp thiết kế phần mềm cho phép tách biệt các thành phần của ứng dụng thành các module độc lập nhau, mỗi module bao gồm một hoặc nhiều component và các giao diện để truy cập đến các component này.
// Trong kiến trúc này, mỗi component đại diện cho một phần của ứng dụng, và có thể đăng ký và sử dụng bởi các module khác nhau. Các component này được đóng gói lại trong các module để cung cấp tính độc lập và khả năng tái sử dụng cao.
// Trong Rust, cấu trúc Module<Component<Interface<Method<Fn Parameter>>>> có thể được định nghĩa như sau:

mod module {
    pub mod component {

        pub trait Interface {
            fn method(&self, param: FnParameter);
        }

        pub struct Component<T: Interface> {
            pub interface: T,
        }

        pub struct FnParameter;
    }
}
// Trong đó, module là một module chính của ứng dụng, component là một module con bên trong module, Interface là một trait đại diện cho giao diện của component, Component là một struct đại diện cho một component cụ thể với một interface được cung cấp.
// Các phương thức và tham số của interface có thể được định nghĩa tùy ý để phù hợp với nhu cầu của ứng dụng.
// Việc đăng ký và sử dụng các component trong module có thể được thực hiện bằng cách sử dụng các hàm định nghĩa trong module. Ví dụ:

mod my_module {
    use crate::module::component::{Component, FnParameter, Interface};

    pub struct MyInterfaceComponent;

    impl Interface for MyInterfaceComponent {
        fn method(&self, param: FnParameter) {
            // code implementation here
        }
    }

    pub fn register_component() -> Component<MyInterfaceComponent> {
        Component {
            interface: MyInterfaceComponent {},
        }
    }
}
// Trong đó, MyInterfaceComponent là một implementation của Interface, register_component() là một hàm public của module my_module để đăng ký một component mới. Khi được gọi, hàm này trả về một instance của struct Component chứa MyInterfaceComponent.
// Sau khi đăng ký, các component có thể được sử dụng bởi các module khác trong ứng dụng, và các method của component có thể được gọi thông qua các giao diện.
// Đổi tên my_module lại thành tên application và viết ví dụ todo có core, service, model, data, uitility và ui
// Sure! Dưới đây là một ví dụ về cách sử dụng kiến trúc Module Component Based Architecture trong Rust để xây dựng một ứng dụng Todo đơn giản với các module core, service, model, data, utility và UI.

mod application {
    // Core module
    pub mod core {
        #[derive(Clone, Debug, PartialEq)]
        pub struct TodoItem {
            pub id: u32,
            pub title: String,
            pub description: String,
            pub completed: bool,
        }
    }

    // Service module
    pub mod service {
        use crate::application::core::TodoItem;

        pub trait TodoService {
            fn add_todo_item(&mut self, title: String, description: String) -> TodoItem;
            fn get_todo_item(&self, id: u32) -> Option<TodoItem>;
            fn get_all_todo_items(&self) -> Vec<TodoItem>;
            fn update_todo_item(
                &mut self,
                id: u32,
                title: String,
                description: String,
                completed: bool,
            ) -> Option<TodoItem>;
            fn remove_todo_item(&mut self, id: u32) -> Option<TodoItem>;
        }

        pub struct DefaultTodoService {
            todo_items: Vec<TodoItem>,
            last_id: u32,
        }

        impl DefaultTodoService {
            pub fn new() -> DefaultTodoService {
                DefaultTodoService {
                    todo_items: Vec::new(),
                    last_id: 0,
                }
            }
        }

        impl TodoService for DefaultTodoService {
            fn add_todo_item(&mut self, title: String, description: String) -> TodoItem {
                let new_todo_item = TodoItem {
                    id: self.last_id + 1,
                    title: title,
                    description: description,
                    completed: false,
                };

                self.todo_items.push(new_todo_item.clone());
                self.last_id += 1;

                new_todo_item
            }

            fn get_todo_item(&self, id: u32) -> Option<TodoItem> {
                self.todo_items.iter().find(|&i| i.id == id).cloned()
            }

            fn get_all_todo_items(&self) -> Vec<TodoItem> {
                self.todo_items.clone()
            }

            fn update_todo_item(
                &mut self,
                id: u32,
                title: String,
                description: String,
                completed: bool,
            ) -> Option<TodoItem> {
                if let Some(todo_item) = self.todo_items.iter_mut().find(|i| i.id == id) {
                    todo_item.title = title;
                    todo_item.description = description;
                    todo_item.completed = completed;

                    Some(todo_item.clone())
                } else {
                    None
                }
            }

            fn remove_todo_item(&mut self, id: u32) -> Option<TodoItem> {
                let index = self.todo_items.iter().position(|i| i.id == id)?;

                Some(self.todo_items.remove(index))
            }
        }
    }

    // Model module
    pub mod model {
        use crate::application::core::TodoItem;

        pub struct TodoListModel {
            pub todo_items: Vec<TodoItem>,
        }

        impl TodoListModel {
            pub fn new(todo_items: Vec<TodoItem>) -> TodoListModel {
                TodoListModel {
                    todo_items: todo_items,
                }
            }
        }
    }

    // Data module
    pub mod data {
        use crate::application::model::TodoListModel;
        use crate::application::service::{DefaultTodoService, TodoService};

        pub struct TodoData {
            pub todo_service: Box<dyn TodoService>,
            pub todo_list_model: TodoListModel,
        }

        impl TodoData {
            pub fn new() -> TodoData {
                let default_todo_service = DefaultTodoService::new();

                TodoData {
                    todo_service: Box::new(default_todo_service),
                    todo_list_model: TodoListModel::new(Vec::new()),
                }
            }
        }
    }

    // Utility module
    pub mod utility {
        pub fn print_todo_item(todo_item: &crate::application::core::TodoItem) {
            println!("ID: {}", todo_item.id);
            println!("Title: {}", todo_item.title);
            println!("Description: {}", todo_item.description);
            println!("Completed: {}", todo_item.completed);
        }
    }

    // UI module
    pub mod ui {
        use std::io;

        use crate::application::data::TodoData;
        use crate::application::model::TodoListModel;
        use crate::application::service::TodoService;
        use crate::application::utility::print_todo_item;

        pub fn run() {
            let mut todo_data = TodoData::new();

            loop {
                println!("1. Add new todo item");
                println!("2. Get todo item");
                println!("3. Get all todo items");
                println!("4. Update todo item");
                println!("5. Remove todo item");
                println!("6. Exit");

                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                let input = input.trim();

                match input {
                    "1" => {
                        println!("Enter title:");
                        let mut title = String::new();
                        io::stdin()
                            .read_line(&mut title)
                            .expect("Failed to read line");

                        println!("Enter description:");
                        let mut description = String::new();
                        io::stdin()
                            .read_line(&mut description)
                            .expect("Failed to read line");

                        let new_todo_item = todo_data.todo_service.add_todo_item(
                            title.trim().to_string(),
                            description.trim().to_string(),
                        );

                        println!("Added new todo item:");
                        print_todo_item(&new_todo_item);
                    }
                    "2" => {
                        println!("Enter todo item ID:");
                        let mut id_input = String::new();
                        io::stdin()
                            .read_line(&mut id_input)
                            .expect("Failed to read line");

                        let id = id_input.trim().parse::<u32>().unwrap();

                        if let Some(todo_item) = todo_data.todo_service.get_todo_item(id) {
                            print_todo_item(&todo_item);
                        } else {
                            println!("Todo item not found.");
                        }
                    }
                    "3" => {
                        let todo_items = todo_data.todo_service.get_all_todo_items();

                        if todo_items.is_empty() {
                            println!("No todo items found.");
                        } else {
                            println!("All todo items:");

                            for todo_item in todo_items {
                                print_todo_item(&todo_item);
                            }
                        }
                    }
                    "4" => {
                        println!("Enter todo item ID:");
                        let mut id_input = String::new();
                        io::stdin()
                            .read_line(&mut id_input)
                            .expect("Failed to read line");

                        let id = id_input.trim().parse::<u32>().unwrap();

                        if let Some(todo_item) = todo_data.todo_service.get_todo_item(id) {
                            println!("Enter new title:");
                            let mut title = String::new();
                            io::stdin()
                                .read_line(&mut title)
                                .expect("Failed to read line");

                            println!("Enter new description:");
                            let mut description = String::new();
                            io::stdin()
                                .read_line(&mut description)
                                .expect("Failed to read line");

                            println!("Enter new completed status:");
                            let mut completed_input = String::new();
                            io::stdin()
                                .read_line(&mut completed_input)
                                .expect("Failed to read line");

                            let completed = completed_input.trim().parse::<bool>().unwrap();

                            let updated_todo_item = todo_data.todo_service.update_todo_item(
                                id,
                                title.trim().to_string(),
                                description.trim().to_string(),
                                completed,
                            );

                            println!("Updated todo item:");

                            print_todo_item(&updated_todo_item.unwrap());
                        } else {
                            println!("Todo item not found.");
                        }
                    }
                    "5" => {
                        println!("Enter todo item ID:");
                        let mut id_input = String::new();
                        io::stdin()
                            .read_line(&mut id_input)
                            .expect("Failed to read line");

                        let id = id_input.trim().parse::<u32>().unwrap();

                        if let Some(todo_item) = todo_data.todo_service.get_todo_item(id) {
                            todo_data.todo_service.remove_todo_item(id);

                            println!("Removed todo item:");

                            print_todo_item(&todo_item);
                        } else {
                            println!("Todo item not found.");
                        }
                    }
                    "6" => {
                        break;
                    }
                    _ => {
                        println!("Invalid input.");
                    }
                }
            }
        }
    }

    // Oh, tôi xin lỗi vì đã bỏ qua phần đó trong câu hỏi trước. Dưới đây là một phiên bản thay đổi để sử dụng kiến trúc Module Component Based Architecture đăng ký từng component cho module của todo application.
    // rust

    // application.rs

    use std::any::Any;

    pub struct Module<T> {
        pub components: Vec<Box<dyn Any>>,
        pub module: T,
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
    }

    // core.rs

    #[derive(Clone, Debug, PartialEq)]
    pub struct TodoItem {
        pub id: u32,
        pub title: String,
        pub description: String,
        pub completed: bool,
    }

    pub trait TodoService {
        fn add_todo_item(&mut self, title: String, description: String) -> TodoItem;
        fn get_todo_item(&self, id: u32) -> Option<TodoItem>;
        fn get_all_todo_items(&self) -> Vec<TodoItem>;
        fn update_todo_item(&mut self, id: u32, title: String, description: String) -> bool;
        fn remove_todo_item(&mut self, id: u32) -> bool;
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct DefaultTodoService {
        todo_items: Vec<TodoItem>,
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

    impl TodoService for DefaultTodoService {
        fn add_todo_item(&mut self, title: String, description: String) -> TodoItem {
            let new_todo_item = TodoItem {
                id: self.generate_todo_item_id(),
                title,
                description,
                completed: false,
            };

            self.todo_items.push(new_todo_item.clone());

            new_todo_item
        }

        fn get_todo_item(&self, id: u32) -> Option<TodoItem> {
            self.todo_items
                .iter()
                .find(|&todo_item| todo_item.id == id)
                .cloned()
        }

        fn get_all_todo_items(&self) -> Vec<TodoItem> {
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

    // model.rs

    // use crate::core::{TodoItem, TodoService};

    pub struct TodoModel {
        todo_service: Box<dyn TodoService>,
    }

    impl TodoModel {
        pub fn new(todo_service: Box<dyn TodoService>) -> TodoModel {
            TodoModel { todo_service }
        }

        pub fn add_todo_item(&mut self, title: String, description: String) -> TodoItem {
            self.todo_service.add_todo_item(title, description)
        }

        pub fn get_todo_item(&self, id: u32) -> Option<TodoItem> {
            self.todo_service.get_todo_item(id)
        }

        pub fn get_all_todo_items(&self) -> Vec<TodoItem> {
            self.todo_service.get_all_todo_items()
        }

        pub fn update_todo_item(&mut self, id: u32, title: String, description: String) -> bool {
            self.todo_service.update_todo_item(id, title, description)
        }

        pub fn remove_todo_item(&mut self, id: u32) -> bool {
            self.todo_service.remove_todo_item(id)
        }
    }

    // data.rs

    // use crate::core::{TodoItem, TodoService};

    pub struct TodoData {
        todo_service: Box<dyn TodoService>,
    }

    impl TodoData {
        pub fn new(todo_service: Box<dyn TodoService>) -> TodoData {
            TodoData { todo_service }
        }

        pub fn get_all_todo_items(&self) -> Vec<TodoItem> {
            self.todo_service.get_all_todo_items()
        }
    }

    // utility.rs

    pub fn format_todo_item(todo_item: &TodoItem) -> String {
        format!(
            "ID: {}\nTitle: {}\nDescription: {}\nCompleted: {}",
            todo_item.id, todo_item.title, todo_item.description, todo_item.completed
        )
    }

    // ui.rs

    // use crate::model::TodoModel;
    // use crate::utility::format_todo_item;
    use std::io::{self, Write};

    pub struct ConsoleUI {
        todo_model: TodoModel,
    }

    impl ConsoleUI {
        pub fn new(todo_model: TodoModel) -> ConsoleUI {
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
// ```
// Trong ví dụ trên, chúng ta đã sử dụng kiến trúc đăng ký module và các component cho ứng dụng Todo. Cụ thể là:

// - `core` module chứa các cấu trúc dữ liệu và phương thức liên quan đến TodoItem và TodoService.
// - `service` module chứa các phương thức cung cấp dịch vụ Todo.
// - `model` module chứa các phương thức liên quan đến TodoModel, một bản sao của TodoService.
// - `data` module chứa các phương thức liên quan đến truy cập dữ liệu, một bản sao của TodoService.
// - `utility` module chứa các phương thức tiện ích liên quan đến TodoItem.
// - `ui` module chứa các phương thức liên quan đến giao diện người dùng ConsoleUI.

// Mỗi module đều có các thành phần của nó và được đăng ký trong ứng dụng Todo. Các thành phần này được sử dụng để xây dựng các chức năng của ứng dụng Todo.

/*
Để sử dụng kiến trúc đăng ký module và các component cho một ứng dụng Rust, bạn có thể làm theo các bước sau:
1. Định nghĩa các module và component của ứng dụng, tùy thuộc vào yêu cầu của ứng dụng của bạn. Ví dụ: module core, service, model, data, utility, ui.
2. Tạo một module gốc cho ứng dụng và đăng ký các module và component của ứng dụng đó trong module gốc. Ví dụ:
rust

Copy code
mod app {
    mod core {
        ...
    }
    mod service {
        ...
    }
    mod model {
        ...
    }
    mod data {
        ...
    }
    mod utility {
        ...
    }
    mod ui {
        ...
    }
}

fn main() {
    ...
}
// 3. Trong mỗi module, bạn có thể định nghĩa các cấu trúc dữ liệu và phương thức cần thiết, tùy thuộc vào yêu cầu của module đó.
// 4. Trong module gốc, bạn có thể đăng ký các module và component của ứng dụng bằng cách sử dụng các macro. Ví dụ:
// rust
*/

// mod app {
//     #[macro_use]
//     extern crate lazy_static;

//     mod core {
//         ...
//     }
//     mod service {
//         ...
//     }
//     mod model {
//         ...
//     }
//     mod data {
//         ...
//     }
//     mod utility {
//         ...
//     }
//     mod ui {
//         ...
//     }

//     // Đăng ký các module và component của ứng dụng
//     module_registry! {
//         modules {
//             Core: core::CoreModule,
//             Service: service::ServiceModule,
//             Model: model::ModelModule,
//             Data: data::DataModule,
//             Utility: utility::UtilityModule,
//             UI: ui::UIModule,
//         }

//         components {
//             Core(TodoItem, TodoService),
//             Service(TodoService),
//             Model(TodoModel),
//             Data(TodoData),
//             Utility(TodoItem),
//             UI(ConsoleUI),
//         }
//     }
// }

// fn main() {
//     ...
// }
// Trong ví dụ trên, chúng ta đã đăng ký các module và component của ứng dụng Todo bằng cách sử dụng macro module_registry!. Các module và component đã được đăng ký trong macro này sẽ được sử dụng trong ứng dụng Todo.
// 5. Cuối cùng, bạn có thể sử dụng các module và component của ứng dụng để xây dựng các chức năng của ứng dụng. Ví dụ:
// rust

// Trong ví dụ trên, chúng ta đã sử dụng module UI và component ConsoleUI để khởi động giao diện người dùng của ứng dụng Todo.
