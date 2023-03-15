

fn main() {
    let memory = Memory::<Todo>::new();
let data = Box::new(memory);
let repository = Repository::new(data.clone(), Box::new(Utility {}));
let model = Model { data: Box::new(repository) };
let utility = Utility {};
let service = Service {
    utility: utility,
    model: model,
};
let evals = Evals {};
let core = Core {
    data: service,
    evals: evals,
};
let app = Application { core };

}

//Sử dụng Component Based Architecture (CBA) để tách biệt các thành phần của ứng dụng ra khỏi nhau. sử dụng generic để định nghĩa các thành phần của ứng dụng.
// Định nghĩa Application là thành phần chính của ứng dụng, nó sẽ chứa các thành phần khác của ứng dụng.
struct Application<T: Core> {
    core: T,
}

impl<T: Core> Application<T> {
    fn create(&self, data: T::Data) -> Result<(), T::Error> {
        self.core.create(data)
    }

    fn update(&self, data: T::Data) -> Result<(), T::Error> {
        self.core.update(data)
    }

    fn delete(&self, data: T::Data) -> Result<(), T::Error> {
        self.core.delete(data)
    }
}

// Định nghĩa Core là thành phần chính của ứng dụng, nó sẽ chứa các thành phần khác của ứng dụng.
trait Component {
    type Data;
    type Error;

    fn create(&self, data: Self::Data) -> Result<(), Self::Error>;
    fn update(&self, data: Self::Data) -> Result<(), Self::Error>;
    fn delete(&self, data: Self::Data) -> Result<(), Self::Error>;
}

// tạo một struct mới implement component Data và component Utility, và sử dụng injection để kết nối với các component khác
struct Repository<T> {
    data: Box<dyn Data<Data=T, Error=()>>,
    utility: Box<dyn Utility<Data=T>>,
}

impl<T> Repository<T> {
    fn new(data: Box<dyn Data<Data=T, Error=()>>, utility: Box<dyn Utility<Data=T>>) -> Self {
        Repository {
            data: data,
            utility: utility,
        }
    }
}

impl<T> Data for Repository<T>
where
    T: Clone + PartialEq,
{
    type Data = T;
    type Error = ();

    fn create(&self, data: Self::Data) -> Result<(), Self::Error> {
        self.data.create(data)?;
        Ok(())
    }

    fn update(&self, data: Self::Data) -> Result<(), Self::Error> {
        self.data.update(data)?;
        Ok(())
    }

    fn delete(&self, data: Self::Data) -> Result<(), Self::Error> {
        self.data.delete(data)?;
        Ok(())
    }
}

impl<T> Utility for Repository<T>
where
    T: Clone + PartialEq,
{
    fn get_all(&self) -> Vec<T> {
        self.data.get_all()
    }

    fn find(&self, data: T) -> Option<T> {
        self.data.find(data)
    }
}

// tạo một struct mới implement component Data và sử dụng injection để kết nối với các component khác
struct Memory<T> {
    items: Vec<T>,
}

impl<T> Memory<T> {
    fn new() -> Self {
        Memory {
            items: Vec::new(),
        }
    }
}

impl<T> Data for Memory<T>
where
    T: Clone + PartialEq,
{
    type Data = T;
    type Error = ();

    fn create(&self, data: Self::Data) -> Result<(), Self::Error> {
        self.items.push(data);
        Ok(())
    }

    fn update(&self, data: Self::Data) -> Result<(), Self::Error> {
        if let Some(item) = self.items.iter_mut().find(|item| *item == &data) {
            *item = data;
            Ok(())
        } else {
            Err(())
        }
    }

    fn delete(&self, data: Self::Data) -> Result<(), Self::Error> {
        if let Some(index) = self.items.iter().position(|item| *item == data) {
            self.items.remove(index);
            Ok(())
        } else {
            Err(())
        }
    }
}
