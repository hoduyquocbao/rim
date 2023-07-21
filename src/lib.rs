
mod storage {
// Khai báo các thư viện cần thiết
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;

// Khai báo một enum để đại diện cho các loại quyền truy cập
#[derive(Debug, Clone, Copy, PartialEq)]
enum Access {
    Private, // Chỉ có chủ sở hữu mới có thể truy cập
    Public,  // Bất kỳ ai cũng có thể truy cập
    Shared,  // Chỉ có những người được chia sẻ mới có thể truy cập
}

// Khai báo một struct để đại diện cho một đối tượng lưu trữ
#[derive(Debug)]
struct Storage {
    name: String,             // Tên của đối tượng lưu trữ
    path: PathBuf,            // Đường dẫn của đối tượng lưu trữ trên hệ thống tập tin
    size: u64,                // Kích thước của đối tượng lưu trữ (tính bằng byte)
    files: Vec<PathBuf>,      // Danh sách các tập tin và dữ liệu trong đối tượng lưu trữ
    access: RwLock<Access>,   // Quyền truy cập của đối tượng lưu trữ (có khóa đọc ghi)
}

// Khai báo các phương thức cho struct Storage
impl Storage {
    // Phương thức new để tạo một đối tượng lưu trữ mới với tên và quyền truy cập cho trước
    fn new(name: &str, access: Access) -> io::Result<Storage> {
        // Tạo một đường dẫn mới từ tên của đối tượng lưu trữ
        let path = Path::new(name);

        // Kiểm tra xem đường dẫn đã tồn tại hay chưa
        if path.exists() {
            // Nếu đã tồn tại, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Storage {} already exists", name),
            ));
        }

        // Nếu chưa tồn tại, tạo một thư mục mới với đường dẫn cho trước
        fs::create_dir(path)?;

        // Tạo một đối tượng lưu trữ mới với các trường khởi tạo như sau
        let storage = Storage {
            name: name.to_string(),     // Tên của đối tượng lưu trữ là chuỗi được sao chép từ tham số name
            path: path.to_path_buf(),   // Đường dẫn của đối tượng lưu trữ là PathBuf được chuyển đổi từ path
            size: 0,                    // Kích thước của đối tượng lưu trữ là 0 (vì chưa có gì trong thư mục)
            files: Vec::new(),          // Danh sách các tập tin và dữ liệu là một Vec rỗng (vì chưa có gì trong thư mục)
            access: RwLock::new(access),// Quyền truy cập của đối tượng lưu trữ là một RwLock được khởi tạo từ tham số access
        };

