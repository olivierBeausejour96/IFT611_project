#[derive(Debug, PartialEq)]
pub struct Path {
    p: Vec<String>,
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
        let p: Vec<String> = s
            .split(|c| c == '\\' || c == '/')
            .map(|x| x.to_string())
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
