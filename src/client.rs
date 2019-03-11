pub mod shared {
   #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct Data<T> where T: PartialOrd + PartialEq {
        pub high: T,
        pub low: T,
        pub opening: T,
        pub closing: T,
    }
    pub trait Serialize {
        fn serialize(&self) -> String;
    }

    pub trait Parse<T> {
        fn parse(data: &str) -> Option<T>;
    }
   
    /// # Brief
    /// 
    /// Converts the struct to a string
    /// 
    /// # Example
    /// 
    /// ```
    /// use ift611_user::shared::{Data, Serialize};
    /// let d = Data {high: 32, low: 32, opening: 32, closing: 32};
    /// assert_eq!(d.serialize(), "Data{32,32,32,32}");
    /// ```
    impl Serialize for Data<u32> {
        fn serialize(&self) -> String {
            format!("Data{{{},{},{},{}}}", self.high, self.low, self.opening, self.closing)
        }
    }

    /// # Brief
    /// 
    /// Converts a formatted string to a Data instance 
    /// 
    /// # Example
    /// 
    /// ```
    /// use ift611_user::shared::{Data, Parse};
    /// let s = "Data{32,32,32,32}";
    /// assert_eq!(Data::parse(s).unwrap(), Data{high:32, low:32, opening:32, closing:32});
    /// ```
    impl Parse<Data<u32>> for Data<u32> {
        ///Disclaimer: this is probably dumb, but it seems to be working so it's okay for a V1
        fn parse(data: &str) -> Option<Data<u32>> {
            if data.starts_with("Data{") && data.ends_with('}') {
                let v : Vec<&str> = data[5..data.len()-1]
                .split(',')
                .collect();
                let v : Vec<u32> = v.into_iter()
                .map(
                    |x| 
                    x.parse::<u32>()
                    .unwrap())
                .collect();
                Some(Data{high: v[0], low: v[1], opening:v[2], closing:v[3]})
            }
            else {
                None
            }
        }
    }
}

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
        /// use ift611_user::html::Path;
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
        /// use ift611_user::html::Path;
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
        /// use ift611_user::html::Path;
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
    /// use ift611_user::html::get_string;
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
    /// use ift611_user::html::{Path, get_custom_string};
    /// let p = Path::new("~\\rust_projects\\ift611\\ift_611user");
    /// assert_eq!(get_custom_string(&p), "GET ~/rust_projects/ift611/ift_611user HTTP/1.1\r\n")
    /// ```
    pub fn get_custom_string(p: &Path) -> String {
        format!("GET {} HTTP/1.1\r\n", &p.to_string_custom("/"))    
    }
}

pub mod dummy_dot_product {
    use crate::client::shared::Data;
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
    /// use ift611_user::dummy_dot_product::{Action, get_decision};
    /// use ift611_user::shared::Data;
    /// let mut expected_sell_data : [Data<usize>; 100] = [Data { high: 0, low: 0, opening: 0, closing: 0}; 100];
    /// (0..100)
    /// .into_iter()
    /// .for_each(
    ///     |x|
    ///     expected_sell_data[x] = Data { high: x+1, low: x, opening: x, closing: x+1 });
    /// let a = get_decision(&expected_sell_data);
    /// assert_eq!(a, Action::Sell);
    /// let mut expected_buy_data : [Data<usize>; 100] = [Data { high: 0, low: 0, opening: 0, closing: 0}; 100];
    /// (0..100)
    /// .into_iter()
    /// .for_each(
    ///     |x|
    ///     expected_buy_data[x] = Data { high: 101-x, low: 101-x-1, opening: 101-x, closing: 101-x-1 });
    /// let a = get_decision(&expected_buy_data);
    /// assert_eq!(a, Action::Buy);
    /// ```
    pub fn get_decision<T>(data: &[Data<T>; 100]) -> Action where T : PartialOrd {
        let first = &data.first().unwrap();
        let last = &data.first().unwrap();

        // super smart ai decision process
        if last.closing < first.opening {
            Action::Buy
        }
        else {
            Action::Sell
        }
    }
}

use crate::common;
pub fn execute() {
    println!("Lib Hello World!");
    common::execute();
}