        // Trả về đối tượng lưu trữ mới với kết quả Ok
        Ok(storage)
    }

    // Phương thức open để mở một đối tượng lưu trữ đã tồn tại với tên cho trước
    fn open(name: &str) -> io::Result<Storage> {
        // Tạo một đường dẫn mới từ tên của đối tượng lưu trữ
        let path = Path::new(name);

        // Kiểm tra xem đường dẫn có tồn tại hay không
        if !path.exists() {
            // Nếu không tồn tại, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Storage {} not found", name),
            ));
        }

        // Nếu tồn tại, kiểm tra xem đường dẫn có phải là một thư mục hay không
        if !path.is_dir() {
            // Nếu không phải là một thư mục, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Storage {} is not a directory", name),
            ));
        }

        // Nếu là một thư mục, tạo một biến để lưu trữ kích thước của đối tượng lưu trữ
        let mut size = 0;

        // Tạo một biến để lưu trữ danh sách các tập tin và dữ liệu trong đối tượng lưu trữ
        let mut files = Vec::new();

        // Tạo một biến để lưu trữ quyền truy cập của đối tượng lưu trữ (mặc định là Private)
        let mut access = Access::Private;

        // Duyệt qua các mục con trong thư mục với đường dẫn cho trước
        for entry in fs::read_dir(path)? {
            // Lấy ra đường dẫn của mục con
            let entry_path = entry?.path();

            // Kiểm tra xem mục con có phải là một tập tin hay không
            if entry_path.is_file() {
                // Nếu là một tập tin, lấy ra kích thước của tập tin
                let file_size = fs::metadata(&entry_path)?.len();

                // Cộng kích thước của tập tin vào kích thước của đối tượng lưu trữ
                size += file_size;

                // Thêm đường dẫn của tập tin vào danh sách các tập tin và dữ liệu
                files.push(entry_path);
            } else {
                // Nếu không phải là một tập tin, kiểm tra xem có phải là thư mục .access hay không
                if entry_path.file_name() == Some(".access".as_ref()) {
                    // Nếu là thư mục .access, mở tập tin .access trong thư mục đó
                    let access_file = File::open(entry_path.join(".access"))?;

                    // Tạo một bộ đọc để đọc nội dung của tập tin .access
                    let mut reader = BufReader::new(access_file);

                    // Tạo một chuỗi để lưu trữ nội dung của tập tin .access
                    let mut content = String::new();

                    // Đọc nội dung của tập tin .access vào chuỗi
                    reader.read_to_string(&mut content)?;

                    // Loại bỏ các khoảng trắng ở đầu và cuối chuỗi
                    content = content.trim().to_string();

                    // Kiểm tra xem nội dung của chuỗi có phải là Private, Public hay Shared không
                    match content.as_str() {
                        "Private" => access = Access::Private, // Nếu là Private, gán quyền truy cập là Private
                        "Public" => access = Access::Public,   // Nếu là Public, gán quyền truy cập là Public
                        "Shared" => access = Access::Shared,   // Nếu là Shared, gán quyền truy cập là Shared
                        _ => {
                            // Nếu không phải là ba giá trị trên, trả về một lỗi io với thông báo
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Invalid access value: {}", content),
                            ));
                        }
                    }
                }
            }
        }

        // Tạo một đối tượng lưu trữ mới với các trường khởi tạo như sau
        let storage = Storage {
            name: name.to_string(),     // Tên của đối tượng lưu trữ là chuỗi được sao chép từ tham số name
            path: path.to_path_buf(),   // Đường dẫn của đối tượng lưu trữ là PathBuf được chuyển đổi từ path
            size,                       // Kích thước của đối tượng lưu trữ là biến size đã tính toán ở trên
            files,                      // Danh sách các tập tin và dữ liệu là biến files đã thu thập ở trên
            access: RwLock::new(access),// Quyền truy cập của đối tượng lưu trữ là một RwLock được khởi tạo từ biến access đã xác định ở trên
        };

        // Trả về đối tượng lưu trữ mới với kết quả Ok
        Ok(storage)
    }

    // Phương thức info để lấy thông tin của đối tượng lưu trữ
    fn info(&self) -> String {
        // Tạo một chuỗi để lưu trữ thông tin của đối tượng lưu trữ
        let mut info = String::new();

        // Thêm tên của đối tượng lưu trữ vào chuỗi
        info.push_str(&format!("Name: {}\n", self.name));

        // Thêm đường dẫn của đối tượng lưu trữ vào chuỗi
        info.push_str(&format!("Path: {:?}\n", self.path));

        // Thêm kích thước của đối tượng lưu trữ vào chuỗi
        info.push_str(&format!("Size: {} bytes\n", self.size));

        // Thêm số lượng các tập tin và dữ liệu trong đối tượng lưu trữ vào chuỗi
        info.push_str(&format!("Files: {} items\n", self.files.len()));

        // Thêm quyền truy cập của đối tượng lưu trữ vào chuỗi (cần khóa đọc để truy cập)
        info.push_str(&format!("Access: {:?}\n", *self.access.read().unwrap()));

        // Trả về chuỗi thông tin
        info
    }

    // Phương thức set_access để thiết lập quyền truy cập cho đối tượng lưu trữ
    fn set_access(&self, access: Access) -> io::Result<()> {
        // Tạo một đường dẫn mới cho thư mục .access trong đối tượng lưu trữ
        let access_dir = self.path.join(".access");

        // Kiểm tra xem thư mục .access đã tồn tại hay chưa
        if !access_dir.exists() {
            // Nếu chưa tồn tại, tạo một thư mục mới với đường dẫn cho trước
            fs::create_dir(&access_dir)?;
        }

        // Tạo một đường dẫn mới cho tập tin .access trong thư mục .access
        let access_file = access_dir.join(".access");

        // Mở hoặc tạo một tập tin mới với đường dẫn cho trước, cho phép ghi và xoá nội dung cũ
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&access_file)?;

        // Tạo một bộ ghi để ghi nội dung vào tập tin .access
        let mut writer = BufWriter::new(file);

        // Ghi giá trị của quyền truy cập vào tập tin .access (cần chuyển enum sang chuỗi)
        writer.write_all(access.to_string().as_bytes())?;

        // Đồng bộ nội dung với hệ thống tập tin
        writer.flush()?;

        // Cập nhật quyền truy cập của đối tượng lưu trữ (cần khóa ghi để thay đổi)
        *self.access.write().unwrap() = access;

        // Trả về kết quả Ok
        Ok(())
    }

    // Phương thức upload để tải lên một tập tin hoặc dữ liệu vào đối tượng lưu trữ
    fn upload(&mut self, source: &Path) -> io::Result<()> {
        // Kiểm tra xem nguồn có tồn tại hay không
        if !source.exists() {
            // Nếu không tồn tại, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Source {} not found", source.display()),
            ));
        }

        // Kiểm tra xem nguồn có phải là một tập tin hay không
        if !source.is_file() {
            // Nếu không phải là một tập tin, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Source {} is not a file", source.display()),
            ));
        }

        // Tạo một đường dẫn mới cho đích bằng cách nối tên của nguồn vào đường dẫn của đối tượng lưu trữ
        let dest = self.path.join(source.file_name().unwrap());

        // Kiểm tra xem đích đã tồn tại hay chưa
        if dest.exists() {
            // Nếu đã tồn tại, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Destination {} already exists", dest.display()),
            ));
        }

        // Nếu chưa tồn tại, sao chép nội dung từ nguồn sang đích
        fs::copy(source, &dest)?;

        // Lấy ra kích thước của tập tin nguồn
        let file_size = fs::metadata(source)?.len();

        // Cộng kích thước của tập tin nguồn vào kích thước của đối tượng lưu trữ
        self.size += file_size;

        // Thêm đường dẫn của đích vào danh sách các tập tin và dữ liệu của đối tượng lưu trữ
        self.files.push(dest);

        // Trả về kết quả Ok
        Ok(())
    }

    // Phương thức download để tải xuống một tập tin hoặc dữ liệu từ đối tượng lưu trữ
    fn download(&self, name: &str, dest: &Path) -> io::Result<()> {
        // Tạo một đường dẫn mới cho nguồn bằng cách nối tên của tập tin hoặc dữ liệu vào đường dẫn của đối tượng lưu trữ
        let source = self.path.join(name);

        // Kiểm tra xem nguồn có tồn tại hay không
        if !source.exists() {
            // Nếu không tồn tại, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Source {} not found", source.display()),
            ));
        }

        // Kiểm tra xem nguồn có phải là một tập tin hay không
        if !source.is_file() {
            // Nếu không phải là một tập tin, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Source {} is not a file", source.display()),
            ));
        }

        // Kiểm tra xem đích có tồn tại hay không
        if dest.exists() {
            // Nếu đã tồn tại, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Destination {} already exists", dest.display()),
            ));
        }

        // Nếu chưa tồn tại, sao chép nội dung từ nguồn sang đích
        fs::copy(&source, dest)?;

        // Trả về kết quả Ok
        Ok(())
    }

    // Phương thức view để xem nội dung của một tập tin hoặc dữ liệu trong đối tượng lưu trữ
    fn view(&self, name: &str) -> io::Result<String> {
        // Tạo một đường dẫn mới cho nguồn bằng cách nối tên của tập tin hoặc dữ liệu vào đường dẫn của đối tượng lưu trữ
        let source = self.path.join(name);

        // Kiểm tra xem nguồn có tồn tại hay không
        if !source.exists() {
            // Nếu không tồn tại, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Source {} not found", source.display()),
            ));
        }

        // Kiểm tra xem nguồn có phải là một tập tin hay không
        if !source.is_file() {
            // Nếu không phải là một tập tin, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Source {} is not a file", source.display()),
            ));
        }

        // Mở tập tin nguồn để đọc nội dung
        let file = File::open(&source)?;

        // Tạo một bộ đọc để đọc nội dung của tập tin nguồn
        let mut reader = BufReader::new(file);

        // Tạo một chuỗi để lưu trữ nội dung của tập tin nguồn
        let mut content = String::new();

        // Đọc nội dung của tập tin nguồn vào chuỗi
        reader.read_to_string(&mut content)?;

        // Trả về chuỗi nội dung với kết quả Ok
        Ok(content)
    }

    // Phương thức edit để sửa nội dung của một tập tin hoặc dữ liệu trong đối tượng lưu trữ
    fn edit(&mut self, name: &str, content: &str) -> io::Result<()> {
        // Tạo một đường dẫn mới cho nguồn bằng cách nối tên của tập tin hoặc dữ liệu vào đường dẫn của đối tượng lưu trữ
        let source = self.path.join(name);

        // Kiểm tra xem nguồn có tồn tại hay không
        if !source.exists() {
            // Nếu không tồn tại, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Source {} not found", source.display()),
            ));
        }

        // Kiểm tra xem nguồn có phải là một tập tin hay không
        if !source.is_file() {
            // Nếu không phải là một tập tin, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Source {} is not a file", source.display()),
            ));
        }

        // Mở hoặc tạo một tập tin mới với đường dẫn cho trước, cho phép ghi và xoá nội dung cũ
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&source)?;

        // Tạo một bộ ghi để ghi nội dung vào tập tin nguồn
        let mut writer = BufWriter::new(file);

        // Ghi nội dung từ tham số content vào tập tin nguồn
        writer.write_all(content.as_bytes())?;

        // Đồng bộ nội dung với hệ thống tập tin
        writer.flush()?;

        // Lấy ra kích thước mới của tập tin nguồn
        let new_size = fs::metadata(&source)?.len();

        // Tìm vị trí của đường dẫn nguồn trong danh sách các tập tin và dữ liệu của đối tượng lưu trữ
        if let Some(index) = self.files.iter().position(|p| p == &source) {
            // Nếu tìm thấy, lấy ra kích thước cũ của tập tin nguồn
            let old_size = fs::metadata(&self.files[index])?.len();

            // Trừ kích thước cũ khỏi kích thước của đối tượng lưu trữ
            self.size -= old_size;

            // Cộng kích thước mới vào kích thước của đối tượng lưu trữ
            self.size += new_size;
        }

        // Trả về kết quả Ok
        Ok(())
    }

    // Phương thức delete để xoá một tập tin hoặc dữ liệu khỏi đối tượng lưu trữ
    fn delete(&mut self, name: &str) -> io::Result<()> {
        // Tạo một đường dẫn mới cho nguồn bằng cách nối tên của tập tin hoặc dữ liệu vào đường dẫn của đối tượng lưu trữ
        let source = self.path.join(name);

        // Kiểm tra xem nguồn có tồn tại hay không
        if !source.exists() {
            // Nếu không tồn tại, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Source {} not found", source.display()),
            ));
        }

        // Kiểm tra xem nguồn có phải là một tập tin hay không
        if !source.is_file() {
            // Nếu không phải là một tập tin, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Source {} is not a file", source.display()),
            ));
        }

        // Xoá tập tin nguồn khỏi hệ thống tập tin
        fs::remove_file(&source)?;

        // Lấy ra kích thước của tập tin nguồn
        let file_size = fs::metadata(&source)?.len();

        // Trừ kích thước của tập tin nguồn khỏi kích thước của đối tượng lưu trữ
        self.size -= file_size;

        // Tìm vị trí của đường dẫn nguồn trong danh sách các tập tin và dữ liệu của đối tượng lưu trữ
        if let Some(index) = self.files.iter().position(|p| p == &source) {
            // Nếu tìm thấy, xoá đường dẫn nguồn khỏi danh sách
            self.files.remove(index);
        }

        // Trả về kết quả Ok
        Ok(())
    }

    // Phương thức share để chia sẻ một tập tin hoặc dữ liệu trong đối tượng lưu trữ với một đối tượng lưu trữ khác
    fn share(&self, name: &str, dest: &mut Storage) -> io::Result<()> {
        // Tạo một đường dẫn mới cho nguồn bằng cách nối tên của tập tin hoặc dữ liệu vào đường dẫn của đối tượng lưu trữ
        let source = self.path.join(name);

        // Kiểm tra xem nguồn có tồn tại hay không
        if !source.exists() {
            // Nếu không tồn tại, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Source {} not found", source.display()),
            ));
        }

        // Kiểm tra xem nguồn có phải là một tập tin hay không
        if !source.is_file() {
            // Nếu không phải là một tập tin, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Source {} is not a file", source.display()),
            ));
        }

        // Tạo một đường dẫn mới cho đích bằng cách nối tên của nguồn vào đường dẫn của đối tượng lưu trữ đích
        let dest_path = dest.path.join(source.file_name().unwrap());

        // Kiểm tra xem đích đã tồn tại hay chưa
        if dest_path.exists() {
            // Nếu đã tồn tại, trả về một lỗi io với thông báo
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Destination {} already exists", dest_path.display()),
            ));
        }

        // Nếu chưa tồn tại, sao chép nội dung từ nguồn sang đích
        fs::copy(&source, &dest_path)?;

        // Lấy ra kích thước của tập tin nguồn
        let file_size = fs::metadata(&source)?.len();

        // Cộng kích thước của tập tin nguồn vào kích thước của đối tượng lưu trữ đích
        dest.size += file_size;

        // Thêm đường dẫn của đích vào danh sách các tập tin và dữ liệu của đối tượng lưu trữ đích
        dest.files.push(dest_path);

        // Trả về kết quả Ok
        Ok(())
    }
}

