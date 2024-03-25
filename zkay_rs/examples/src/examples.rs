use lazy_static::lazy_static;
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};
use zkay_config::config::CFG;
use zkay_utils::helpers::get_contract_names;
lazy_static! {
    pub static ref EXAMPLES_DIR: PathBuf = PathBuf::from(&env!("CARGO_MANIFEST_DIR"));//std::env::current_dir().unwrap();
    pub static ref CODE_DIR: PathBuf = (*EXAMPLES_DIR).join("code");///Users/lisheng/mygit/arron/zkay-rs/zkay_rs/examples/code
    pub static ref TYPE_ERROR_DIR: PathBuf = (*EXAMPLES_DIR).join("type_errors");
    pub static ref OTHERS_DIR: PathBuf = (*EXAMPLES_DIR).join("others");
    pub static ref SIMPLE_STORAGE: Example = Example::new(
        (*CODE_DIR)
            .join("SimpleStorage.zkay")
            .to_str()
            .unwrap()
            .to_owned()
    );
    pub static ref FUNCTIONS: Example = Example::new(
        (*CODE_DIR)
            .join("Functions.zkay")
            .to_str()
            .unwrap()
            .to_owned()
    );
    pub static ref ADDITION: Example = Example::new(
        (*CODE_DIR)
            .join("Addition.zkay")
            .to_str()
            .unwrap()
            .to_owned()
    );
    pub static ref EMPTY: Example =
        Example::new((*CODE_DIR).join("Empty.zkay").to_str().unwrap().to_owned());
    pub static ref SIMPLE: Example =
        Example::new((*CODE_DIR).join("Simple.zkay").to_str().unwrap().to_owned());
    pub static ref CONTROL_FLOW: Example = Example::new(
        (*CODE_DIR)
            .join("ControlFlow.zkay")
            .to_str()
            .unwrap()
            .to_owned()
    );
    pub static ref ANALYSIS: Example = Example::new(
        (*CODE_DIR)
            .join("Analysis.zkay")
            .to_str()
            .unwrap()
            .to_owned()
    );
    pub static ref PRIVATE_ADDITION: Example = Example::new(
        (*CODE_DIR)
            .join("PrivateAddition.zkay")
            .to_str()
            .unwrap()
            .to_owned()
    );
    pub static ref POWER_GRID: Example = Example::new(
        (*CODE_DIR)
            .join("PowerGrid.zkay")
            .to_str()
            .unwrap()
            .to_owned()
    );
    pub static ref FINAL_USE_BEFORE_WRITE: Example = Example::new(
        (*OTHERS_DIR)
            .join("FinalUseBeforeWrite.zkay")
            .to_str()
            .unwrap()
            .to_owned()
    );
    pub static ref ADD_USER: Example = Example::new(
        (*OTHERS_DIR)
            .join("AddUser.sol")
            .to_str()
            .unwrap()
            .to_owned()
    );
    pub static ref EMPTY_NORMALIZED: String = format!(
        "pragma zkay >= {} ; contract Empty {{ }} ",
        CFG.lock().unwrap().zkay_version()
    );
    pub static ref SIMPLE_STORAGE_NORMALIZED: String = format!(
        r#"pragma zkay >= {} ; contract SimpleStorage {{ "
                            "uint @ all storedData ; "
                            "function set ( uint @ all x ) public {{ storedData = x ; }} "
                            "function get ( ) public returns ( uint @ all ) {{ return storedData ; }} }} "#,
        CFG.lock().unwrap().zkay_version()
    );
    pub static ref ALL_EXAMPLES: Vec<(String, Example)> =
        collect_examples(&CODE_DIR.to_str().unwrap().to_owned());
    pub static ref TYPE_ERROR_EXAMPLES: Vec<(String, Example)> =
        collect_examples(&TYPE_ERROR_DIR.to_str().unwrap().to_owned());
}
pub struct Example {
    pub file_location: String,
    pub file_name: String,
}

impl Example {
    pub fn new(file_location: String) -> Self {
        let file_name = Path::new(&file_location.clone())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        Self {
            file_location,
            file_name,
        }
    }

    pub fn code(&self) -> String {
        let mut file = File::open(&self.file_location).expect("");
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        let s = s.replace("\t", &CFG.lock().unwrap().user_config.indentation());
        s
    }

    pub fn stream(&self) { // return FileStream(self.file_location)
    }
    pub fn name(&self) -> String {
        let names = get_contract_names(&self.file_location);
        assert!(names.len() == 1);
        names[0].clone()
    }

    pub fn normalized(&self) -> String {
        if &self.name() == "Empty" {
            EMPTY_NORMALIZED.clone()
        } else if &self.name() == "SimpleStorage" {
            SIMPLE_STORAGE_NORMALIZED.clone()
        } else {
            String::new()
        }
    }
}

pub fn collect_examples(directory: &String) -> Vec<(String, Example)> {
    let mut examples = vec![];
    println!("{:?}", directory);
    for f in fs::read_dir(directory).unwrap() {
        // println!("=f=={:?}",f);
        if f.as_ref()
            .unwrap()
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .ends_with(".zkay")
        {
            let e = Example::new(f.unwrap().path().to_str().unwrap().to_string());
            examples.push((e.name(), e));
        }
    }
    examples
}

pub fn get_code_example(name: &String) -> Vec<(String, Example)> {
    let e = Example::new((*CODE_DIR).join(name).to_str().unwrap().to_string());
    vec![(e.name(), e)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_examples_abs_path() {
        // let this_file = file!();
        // println!("defined in file: {this_file}");
        //         println!("{:?}", *EXAMPLES_DIR);
        for (name, _example) in ALL_EXAMPLES.iter() {
            println!("{:?}", name);
            // let  _ast = get_processed_ast(&example.code(), Some(!0b00001000));
            assert!(true);
        }
        assert!(true);
    }

    #[test]
    pub fn test_examples_abs_path_env() {
        use std::path::Path;

        // println!("FILE: {:?}", FILE);
        // println!("src path: {:?}", Path::new(FILE).parent());
        println!(
            "directory is: {:?}",
            crate::file_abs_workspace!().parent().unwrap()
        );

        // println!(
        //     "===directory is: {:?}",
        //     crate::file_abs!().parent().unwrap()
        // );
        // let this_file = file!();
        // println!("defined in file: {this_file}");
        //         println!("{:?}", *EXAMPLES_DIR);
        for (name, _example) in ALL_EXAMPLES.iter() {
            // println!("{:?}", name);
            // let  _ast = get_processed_ast(&example.code(), Some(!0b00001000));
            assert!(true);
        }
        assert!(true);
    }
}

// const FILE: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/", file!());

// #[macro_export]
// macro_rules! file_abs {
//     () => {
//         std::path::Path::new(&(env!("CARGO_MANIFEST_DIR").to_owned() + file!()))
//     };
// }

#[macro_export]
macro_rules! file_abs_workspace {
    () => {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(crate::examples::pop_first_two_path_components(file!()))
    };
}

pub fn pop_first_two_path_components(path: &str) -> PathBuf {
    let mut components = std::path::Path::new(path).components();
    components.next();
    components.next();
    components.as_path().to_path_buf()
}
