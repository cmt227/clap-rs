use std::collections::HashSet;
use std::collections::BTreeSet;
use std::fmt::{ Display, Formatter, Result };

pub struct PosBuilder<'n> {
    pub name: &'n str,
    /// The string of text that will displayed to the user when the application's
    /// `help` text is displayed
    pub help: Option<&'n str>,
    /// If this is a required by default when using the command line program
    /// i.e. a configuration file that's required for the program to function
    /// **NOTE:** required by default means, it is required *until* mutually
    /// exclusive arguments are evaluated.
    pub required: bool,
    /// Allow multiple occurrences of an option argument such as "-c some -c other"
    pub multiple: bool,
    /// A list of names of other arguments that are *required* to be used when 
    /// this flag is used
    pub requires: Option<HashSet<&'n str>>,
    /// A list of names for other arguments that *may not* be used with this flag
    pub blacklist: Option<HashSet<&'n str>>,
    /// A list of possible values for this argument
    pub possible_vals: Option<BTreeSet<&'n str>>,
    /// The index of the argument
    pub index: u8 
}

impl<'n> Display for PosBuilder<'n> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{}{}{}", if self.required { "<" } else {"["}, self.name,if self.required { ">" } else {"]"}, if self.multiple {"..."}else{""})
    }
}

impl<'n> Clone for PosBuilder<'n> {
    fn clone(&self) -> PosBuilder<'n> {
        PosBuilder {
            name: self.name.clone(),
            index: self.index,
            help: self.help.clone(),
            required: self.required,
            multiple: self.multiple,
            blacklist: self.blacklist.clone(),
            requires: self.requires.clone(),
            possible_vals: self.possible_vals.clone(),
        }
    }
}