// Khai báo một hàm để chuyển enum Access sang chuỗi
impl ToString for Access {
    fn to_string(&self) -> String {
        match self {
            Access::Private => "Private".to_string(),
            Access::Public => "Public".to_string(),
            Access::Shared => "Shared".to_string(),
        }
    }
}

// Khai báo một module để chứa các unit test
#[cfg(test)]
mod tests {
    // Nhập các thư viện cần thiết
    use super::*;
    use std::fs;
    use tempfile::tempdir;

        // Khai báo một hàm để tạo một đối tượng lưu trữ tạm thời với tên và quyền truy cập cho trước
    fn create_temp_storage(name: &str, access: Access) -> io::Result<Storage> {
        // Lấy ra đường dẫn của thư mục tạm thời của hệ thống
        let temp_dir = env::temp_dir();

        // Tạo một đường dẫn mới bằng cách nối tên của đối tượng lưu trữ vào đường dẫn của thư mục tạm thời
        let path = temp_dir.join(name);

        // Tạo một thư mục mới với đường dẫn mới
        fs::create_dir_all(&path)?;

        // Tạo một đối tượng lưu trữ mới với đường dẫn mới và quyền truy cập cho trước
        let storage = Storage::new(path.to_str().unwrap(), access)?;

        // Trả về đối tượng lưu trữ mới với kết quả Ok
        Ok(storage)
    }

