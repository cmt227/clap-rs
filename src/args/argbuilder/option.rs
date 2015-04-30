use std::collections::HashSet;
use std::collections::BTreeSet;
use std::fmt::{ Display, Formatter, Result };

pub struct OptBuilder<'n> {
    pub name: &'n str,
    /// The short version (i.e. single character) of the argument, no preceding `-`
    pub short: Option<char>,
    /// The long version of the flag (i.e. word) without the preceding `--`
    pub long: Option<&'n str>,
    /// The string of text that will displayed to the user when the application's
    /// `help` text is displayed
    pub help: Option<&'n str>,
    /// Allow multiple occurrences of an option argument such as "-c some -c other"
    pub multiple: bool,
    /// A list of names for other arguments that *may not* be used with this flag
    pub blacklist: Option<HashSet<&'n str>>,
    /// If this is a required by default when using the command line program
    /// i.e. a configuration file that's required for the program to function
    /// **NOTE:** required by default means, it is required *until* mutually
    /// exclusive arguments are evaluated.
    pub required: bool,
    /// A list of possible values for this argument
    pub possible_vals: Option<BTreeSet<&'n str>>,
    /// A list of names of other arguments that are *required* to be used when 
    /// this flag is used
    pub requires: Option<HashSet<&'n str>>,
}

impl<'n> Display for OptBuilder<'n> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} <{}>{}", if self.long.is_some() { format!("--{}", self.long.unwrap())} else {format!("-{}", self.short.unwrap())}, self.name, if self.multiple{"..."}else{""})
    }
}

impl<'n> Clone for OptBuilder<'n> {
    fn clone(&self) -> OptBuilder<'n> {
        OptBuilder {
            name: self.name.clone(),
            long: self.long.clone(),
            help: self.help.clone(),
            required: self.required,
            multiple: self.multiple,
            blacklist: self.blacklist.clone(),
            requires: self.requires.clone(),
            short: self.short.clone(),
            possible_vals: self.possible_vals.clone(),
        }
    }
}