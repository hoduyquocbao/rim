use std::{ any::{ Any, type_name }, clone, fmt::{ Arguments, Display, Debug } };
// Tạo một trait Injection có các phương thức name, as_any, calls
pub trait Injection: Any {
    // phương thứ trả về tên của kiểu dữ liệu
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
    // phương thức trả về kiểu dữ liệu Any
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    // phương thức gọi hàm với tham số là Arguments format_args
    fn calls(&self, args: Arguments);
}
// impl Injection cho Box<dyn Injection>
impl Injection for Box<dyn Injection> {
    fn type_name(&self) -> &'static str {
        (**self).type_name()
    }
    fn as_any(&self) -> &dyn Any {
        (**self).as_any()
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        (**self).as_any_mut()
    }
    fn calls(&self, args: Arguments) {
        (**self).calls(args)
    }
}
// impl Clone cho Box<dyn Injection>
impl clone::Clone for Box<dyn Injection> {
    fn clone(&self) -> Self {
        self.as_any().downcast_ref::<Self>().unwrap().clone()
    }
}
// impl Display cho Box<dyn Injection>
impl Display for Box<dyn Injection> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method = self.as_any().downcast_ref::<Self>().unwrap();
        write!(f, "{}", method)
    }
}
impl Debug for Box<dyn Injection> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method = self.as_any().downcast_ref::<Self>().unwrap();
        write!(f, "{:?}", method)
    }
}
#[derive(Clone, Debug)]
// Tạo một struct Module có một thuộc tính là một mảng chứa các kiểu dữ liệu Injection
pub struct Module {
    pub injection: Vec<Box<dyn Injection>>,
}
// Khởi tạo một impl cho struct Module có phương thức new, add và get để thêm, lấy dữ liệu từ mảng injection
impl Module {
    pub fn new() -> Self {
        Module {
            injection: Vec::new(),
        }
    }
    // phương thức add có tham số là một kiểu dữ liệu Injection có thể là một struct hoặc một hàm
    pub fn add<C>(&mut self, injection: C) where C: Injection + 'static {
        // thêm vào mảng injection
        self.injection.push(Box::new(injection));
    }
    // phương thức get có tham số là một kiểu dữ liệu Injection có thể là một struct hoặc một hàm
    pub fn get<C>(&self) -> Option<&C> where C: Injection + 'static {
        // tìm kiếm trong mảng injection có phần tử nào có kiểu dữ liệu là Component
        self.injection.iter().find_map(|x| x.as_any().downcast_ref::<C>())
    }
    // phương thức get_mut có tham số là một kiểu dữ liệu Injection có thể là một struct hoặc một hàm
    pub fn get_mut<C>(&mut self) -> Option<&mut C> where C: Injection + 'static {
        // tìm kiếm trong mảng injection có phần tử nào có kiểu dữ liệu là Component
        self.injection.iter_mut().find_map(|x| x.as_any_mut().downcast_mut::<C>())
    }
    // phương thức function có tham số là Arguments format_args trả về một Option<&Method>
    pub fn function(&self, name: Arguments) -> Option<&Method> {
        self.injection.iter().find_map(|x| {
            x.as_any()
                .downcast_ref::<Method>()
                .map_or(None, |m| {
                    if m.name == name.as_str().unwrap() { Some(m) } else { None }
                })
        })
    }
}
// impl Injection cho struct Module có phương thức name, as_any, calls
impl Injection for Module {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
    fn calls(&self, args: Arguments) {
        let method = self.function(args).unwrap();
        method.calls(args);
    }
}
// Tạo một struct Method có hai thuộc tính là tên của hàm và hàm
#[derive(Clone)]
pub struct Method {
    // tên của hàm là một chuỗi
    pub name: String,
    // hàm có tham số là Arguments format_args
    pub function: fn(Arguments) -> (),
}
// impl Injection cho struct Method có phương thức name, as_any, calls
impl Method {
    // hàm khởi tạo của struct Method
    pub fn new(name: &'static str, function: fn(Arguments) -> ()) -> Self {
        Method {
            name: name.to_string(),
            function,
        }
    }
    // phương thức gọi hàm với tham số là Arguments format_args
    pub fn calls(&self, args: Arguments) {
        (self.function)(args);
    }
}
impl Injection for Method {
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
    fn calls(&self, args: Arguments) {
        (self.function)(args);
    }
}
// fn main() {
// let mut module = Module::new();
// module.add(Method::new("test", test));
// let method = module.function(format_args!("test"));
// let args = format_args!("test1");
// method.unwrap().calls(args);
// }
// fn test(args: Arguments) {
// println!("{}", args);
// }
// Định nghĩa cấu trúc Todo
#[derive(Debug, Clone)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}
// Định nghĩa Injection cho Todo
impl Injection for Todo {
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
    fn calls(&self, args: Arguments) -> () {
        println!("{}", args);
        // self.args;
    }
}
// Thêm phương thức mark_completed cho Todo
impl Todo {
    pub fn new(id: i32, title: String, completed: bool) -> Self {
        Todo {
            id,
            title,
            completed,
        }
    }
    fn mark_completed(&mut self) {
        self.completed = true;
    }
}
impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
fn main() {
    // Khởi tạo một module và thêm một cấu trúc Todo vào đó
    let mut module = Module::new();
    module.add(Todo::new(1, "Buy groceries".to_string(), false));
    // module.add(Method::new("mark_completed", Todo::as_any().downcast_ref::<Todo>().unwrap().mark_completed));

    // Lấy cấu trúc Todo từ module
    let mut todo = module.get_mut::<Todo>().unwrap();
    // Đánh dấu Todo là đã hoàn thành
    todo.calls(format_args!("mark_completed()"));
    todo.mark_completed();
    // In ra tiêu đề và trạng thái của cấu trúc Todo
    // println!("Todo title: {:?}", );
    println!("Todo status: {}", if todo.completed { "Completed" } else { "Not completed" });
}