    // Khai báo một hàm để kiểm tra phương thức new của struct Storage
    #[test]
    fn test_new() {
        // Gọi phương thức new với tên và quyền truy cập cho trước
        let storage = Storage::new("test", Access::Private);

        // Kiểm tra xem kết quả có phải là Ok hay không
        assert!(storage.is_ok());

        // Nếu là Ok, lấy ra giá trị của đối tượng lưu trữ
        let storage = storage.unwrap();

        // Kiểm tra xem các trường của đối tượng lưu trữ có khớp với các giá trị mong muốn hay không
        assert_eq!(storage.name, "test");
        assert_eq!(storage.path, Path::new("test"));
        assert_eq!(storage.size, 0);
        assert_eq!(storage.files.len(), 0);
        assert_eq!(*storage.access.read().unwrap(), Access::Private);

        // Xoá đối tượng lưu trữ khỏi hệ thống tập tin
        fs::remove_dir_all("test").unwrap();
    }

    // Khai báo một hàm để kiểm tra phương thức open của struct Storage
    #[test]
    fn test_open() {
        // Tạo một đối tượng lưu trữ tạm thời với tên và quyền truy cập cho trước
        let storage = create_temp_storage("test", Access::Public).unwrap();

        // Lấy ra đường dẫn của đối tượng lưu trữ tạm thời
        let path = storage.path.clone();

        // Gọi phương thức open với tên của đối tượng lưu trữ tạm thời
        let storage = Storage::open("test");

        // Kiểm tra xem kết quả có phải là Ok hay không
        assert!(storage.is_ok());

        // Nếu là Ok, lấy ra giá trị của đối tượng lưu trữ
        let storage = storage.unwrap();

        // Kiểm tra xem các trường của đối tượng lưu trữ có khớp với các giá trị mong muốn hay không
        assert_eq!(storage.name, "test");
        assert_eq!(storage.path, path);
        assert_eq!(storage.size, 0);
        assert_eq!(storage.files.len(), 0);
        assert_eq!(*storage.access.read().unwrap(), Access::Public);

        // Xoá đối tượng lưu trữ khỏi hệ thống tập tin
        fs::remove_dir_all(path).unwrap();
    }

