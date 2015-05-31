initSidebarItems({"struct":[["App","Used to create a representation of a command line program and all possible command line arguments for parsing at runtime."],["Arg","The abstract representation of a command line argument used by the consumer of the library. Used to set all the options and relationships that define a valid argument for the program."],["ArgGroup","`ArgGroup`s are a family of related arguments and way for you to say, \"Any of these arguments\". By placing arguments in a logical group, you can make easier requirement and exclusion rules intead of having to list each individually, or when you want a rule to apply \"any but not all\" arguments."],["ArgMatches","Used to get information about the arguments that where supplied to the program at runtime by the user. To get a new instance of this struct you use `.get_matches()` of the `App` struct."],["SubCommand","The abstract representation of a command line subcommand used by the consumer of the library."]],"enum":[["Format",""]],"macro":[["arg_enum!","Convenience macro to generate more complete enums with variants to be used as a type when parsing arguments. This enum also provides a `variants()` function which can be used to retrieve a `Vec<&'static str>` of the variant names."],["crate_version!","Allows you pull the version for an from your Cargo.toml as MAJOR.MINOR.PATCH_PKGVERSION_PRE"],["simple_enum!","Convenience macro generated a simple enum with variants to be used as a type when parsing arguments. This enum also provides a `variants()` function which can be used to retrieve a `Vec<&'static str>` of the variant names."],["value_t!","Convenience macro getting a typed value `T` where `T` implements `std::str::FromStr` This macro returns a `Result<T,String>` which allows you as the developer to decide what you'd like to do on a failed parse. There are two types of errors, parse failures and those where the argument wasn't present (such as a non-required argument)."],["value_t_or_exit!","Convenience macro getting a typed value `T` where `T` implements `std::str::FromStr` This macro returns a `T` or `Vec<T>` or exits with a usage string upon failure. This removes some of the boiler plate to handle failures from value_t! above."]]});