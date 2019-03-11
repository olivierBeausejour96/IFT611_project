use crate::common;

pub mod html {
    #[derive(Debug, PartialEq)]
    pub struct Path {
        p : Vec<String>,
    }

    impl Path {

        /// # Brief
        /// 
        /// Construct a new Path instance from a string formatted with either / or \\ delimiters
        /// 
        /// # Example
        /// 
        /// ```
        /// use IFT611_project::client::html::Path;
        /// let sp0 = "~\\rust_projects\\ift611\\ift_611user\\";
        /// let sp1 = "~/rust_projects/ift611/ift_611user/";
        /// let p0 = Path::new(sp0);
        /// let p1 = Path::new(sp1);
        /// 
        /// assert_eq!(p0, p1);
        /// ```
        pub fn new(s: &str) -> Path {
            let p : Vec<&str> = s.split(|c| c == '\\' || c == '/')
            .collect();
            let p : Vec<String> = p.into_iter()
            .map(
                |x|
                String::from(x))
            .collect();
            Path { p }
        }

        /// # Brief
        /// 
        /// Construct a string representing the Path using the default delimiter '/'
        /// This function is simply defined as:
        /// self.to_string_custom("/")
        /// 
        /// # Example
        /// 
        /// ```
        /// use IFT611_project::client::html::Path;
        /// let p = Path::new("~/rust_projects/ift611/ift_611user/");
        /// assert_eq!(p.to_string(), "~/rust_projects/ift611/ift_611user/");
        /// ```
        pub fn to_string(&self) -> String {
            self.to_string_custom("/")
        }

        /// # Brief
        /// 
        /// Construct a string representing the Path using the given delimiter
        /// 
        /// # Example
        /// 
        /// ```
        /// use IFT611_project::client::html::Path;
        /// let p = Path::new("~/rust_projects/ift611/ift_611user/");
        /// assert_eq!(p.to_string_custom("\\"), "~\\rust_projects\\ift611\\ift_611user\\");
        /// ```
        pub fn to_string_custom(&self, delim: &str) -> String {
            let mut s = String::new();
            for r in &self.p {
                s.push_str(&r);
                s.push_str(delim);
            }
            s.pop();
            s
        }
    }

    /// # Brief 
    /// 
    /// Constructs a default html get Request for the root path
    /// 
    /// # Example
    /// 
    /// ```
    /// use IFT611_project::client::html::get_string;
    /// assert_eq!(get_string(), "GET / HTTP/1.1\r\n");
    /// ```
    pub fn get_string() -> String {
        get_custom_string(&Path::new("/"))
    }

    /// # Brief
    /// 
    /// Constructs a default html get Request for the given path
    /// 
    /// # Example
    /// 
    /// ```
    /// use IFT611_project::client::html::{Path, get_custom_string};
    /// let p = Path::new("~\\rust_projects\\ift611\\ift_611user");
    /// assert_eq!(get_custom_string(&p), "GET ~/rust_projects/ift611/ift_611user HTTP/1.1\r\n")
    /// ```
    pub fn get_custom_string(p: &Path) -> String {
        format!("GET {} HTTP/1.1\r\n", &p.to_string_custom("/"))    
    }
}

pub mod dummy_dot_product {
    use super::common::Record;
    #[derive(Debug, PartialEq)]
    pub enum Action {
        Sell,
        Buy,
        Hold
    }

    /// # Brief
    /// 
    /// Given a window period of 100 time units, return either a buy or sell action using super dooper complex algorithm
    /// 
    /// # Example
    /// 
    /// ```
    /// use IFT611_project::client::dummy_dot_product::{Action, get_decision};
    /// use IFT611_project::common::Record;
    /// assert_eq!(5,5);
    /// let mut expected_sell_data = [Record {open: 32.0, high: 32.0, low: 32.0, close: 32.0, volume: 64.0}; 100];
    /// (0..100).into_iter()
    /// .for_each(
    ///     |x|
    ///     expected_sell_data[x] = Record {open: (x as f32), high: ((x+1) as f32), low: (x as f32), close: ((x+1) as f32), volume: 64.0 });
    /// let a = get_decision(&expected_sell_data);
    /// assert_eq!(a, Action::Sell);
    /// ```
    pub fn get_decision(data: &[Record; 100]) -> Action {
        let first = &data.first().unwrap();
        let last = &data.first().unwrap();

        // super smart ai decision process
        if last.close < first.open {
            Action::Buy
        }
        else {
            Action::Sell
        }
    }
}

pub fn execute() {
    println!("Lib Hello World!");
    common::execute();
}