    // Khai báo một hàm để kiểm tra phương thức info của struct Storage
    #[test]
    fn test_info() {
        // Tạo một đối tượng lưu trữ tạm thời với tên và quyền truy cập cho trước
        let storage = create_temp_storage("test", Access::Shared).unwrap();

        // Gọi phương thức info để lấy thông tin của đối tượng lưu trữ
        let info = storage.info();

        // Tạo một chuỗi để lưu trữ thông tin mong muốn của đối tượng lưu trữ
        let expected_info = format!(
            "Name: test\nPath: {:?}\nSize: 0 bytes\nFiles: 0 items\nAccess: Shared\n",
            storage.path
        );

        // Kiểm tra xem thông tin có khớp với thông tin mong muốn hay không
        assert_eq!(info, expected_info);

        // Xoá đối tượng lưu trữ khỏi hệ thống tập tin
        fs::remove_dir_all(storage.path).unwrap();
    }

    // Khai báo một hàm để kiểm tra phương thức set_access của struct Storage
    #[test]
    fn test_set_access() {
        // Tạo một đối tượng lưu trữ tạm thời với tên và quyền truy cập cho trước
        let storage = create_temp_storage("test", Access::Private).unwrap();

        // Lấy ra đường dẫn của đối tượng lưu trữ tạm thời
        let path = storage.path.clone();

        // Gọi phương thức set_access để thiết lập quyền truy cập mới cho đối tượng lưu trữ
        let result = storage.set_access(Access::Public);

        // Kiểm tra xem kết quả có phải là Ok hay không
        assert!(result.is_ok());

        // Kiểm tra xem quyền truy cập của đối tượng lưu trữ có được cập nhật hay không
        assert_eq!(*storage.access.read().unwrap(), Access::Public);

        // Kiểm tra xem thư mục .access và tập tin .access có được tạo hay không
        assert!(path.join(".access").is_dir());
        assert!(path.join(".access/.access").is_file());

        // Mở tập tin .access để đọc nội dung
        let file = File::open(path.join(".access/.access")).unwrap();

        // Tạo một bộ đọc để đọc nội dung của tập tin .access
        let mut reader = BufReader::new(file);

        // Tạo một chuỗi để lưu trữ nội dung của tập tin .access
        let mut content = String::new();

        // Đọc nội dung của tập tin .access vào chuỗi
        reader.read_to_string(&mut content).unwrap();

        // Loại bỏ các khoảng trắng ở đầu và cuối chuỗi
        content = content.trim().to_string();

        // Kiểm tra xem nội dung của chuỗi có khớp với quyền truy cập mới hay không
        assert_eq!(content, "Public");

        // Xoá đối tượng lưu trữ khỏi hệ thống tập tin
        fs::remove_dir_all(path).unwrap();
    }

    // Khai báo một hàm để kiểm tra phương thức upload của struct Storage
    #[test]
    fn test_upload() {
        // Tạo một đối tượng lưu trữ tạm thời với tên và quyền truy cập cho trước
        let mut storage = create_temp_storage("test", Access::Private).unwrap();

        // Lấy ra đường dẫn của đối tượng lưu trữ tạm thời
        let path = storage.path.clone();

        // Tạo một thư mục tạm thời
        let dir = tempdir().unwrap();

        // Lấy ra đường dẫn của thư mục tạm thời
        let temp_path = dir.path();

        // Tạo một đường dẫn mới cho nguồn bằng cách nối tên của tập tin vào đường dẫn của thư mục tạm thời
        let source = temp_path.join("source.txt");

        // Mở hoặc tạo một tập tin mới với đường dẫn cho trước, cho phép ghi và xoá nội dung cũ
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&source)
            .unwrap();

        // Tạo một bộ ghi để ghi nội dung vào tập tin nguồn
        let mut writer = BufWriter::new(file);

        // Ghi nội dung từ chuỗi "Hello, world!" vào tập tin nguồn
        writer.write_all(b"Hello, world!").unwrap();

        // Đồng bộ nội dung với hệ thống tập tin
        writer.flush().unwrap();

        // Gọi phương thức upload để tải lên tập tin nguồn vào đối tượng lưu trữ
        let result = storage.upload(&source);

        // Kiểm tra xem kết quả có phải là Ok hay không
        assert!(result.is_ok());

        // Kiểm tra xem kích thước của đối tượng lưu trữ có được cập nhật hay không
        assert_eq!(storage.size, 13);

        // Kiểm tra xem danh sách các tập tin và dữ liệu của đối tượng lưu trữ có được cập nhật hay không
        assert_eq!(storage.files.len(), 1);
        assert_eq!(storage.files[0], path.join("source.txt"));

        // Kiểm tra xem tập tin nguồn có được sao chép sang đối tượng lưu trữ hay không
        assert!(path.join("source.txt").is_file());

        // Xoá đối tượng lưu trữ khỏi hệ thống tập tin
        fs::remove_dir_all(path).unwrap();
    }

    // Khai báo một hàm để kiểm tra phương thức download của struct Storage
    #[test]
    fn test_download() {
        // Tạo một đối tượng lưu trữ tạm thời với tên và quyền truy cập cho trước
        let mut storage = create_temp_storage("test", Access::Private).unwrap();

        // Lấy ra đường dẫn của đối tượng lưu trữ tạm thời
        let path = storage.path.clone();

        // Tạo một thư mục tạm thời
        let dir = tempdir().unwrap();

        // Lấy ra đường dẫn của thư mục tạm thời
        let temp_path = dir.path();

        // Tạo một đường dẫn mới cho nguồn bằng cách nối tên của tập tin vào đường dẫn của đối tượng lưu trữ
        let source = path.join("source.txt");

        // Mở hoặc tạo một tập tin mới với đường dẫn cho trước, cho phép ghi và xoá nội dung cũ
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&source)
            .unwrap();

        // Tạo một bộ ghi để ghi nội dung vào tập tin nguồn
        let mut writer = BufWriter::new(file);

        // Ghi nội dung từ chuỗi "Hello, world!" vào tập tin nguồn
        writer.write_all(b"Hello, world!").unwrap();

        // Đồng bộ nội dung với hệ thống tập tin
        writer.flush().unwrap();

        // Cập nhật kích thước và danh sách các tập tin và dữ liệu của đối tượng lưu trữ
        storage.size += 13;
        storage.files.push(source.clone());

        // Tạo một đường dẫn mới cho đích bằng cách nối tên của nguồn vào đường dẫn của thư mục tạm thời
        let dest = temp_path.join("source.txt");

        // Gọi phương thức download để tải xuống tập tin nguồn từ đối tượng lưu trữ
        let result = storage.download("source.txt", &dest);

        // Kiểm tra xem kết quả có phải là Ok hay không
        assert!(result.is_ok());

        // Kiểm tra xem kích thước của đối tượng lưu trữ có không thay đổi hay không
        assert_eq!(storage.size, 13);

        // Kiểm tra xem danh sách các tập tin và dữ liệu của đối tượng lưu trữ có không thay đổi hay không
        assert_eq!(storage.files.len(), 1);
        assert_eq!(storage.files[0], source);

        // Kiểm tra xem tập tin nguồn có được sao chép sang đích hay không
        assert!(dest.is_file());

        // Mở tập tin đích để đọc nội dung
        let file = File::open(&dest).unwrap();

        // Tạo một bộ đọc để đọc nội dung của tập tin đích
        let mut reader = BufReader::new(file);

        // Tạo một chuỗi để lưu trữ nội dung của tập tin đích
        let mut content = String::new();

        // Đọc nội dung của tập tin đích vào chuỗi
        reader.read_to_string(&mut content).unwrap();

        // Loại bỏ các khoảng trắng ở đầu và cuối chuỗi
        content = content.trim().to_string();

        // Kiểm tra xem nội dung của chuỗi có khớp với nội dung của tập tin nguồn hay không
        assert_eq!(content, "Hello, world!");

        // Xoá đối tượng lưu trữ khỏi hệ thống tập tin
        fs::remove_dir_all(path).unwrap();
    }

    // Khai báo một hàm để kiểm tra phương thức view của struct Storage
    #[test]
    fn test_view() {
        // Tạo một đối tượng lưu trữ tạm thời với tên và quyền truy cập cho trước
        let mut storage = create_temp_storage("test", Access::Private).unwrap();

        // Lấy ra đường dẫn của đối tượng lưu trữ tạm thời
        let path = storage.path.clone();

        // Tạo một đường dẫn mới cho nguồn bằng cách nối tên của tập tin vào đường dẫn của đối tượng lưu trữ
        let source = path.join("source.txt");

        // Mở hoặc tạo một tập tin mới với đường dẫn cho trước, cho phép ghi và xoá nội dung cũ
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&source)
            .unwrap();

        // Tạo một bộ ghi để ghi nội dung vào tập tin nguồn
        let mut writer = BufWriter::new(file);

        // Ghi nội dung từ chuỗi "Hello, world!" vào tập tin nguồn
        writer.write_all(b"Hello, world!").unwrap();

        // Đồng bộ nội dung với hệ thống tập tin
        writer.flush().unwrap();

        // Cập nhật kích thước và danh sách các tập tin và dữ liệu của đối tượng lưu trữ
        storage.size += 13;
        storage.files.push(source.clone());

        // Gọi phương thức view để xem nội dung của tập tin nguồn trong đối tượng lưu trữ
        let result = storage.view("source.txt");

        // Kiểm tra xem kết quả có phải là Ok hay không
        assert!(result.is_ok());

        // Nếu là Ok, lấy ra giá trị của chuỗi nội dung
        let content = result.unwrap();

        // Loại bỏ các khoảng trắng ở đầu và cuối chuỗi
        let content = content.trim().to_string();

        // Kiểm tra xem chuỗi nội dung có khớp với nội dung của tập tin nguồn hay không
        assert_eq!(content, "Hello, world!");

        // Xoá đối tượng lưu trữ khỏi hệ thống tập tin
        fs::remove_dir_all(path).unwrap();
    }

    // Khai báo một hàm để kiểm tra phương thức edit của struct Storage
    #[test]
    fn test_edit() {
        // Tạo một đối tượng lưu trữ tạm thời với tên và quyền truy cập cho trước
        let mut storage = create_temp_storage("test", Access::Private).unwrap();

        // Lấy ra đường dẫn của đối tượng lưu trữ tạm thời
        let path = storage.path.clone();

        // Tạo một đường dẫn mới cho nguồn bằng cách nối tên của tập tin vào đường dẫn của đối tượng lưu trữ
        let source = path.join("source.txt");

        // Mở hoặc tạo một tập tin mới với đường dẫn cho trước, cho phép ghi và xoá nội dung cũ
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&source)
            .unwrap();

        // Tạo một bộ ghi để ghi nội dung vào tập tin nguồn
        let mut writer = BufWriter::new(file);

        // Ghi nội dung từ chuỗi "Hello, world!" vào tập tin nguồn
        writer.write_all(b"Hello, world!").unwrap();

        // Đồng bộ nội dung với hệ thống tập tin
        writer.flush().unwrap();

        // Cập nhật kích thước và danh sách các tập tin và dữ liệu của đối tượng lưu trữ
        storage.size += 13;
        storage.files.push(source.clone());

        // Gọi phương thức edit để sửa nội dung của tập tin nguồn trong đối tượng lưu trữ
        let result = storage.edit("source.txt", "Goodbye, world!");

        // Kiểm tra xem kết quả có phải là Ok hay không
        assert!(result.is_ok());

        // Kiểm tra xem kích thước của đối tượng lưu trữ có được cập nhật hay không
        assert_eq!(storage.size, 15);

        // Kiểm tra xem danh sách các tập tin và dữ liệu của đối tượng lưu trữ có không thay đổi hay không
        assert_eq!(storage.files.len(), 1);
        assert_eq!(storage.files[0], source);

        // Mở tập tin nguồn để đọc nội dung
        let file = File::open(&source).unwrap();

        // Tạo một bộ đọc để đọc nội dung của tập tin nguồn
        let mut reader = BufReader::new(file);

        // Tạo một chuỗi để lưu trữ nội dung của tập tin nguồn
        let mut content = String::new();

        // Đọc nội dung của tập tin nguồn vào chuỗi
        reader.read_to_string(&mut content).unwrap();

        // Loại bỏ các khoảng trắng ở đầu và cuối chuỗi
        content = content.trim().to_string();

        // Kiểm tra xem chuỗi nội dung có khớp với nội dung mới hay không
        assert_eq!(content, "Goodbye, world!");

        // Xoá đối tượng lưu trữ khỏi hệ thống tập tin
        fs::remove_dir_all(path).unwrap();
    }

    // Khai báo một hàm để kiểm tra phương thức delete của struct Storage
    #[test]
    fn test_delete() {
        // Tạo một đối tượng lưu trữ tạm thời với tên và quyền truy cập cho trước
        let mut storage = create_temp_storage("test", Access::Private).unwrap();

        // Lấy ra đường dẫn của đối tượng lưu trữ tạm thời
        let path = storage.path.clone();

        // Tạo một đường dẫn mới cho nguồn bằng cách nối tên của tập tin vào đường dẫn của đối tượng lưu trữ
        let source = path.join("source.txt");

        // Mở hoặc tạo một tập tin mới với đường dẫn cho trước, cho phép ghi và xoá nội dung cũ
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&source)
            .unwrap();

        // Tạo một bộ ghi để ghi nội dung vào tập tin nguồn
        let mut writer = BufWriter::new(file);

        // Ghi nội dung từ chuỗi "Hello, world!" vào tập tin nguồn
        writer.write_all(b"Hello, world!").unwrap();

        // Đồng bộ nội dung với hệ thống tập tin
        writer.flush().unwrap();

        // Cập nhật kích thước và danh sách các tập tin và dữ liệu của đối tượng lưu trữ
        storage.size += 13;
        storage.files.push(source.clone());

        // Gọi phương thức delete để xoá tập tin nguồn khỏi đối tượng lưu trữ
        let result = storage.delete("source.txt");

        // Kiểm tra xem kết quả có phải là Ok hay không
        assert!(result.is_ok());

        // Kiểm tra xem kích thước của đối tượng lưu trữ có được cập nhật hay không
        assert_eq!(storage.size, 0);

        // Kiểm tra xem danh sách các tập tin và dữ liệu của đối tượng lưu trữ có được cập nhật hay không
        assert_eq!(storage.files.len(), 0);

        // Kiểm tra xem tập tin nguồn có được xoá khỏi hệ thống tập tin hay không
        assert!(!source.is_file());

        // Xoá đối tượng lưu trữ khỏi hệ thống tập tin
        fs::remove_dir_all(path).unwrap();
    }

    // Khai báo một hàm để kiểm tra phương thức share của struct Storage
    #[test]
    fn test_share() {
        // Tạo một đối tượng lưu trữ tạm thời với tên và quyền truy cập cho trước
        let mut storage = create_temp_storage("test", Access::Private).unwrap();

        // Lấy ra đường dẫn của đối tượng lưu trữ tạm thời
        let path = storage.path.clone();

        // Tạo một đối tượng lưu trữ tạm thời khác với tên và quyền truy cập cho trước
        let mut dest = create_temp_storage("dest", Access::Public).unwrap();

        // Lấy ra đường dẫn của đối tượng lưu trữ tạm thời khác
        let dest_path = dest.path.clone();

        // Tạo một đường dẫn mới cho nguồn bằng cách nối tên của tập tin vào đường dẫn của đối tượng lưu trữ
        let source = path.join("source.txt");

        // Mở hoặc tạo một tập tin mới với đường dẫn cho trước, cho phép ghi và xoá nội dung cũ
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&source)
            .unwrap();

        // Tạo một bộ ghi để ghi nội dung vào tập tin nguồn
        let mut writer = BufWriter::new(file);

        // Ghi nội dung từ chuỗi "Hello, world!" vào tập tin nguồn
        writer.write_all(b"Hello, world!").unwrap();

        // Đồng bộ nội dung với hệ thống tập tin
        writer.flush().unwrap();

        // Cập nhật kích thước và danh sách các tập tin và dữ liệu của đối tượng lưu trữ
        storage.size += 13;
        storage.files.push(source.clone());

        // Gọi phương thức share để chia sẻ tập tin nguồn với đối tượng lưu trữ khác
        let result = storage.share("source.txt", &mut dest);

        // Kiểm tra xem kết quả có phải là Ok hay không
        assert!(result.is_ok());

        // Kiểm tra xem kích thước của đối tượng lưu trữ có không thay đổi hay không
        assert_eq!(storage.size, 13);

        // Kiểm tra xem danh sách các tập tin và dữ liệu của đối tượng lưu trữ có không thay đổi hay không
        assert_eq!(storage.files.len(), 1);
        assert_eq!(storage.files[0], source);

        // Kiểm tra xem kích thước của đối tượng lưu trữ khác có được cập nhật hay không
        assert_eq!(dest.size, 13);

        // Kiểm tra xem danh sách các tập tin và dữ liệu của đối tượng lưu trữ khác có được cập nhật hay không
        assert_eq!(dest.files.len(), 1);
        assert_eq!(dest.files[0], dest_path.join("source.txt"));

        // Kiểm tra xem tập tin nguồn có được sao chép sang đối tượng lưu trữ khác hay không
        assert!(dest_path.join("source.txt").is_file());

        // Xoá hai đối tượng lưu trữ khỏi hệ thống tập tin
        fs::remove_dir_all(path).unwrap();
        fs::remove_dir_all(dest_path).unwrap();
    }
}
}