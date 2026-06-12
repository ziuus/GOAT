mod cli {
    //! CLI argument parsing for GOAT using `clap`.
    //!
    //! Defines the top-level CLI structure and handles all non-TUI subcommands.
    //!
    //! # Mode selection
    //!
    //! | Invocation                            | Mode                      |
    //! |---------------------------------------|---------------------------|
    //! | `goat`                                | Interactive TUI           |
    //! | `goat --headless`                     | Headless stdin/stdout     |
    //! | `goat --profile <name>`               | TUI with specific profile |
    //! | `goat --headless --profile <name>`    | Headless + profile        |
    //! | `goat doctor`                         | Print readiness report    |
    //! | `goat config-path`                    | Print config path         |
    //! | `goat data-path`                      | Print data dir            |
    //! | `goat db-path`                        | Print database path       |
    //! | `goat sessions`                       | List recent sessions      |
    //! | `goat new-session`                    | Create a new session      |
    //! | `goat migrate-db`                     | Migrate legacy DB         |
    //! | `goat models`                         | List providers/profiles   |
    use clap::{Parser, Subcommand};
    use std::path::PathBuf;
    /// GOAT — General Omniscient Agentic Tool
    ///
    /// Universal AI CLI/TUI agent platform.
    /// Run without arguments to launch the interactive TUI.
    /// Use --headless for non-TUI mode.
    #[command(
        name = "goat",
        version = env!("CARGO_PKG_VERSION"),
        about = "GOAT — Local-first Agent OS",
        long_about = "GOAT is a Rust-first, terminal-native AI agent platform.\n\n\
                  Modes:\n  \
                    goat              Start interactive TUI\n  \
                    goat --headless   Start headless stdin/stdout mode\n  \
                    goat doctor       System readiness check\n  \
                    goat sessions     List recent sessions\n\n\
                  Paths:\n  \
                    Config:   ~/.config/goat/goat.toml\n  \
                    Data:     ~/.local/share/goat/\n  \
                    Database: ~/.local/share/goat/goat.db\n  \
                    Logs:     ~/.local/share/goat/logs/"
    )]
    pub struct Cli {
        /// Path to a custom config file (overrides ~/.config/goat/goat.toml).
        #[arg(long, value_name = "PATH", global = true)]
        pub config: Option<PathBuf>,
        /// Path to a custom brain database file (overrides XDG data path).
        #[arg(long, value_name = "PATH", global = true)]
        pub db: Option<PathBuf>,
        /// Run in headless mode: read from stdin, print to stdout. No TUI.
        #[arg(long, global = true)]
        pub headless: bool,
        /// Disable brain (SQLite memory). Runs without persistent session storage.
        /// History is ephemeral and lost when GOAT exits.
        #[arg(long, global = true)]
        pub no_brain: bool,
        /// Select a model profile by name (e.g. balanced, coding, cheap, powerful).
        /// Overrides the default profile from goat.toml.
        /// Run `goat models` to list available profiles.
        #[arg(long, value_name = "PROFILE", global = true)]
        pub profile: Option<String>,
        /// Subcommand to run. If omitted, the TUI (or --headless) mode is used.
        #[command(subcommand)]
        pub command: Option<Command>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications, clippy::redundant_locals)]
    impl clap::Parser for Cli {}
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::CommandFactory for Cli {
        fn command<'b>() -> clap::Command {
            let __clap_app = clap::Command::new("goat");
            <Self as clap::Args>::augment_args(__clap_app)
        }
        fn command_for_update<'b>() -> clap::Command {
            let __clap_app = clap::Command::new("goat");
            <Self as clap::Args>::augment_args_for_update(__clap_app)
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::FromArgMatches for Cli {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            let v = Cli {
                config: __clap_arg_matches.remove_one::<PathBuf>("config"),
                db: __clap_arg_matches.remove_one::<PathBuf>("db"),
                headless: __clap_arg_matches
                    .remove_one::<bool>("headless")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "the following required argument was not provided: headless",
                    ))?,
                no_brain: __clap_arg_matches
                    .remove_one::<bool>("no_brain")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "the following required argument was not provided: no_brain",
                    ))?,
                profile: __clap_arg_matches.remove_one::<String>("profile"),
                command: {
                    if __clap_arg_matches
                        .subcommand_name()
                        .map(<Command as clap::Subcommand>::has_subcommand)
                        .unwrap_or(false)
                    {
                        Some(
                            <Command as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        )
                    } else {
                        None
                    }
                },
            };
            ::std::result::Result::Ok(v)
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            if __clap_arg_matches.contains_id("config") {
                #[allow(non_snake_case)]
                let config = &mut self.config;
                *config = __clap_arg_matches.remove_one::<PathBuf>("config");
            }
            if __clap_arg_matches.contains_id("db") {
                #[allow(non_snake_case)]
                let db = &mut self.db;
                *db = __clap_arg_matches.remove_one::<PathBuf>("db");
            }
            if __clap_arg_matches.contains_id("headless") {
                #[allow(non_snake_case)]
                let headless = &mut self.headless;
                *headless = __clap_arg_matches
                    .remove_one::<bool>("headless")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "the following required argument was not provided: headless",
                    ))?;
            }
            if __clap_arg_matches.contains_id("no_brain") {
                #[allow(non_snake_case)]
                let no_brain = &mut self.no_brain;
                *no_brain = __clap_arg_matches
                    .remove_one::<bool>("no_brain")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "the following required argument was not provided: no_brain",
                    ))?;
            }
            if __clap_arg_matches.contains_id("profile") {
                #[allow(non_snake_case)]
                let profile = &mut self.profile;
                *profile = __clap_arg_matches.remove_one::<String>("profile");
            }
            {
                #[allow(non_snake_case)]
                let command = &mut self.command;
                if let Some(command) = command.as_mut() {
                    <Command as clap::FromArgMatches>::update_from_arg_matches_mut(
                        command,
                        __clap_arg_matches,
                    )?;
                } else {
                    *command = Some(
                        <Command as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?,
                    );
                }
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::Args for Cli {
        fn group_id() -> Option<clap::Id> {
            Some(clap::Id::from("Cli"))
        }
        fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app = __clap_app
                    .group(
                        clap::ArgGroup::new("Cli")
                            .multiple(true)
                            .args({
                                let members: [clap::Id; 5usize] = [
                                    clap::Id::from("config"),
                                    clap::Id::from("db"),
                                    clap::Id::from("headless"),
                                    clap::Id::from("no_brain"),
                                    clap::Id::from("profile"),
                                ];
                                members
                            }),
                    );
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("config")
                            .value_name("CONFIG")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    PathBuf,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help(
                                "Path to a custom config file (overrides ~/.config/goat/goat.toml)",
                            )
                            .long_help(None)
                            .long("config")
                            .value_name("PATH")
                            .global(true);
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("db")
                            .value_name("DB")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    PathBuf,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help(
                                "Path to a custom brain database file (overrides XDG data path)",
                            )
                            .long_help(None)
                            .long("db")
                            .value_name("PATH")
                            .global(true);
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("headless")
                            .value_name("HEADLESS")
                            .required(true && clap::ArgAction::SetTrue.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    bool,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::SetTrue);
                        let arg = arg
                            .help(
                                "Run in headless mode: read from stdin, print to stdout. No TUI",
                            )
                            .long_help(None)
                            .long("headless")
                            .global(true);
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("no_brain")
                            .value_name("NO_BRAIN")
                            .required(true && clap::ArgAction::SetTrue.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    bool,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::SetTrue);
                        let arg = arg
                            .help(
                                "Disable brain (SQLite memory). Runs without persistent session storage. History is ephemeral and lost when GOAT exits",
                            )
                            .long_help(None)
                            .long("no-brain")
                            .global(true);
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("profile")
                            .value_name("PROFILE")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help(
                                "Select a model profile by name (e.g. balanced, coding, cheap, powerful). Overrides the default profile from goat.toml. Run `goat models` to list available profiles",
                            )
                            .long_help(None)
                            .long("profile")
                            .value_name("PROFILE")
                            .global(true);
                        let arg = arg;
                        arg
                    });
                let __clap_app = <Command as clap::Subcommand>::augment_subcommands(
                    __clap_app,
                );
                let __clap_app = __clap_app;
                __clap_app
                    .about("GOAT — General Omniscient Agentic Tool")
                    .long_about(
                        "GOAT — General Omniscient Agentic Tool\n\nUniversal AI CLI/TUI agent platform. Run without arguments to launch the interactive TUI. Use --headless for non-TUI mode.",
                    )
                    .version("0.14.0-alpha.1")
                    .about("GOAT — Local-first Agent OS")
                    .long_about(
                        "GOAT is a Rust-first, terminal-native AI agent platform.\n\n\
                  Modes:\n  \
                    goat              Start interactive TUI\n  \
                    goat --headless   Start headless stdin/stdout mode\n  \
                    goat doctor       System readiness check\n  \
                    goat sessions     List recent sessions\n\n\
                  Paths:\n  \
                    Config:   ~/.config/goat/goat.toml\n  \
                    Data:     ~/.local/share/goat/\n  \
                    Database: ~/.local/share/goat/goat.db\n  \
                    Logs:     ~/.local/share/goat/logs/",
                    )
            }
        }
        fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app = __clap_app
                    .group(
                        clap::ArgGroup::new("Cli")
                            .multiple(true)
                            .args({
                                let members: [clap::Id; 5usize] = [
                                    clap::Id::from("config"),
                                    clap::Id::from("db"),
                                    clap::Id::from("headless"),
                                    clap::Id::from("no_brain"),
                                    clap::Id::from("profile"),
                                ];
                                members
                            }),
                    );
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("config")
                            .value_name("CONFIG")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    PathBuf,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help(
                                "Path to a custom config file (overrides ~/.config/goat/goat.toml)",
                            )
                            .long_help(None)
                            .long("config")
                            .value_name("PATH")
                            .global(true);
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("db")
                            .value_name("DB")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    PathBuf,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help(
                                "Path to a custom brain database file (overrides XDG data path)",
                            )
                            .long_help(None)
                            .long("db")
                            .value_name("PATH")
                            .global(true);
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("headless")
                            .value_name("HEADLESS")
                            .required(true && clap::ArgAction::SetTrue.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    bool,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::SetTrue);
                        let arg = arg
                            .help(
                                "Run in headless mode: read from stdin, print to stdout. No TUI",
                            )
                            .long_help(None)
                            .long("headless")
                            .global(true);
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("no_brain")
                            .value_name("NO_BRAIN")
                            .required(true && clap::ArgAction::SetTrue.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    bool,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::SetTrue);
                        let arg = arg
                            .help(
                                "Disable brain (SQLite memory). Runs without persistent session storage. History is ephemeral and lost when GOAT exits",
                            )
                            .long_help(None)
                            .long("no-brain")
                            .global(true);
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("profile")
                            .value_name("PROFILE")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help(
                                "Select a model profile by name (e.g. balanced, coding, cheap, powerful). Overrides the default profile from goat.toml. Run `goat models` to list available profiles",
                            )
                            .long_help(None)
                            .long("profile")
                            .value_name("PROFILE")
                            .global(true);
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = <Command as clap::Subcommand>::augment_subcommands(
                    __clap_app,
                );
                let __clap_app = __clap_app
                    .subcommand_required(false)
                    .arg_required_else_help(false);
                __clap_app
                    .about("GOAT — General Omniscient Agentic Tool")
                    .long_about(
                        "GOAT — General Omniscient Agentic Tool\n\nUniversal AI CLI/TUI agent platform. Run without arguments to launch the interactive TUI. Use --headless for non-TUI mode.",
                    )
                    .version("0.14.0-alpha.1")
                    .about("GOAT — Local-first Agent OS")
                    .long_about(
                        "GOAT is a Rust-first, terminal-native AI agent platform.\n\n\
                  Modes:\n  \
                    goat              Start interactive TUI\n  \
                    goat --headless   Start headless stdin/stdout mode\n  \
                    goat doctor       System readiness check\n  \
                    goat sessions     List recent sessions\n\n\
                  Paths:\n  \
                    Config:   ~/.config/goat/goat.toml\n  \
                    Data:     ~/.local/share/goat/\n  \
                    Database: ~/.local/share/goat/goat.db\n  \
                    Logs:     ~/.local/share/goat/logs/",
                    )
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Cli {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "config",
                "db",
                "headless",
                "no_brain",
                "profile",
                "command",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.config,
                &self.db,
                &self.headless,
                &self.no_brain,
                &self.profile,
                &&self.command,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Cli", names, values)
        }
    }
    pub enum Command {
        /// Print the path of the active config file and exit.
        #[command(name = "config-path")]
        ConfigPath,
        /// Print the active data directory path and exit.
        #[command(name = "data-path")]
        DataPath,
        /// Print the active brain database path and exit.
        #[command(name = "db-path")]
        DbPath,
        /// Check system readiness and print a health report.
        ///
        /// Checks: OS, GOAT version, config file + permissions, data directory,
        /// database, legacy DB migration status, provider keys, profile + chain,
        /// ApprovalGate, headless readiness, log directory.
        #[command(name = "doctor")]
        Doctor,
        /// Migrate the legacy project-root goat_brain.db to the XDG data path.
        ///
        /// Copies ./goat_brain.db → XDG path. Original is NOT deleted.
        #[command(name = "migrate-db")]
        MigrateDb,
        /// List recent sessions from the brain database.
        ///
        /// Shows session ID, title, timestamps, and UUID/legacy classification.
        #[command(name = "sessions")]
        Sessions,
        /// Create a new session and print its ID.
        ///
        /// Does not destroy old sessions. The new session UUID is printed to stdout.
        #[command(name = "new-session")]
        NewSession,
        /// Seed demo data for the dashboard (Phase 6.5).
        /// Generates local-first JSONL mock data to visualize all Prime Agent UI flows.
        #[command(name = "seed-demo")]
        SeedDemo {
            /// Clear existing demo data before seeding.
            #[arg(long)]
            clear: bool,
        },
        /// List and switch model profiles and providers.
        #[command(name = "models")]
        Models {
            /// Optional specific action (e.g., 'list', 'route')
            #[arg(default_value = "list")]
            action: String,
            /// Additional arguments depending on action
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        /// Manage Universal Model Providers (Phase 6.6)
        #[command(name = "providers")]
        Providers {
            /// Action to perform: list, doctor
            #[arg(default_value = "list")]
            action: String,
        },
        /// Manage Brain Memory and Context Packs (Phase 6.7)
        #[command(name = "brain")]
        Brain {
            /// Action to perform: dedupe, ingest, pack
            action: String,
            /// Additional arguments depending on action
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        /// Manage Safe Extensions and Plugin Marketplace (Phase 6.8)
        #[command(name = "extensions")]
        Extensions {
            /// Action to perform: list, discover, audit, install, enable, disable, remove
            #[arg(default_value = "list")]
            action: String,
            /// ID or Path to extension depending on action
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        /// Safe approval-gated browser automation workflows (Phase 6.9)
        #[command(name = "browser")]
        Browser {
            /// Subcommand: workflows, screenshot, inspect, qa, landing-review, dashboard-qa, health
            action: String,
            /// URL or workflow-id depending on action
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        /// Builder agent workspace operations (Phase 7.1)
        #[command(name = "builder")]
        Builder {
            /// Subcommand: inspect, plan, diff-review, test-plan, validate, rollback-plan
            action: String,
            /// Goal or argument depending on action
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        /// Researcher agent operations (Phase 7.5)
        #[command(name = "researcher")]
        Researcher {
            /// Subcommand: projects, new, add-source, ingest-browser, brief, competitors, compare-tech, report
            action: String,
            /// Goal or argument depending on action
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        /// Manage tools, permissions, and tool registry.
        #[command(name = "tools")]
        Tools {
            /// Action to perform: list, show, categories, doctor, audit.
            #[arg(default_value = "list")]
            action: String,
            /// Optional argument for the action (e.g. tool name).
            arg: Option<String>,
        },
        /// Internal Subagent Framework management.
        Subagents {
            /// Action to perform: list, show, audit.
            #[arg(default_value = "list")]
            action: String,
            /// Optional argument for the action (e.g. subagent name).
            arg: Option<String>,
        },
        /// Run a single internal subagent turn.
        AskAgent {
            /// The name of the subagent to run.
            name: String,
            /// Task for the agent.
            task: String,
        },
        /// Manage External Agent Adapters (Phase 2.8).
        #[command(name = "external-agents")]
        ExternalAgents {
            /// Action: list, detect, doctor, audit, show.
            #[arg(default_value = "list")]
            action: String,
            /// Target agent name for 'show'.
            arg: Option<String>,
        },
        /// Delegate a task to an external agent.
        #[command(name = "delegate-external")]
        DelegateExternal {
            /// External agent name.
            agent: String,
            /// Task summary/prompt.
            task: String,
        },
        /// Mission Control workspace operations.
        #[command(name = "mission")]
        Mission {
            /// Action to perform: plan, status.
            #[arg(default_value = "status")]
            action: String,
            /// Additional arguments depending on action
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        /// Learn the current or specified project folder.
        #[command(name = "learn")]
        Learn {
            /// Path to learn (defaults to current directory)
            path: Option<String>,
        },
        /// Manage proposed code changes (patches).
        #[command(name = "patch")]
        Patch {
            /// Action: propose, list, show, apply
            #[arg(default_value = "list")]
            action: String,
            /// Additional arguments: mission_id or patch_id
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        /// Manage project checkpoints.
        #[command(name = "checkpoint")]
        CheckpointCmd {
            /// Action: list, restore
            #[arg(default_value = "list")]
            action: String,
            /// Additional arguments: checkpoint_id
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        /// Project workspace operations.
        #[command(name = "projects")]
        Projects {
            /// Action to perform: list, new, show.
            #[arg(default_value = "list")]
            action: String,
            /// Additional arguments depending on action
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        #[command(name = "mcp")]
        Mcp {
            /// Action to perform: status, list, show, doctor, start, stop, restart.
            #[arg(default_value = "status")]
            action: String,
            /// Target server name for 'show', 'start', 'stop', 'restart'.
            arg: Option<String>,
        },
        /// Manage hooks
        #[command(name = "hooks")]
        Hooks {
            /// Action to perform: list, show, enable, disable, run
            #[arg(default_value = "list")]
            action: String,
            /// Hook name
            arg: Option<String>,
        },
        /// Manage scheduled tasks
        #[command(name = "schedule")]
        Schedule {
            /// Action to perform: list, add, show, enable, disable, run, delete
            #[arg(default_value = "list")]
            action: String,
            /// Additional arguments depending on action
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        /// Manage background jobs
        #[command(name = "jobs")]
        Jobs {
            /// Action to perform: list, show, cancel
            #[arg(default_value = "list")]
            action: String,
            /// Job ID
            arg: Option<String>,
        },
        /// Manage GOAT Daemon
        #[command(name = "daemon")]
        Daemon {
            /// Action to perform: start, status, stop, doctor
            #[arg(default_value = "start")]
            action: String,
        },
        /// Manage GOAT Web Dashboard
        #[command(name = "dashboard")]
        Dashboard {
            /// Action to perform: dev, path, doctor
            #[arg(default_value = "dev")]
            action: String,
        },
        /// Manage GOAT Desktop App
        #[command(name = "desktop")]
        Desktop {
            /// Action to perform: run, dev, path, doctor
            #[arg(default_value = "run")]
            action: String,
        },
        /// Show project awareness status or scan the current directory.
        #[command(name = "project")]
        Project {
            /// "status" (default) or "scan"
            #[arg(default_value = "status")]
            action: String,
        },
        /// Manage GOAT curated memory files.
        #[command(name = "memory")]
        Memory {
            /// "status", "show", "path", "edit", "add-user", or "add-note"
            action: String,
            /// The text to add (for add-user and add-note)
            text: Option<String>,
        },
        /// Search past conversation interactions.
        #[command(name = "recall")]
        Recall { query: String },
        /// Manage GOAT reusable skills.
        #[command(name = "skills")]
        Skills {
            /// "list", "show", "path", "create", "validate", "search", "create-from-session"
            #[arg(default_value = "list")]
            action: String,
            /// The name or query
            arg: Option<String>,
            /// Session ID to extract from (for create-from-session)
            #[arg(long)]
            session: Option<String>,
        },
        /// Show or refresh the repo map for the current project.
        ///
        /// goat repo-map          → show cached or auto-scan
        /// goat repo-map refresh  → force rescan
        /// goat repo-map show     → show compact repo map
        #[command(name = "repo-map")]
        RepoMap {
            /// "show" (default), "refresh"
            #[arg(default_value = "show")]
            action: String,
        },
        /// Run the project's check command (e.g. cargo check, tsc, go build).
        ///
        /// Command is detected from the project. Requires approval before execution.
        #[command(name = "check")]
        Check,
        /// Run the project's test command (e.g. cargo test, pytest, npm test).
        ///
        /// Command is detected from the project. Requires approval before execution.
        #[command(name = "test")]
        Test {
            /// Optional test filter / extra args passed to the test runner.
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        /// Run the project's lint command (e.g. cargo clippy, eslint, ruff).
        ///
        /// Command is detected from the project. Requires approval before execution.
        #[command(name = "lint")]
        Lint,
        /// Run the project's format command (e.g. cargo fmt, prettier, ruff format).
        ///
        /// Command is detected from the project. Requires approval before execution.
        #[command(name = "format")]
        Format,
        /// Inspect or manage pending code patches.
        ///
        /// goat patch          → show pending patch (if any)
        /// goat patch apply    → apply the pending patch (requires approval)
        /// goat patch discard  → discard pending patch
        #[command(name = "patch")]
        Patch {
            /// "show" (default), "apply", or "discard"
            #[arg(default_value = "show")]
            action: String,
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        /// Manage safety checkpoints.
        #[command(name = "checkpoint")]
        CheckpointCmd {
            /// "list" (default), "create", "show", "diff"
            #[arg(default_value = "list")]
            action: String,
            #[arg(trailing_var_arg = true)]
            args: Vec<String>,
        },
        /// Rollback to a specific checkpoint.
        #[command(name = "rollback")]
        Rollback {
            /// Checkpoint ID
            id: String,
        },
        /// Manage git branches safely.
        #[command(name = "branch")]
        Branch {
            /// "current" (default), "create"
            #[arg(default_value = "current")]
            action: String,
            /// Branch name
            name: Option<String>,
        },
        /// Prepare and create git commits.
        #[command(name = "commit")]
        Commit {
            /// "message" (default), "create"
            #[arg(default_value = "message")]
            action: String,
        },
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::FromArgMatches for Command {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            if let Some((__clap_name, mut __clap_arg_sub_matches)) = __clap_arg_matches
                .remove_subcommand()
            {
                let __clap_arg_matches = &mut __clap_arg_sub_matches;
                if __clap_name == "config-path" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::ConfigPath);
                }
                if __clap_name == "data-path" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::DataPath);
                }
                if __clap_name == "db-path" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::DbPath);
                }
                if __clap_name == "doctor" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Doctor);
                }
                if __clap_name == "migrate-db" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::MigrateDb);
                }
                if __clap_name == "sessions" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Sessions);
                }
                if __clap_name == "new-session" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::NewSession);
                }
                if __clap_name == "seed-demo" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::SeedDemo {
                        clear: __clap_arg_matches
                            .remove_one::<bool>("clear")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: clear",
                            ))?,
                    });
                }
                if __clap_name == "models" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Models {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "providers" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Providers {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                    });
                }
                if __clap_name == "brain" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Brain {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "extensions" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Extensions {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "browser" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Browser {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "builder" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Builder {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "researcher" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Researcher {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "tools" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Tools {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        arg: __clap_arg_matches.remove_one::<String>("arg"),
                    });
                }
                if __clap_name == "subagents" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Subagents {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        arg: __clap_arg_matches.remove_one::<String>("arg"),
                    });
                }
                if __clap_name == "ask-agent" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::AskAgent {
                        name: __clap_arg_matches
                            .remove_one::<String>("name")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: name",
                            ))?,
                        task: __clap_arg_matches
                            .remove_one::<String>("task")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: task",
                            ))?,
                    });
                }
                if __clap_name == "external-agents"
                    && !__clap_arg_matches.contains_id("")
                {
                    return ::std::result::Result::Ok(Self::ExternalAgents {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        arg: __clap_arg_matches.remove_one::<String>("arg"),
                    });
                }
                if __clap_name == "delegate-external"
                    && !__clap_arg_matches.contains_id("")
                {
                    return ::std::result::Result::Ok(Self::DelegateExternal {
                        agent: __clap_arg_matches
                            .remove_one::<String>("agent")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: agent",
                            ))?,
                        task: __clap_arg_matches
                            .remove_one::<String>("task")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: task",
                            ))?,
                    });
                }
                if __clap_name == "mission" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Mission {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "learn" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Learn {
                        path: __clap_arg_matches.remove_one::<String>("path"),
                    });
                }
                if __clap_name == "patch" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Patch {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "checkpoint" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::CheckpointCmd {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "projects" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Projects {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "mcp" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Mcp {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        arg: __clap_arg_matches.remove_one::<String>("arg"),
                    });
                }
                if __clap_name == "hooks" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Hooks {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        arg: __clap_arg_matches.remove_one::<String>("arg"),
                    });
                }
                if __clap_name == "schedule" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Schedule {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "jobs" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Jobs {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        arg: __clap_arg_matches.remove_one::<String>("arg"),
                    });
                }
                if __clap_name == "daemon" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Daemon {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                    });
                }
                if __clap_name == "dashboard" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Dashboard {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                    });
                }
                if __clap_name == "desktop" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Desktop {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                    });
                }
                if __clap_name == "project" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Project {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                    });
                }
                if __clap_name == "memory" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Memory {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        text: __clap_arg_matches.remove_one::<String>("text"),
                    });
                }
                if __clap_name == "recall" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Recall {
                        query: __clap_arg_matches
                            .remove_one::<String>("query")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: query",
                            ))?,
                    });
                }
                if __clap_name == "skills" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Skills {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        arg: __clap_arg_matches.remove_one::<String>("arg"),
                        session: __clap_arg_matches.remove_one::<String>("session"),
                    });
                }
                if __clap_name == "repo-map" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::RepoMap {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                    });
                }
                if __clap_name == "check" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Check);
                }
                if __clap_name == "test" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Test {
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "lint" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Lint);
                }
                if __clap_name == "format" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Format);
                }
                if __clap_name == "patch" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Patch {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "checkpoint" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::CheckpointCmd {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        args: __clap_arg_matches
                            .remove_many::<String>("args")
                            .map(|v| v.collect::<Vec<_>>())
                            .unwrap_or_else(Vec::new),
                    });
                }
                if __clap_name == "rollback" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Rollback {
                        id: __clap_arg_matches
                            .remove_one::<String>("id")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: id",
                            ))?,
                    });
                }
                if __clap_name == "branch" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Branch {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                        name: __clap_arg_matches.remove_one::<String>("name"),
                    });
                }
                if __clap_name == "commit" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Commit {
                        action: __clap_arg_matches
                            .remove_one::<String>("action")
                            .ok_or_else(|| clap::Error::raw(
                                clap::error::ErrorKind::MissingRequiredArgument,
                                "the following required argument was not provided: action",
                            ))?,
                    });
                }
                ::std::result::Result::Err(
                    clap::Error::raw(
                        clap::error::ErrorKind::InvalidSubcommand,
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "the subcommand \'{0}\' wasn\'t recognized",
                                    __clap_name,
                                ),
                            )
                        }),
                    ),
                )
            } else {
                ::std::result::Result::Err(
                    clap::Error::raw(
                        clap::error::ErrorKind::MissingSubcommand,
                        "a subcommand is required but one was not provided",
                    ),
                )
            }
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut<'b>(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
                match self {
                    Self::ConfigPath if "config-path" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::DataPath if "data-path" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::DbPath if "db-path" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::Doctor if "doctor" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::MigrateDb if "migrate-db" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::Sessions if "sessions" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::NewSession if "new-session" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::SeedDemo { clear } if "seed-demo" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("clear") {
                                *clear = __clap_arg_matches
                                    .remove_one::<bool>("clear")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: clear",
                                    ))?;
                            }
                        }
                    }
                    Self::Models { action, args } if "models" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::Providers { action } if "providers" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                        }
                    }
                    Self::Brain { action, args } if "brain" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::Extensions { action, args } if "extensions" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::Browser { action, args } if "browser" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::Builder { action, args } if "builder" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::Researcher { action, args } if "researcher" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::Tools { action, arg } if "tools" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("arg") {
                                *arg = __clap_arg_matches.remove_one::<String>("arg");
                            }
                        }
                    }
                    Self::Subagents { action, arg } if "subagents" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("arg") {
                                *arg = __clap_arg_matches.remove_one::<String>("arg");
                            }
                        }
                    }
                    Self::AskAgent { name, task } if "ask-agent" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("name") {
                                *name = __clap_arg_matches
                                    .remove_one::<String>("name")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: name",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("task") {
                                *task = __clap_arg_matches
                                    .remove_one::<String>("task")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: task",
                                    ))?;
                            }
                        }
                    }
                    Self::ExternalAgents {
                        action,
                        arg,
                    } if "external-agents" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("arg") {
                                *arg = __clap_arg_matches.remove_one::<String>("arg");
                            }
                        }
                    }
                    Self::DelegateExternal {
                        agent,
                        task,
                    } if "delegate-external" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("agent") {
                                *agent = __clap_arg_matches
                                    .remove_one::<String>("agent")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: agent",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("task") {
                                *task = __clap_arg_matches
                                    .remove_one::<String>("task")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: task",
                                    ))?;
                            }
                        }
                    }
                    Self::Mission { action, args } if "mission" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::Learn { path } if "learn" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("path") {
                                *path = __clap_arg_matches.remove_one::<String>("path");
                            }
                        }
                    }
                    Self::Patch { action, args } if "patch" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::CheckpointCmd {
                        action,
                        args,
                    } if "checkpoint" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::Projects { action, args } if "projects" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::Mcp { action, arg } if "mcp" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("arg") {
                                *arg = __clap_arg_matches.remove_one::<String>("arg");
                            }
                        }
                    }
                    Self::Hooks { action, arg } if "hooks" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("arg") {
                                *arg = __clap_arg_matches.remove_one::<String>("arg");
                            }
                        }
                    }
                    Self::Schedule { action, args } if "schedule" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::Jobs { action, arg } if "jobs" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("arg") {
                                *arg = __clap_arg_matches.remove_one::<String>("arg");
                            }
                        }
                    }
                    Self::Daemon { action } if "daemon" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                        }
                    }
                    Self::Dashboard { action } if "dashboard" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                        }
                    }
                    Self::Desktop { action } if "desktop" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                        }
                    }
                    Self::Project { action } if "project" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                        }
                    }
                    Self::Memory { action, text } if "memory" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("text") {
                                *text = __clap_arg_matches.remove_one::<String>("text");
                            }
                        }
                    }
                    Self::Recall { query } if "recall" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("query") {
                                *query = __clap_arg_matches
                                    .remove_one::<String>("query")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: query",
                                    ))?;
                            }
                        }
                    }
                    Self::Skills { action, arg, session } if "skills" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("arg") {
                                *arg = __clap_arg_matches.remove_one::<String>("arg");
                            }
                            if __clap_arg_matches.contains_id("session") {
                                *session = __clap_arg_matches
                                    .remove_one::<String>("session");
                            }
                        }
                    }
                    Self::RepoMap { action } if "repo-map" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                        }
                    }
                    Self::Check if "check" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::Test { args } if "test" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::Lint if "lint" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::Format if "format" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::Patch { action, args } if "patch" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::CheckpointCmd {
                        action,
                        args,
                    } if "checkpoint" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("args") {
                                *args = __clap_arg_matches
                                    .remove_many::<String>("args")
                                    .map(|v| v.collect::<Vec<_>>())
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    Self::Rollback { id } if "rollback" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("id") {
                                *id = __clap_arg_matches
                                    .remove_one::<String>("id")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: id",
                                    ))?;
                            }
                        }
                    }
                    Self::Branch { action, name } if "branch" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                            if __clap_arg_matches.contains_id("name") {
                                *name = __clap_arg_matches.remove_one::<String>("name");
                            }
                        }
                    }
                    Self::Commit { action } if "commit" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("action") {
                                *action = __clap_arg_matches
                                    .remove_one::<String>("action")
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::error::ErrorKind::MissingRequiredArgument,
                                        "the following required argument was not provided: action",
                                    ))?;
                            }
                        }
                    }
                    s => {
                        *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?;
                    }
                }
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::Subcommand for Command {
        fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("config-path");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("Print the path of the active config file and exit")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("data-path");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("Print the active data directory path and exit")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("db-path");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("Print the active brain database path and exit")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("doctor");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("Check system readiness and print a health report")
                        .long_about(
                            "Check system readiness and print a health report.\n\nChecks: OS, GOAT version, config file + permissions, data directory, database, legacy DB migration status, provider keys, profile + chain, ApprovalGate, headless readiness, log directory.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("migrate-db");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about(
                            "Migrate the legacy project-root goat_brain.db to the XDG data path",
                        )
                        .long_about(
                            "Migrate the legacy project-root goat_brain.db to the XDG data path.\n\nCopies ./goat_brain.db → XDG path. Original is NOT deleted.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("sessions");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("List recent sessions from the brain database")
                        .long_about(
                            "List recent sessions from the brain database.\n\nShows session ID, title, timestamps, and UUID/legacy classification.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("new-session");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("Create a new session and print its ID")
                        .long_about(
                            "Create a new session and print its ID.\n\nDoes not destroy old sessions. The new session UUID is printed to stdout.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("seed-demo");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("SeedDemo")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [clap::Id::from("clear")];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("clear")
                                    .value_name("CLEAR")
                                    .required(true && clap::ArgAction::SetTrue.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            bool,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::SetTrue);
                                let arg = arg
                                    .help("Clear existing demo data before seeding")
                                    .long_help(None)
                                    .long("clear");
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about(
                                "Seed demo data for the dashboard (Phase 6.5). Generates local-first JSONL mock data to visualize all Prime Agent UI flows",
                            )
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("models");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Models")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Optional specific action (e.g., 'list', 'route')")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("List and switch model profiles and providers")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("providers");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Providers")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: list, doctor")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage Universal Model Providers (Phase 6.6)")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("brain");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Brain")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: dedupe, ingest, pack")
                                    .long_help(None);
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage Brain Memory and Context Packs (Phase 6.7)")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("extensions");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Extensions")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Action to perform: list, discover, audit, install, enable, disable, remove",
                                    )
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("ID or Path to extension depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about(
                                "Manage Safe Extensions and Plugin Marketplace (Phase 6.8)",
                            )
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("browser");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Browser")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Subcommand: workflows, screenshot, inspect, qa, landing-review, dashboard-qa, health",
                                    )
                                    .long_help(None);
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("URL or workflow-id depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about(
                                "Safe approval-gated browser automation workflows (Phase 6.9)",
                            )
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("builder");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Builder")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Subcommand: inspect, plan, diff-review, test-plan, validate, rollback-plan",
                                    )
                                    .long_help(None);
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Goal or argument depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Builder agent workspace operations (Phase 7.1)")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("researcher");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Researcher")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Subcommand: projects, new, add-source, ingest-browser, brief, competitors, compare-tech, report",
                                    )
                                    .long_help(None);
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Goal or argument depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Researcher agent operations (Phase 7.5)")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("tools");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Tools")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Action to perform: list, show, categories, doctor, audit",
                                    )
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Optional argument for the action (e.g. tool name)")
                                    .long_help(None);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage tools, permissions, and tool registry")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("subagents");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Subagents")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: list, show, audit")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Optional argument for the action (e.g. subagent name)",
                                    )
                                    .long_help(None);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Internal Subagent Framework management")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("ask-agent");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("AskAgent")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("name"),
                                            clap::Id::from("task"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("name")
                                    .value_name("NAME")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("The name of the subagent to run")
                                    .long_help(None);
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("task")
                                    .value_name("TASK")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("Task for the agent").long_help(None);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Run a single internal subagent turn")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("external-agents");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("ExternalAgents")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action: list, detect, doctor, audit, show")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Target agent name for 'show'")
                                    .long_help(None);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage External Agent Adapters (Phase 2.8)")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("delegate-external");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("DelegateExternal")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("agent"),
                                            clap::Id::from("task"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("agent")
                                    .value_name("AGENT")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("External agent name").long_help(None);
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("task")
                                    .value_name("TASK")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("Task summary/prompt").long_help(None);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Delegate a task to an external agent")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("mission");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Mission")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: plan, status")
                                    .long_help(None)
                                    .default_value("status");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Mission Control workspace operations")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("learn");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Learn")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [clap::Id::from("path")];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("path")
                                    .value_name("PATH")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Path to learn (defaults to current directory)")
                                    .long_help(None);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Learn the current or specified project folder")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("patch");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Patch")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action: propose, list, show, apply")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments: mission_id or patch_id")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage proposed code changes (patches)")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("checkpoint");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("CheckpointCmd")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action: list, restore")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments: checkpoint_id")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage project checkpoints")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("projects");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Projects")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: list, new, show")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Project workspace operations")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("mcp");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Mcp")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Action to perform: status, list, show, doctor, start, stop, restart",
                                    )
                                    .long_help(None)
                                    .default_value("status");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Target server name for 'show', 'start', 'stop', 'restart'",
                                    )
                                    .long_help(None);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("hooks");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Hooks")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: list, show, enable, disable, run")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("Hook name").long_help(None);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand.about("Manage hooks").long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("schedule");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Schedule")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Action to perform: list, add, show, enable, disable, run, delete",
                                    )
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage scheduled tasks")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("jobs");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Jobs")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: list, show, cancel")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("Job ID").long_help(None);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage background jobs")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("daemon");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Daemon")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: start, status, stop, doctor")
                                    .long_help(None)
                                    .default_value("start");
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand.about("Manage GOAT Daemon").long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("dashboard");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Dashboard")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: dev, path, doctor")
                                    .long_help(None)
                                    .default_value("dev");
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage GOAT Web Dashboard")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("desktop");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Desktop")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: run, dev, path, doctor")
                                    .long_help(None)
                                    .default_value("run");
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage GOAT Desktop App")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("project");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Project")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("\"status\" (default) or \"scan\"")
                                    .long_help(None)
                                    .default_value("status");
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about(
                                "Show project awareness status or scan the current directory",
                            )
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("memory");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Memory")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("text"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "\"status\", \"show\", \"path\", \"edit\", \"add-user\", or \"add-note\"",
                                    )
                                    .long_help(None);
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("text")
                                    .value_name("TEXT")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("The text to add (for add-user and add-note)")
                                    .long_help(None);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage GOAT curated memory files")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("recall");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Recall")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [clap::Id::from("query")];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("query")
                                    .value_name("QUERY")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg;
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Search past conversation interactions")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("skills");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Skills")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 3usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                            clap::Id::from("session"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "\"list\", \"show\", \"path\", \"create\", \"validate\", \"search\", \"create-from-session\"",
                                    )
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("The name or query").long_help(None);
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("session")
                                    .value_name("SESSION")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Session ID to extract from (for create-from-session)",
                                    )
                                    .long_help(None)
                                    .long("session");
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage GOAT reusable skills")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("repo-map");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("RepoMap")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("\"show\" (default), \"refresh\"")
                                    .long_help(None)
                                    .default_value("show");
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about(
                                "Show or refresh the repo map for the current project",
                            )
                            .long_about(
                                "Show or refresh the repo map for the current project.\n\ngoat repo-map          → show cached or auto-scan goat repo-map refresh  → force rescan goat repo-map show     → show compact repo map",
                            )
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("check");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about(
                            "Run the project's check command (e.g. cargo check, tsc, go build)",
                        )
                        .long_about(
                            "Run the project's check command (e.g. cargo check, tsc, go build).\n\nCommand is detected from the project. Requires approval before execution.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("test");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Test")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [clap::Id::from("args")];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help(
                                        "Optional test filter / extra args passed to the test runner",
                                    )
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about(
                                "Run the project's test command (e.g. cargo test, pytest, npm test)",
                            )
                            .long_about(
                                "Run the project's test command (e.g. cargo test, pytest, npm test).\n\nCommand is detected from the project. Requires approval before execution.",
                            )
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("lint");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about(
                            "Run the project's lint command (e.g. cargo clippy, eslint, ruff)",
                        )
                        .long_about(
                            "Run the project's lint command (e.g. cargo clippy, eslint, ruff).\n\nCommand is detected from the project. Requires approval before execution.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("format");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about(
                            "Run the project's format command (e.g. cargo fmt, prettier, ruff format)",
                        )
                        .long_about(
                            "Run the project's format command (e.g. cargo fmt, prettier, ruff format).\n\nCommand is detected from the project. Requires approval before execution.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("patch");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Patch")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("\"show\" (default), \"apply\", or \"discard\"")
                                    .long_help(None)
                                    .default_value("show");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg.trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Inspect or manage pending code patches")
                            .long_about(
                                "Inspect or manage pending code patches.\n\ngoat patch          → show pending patch (if any) goat patch apply    → apply the pending patch (requires approval) goat patch discard  → discard pending patch",
                            )
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("checkpoint");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("CheckpointCmd")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("\"list\" (default), \"create\", \"show\", \"diff\"")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg.trailing_var_arg(true);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage safety checkpoints")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("rollback");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Rollback")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [clap::Id::from("id")];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("id")
                                    .value_name("ID")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("Checkpoint ID").long_help(None);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Rollback to a specific checkpoint")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("branch");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Branch")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("name"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("\"current\" (default), \"create\"")
                                    .long_help(None)
                                    .default_value("current");
                                let arg = arg;
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("name")
                                    .value_name("NAME")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("Branch name").long_help(None);
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Manage git branches safely")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("commit");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Commit")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("\"message\" (default), \"create\"")
                                    .long_help(None)
                                    .default_value("message");
                                let arg = arg;
                                arg
                            });
                        __clap_subcommand
                            .about("Prepare and create git commits")
                            .long_about(None)
                    }
                });
            __clap_app
        }
        fn augment_subcommands_for_update<'b>(
            __clap_app: clap::Command,
        ) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("config-path");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("Print the path of the active config file and exit")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("data-path");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("Print the active data directory path and exit")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("db-path");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("Print the active brain database path and exit")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("doctor");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("Check system readiness and print a health report")
                        .long_about(
                            "Check system readiness and print a health report.\n\nChecks: OS, GOAT version, config file + permissions, data directory, database, legacy DB migration status, provider keys, profile + chain, ApprovalGate, headless readiness, log directory.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("migrate-db");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about(
                            "Migrate the legacy project-root goat_brain.db to the XDG data path",
                        )
                        .long_about(
                            "Migrate the legacy project-root goat_brain.db to the XDG data path.\n\nCopies ./goat_brain.db → XDG path. Original is NOT deleted.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("sessions");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("List recent sessions from the brain database")
                        .long_about(
                            "List recent sessions from the brain database.\n\nShows session ID, title, timestamps, and UUID/legacy classification.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("new-session");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("Create a new session and print its ID")
                        .long_about(
                            "Create a new session and print its ID.\n\nDoes not destroy old sessions. The new session UUID is printed to stdout.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("seed-demo");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("SeedDemo")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [clap::Id::from("clear")];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("clear")
                                    .value_name("CLEAR")
                                    .required(true && clap::ArgAction::SetTrue.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            bool,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::SetTrue);
                                let arg = arg
                                    .help("Clear existing demo data before seeding")
                                    .long_help(None)
                                    .long("clear");
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about(
                                "Seed demo data for the dashboard (Phase 6.5). Generates local-first JSONL mock data to visualize all Prime Agent UI flows",
                            )
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("models");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Models")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Optional specific action (e.g., 'list', 'route')")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("List and switch model profiles and providers")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("providers");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Providers")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: list, doctor")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage Universal Model Providers (Phase 6.6)")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("brain");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Brain")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: dedupe, ingest, pack")
                                    .long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage Brain Memory and Context Packs (Phase 6.7)")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("extensions");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Extensions")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Action to perform: list, discover, audit, install, enable, disable, remove",
                                    )
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("ID or Path to extension depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about(
                                "Manage Safe Extensions and Plugin Marketplace (Phase 6.8)",
                            )
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("browser");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Browser")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Subcommand: workflows, screenshot, inspect, qa, landing-review, dashboard-qa, health",
                                    )
                                    .long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("URL or workflow-id depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about(
                                "Safe approval-gated browser automation workflows (Phase 6.9)",
                            )
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("builder");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Builder")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Subcommand: inspect, plan, diff-review, test-plan, validate, rollback-plan",
                                    )
                                    .long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Goal or argument depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Builder agent workspace operations (Phase 7.1)")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("researcher");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Researcher")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Subcommand: projects, new, add-source, ingest-browser, brief, competitors, compare-tech, report",
                                    )
                                    .long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Goal or argument depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Researcher agent operations (Phase 7.5)")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("tools");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Tools")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Action to perform: list, show, categories, doctor, audit",
                                    )
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Optional argument for the action (e.g. tool name)")
                                    .long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage tools, permissions, and tool registry")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("subagents");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Subagents")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: list, show, audit")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Optional argument for the action (e.g. subagent name)",
                                    )
                                    .long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Internal Subagent Framework management")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("ask-agent");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("AskAgent")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("name"),
                                            clap::Id::from("task"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("name")
                                    .value_name("NAME")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("The name of the subagent to run")
                                    .long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("task")
                                    .value_name("TASK")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("Task for the agent").long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Run a single internal subagent turn")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("external-agents");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("ExternalAgents")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action: list, detect, doctor, audit, show")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Target agent name for 'show'")
                                    .long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage External Agent Adapters (Phase 2.8)")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("delegate-external");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("DelegateExternal")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("agent"),
                                            clap::Id::from("task"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("agent")
                                    .value_name("AGENT")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("External agent name").long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("task")
                                    .value_name("TASK")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("Task summary/prompt").long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Delegate a task to an external agent")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("mission");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Mission")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: plan, status")
                                    .long_help(None)
                                    .default_value("status");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Mission Control workspace operations")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("learn");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Learn")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [clap::Id::from("path")];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("path")
                                    .value_name("PATH")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Path to learn (defaults to current directory)")
                                    .long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Learn the current or specified project folder")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("patch");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Patch")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action: propose, list, show, apply")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments: mission_id or patch_id")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage proposed code changes (patches)")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("checkpoint");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("CheckpointCmd")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action: list, restore")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments: checkpoint_id")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage project checkpoints")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("projects");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Projects")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: list, new, show")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Project workspace operations")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("mcp");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Mcp")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Action to perform: status, list, show, doctor, start, stop, restart",
                                    )
                                    .long_help(None)
                                    .default_value("status");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Target server name for 'show', 'start', 'stop', 'restart'",
                                    )
                                    .long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("hooks");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Hooks")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: list, show, enable, disable, run")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("Hook name").long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand.about("Manage hooks").long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("schedule");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Schedule")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Action to perform: list, add, show, enable, disable, run, delete",
                                    )
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help("Additional arguments depending on action")
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage scheduled tasks")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("jobs");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Jobs")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: list, show, cancel")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("Job ID").long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage background jobs")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("daemon");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Daemon")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: start, status, stop, doctor")
                                    .long_help(None)
                                    .default_value("start");
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand.about("Manage GOAT Daemon").long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("dashboard");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Dashboard")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: dev, path, doctor")
                                    .long_help(None)
                                    .default_value("dev");
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage GOAT Web Dashboard")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("desktop");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Desktop")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("Action to perform: run, dev, path, doctor")
                                    .long_help(None)
                                    .default_value("run");
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage GOAT Desktop App")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("project");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Project")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("\"status\" (default) or \"scan\"")
                                    .long_help(None)
                                    .default_value("status");
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about(
                                "Show project awareness status or scan the current directory",
                            )
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("memory");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Memory")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("text"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "\"status\", \"show\", \"path\", \"edit\", \"add-user\", or \"add-note\"",
                                    )
                                    .long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("text")
                                    .value_name("TEXT")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("The text to add (for add-user and add-note)")
                                    .long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage GOAT curated memory files")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("recall");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Recall")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [clap::Id::from("query")];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("query")
                                    .value_name("QUERY")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg;
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Search past conversation interactions")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("skills");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Skills")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 3usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("arg"),
                                            clap::Id::from("session"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "\"list\", \"show\", \"path\", \"create\", \"validate\", \"search\", \"create-from-session\"",
                                    )
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("arg")
                                    .value_name("ARG")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("The name or query").long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("session")
                                    .value_name("SESSION")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help(
                                        "Session ID to extract from (for create-from-session)",
                                    )
                                    .long_help(None)
                                    .long("session");
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage GOAT reusable skills")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("repo-map");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("RepoMap")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("\"show\" (default), \"refresh\"")
                                    .long_help(None)
                                    .default_value("show");
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about(
                                "Show or refresh the repo map for the current project",
                            )
                            .long_about(
                                "Show or refresh the repo map for the current project.\n\ngoat repo-map          → show cached or auto-scan goat repo-map refresh  → force rescan goat repo-map show     → show compact repo map",
                            )
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("check");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about(
                            "Run the project's check command (e.g. cargo check, tsc, go build)",
                        )
                        .long_about(
                            "Run the project's check command (e.g. cargo check, tsc, go build).\n\nCommand is detected from the project. Requires approval before execution.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("test");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Test")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [clap::Id::from("args")];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg
                                    .help(
                                        "Optional test filter / extra args passed to the test runner",
                                    )
                                    .long_help(None)
                                    .trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about(
                                "Run the project's test command (e.g. cargo test, pytest, npm test)",
                            )
                            .long_about(
                                "Run the project's test command (e.g. cargo test, pytest, npm test).\n\nCommand is detected from the project. Requires approval before execution.",
                            )
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("lint");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about(
                            "Run the project's lint command (e.g. cargo clippy, eslint, ruff)",
                        )
                        .long_about(
                            "Run the project's lint command (e.g. cargo clippy, eslint, ruff).\n\nCommand is detected from the project. Requires approval before execution.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("format");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about(
                            "Run the project's format command (e.g. cargo fmt, prettier, ruff format)",
                        )
                        .long_about(
                            "Run the project's format command (e.g. cargo fmt, prettier, ruff format).\n\nCommand is detected from the project. Requires approval before execution.",
                        )
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("patch");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Patch")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("\"show\" (default), \"apply\", or \"discard\"")
                                    .long_help(None)
                                    .default_value("show");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg.trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Inspect or manage pending code patches")
                            .long_about(
                                "Inspect or manage pending code patches.\n\ngoat patch          → show pending patch (if any) goat patch apply    → apply the pending patch (requires approval) goat patch discard  → discard pending patch",
                            )
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("checkpoint");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("CheckpointCmd")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("args"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("\"list\" (default), \"create\", \"show\", \"diff\"")
                                    .long_help(None)
                                    .default_value("list");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("args")
                                    .value_name("ARGS")
                                    .num_args(1..)
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Append);
                                let arg = arg.trailing_var_arg(true);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage safety checkpoints")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("rollback");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Rollback")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [clap::Id::from("id")];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("id")
                                    .value_name("ID")
                                    .required(true && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("Checkpoint ID").long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Rollback to a specific checkpoint")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("branch");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Branch")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 2usize] = [
                                            clap::Id::from("action"),
                                            clap::Id::from("name"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("\"current\" (default), \"create\"")
                                    .long_help(None)
                                    .default_value("current");
                                let arg = arg.required(false);
                                arg
                            });
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("name")
                                    .value_name("NAME")
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg.help("Branch name").long_help(None);
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Manage git branches safely")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("commit");
                    {
                        let __clap_subcommand = __clap_subcommand
                            .group(
                                clap::ArgGroup::new("Commit")
                                    .multiple(true)
                                    .args({
                                        let members: [clap::Id; 1usize] = [
                                            clap::Id::from("action"),
                                        ];
                                        members
                                    }),
                            );
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("action")
                                    .value_name("ACTION")
                                    .required(false && clap::ArgAction::Set.takes_values())
                                    .value_parser({
                                        use ::clap_builder::builder::impl_prelude::*;
                                        let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                            String,
                                        >::new();
                                        (&&&&&&auto).value_parser()
                                    })
                                    .action(clap::ArgAction::Set);
                                let arg = arg
                                    .help("\"message\" (default), \"create\"")
                                    .long_help(None)
                                    .default_value("message");
                                let arg = arg.required(false);
                                arg
                            });
                        __clap_subcommand
                            .about("Prepare and create git commits")
                            .long_about(None)
                    }
                });
            __clap_app
        }
        fn has_subcommand(__clap_name: &str) -> bool {
            if "config-path" == __clap_name {
                return true;
            }
            if "data-path" == __clap_name {
                return true;
            }
            if "db-path" == __clap_name {
                return true;
            }
            if "doctor" == __clap_name {
                return true;
            }
            if "migrate-db" == __clap_name {
                return true;
            }
            if "sessions" == __clap_name {
                return true;
            }
            if "new-session" == __clap_name {
                return true;
            }
            if "seed-demo" == __clap_name {
                return true;
            }
            if "models" == __clap_name {
                return true;
            }
            if "providers" == __clap_name {
                return true;
            }
            if "brain" == __clap_name {
                return true;
            }
            if "extensions" == __clap_name {
                return true;
            }
            if "browser" == __clap_name {
                return true;
            }
            if "builder" == __clap_name {
                return true;
            }
            if "researcher" == __clap_name {
                return true;
            }
            if "tools" == __clap_name {
                return true;
            }
            if "subagents" == __clap_name {
                return true;
            }
            if "ask-agent" == __clap_name {
                return true;
            }
            if "external-agents" == __clap_name {
                return true;
            }
            if "delegate-external" == __clap_name {
                return true;
            }
            if "mission" == __clap_name {
                return true;
            }
            if "learn" == __clap_name {
                return true;
            }
            if "patch" == __clap_name {
                return true;
            }
            if "checkpoint" == __clap_name {
                return true;
            }
            if "projects" == __clap_name {
                return true;
            }
            if "mcp" == __clap_name {
                return true;
            }
            if "hooks" == __clap_name {
                return true;
            }
            if "schedule" == __clap_name {
                return true;
            }
            if "jobs" == __clap_name {
                return true;
            }
            if "daemon" == __clap_name {
                return true;
            }
            if "dashboard" == __clap_name {
                return true;
            }
            if "desktop" == __clap_name {
                return true;
            }
            if "project" == __clap_name {
                return true;
            }
            if "memory" == __clap_name {
                return true;
            }
            if "recall" == __clap_name {
                return true;
            }
            if "skills" == __clap_name {
                return true;
            }
            if "repo-map" == __clap_name {
                return true;
            }
            if "check" == __clap_name {
                return true;
            }
            if "test" == __clap_name {
                return true;
            }
            if "lint" == __clap_name {
                return true;
            }
            if "format" == __clap_name {
                return true;
            }
            if "patch" == __clap_name {
                return true;
            }
            if "checkpoint" == __clap_name {
                return true;
            }
            if "rollback" == __clap_name {
                return true;
            }
            if "branch" == __clap_name {
                return true;
            }
            if "commit" == __clap_name {
                return true;
            }
            false
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Command {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Command::ConfigPath => ::core::fmt::Formatter::write_str(f, "ConfigPath"),
                Command::DataPath => ::core::fmt::Formatter::write_str(f, "DataPath"),
                Command::DbPath => ::core::fmt::Formatter::write_str(f, "DbPath"),
                Command::Doctor => ::core::fmt::Formatter::write_str(f, "Doctor"),
                Command::MigrateDb => ::core::fmt::Formatter::write_str(f, "MigrateDb"),
                Command::Sessions => ::core::fmt::Formatter::write_str(f, "Sessions"),
                Command::NewSession => ::core::fmt::Formatter::write_str(f, "NewSession"),
                Command::SeedDemo { clear: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "SeedDemo",
                        "clear",
                        &__self_0,
                    )
                }
                Command::Models { action: __self_0, args: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Models",
                        "action",
                        __self_0,
                        "args",
                        &__self_1,
                    )
                }
                Command::Providers { action: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Providers",
                        "action",
                        &__self_0,
                    )
                }
                Command::Brain { action: __self_0, args: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Brain",
                        "action",
                        __self_0,
                        "args",
                        &__self_1,
                    )
                }
                Command::Extensions { action: __self_0, args: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Extensions",
                        "action",
                        __self_0,
                        "args",
                        &__self_1,
                    )
                }
                Command::Browser { action: __self_0, args: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Browser",
                        "action",
                        __self_0,
                        "args",
                        &__self_1,
                    )
                }
                Command::Builder { action: __self_0, args: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Builder",
                        "action",
                        __self_0,
                        "args",
                        &__self_1,
                    )
                }
                Command::Researcher { action: __self_0, args: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Researcher",
                        "action",
                        __self_0,
                        "args",
                        &__self_1,
                    )
                }
                Command::Tools { action: __self_0, arg: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Tools",
                        "action",
                        __self_0,
                        "arg",
                        &__self_1,
                    )
                }
                Command::Subagents { action: __self_0, arg: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Subagents",
                        "action",
                        __self_0,
                        "arg",
                        &__self_1,
                    )
                }
                Command::AskAgent { name: __self_0, task: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "AskAgent",
                        "name",
                        __self_0,
                        "task",
                        &__self_1,
                    )
                }
                Command::ExternalAgents { action: __self_0, arg: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ExternalAgents",
                        "action",
                        __self_0,
                        "arg",
                        &__self_1,
                    )
                }
                Command::DelegateExternal { agent: __self_0, task: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "DelegateExternal",
                        "agent",
                        __self_0,
                        "task",
                        &__self_1,
                    )
                }
                Command::Mission { action: __self_0, args: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Mission",
                        "action",
                        __self_0,
                        "args",
                        &__self_1,
                    )
                }
                Command::Learn { path: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Learn",
                        "path",
                        &__self_0,
                    )
                }
                Command::Patch { action: __self_0, args: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Patch",
                        "action",
                        __self_0,
                        "args",
                        &__self_1,
                    )
                }
                Command::CheckpointCmd { action: __self_0, args: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "CheckpointCmd",
                        "action",
                        __self_0,
                        "args",
                        &__self_1,
                    )
                }
                Command::Projects { action: __self_0, args: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Projects",
                        "action",
                        __self_0,
                        "args",
                        &__self_1,
                    )
                }
                Command::Mcp { action: __self_0, arg: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Mcp",
                        "action",
                        __self_0,
                        "arg",
                        &__self_1,
                    )
                }
                Command::Hooks { action: __self_0, arg: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Hooks",
                        "action",
                        __self_0,
                        "arg",
                        &__self_1,
                    )
                }
                Command::Schedule { action: __self_0, args: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Schedule",
                        "action",
                        __self_0,
                        "args",
                        &__self_1,
                    )
                }
                Command::Jobs { action: __self_0, arg: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Jobs",
                        "action",
                        __self_0,
                        "arg",
                        &__self_1,
                    )
                }
                Command::Daemon { action: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Daemon",
                        "action",
                        &__self_0,
                    )
                }
                Command::Dashboard { action: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Dashboard",
                        "action",
                        &__self_0,
                    )
                }
                Command::Desktop { action: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Desktop",
                        "action",
                        &__self_0,
                    )
                }
                Command::Project { action: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Project",
                        "action",
                        &__self_0,
                    )
                }
                Command::Memory { action: __self_0, text: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Memory",
                        "action",
                        __self_0,
                        "text",
                        &__self_1,
                    )
                }
                Command::Recall { query: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Recall",
                        "query",
                        &__self_0,
                    )
                }
                Command::Skills {
                    action: __self_0,
                    arg: __self_1,
                    session: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Skills",
                        "action",
                        __self_0,
                        "arg",
                        __self_1,
                        "session",
                        &__self_2,
                    )
                }
                Command::RepoMap { action: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "RepoMap",
                        "action",
                        &__self_0,
                    )
                }
                Command::Check => ::core::fmt::Formatter::write_str(f, "Check"),
                Command::Test { args: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Test",
                        "args",
                        &__self_0,
                    )
                }
                Command::Lint => ::core::fmt::Formatter::write_str(f, "Lint"),
                Command::Format => ::core::fmt::Formatter::write_str(f, "Format"),
                Command::Patch { action: __self_0, args: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Patch",
                        "action",
                        __self_0,
                        "args",
                        &__self_1,
                    )
                }
                Command::CheckpointCmd { action: __self_0, args: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "CheckpointCmd",
                        "action",
                        __self_0,
                        "args",
                        &__self_1,
                    )
                }
                Command::Rollback { id: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Rollback",
                        "id",
                        &__self_0,
                    )
                }
                Command::Branch { action: __self_0, name: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Branch",
                        "action",
                        __self_0,
                        "name",
                        &__self_1,
                    )
                }
                Command::Commit { action: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Commit",
                        "action",
                        &__self_0,
                    )
                }
            }
        }
    }
    /// Handle CLI subcommands that do not need TUI or headless mode.
    ///
    /// Returns `true` if a subcommand was handled (caller should exit after),
    /// `false` if the TUI or headless loop should be launched.
    pub async fn handle_subcommand(
        cli: &Cli,
        paths: &crate::paths::GoatPaths,
        config: &crate::config::Config,
    ) -> anyhow::Result<bool> {
        let Some(ref cmd) = cli.command else {
            return Ok(false);
        };
        match cmd {
            Command::ConfigPath => {
                {
                    ::std::io::_print(
                        format_args!("{0}\n", paths.config_file.display()),
                    );
                };
                Ok(true)
            }
            Command::DataPath => {
                {
                    ::std::io::_print(format_args!("{0}\n", paths.data_dir.display()));
                };
                Ok(true)
            }
            Command::DbPath => {
                {
                    ::std::io::_print(format_args!("{0}\n", paths.db_file.display()));
                };
                Ok(true)
            }
            Command::Doctor => {
                let checks = crate::paths::run_doctor(paths, config, cli.headless);
                crate::paths::print_doctor_results(&checks);
                Ok(true)
            }
            Command::MigrateDb => {
                handle_migrate_db(paths)?;
                Ok(true)
            }
            Command::Sessions => {
                handle_sessions_command(paths)?;
                Ok(true)
            }
            Command::NewSession => {
                handle_new_session_command(paths)?;
                Ok(true)
            }
            Command::SeedDemo { clear } => {
                handle_seed_demo_command(paths, *clear).await?;
                Ok(true)
            }
            Command::Models { action, args } => {
                handle_models_command(config, action, args)?;
                Ok(true)
            }
            Command::Providers { action } => {
                handle_providers_command(config, action)?;
                Ok(true)
            }
            Command::Brain { action, args } => {
                let manager = crate::brain_index::BrainIndexManager::new(
                    paths.clone(),
                    config.brain_index.clone(),
                    &config.embeddings,
                );
                match action.as_str() {
                    "dedupe" => {
                        {
                            ::std::io::_print(
                                format_args!("[BRAIN] Starting deduplication...\n"),
                            );
                        };
                        let count = manager.dedupe()?;
                        {
                            ::std::io::_print(
                                format_args!(
                                    "[BRAIN] Deduplication complete. Removed {0} duplicates.\n",
                                    count,
                                ),
                            );
                        };
                    }
                    "pack" => {
                        let query = args.join(" ");
                        if query.is_empty() {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "[BRAIN] Please provide a query for the context pack.\n",
                                    ),
                                );
                            };
                            return Ok(true);
                        }
                        let builder = crate::brain_context::BrainContextPackBuilder::new(
                                &manager,
                                query,
                            )
                            .limit_items(5);
                        let pack = builder.build().await?;
                        {
                            ::std::io::_print(
                                format_args!("[BRAIN] Context Pack Generated:\n"),
                            );
                        };
                        {
                            ::std::io::_print(format_args!("Title: {0}\n", pack.title));
                        };
                        {
                            ::std::io::_print(
                                format_args!("Summary: {0}\n", pack.summary),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Size: {0} characters\n", pack.estimated_size),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Items: {0}\n", pack.items.len()),
                            );
                        };
                        for (i, doc) in pack.items.iter().enumerate() {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "  {0}) [{1:?}] {2}\n",
                                        i + 1,
                                        doc.kind,
                                        doc.title,
                                    ),
                                );
                            };
                        }
                    }
                    _ => {
                        {
                            ::std::io::_print(
                                format_args!("[BRAIN] Unknown action: {0}\n", action),
                            );
                        };
                    }
                }
                Ok(true)
            }
            Command::Project { action } => {
                handle_project_command(paths, config, action)?;
                Ok(true)
            }
            Command::Memory { action, text } => {
                handle_memory_command(paths, config, action, text.as_deref())?;
                Ok(true)
            }
            Command::Recall { query } => {
                handle_recall_command(paths, query)?;
                Ok(true)
            }
            Command::Skills { action, arg, session } => {
                handle_skills_command(
                        paths,
                        config,
                        action,
                        arg.as_deref(),
                        session.as_deref(),
                    )
                    .await?;
                Ok(true)
            }
            Command::RepoMap { action } => {
                handle_repo_map_command(paths, config, action)?;
                Ok(true)
            }
            Command::Check => {
                handle_dev_command("check")?;
                Ok(true)
            }
            Command::Test { args } => {
                let extra = args.join(" ");
                handle_dev_command_with_args(
                    "test",
                    if extra.is_empty() { None } else { Some(&extra) },
                )?;
                Ok(true)
            }
            Command::Lint => {
                handle_dev_command("lint")?;
                Ok(true)
            }
            Command::Format => {
                handle_dev_command("format")?;
                Ok(true)
            }
            Command::Daemon { action } => {
                handle_daemon_command(paths, config, action).await?;
                Ok(true)
            }
            Command::Dashboard { action } => {
                handle_dashboard_command(action);
                Ok(true)
            }
            Command::Desktop { action } => {
                handle_desktop_command(action);
                Ok(true)
            }
            Command::Rollback { id } => {
                {
                    ::std::io::_print(
                        format_args!(
                            "Rollback via CLI defaults to \'plan\' mode to prevent accidental data loss.\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "To safely restore or perform a destructive rollback, launch GOAT (cargo run) and type:\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("  /rollback plan {0}\n", id));
                };
                {
                    ::std::io::_print(format_args!("  /rollback restore {0}\n", id));
                };
                {
                    ::std::io::_print(format_args!("  /rollback destructive {0}\n", id));
                };
                Ok(true)
            }
            Command::Branch { action, name } => {
                let root = std::env::current_dir().unwrap_or_default();
                match action.as_str() {
                    "current" => {
                        if let Some(git) = crate::repo_map::GitStatus::read(&root) {
                            {
                                ::std::io::_print(
                                    format_args!("Current branch: {0}\n", git.branch),
                                );
                            };
                        } else {
                            {
                                ::std::io::_print(
                                    format_args!("Not in a git repository.\n"),
                                );
                            };
                        }
                    }
                    "create" => {
                        if let Some(n) = name {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "Branch creation requires approval. Please run GOAT interactively and type /branch create {0}\n",
                                        n,
                                    ),
                                );
                            };
                        } else {
                            {
                                ::std::io::_print(
                                    format_args!("Please specify a branch name.\n"),
                                );
                            };
                        }
                    }
                    _ => {
                        ::std::io::_print(
                            format_args!("Unknown action. Use current or create.\n"),
                        );
                    }
                }
                Ok(true)
            }
            Command::Commit { action } => {
                match action.as_str() {
                    "message" => {
                        let root = std::env::current_dir().unwrap_or_default();
                        let status_out = std::process::Command::new("git")
                            .args(["-C", &root.to_string_lossy(), "status", "--short"])
                            .output()
                            .ok()
                            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                            .unwrap_or_default();
                        let diff_out = std::process::Command::new("git")
                            .args([
                                "-C",
                                &root.to_string_lossy(),
                                "diff",
                                "--cached",
                                "--stat",
                            ])
                            .output()
                            .ok()
                            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
                            .unwrap_or_default();
                        if status_out.trim().is_empty() {
                            {
                                ::std::io::_print(
                                    format_args!("No changes detected. Working tree clean.\n"),
                                );
                            };
                        } else {
                            {
                                ::std::io::_print(
                                    format_args!("Proposed deterministic commit message:\n\n"),
                                );
                            };
                            {
                                ::std::io::_print(
                                    format_args!("feat: Update project files\n\n"),
                                );
                            };
                            for line in status_out
                                .lines()
                                .filter(|l| !l.trim().is_empty())
                            {
                                {
                                    ::std::io::_print(format_args!("- {0}\n", line.trim()));
                                };
                            }
                            if !diff_out.trim().is_empty() {
                                {
                                    ::std::io::_print(
                                        format_args!("\nDiff stat:\n{0}\n", diff_out.trim()),
                                    );
                                };
                            }
                        }
                    }
                    "create" => {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Commit creation requires approval. Please run GOAT interactively and type /commit create\n",
                                ),
                            );
                        };
                    }
                    _ => {
                        ::std::io::_print(
                            format_args!("Unknown action. Use message or create.\n"),
                        );
                    }
                }
                Ok(true)
            }
            Command::Extensions { action, args } => {
                handle_extensions_command(paths, config, &action, &args)?;
                Ok(true)
            }
            Command::Browser { action, args } => {
                handle_browser_command(paths, config, &action, &args)?;
                Ok(true)
            }
            Command::Builder { action, args } => {
                handle_builder_command(paths, config, &action, &args)?;
                Ok(true)
            }
            Command::Researcher { action, args } => {
                handle_researcher_command(paths, config, &action, &args)?;
                Ok(true)
            }
            Command::Tools { action, arg } => {
                handle_tools_command(paths, config, &action, arg.as_deref())?;
                Ok(true)
            }
            Command::Subagents { action, arg } => {
                handle_subagents_command(paths, config, &action, arg.as_deref())?;
                Ok(true)
            }
            Command::AskAgent { name, task } => {
                let (rt, _) = crate::runtime::GoatRuntime::bootstrap(
                    config.clone(),
                    paths.clone(),
                    ::alloc::vec::Vec::new(),
                    false,
                    None,
                );
                handle_ask_agent_command(&name, &task, &rt).await?;
                Ok(true)
            }
            Command::ExternalAgents { action, arg } => {
                let (rt, _) = crate::runtime::GoatRuntime::bootstrap(
                    config.clone(),
                    paths.clone(),
                    ::alloc::vec::Vec::new(),
                    false,
                    None,
                );
                handle_external_agents_command(rt, &action, arg.as_deref());
                Ok(true)
            }
            Command::DelegateExternal { agent, task } => {
                let (rt, _) = crate::runtime::GoatRuntime::bootstrap(
                    config.clone(),
                    paths.clone(),
                    ::alloc::vec::Vec::new(),
                    false,
                    None,
                );
                handle_delegate_external_command(rt, &agent, &task).await;
                Ok(true)
            }
            Command::Mcp { action, arg } => {
                handle_mcp_command(paths, config, action, arg)?;
                Ok(true)
            }
            Command::Hooks { action, arg } => {
                handle_hooks_command(paths, config, &action, arg.as_deref())?;
                Ok(true)
            }
            Command::Schedule { action, args } => {
                handle_schedule_command(paths, config, &action, &args)?;
                Ok(true)
            }
            Command::Jobs { action, arg } => {
                handle_jobs_command(paths, config, &action, arg.as_deref())?;
                Ok(true)
            }
            Command::Mission { action, args } => {
                let mc = crate::mission_control::MissionControlManager::new();
                if action == "plan" && !args.is_empty() {
                    let goal = args.join(" ");
                    let req = crate::mission_control::MissionPlanReq {
                        goal,
                        project_id: None,
                        constraints: None,
                    };
                    let plan = mc.plan_goal(&req);
                    {
                        ::std::io::_print(
                            format_args!(
                                "Created Mission: {0} (Type: {1:?})\n",
                                plan.title,
                                plan.mission_type,
                            ),
                        );
                    };
                    for step in plan.plan_steps {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "  - [{0}] {1} (Agent: {2:?})\n",
                                    step.status,
                                    step.title,
                                    step.assigned_agent,
                                ),
                            );
                        };
                    }
                } else {
                    let missions = mc.get_missions();
                    if let Some(m) = missions.first() {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Active Mission: {0} ({1:?})\n",
                                    m.title,
                                    m.status,
                                ),
                            );
                        };
                        {
                            ::std::io::_print(format_args!("  Goal: {0}\n", m.raw_goal));
                        };
                        {
                            ::std::io::_print(
                                format_args!("  Progress: {0}%\n", m.progress),
                            );
                        };
                    } else {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "No active missions found. Run `goat mission plan \"<goal>\"` to plan one.\n",
                                ),
                            );
                        };
                    }
                }
                {
                    ::std::io::_print(
                        format_args!(
                            "\nView the full Mission Control workspace at http://127.0.0.1:3000/mission-control\n",
                        ),
                    );
                };
                Ok(true)
            }
            Command::Learn { path } => {
                let target_path = path.clone().unwrap_or_else(|| ".".to_string());
                let target_path_buf = std::path::PathBuf::from(&target_path);
                let canonical = target_path_buf
                    .canonicalize()
                    .unwrap_or_else(|_| target_path_buf.clone());
                {
                    ::std::io::_print(
                        format_args!("You are about to scan: {0}\n", canonical.display()),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "This will analyze files for tech stack, commands, and project context.\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Sensitive files (secrets, .env) and large directories (.git, node_modules) will be ignored.\n",
                        ),
                    );
                };
                let mut prompt = String::new();
                {
                    ::std::io::_print(format_args!("Do you want to proceed? [y/N]: \n"));
                };
                std::io::stdin().read_line(&mut prompt).ok();
                if prompt.trim().to_lowercase() != "y" {
                    {
                        ::std::io::_print(format_args!("Scan aborted.\n"));
                    };
                    return Ok(true);
                }
                let scanner = crate::project_intelligence::DeepProjectScanner::new(
                    canonical,
                );
                match scanner.scan() {
                    Ok(pi) => {
                        let manager = crate::project_intelligence::ProjectIntelligenceManager::new();
                        manager.save_project(&pi)?;
                        {
                            ::std::io::_print(
                                format_args!("\nProject learned successfully!\n"),
                            );
                        };
                        {
                            ::std::io::_print(format_args!("Name: {0}\n", pi.name));
                        };
                        {
                            ::std::io::_print(format_args!("ID: {0}\n", pi.project_id));
                        };
                        {
                            ::std::io::_print(
                                format_args!("Stack: {0}\n", pi.detected_stack.join(", ")),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Summary: {0}\n", pi.architecture_summary),
                            );
                        };
                        if !pi.risk_notes.is_empty() {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "Notes: {0} sensitive files ignored.\n",
                                        pi.risk_notes.len(),
                                    ),
                                );
                            };
                        }
                    }
                    Err(e) => {
                        ::std::io::_print(
                            format_args!("Failed to scan project: {0}\n", e),
                        );
                    }
                }
                Ok(true)
            }
            Command::Patch { action, args } => {
                let patch_manager = crate::patch_manager::PatchManager::new();
                if action == "list" {
                    let patches = patch_manager.get_patches();
                    if patches.is_empty() {
                        {
                            ::std::io::_print(format_args!("No patches found.\n"));
                        };
                    } else {
                        for p in patches {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "- {0} [{1}] ({2}) : {3}\n",
                                        p.patch_id,
                                        p.status,
                                        p.project_id,
                                        p.title,
                                    ),
                                );
                            };
                        }
                    }
                } else if action == "show" {
                    if let Some(id) = args.first() {
                        if let Some(p) = patch_manager.get_patch(id) {
                            {
                                ::std::io::_print(
                                    format_args!("Patch: {0} ({1})\n", p.patch_id, p.status),
                                );
                            };
                            {
                                ::std::io::_print(format_args!("Title: {0}\n", p.title));
                            };
                            {
                                ::std::io::_print(
                                    format_args!("Project ID: {0}\n", p.project_id),
                                );
                            };
                            {
                                ::std::io::_print(
                                    format_args!("Mission ID: {0}\n", p.mission_id),
                                );
                            };
                            {
                                ::std::io::_print(
                                    format_args!("Diff Preview:\n{0}\n", p.diff_preview),
                                );
                            };
                        } else {
                            {
                                ::std::io::_print(format_args!("Patch not found.\n"));
                            };
                        }
                    } else {
                        {
                            ::std::io::_print(
                                format_args!("Usage: goat patch show <patch_id>\n"),
                            );
                        };
                    }
                } else if action == "propose" {
                    if let Some(mission_id) = args.first() {
                        let mc = crate::mission_control::MissionControl::new();
                        if let Some(mission) = mc.get_mission(mission_id) {
                            if mission.linked_project.is_empty() {
                                {
                                    ::std::io::_print(
                                        format_args!("Mission is not linked to a project.\n"),
                                    );
                                };
                            } else {
                                let pi_mgr = crate::project_intelligence::ProjectIntelligenceManager::new();
                                if let Some(project) = pi_mgr
                                    .get_project(&mission.linked_project)
                                {
                                    match patch_manager
                                        .generate_patch_proposal(&mission, &project)
                                    {
                                        Ok(patch) => {
                                            patch_manager.save_patch(&patch).unwrap();
                                            {
                                                ::std::io::_print(
                                                    format_args!(
                                                        "Patch proposed successfully! ID: {0}\n",
                                                        patch.patch_id,
                                                    ),
                                                );
                                            };
                                            {
                                                ::std::io::_print(
                                                    format_args!("Title: {0}\n", patch.title),
                                                );
                                            };
                                            {
                                                ::std::io::_print(
                                                    format_args!(
                                                        "Review it with `goat patch show {0}`\n",
                                                        patch.patch_id,
                                                    ),
                                                );
                                            };
                                            {
                                                ::std::io::_print(
                                                    format_args!(
                                                        "Apply it with `goat patch apply {0}`\n",
                                                        patch.patch_id,
                                                    ),
                                                );
                                            };
                                        }
                                        Err(e) => {
                                            ::std::io::_print(
                                                format_args!("Failed to propose patch: {0}\n", e),
                                            );
                                        }
                                    }
                                } else {
                                    {
                                        ::std::io::_print(
                                            format_args!(
                                                "Project intelligence not found for ID: {0}\n",
                                                mission.linked_project,
                                            ),
                                        );
                                    };
                                }
                            }
                        } else {
                            {
                                ::std::io::_print(format_args!("Mission not found.\n"));
                            };
                        }
                    } else {
                        {
                            ::std::io::_print(
                                format_args!("Usage: goat patch propose <mission_id>\n"),
                            );
                        };
                    }
                } else if action == "apply" {
                    if let Some(id) = args.first() {
                        if let Some(mut patch) = patch_manager.get_patch(id) {
                            if patch.status != "proposed" {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "Patch status is \'{0}\', cannot apply.\n",
                                            patch.status,
                                        ),
                                    );
                                };
                                return Ok(true);
                            }
                            let pi_mgr = crate::project_intelligence::ProjectIntelligenceManager::new();
                            if let Some(project) = pi_mgr.get_project(&patch.project_id)
                            {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "You are about to apply patch \'{0}\' to project \'{1}\'.\n",
                                            patch.patch_id,
                                            project.name,
                                        ),
                                    );
                                };
                                {
                                    ::std::io::_print(
                                        format_args!("Diff Preview:\n{0}\n", patch.diff_preview),
                                    );
                                };
                                use std::io::Write;
                                {
                                    ::std::io::_print(
                                        format_args!("Do you approve this patch? [y/N]: "),
                                    );
                                };
                                std::io::stdout().flush().unwrap();
                                let mut input = String::new();
                                std::io::stdin().read_line(&mut input).unwrap();
                                if input.trim().eq_ignore_ascii_case("y") {
                                    let cp_mgr = crate::checkpoint::CheckpointManager::new(
                                        &crate::paths::GoatPaths::default().data_dir,
                                    );
                                    match cp_mgr
                                        .create_checkpoint(
                                            &project.root_path,
                                            &::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!("Pre-patch {0}", patch.patch_id),
                                                )
                                            }),
                                        )
                                    {
                                        Ok(cp) => {
                                            {
                                                ::std::io::_print(
                                                    format_args!("Checkpoint created: {0}\n", cp.id),
                                                );
                                            };
                                            patch.checkpoint_id = Some(cp.id.clone());
                                        }
                                        Err(e) => {
                                            {
                                                ::std::io::_print(
                                                    format_args!("Failed to create checkpoint: {0}\n", e),
                                                );
                                            };
                                            {
                                                ::std::io::_print(
                                                    format_args!("Aborting patch application.\n"),
                                                );
                                            };
                                            return Ok(true);
                                        }
                                    }
                                    match patch_manager
                                        .apply_patch(&mut patch, &project.root_path)
                                    {
                                        Ok(_) => {
                                            {
                                                ::std::io::_print(
                                                    format_args!("Patch applied successfully.\n"),
                                                );
                                            };
                                            let mut commands_to_suggest = Vec::new();
                                            commands_to_suggest.extend(project.test_commands.clone());
                                            commands_to_suggest.extend(project.lint_commands.clone());
                                            commands_to_suggest.extend(project.build_commands.clone());
                                            if !commands_to_suggest.is_empty() {
                                                {
                                                    ::std::io::_print(
                                                        format_args!("\nDetected validation commands:\n"),
                                                    );
                                                };
                                                for cmd in &commands_to_suggest {
                                                    {
                                                        ::std::io::_print(format_args!("- {0}\n", cmd));
                                                    };
                                                }
                                                {
                                                    ::std::io::_print(
                                                        format_args!("Run these commands now? [y/N]: "),
                                                    );
                                                };
                                                std::io::stdout().flush().unwrap();
                                                let mut run_input = String::new();
                                                std::io::stdin().read_line(&mut run_input).unwrap();
                                                if run_input.trim().eq_ignore_ascii_case("y") {
                                                    for cmd in &commands_to_suggest {
                                                        {
                                                            ::std::io::_print(format_args!("Running: {0}\n", cmd));
                                                        };
                                                        let mut parts = cmd.split_whitespace();
                                                        if let Some(prog) = parts.next() {
                                                            let args: Vec<&str> = parts.collect();
                                                            let mut child = std::process::Command::new(prog)
                                                                .args(args)
                                                                .current_dir(&project.root_path)
                                                                .spawn();
                                                            if let Ok(mut c) = child {
                                                                let _ = c.wait();
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            ::std::io::_print(
                                                format_args!("Failed to apply patch: {0}\n", e),
                                            );
                                        }
                                    }
                                } else {
                                    {
                                        ::std::io::_print(
                                            format_args!("Patch application cancelled.\n"),
                                        );
                                    };
                                }
                            } else {
                                {
                                    ::std::io::_print(format_args!("Project not found.\n"));
                                };
                            }
                        } else {
                            {
                                ::std::io::_print(format_args!("Patch not found.\n"));
                            };
                        }
                    } else {
                        {
                            ::std::io::_print(
                                format_args!("Usage: goat patch apply <patch_id>\n"),
                            );
                        };
                    }
                } else {
                    {
                        ::std::io::_print(format_args!("Unknown patch action.\n"));
                    };
                }
                Ok(true)
            }
            Command::CheckpointCmd { action, args } => {
                let cp_mgr = crate::checkpoint::CheckpointManager::new(
                    &crate::paths::GoatPaths::default().data_dir,
                );
                if action == "list" {
                    if let Ok(checkpoints) = cp_mgr.list_checkpoints() {
                        if checkpoints.is_empty() {
                            {
                                ::std::io::_print(format_args!("No checkpoints found.\n"));
                            };
                        } else {
                            for cp in checkpoints {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "- {0} [{1}] {2} (Files changed: {3})\n",
                                            cp.id,
                                            cp.timestamp,
                                            cp.label,
                                            cp.changed_files.len(),
                                        ),
                                    );
                                };
                            }
                        }
                    } else {
                        {
                            ::std::io::_print(
                                format_args!("Failed to list checkpoints.\n"),
                            );
                        };
                    }
                } else if action == "restore" {
                    {
                        ::std::io::_print(
                            format_args!(
                                "Restore functionality will be implemented in the next phase.\n",
                            ),
                        );
                    };
                } else {
                    {
                        ::std::io::_print(format_args!("Unknown checkpoint action.\n"));
                    };
                }
                Ok(true)
            }
            Command::Projects { action, args } => {
                let manager = crate::project_intelligence::ProjectIntelligenceManager::new();
                if action == "list" {
                    let projects = manager.get_projects();
                    if projects.is_empty() {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "No projects learned yet. Run `goat learn <path>` to add one.\n",
                                ),
                            );
                        };
                    } else {
                        {
                            ::std::io::_print(format_args!("Learned Projects:\n"));
                        };
                        for p in projects {
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "- {0} ({1}) | {2}\n",
                                        p.name,
                                        p.project_id,
                                        p.architecture_summary,
                                    ),
                                );
                            };
                        }
                    }
                } else if action == "show" {
                    if let Some(id) = args.first() {
                        if let Some(p) = manager.get_project(id) {
                            {
                                ::std::io::_print(
                                    format_args!("Project: {0} ({1})\n", p.name, p.project_id),
                                );
                            };
                            {
                                ::std::io::_print(
                                    format_args!("Path: {0}\n", p.root_path.display()),
                                );
                            };
                            {
                                ::std::io::_print(
                                    format_args!("Stack: {0}\n", p.detected_stack.join(", ")),
                                );
                            };
                            {
                                ::std::io::_print(format_args!("Commands:\n"));
                            };
                            for cmd in p.available_commands {
                                {
                                    ::std::io::_print(format_args!("  - {0}\n", cmd));
                                };
                            }
                        } else {
                            {
                                ::std::io::_print(format_args!("Project not found.\n"));
                            };
                        }
                    } else {
                        {
                            ::std::io::_print(
                                format_args!("Usage: goat projects show <id>\n"),
                            );
                        };
                    }
                } else if action == "scan" {
                    {
                        ::std::io::_print(
                            format_args!("Use `goat learn <path>` instead.\n"),
                        );
                    };
                } else {
                    {
                        ::std::io::_print(format_args!("Unknown action: {0}\n", action));
                    };
                }
                Ok(true)
            }
        }
    }
    fn handle_sessions_command(paths: &crate::paths::GoatPaths) -> anyhow::Result<()> {
        use anyhow::Context;
        if !paths.db_file.exists() {
            {
                ::std::io::_print(
                    format_args!(
                        "No brain database found at {0}\n",
                        paths.db_file.display(),
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!("Run `goat` to create your first session.\n"),
                );
            };
            return Ok(());
        }
        let brain = crate::brain::Brain::new(&paths.db_file)
            .with_context(|| ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("could not open database: {0}", paths.db_file.display()),
                )
            }))?;
        let records = brain
            .get_session_records()
            .context("could not read sessions from database")?;
        if records.is_empty() {
            {
                ::std::io::_print(
                    format_args!("No sessions found in {0}\n", paths.db_file.display()),
                );
            };
            return Ok(());
        }
        {
            ::std::io::_print(format_args!("Sessions ({0}):\n", records.len()));
        };
        {
            ::std::io::_print(format_args!("{0}\n", "─".repeat(78)));
        };
        {
            ::std::io::_print(
                format_args!(
                    "  {0:<10}  {1:<5}  {2:<20}  {3:<20}  {4}\n",
                    "ID",
                    "Type",
                    "Created",
                    "Updated",
                    "Title",
                ),
            );
        };
        {
            ::std::io::_print(format_args!("{0}\n", "─".repeat(78)));
        };
        for rec in &records {
            let short_id = if rec.id.len() > 8 {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0}…", &rec.id[..8]))
                })
            } else {
                rec.id.clone()
            };
            let kind = if rec.is_uuid() { "uuid" } else { "legacy" };
            let created = rec.created_at.get(..16).unwrap_or(&rec.created_at);
            let updated = rec.updated_at.get(..16).unwrap_or(&rec.updated_at);
            let title = if rec.title.len() > 28 {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("{0}…", &rec.title[..27]))
                })
            } else {
                rec.title.clone()
            };
            {
                ::std::io::_print(
                    format_args!(
                        "  {0:<10}  {1:<5}  {2:<20}  {3:<20}  {4}\n",
                        short_id,
                        kind,
                        created,
                        updated,
                        title,
                    ),
                );
            };
        }
        {
            ::std::io::_print(format_args!("{0}\n", "─".repeat(78)));
        };
        {
            ::std::io::_print(format_args!("Database: {0}\n", paths.db_file.display()));
        };
        Ok(())
    }
    async fn handle_seed_demo_command(
        paths: &crate::paths::GoatPaths,
        clear: bool,
    ) -> anyhow::Result<()> {
        use std::fs;
        {
            ::std::io::_print(
                format_args!("Seeding demo data for dashboard flows...\n"),
            );
        };
        let prime_dir = paths.data_dir.join("agents").join("prime");
        let cofounder_file = prime_dir.join("cofounder").join("ideas.jsonl");
        let learner_goals = prime_dir.join("learner").join("goals.jsonl");
        let learner_roadmaps = prime_dir.join("learner").join("roadmaps.jsonl");
        let promptforge_hist = paths.data_dir.join("promptforge").join("history.jsonl");
        let reports_dir = paths.data_dir.join("reports");
        if clear {
            {
                ::std::io::_print(format_args!("Clearing existing demo data...\n"));
            };
            let clear_jsonl = |path: &std::path::PathBuf| {
                if path.exists() {
                    if let Ok(content) = fs::read_to_string(path) {
                        let filtered: Vec<&str> = content
                            .lines()
                            .filter(|l| !l.contains("[DEMO]"))
                            .collect();
                        let _ = fs::write(path, filtered.join("\n"));
                    }
                }
            };
            clear_jsonl(&cofounder_file);
            clear_jsonl(&learner_goals);
            clear_jsonl(&learner_roadmaps);
            clear_jsonl(&promptforge_hist);
        } else {
            {
                ::std::io::_print(format_args!("Seeding Cofounder ideas...\n"));
            };
            if let Ok(mut cofounder) = crate::agents::cofounder::CofounderManager::new() {
                let _ = cofounder
                    .add_idea(
                        "[DEMO] AI Developer CLI".to_string(),
                        "A terminal-native AI agent platform written in Rust"
                            .to_string(),
                        "Developers".to_string(),
                    );
                let _ = cofounder
                    .add_idea(
                        "[DEMO] HyperFrames Video Studio".to_string(),
                        "Create programmatic videos using React and HTML".to_string(),
                        "Creators".to_string(),
                    );
            }
            {
                ::std::io::_print(format_args!("Seeding Learner goals...\n"));
            };
            if let Ok(learner) = crate::agents::learner::LearnerAgent::new() {
                if let Ok(goal) = learner
                    .create_goal(
                        "[DEMO] Master Rust Concurrency",
                        crate::agents::learner::LearningDomain::Rust,
                    )
                {
                    let _ = learner.create_roadmap(&goal.id);
                }
            }
            {
                ::std::io::_print(format_args!("Seeding Reports...\n"));
            };
            let report_mgr = crate::reports::ReportManager::new();
            let _ = report_mgr
                .generate_report(crate::reports::ReportTemplate {
                    kind: crate::reports::ReportKind::Research,
                    title: "[DEMO] Rust Async Ecosystem".into(),
                    sections: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            [
                                crate::reports::ReportSection {
                                    heading: "Overview".into(),
                                    body: "Tokio remains the dominant runtime for async Rust."
                                        .into(),
                                },
                            ],
                        ),
                    ),
                });
            let _ = report_mgr
                .generate_report(crate::reports::ReportTemplate {
                    kind: crate::reports::ReportKind::CodeReview,
                    title: "[DEMO] Phase 6.5 Audit".into(),
                    sections: ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            [
                                crate::reports::ReportSection {
                                    heading: "Security".into(),
                                    body: "Passed all automated checks.".into(),
                                },
                            ],
                        ),
                    ),
                });
        }
        {
            ::std::io::_print(
                format_args!(
                    "Demo seed/clear complete! Run `goat dashboard` to see the changes.\n",
                ),
            );
        };
        Ok(())
    }
    fn handle_new_session_command(
        paths: &crate::paths::GoatPaths,
    ) -> anyhow::Result<()> {
        use anyhow::Context;
        use uuid::Uuid;
        let session_id = Uuid::new_v4().to_string();
        if paths.db_file.exists() {
            let brain = crate::brain::Brain::new(&paths.db_file)
                .with_context(|| ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "could not open database: {0}",
                            paths.db_file.display(),
                        ),
                    )
                }))?;
            brain
                .create_session(&session_id, "New Session")
                .context("could not create session")?;
            {
                ::std::io::_print(format_args!("{0}\n", session_id));
            };
            {
                ::std::io::_eprint(
                    format_args!("[GOAT] New session created: {0}\n", session_id),
                );
            };
            {
                ::std::io::_eprint(
                    format_args!("[GOAT] Database: {0}\n", paths.db_file.display()),
                );
            };
        } else {
            {
                ::std::io::_print(format_args!("{0}\n", session_id));
            };
            {
                ::std::io::_eprint(
                    format_args!(
                        "[GOAT] No brain database yet. Session ID reserved: {0}\n",
                        session_id,
                    ),
                );
            };
            {
                ::std::io::_eprint(
                    format_args!(
                        "[GOAT] Run `goat` to start and persist this session.\n",
                    ),
                );
            };
        }
        Ok(())
    }
    fn handle_extensions_command(
        paths: &crate::paths::GoatPaths,
        config: &crate::config::Config,
        action: &str,
        args: &[String],
    ) -> anyhow::Result<()> {
        use crate::extensions::ExtensionRegistry;
        let mut registry = ExtensionRegistry::new(
            paths.config_file.parent().unwrap_or(std::path::Path::new("/")),
            &paths.data_dir,
        )?;
        registry.load_state()?;
        match action {
            "list" => {
                {
                    ::std::io::_print(format_args!("Extension Registry (Phase 6.8)\n"));
                };
                {
                    ::std::io::_print(format_args!("{0:-<80}\n", ""));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "{0:<30} | {1:<15} | {2:<15} | {3:<10}\n",
                            "ID",
                            "Kind",
                            "Status",
                            "Trust",
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("{0:-<80}\n", ""));
                };
                let mut records = registry.list_extensions();
                records.sort_by_key(|r| r.manifest.id.clone());
                for r in records {
                    {
                        ::std::io::_print(
                            format_args!(
                                "{0:<30} | {1:<15?} | {2:<15?} | {3:<10?}\n",
                                r.manifest.id,
                                r.manifest.kind,
                                r.status,
                                r.trust_level,
                            ),
                        );
                    };
                }
            }
            "discover" => {
                if args.is_empty() {
                    {
                        ::std::io::_print(
                            format_args!("Usage: goat extensions discover <path>\n"),
                        );
                    };
                    return Ok(());
                }
                let path = std::path::Path::new(&args[0]);
                match registry.discover_local(path) {
                    Ok(id) => {
                        ::std::io::_print(
                            format_args!("Discovered extension: {0}\n", id),
                        );
                    }
                    Err(e) => {
                        ::std::io::_print(
                            format_args!("Error discovering extension: {0}\n", e),
                        );
                    }
                }
            }
            "audit" => {
                if args.is_empty() {
                    {
                        ::std::io::_print(
                            format_args!("Usage: goat extensions audit <id>\n"),
                        );
                    };
                    return Ok(());
                }
                match registry.audit_extension(&args[0]) {
                    Ok(result) => {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Audit Results for {0}: \n",
                                    result.extension_id,
                                ),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Passed: {0}\n", result.passed),
                            );
                        };
                        if result.findings.is_empty() {
                            {
                                ::std::io::_print(format_args!("No findings.\n"));
                            };
                        } else {
                            for finding in result.findings {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "- [{0:?}] {1}\n",
                                            finding.severity,
                                            finding.message,
                                        ),
                                    );
                                };
                            }
                        }
                    }
                    Err(e) => {
                        ::std::io::_print(format_args!("Error: {0}\n", e));
                    }
                }
            }
            "install" => {
                if args.is_empty() {
                    {
                        ::std::io::_print(
                            format_args!("Usage: goat extensions install <id>\n"),
                        );
                    };
                    return Ok(());
                }
                let id = &args[0];
                if let Some(record) = registry.get_extension(id) {
                    if record.trust_level
                        != crate::extensions::ExtensionTrustLevel::LocalBuiltin
                    {
                        {
                            ::std::io::_print(
                                format_args!("Warning: Installing untrusted extension.\n"),
                            );
                        };
                    }
                }
                match registry.install_extension(id) {
                    Ok(_) => {
                        ::std::io::_print(
                            format_args!(
                                "Successfully installed {0}. It is currently DISABLED.\n",
                                id,
                            ),
                        );
                    }
                    Err(e) => {
                        ::std::io::_print(format_args!("Error installing: {0}\n", e));
                    }
                }
            }
            "enable" => {
                if args.is_empty() {
                    {
                        ::std::io::_print(
                            format_args!("Usage: goat extensions enable <id>\n"),
                        );
                    };
                    return Ok(());
                }
                match registry.enable_extension(&args[0]) {
                    Ok(_) => {
                        ::std::io::_print(
                            format_args!("Successfully enabled {0}.\n", args[0]),
                        );
                    }
                    Err(e) => {
                        ::std::io::_print(format_args!("Error enabling: {0}\n", e));
                    }
                }
            }
            "disable" => {
                if args.is_empty() {
                    {
                        ::std::io::_print(
                            format_args!("Usage: goat extensions disable <id>\n"),
                        );
                    };
                    return Ok(());
                }
                match registry.disable_extension(&args[0]) {
                    Ok(_) => {
                        ::std::io::_print(
                            format_args!("Successfully disabled {0}.\n", args[0]),
                        );
                    }
                    Err(e) => {
                        ::std::io::_print(format_args!("Error disabling: {0}\n", e));
                    }
                }
            }
            "remove" => {
                if args.is_empty() {
                    {
                        ::std::io::_print(
                            format_args!("Usage: goat extensions remove <id>\n"),
                        );
                    };
                    return Ok(());
                }
                match registry.remove_extension(&args[0]) {
                    Ok(_) => {
                        ::std::io::_print(
                            format_args!("Successfully removed {0}.\n", args[0]),
                        );
                    }
                    Err(e) => {
                        ::std::io::_print(format_args!("Error removing: {0}\n", e));
                    }
                }
            }
            _ => {
                {
                    ::std::io::_print(format_args!("Unknown action: {0}\n", action));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Supported actions: list, discover, audit, install, enable, disable, remove\n",
                        ),
                    );
                };
            }
        }
        Ok(())
    }
    fn handle_tools_command(
        paths: &crate::paths::GoatPaths,
        config: &crate::config::Config,
        action: &str,
        arg: Option<&str>,
    ) -> anyhow::Result<()> {
        let registry = crate::tool_registry::ToolRegistry::new();
        match action {
            "list" => {
                {
                    ::std::io::_print(
                        format_args!(
                            "GOAT Tool Registry ({0} tools)\n",
                            registry.list_all().len(),
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("{0:-<80}\n", ""));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "{0:<20} | {1:<15} | {2:<10} | {3:<10} | {4}\n",
                            "Name",
                            "Category",
                            "Risk",
                            "Approval",
                            "Permission",
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("{0:-<80}\n", ""));
                };
                for tool in registry.list_all() {
                    let perm = registry.get_permission(&tool.name, &config.tools);
                    let approval = if tool.requires_approval {
                        "Required"
                    } else {
                        "None"
                    };
                    {
                        ::std::io::_print(
                            format_args!(
                                "{0:<20} | {1:<15} | {2:<10} | {3:<10} | {4:?}\n",
                                tool.name,
                                tool.category.to_string(),
                                tool.risk_level.to_string(),
                                approval,
                                perm,
                            ),
                        );
                    };
                }
            }
            "show" => {
                if let Some(name) = arg {
                    if let Some(tool) = registry.get(name) {
                        {
                            ::std::io::_print(format_args!("Tool: {0}\n", tool.name));
                        };
                        {
                            ::std::io::_print(
                                format_args!("Description: {0}\n", tool.description),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Category: {0}\n", tool.category),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Risk Level: {0}\n", tool.risk_level),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Requires Approval: {0}\n",
                                    tool.requires_approval,
                                ),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Read Only: {0}\n", tool.read_only),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Permission Group: {0}\n",
                                    tool.permission_group,
                                ),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Effective Permission: {0:?}\n",
                                    registry.get_permission(&tool.name, &config.tools),
                                ),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Effective Action: {0:?}\n",
                                    registry.evaluate_action(&tool.name, &config.tools),
                                ),
                            );
                        };
                    } else {
                        {
                            ::std::io::_print(
                                format_args!("Tool \'{0}\' not found.\n", name),
                            );
                        };
                    }
                } else {
                    {
                        ::std::io::_print(
                            format_args!(
                                "Please provide a tool name. Example: goat tools show bash\n",
                            ),
                        );
                    };
                }
            }
            "categories" => {
                {
                    ::std::io::_print(format_args!("Tool Categories:\n"));
                };
                {
                    ::std::io::_print(format_args!("- filesystem\n"));
                };
                {
                    ::std::io::_print(format_args!("- shell\n"));
                };
                {
                    ::std::io::_print(format_args!("- project\n"));
                };
                {
                    ::std::io::_print(format_args!("- subagent\n"));
                };
            }
            "doctor" => {
                let tools = registry.list_all();
                {
                    ::std::io::_print(format_args!("Tool Registry Doctor:\n"));
                };
                {
                    ::std::io::_print(format_args!("  Total tools: {0}\n", tools.len()));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  High/Critical risk tools: {0}\n",
                            tools
                                .iter()
                                .filter(|t| {
                                    t.risk_level == crate::approval::RiskLevel::High
                                        || t.risk_level == crate::approval::RiskLevel::Critical
                                })
                                .count(),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  Tool audit log path: {0}\n",
                            paths.tool_audit_log_file.display(),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  Permission configuration enabled: {0}\n",
                            config.tools.enabled,
                        ),
                    );
                };
            }
            "audit" => {
                if paths.tool_audit_log_file.exists() {
                    match std::fs::read_to_string(&paths.tool_audit_log_file) {
                        Ok(content) => {
                            ::std::io::_print(format_args!("{0}\n", content));
                        }
                        Err(e) => {
                            ::std::io::_print(
                                format_args!("Failed to read audit log: {0}\n", e),
                            );
                        }
                    }
                } else {
                    {
                        ::std::io::_print(
                            format_args!(
                                "No audit log found at {0}.\n",
                                paths.tool_audit_log_file.display(),
                            ),
                        );
                    };
                }
            }
            "catalog" => {
                {
                    ::std::io::_print(
                        format_args!("GOAT Tool Catalog (Phase 3.7 Foundation)\n"),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Status: Informational only. No automatic installation yet.\n",
                        ),
                    );
                };
                if paths.tool_catalog_file.exists() {
                    {
                        ::std::io::_print(
                            format_args!(
                                "Catalog loaded from: {0}\n",
                                paths.tool_catalog_file.display(),
                            ),
                        );
                    };
                } else {
                    {
                        ::std::io::_print(
                            format_args!(
                                "Catalog not found at {0}. Using default docs catalog.\n",
                                paths.tool_catalog_file.display(),
                            ),
                        );
                    };
                }
                if let Some(a) = arg {
                    let parts: Vec<&str> = a.splitn(2, ' ').collect();
                    if parts[0] == "search" {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Searching catalog for: {0}\n",
                                    parts.get(1).unwrap_or(&""),
                                ),
                            );
                        };
                    } else if parts[0] == "show" {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Showing catalog entry for: {0}\n",
                                    parts.get(1).unwrap_or(&""),
                                ),
                            );
                        };
                    } else {
                        {
                            ::std::io::_print(
                                format_args!("Unknown catalog action: {0}\n", parts[0]),
                            );
                        };
                    }
                } else {
                    {
                        ::std::io::_print(
                            format_args!("Available Planned Categories:\n"),
                        );
                    };
                    {
                        ::std::io::_print(
                            format_args!(
                                "- filesystem MCP, git tools, browser automation, web search,\n",
                            ),
                        );
                    };
                    {
                        ::std::io::_print(
                            format_args!(
                                "  Playwright/browser-use, image generation, TTS/STT,\n",
                            ),
                        );
                    };
                    {
                        ::std::io::_print(
                            format_args!(
                                "  database tools, GitHub tools, calendar/email tools, local shell\n",
                            ),
                        );
                    };
                }
            }
            "install" | "enable" | "disable" | "remove" => {
                {
                    ::std::io::_print(
                        format_args!("Tool/MCP {0} is planned for Phase 3.8.\n", action),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "No automatic installation yet. Future installs require approval and sandbox checks.\n",
                        ),
                    );
                };
                if let Some(a) = arg {
                    {
                        ::std::io::_print(format_args!("Target: {0}\n", a));
                    };
                }
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!(
                            "Unknown action \'{0}\'. Expected: list, show, categories, doctor, audit, catalog, install, enable, disable.\n",
                            action,
                        ),
                    );
                };
            }
        }
        Ok(())
    }
    fn handle_mcp_command(
        paths: &crate::paths::GoatPaths,
        config: &crate::config::Config,
        action: &String,
        arg: &Option<String>,
    ) -> anyhow::Result<()> {
        match action.as_str() {
            "status" => {
                {
                    ::std::io::_print(
                        format_args!("MCP Status (Phase 3.7 Foundation)\n"),
                    );
                };
                let mcp_conf_exists = paths.mcp_json_file.exists()
                    || paths.mcp_toml_file.exists();
                {
                    ::std::io::_print(
                        format_args!(
                            "MCP config paths: {0} / {1}\n",
                            paths.mcp_json_file.display(),
                            paths.mcp_toml_file.display(),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "MCP config exists: {0}\n",
                            if mcp_conf_exists { "yes" } else { "no" },
                        ),
                    );
                };
                let enabled_count = config
                    .mcp_servers
                    .values()
                    .filter(|s| s.enabled)
                    .count();
                let risky_count = config
                    .mcp_servers
                    .values()
                    .filter(|s| s.risk == "ask" || s.risk == "deny")
                    .count();
                {
                    ::std::io::_print(
                        format_args!(
                            "Configured servers: {0}\n",
                            config.mcp_servers.len(),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Enabled servers: {0}\n", enabled_count),
                    );
                };
                {
                    ::std::io::_print(format_args!("Risky servers: {0}\n", risky_count));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Execution status: allowed (requires ApprovalGate)\n",
                        ),
                    );
                };
            }
            "list" => {
                if config.mcp_servers.is_empty() {
                    {
                        ::std::io::_print(format_args!("No MCP servers configured.\n"));
                    };
                    return Ok(());
                }
                {
                    ::std::io::_print(format_args!("{0:-<80}\n", ""));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "{0:<15} | {1:<8} | {2:<10} | {3:<8} | {4}\n",
                            "Server Name",
                            "Enabled",
                            "Transport",
                            "Risk",
                            "Command",
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("{0:-<80}\n", ""));
                };
                for (name, srv) in &config.mcp_servers {
                    {
                        ::std::io::_print(
                            format_args!(
                                "{0:<15} | {1:<8} | {2:<10} | {3:<8} | {4}\n",
                                name,
                                srv.enabled,
                                srv.transport,
                                srv.risk,
                                srv.command,
                            ),
                        );
                    };
                }
                {
                    ::std::io::_print(format_args!("{0:-<80}\n", ""));
                };
            }
            "show" => {
                let Some(name) = arg else {
                    {
                        ::std::io::_print(format_args!("Usage: goat mcp show <name>\n"));
                    };
                    return Ok(());
                };
                if let Some(srv) = config.mcp_servers.get(name) {
                    {
                        ::std::io::_print(format_args!("MCP Server: {0}\n", name));
                    };
                    {
                        ::std::io::_print(format_args!("Enabled: {0}\n", srv.enabled));
                    };
                    {
                        ::std::io::_print(
                            format_args!("Transport: {0}\n", srv.transport),
                        );
                    };
                    {
                        ::std::io::_print(format_args!("Risk Policy: {0}\n", srv.risk));
                    };
                    {
                        ::std::io::_print(format_args!("Command: {0}\n", srv.command));
                    };
                    {
                        ::std::io::_print(format_args!("Args: {0:?}\n", srv.args));
                    };
                    {
                        ::std::io::_print(
                            format_args!(
                                "Env Vars Configured: {0:?}\n",
                                srv.env.keys().collect::<Vec<_>>(),
                            ),
                        );
                    };
                } else {
                    {
                        ::std::io::_print(
                            format_args!(
                                "MCP server \'{0}\' not found in config.\n",
                                name,
                            ),
                        );
                    };
                }
            }
            "start" | "stop" | "restart" => {
                let Some(name) = arg else {
                    {
                        ::std::io::_print(
                            format_args!("Usage: goat mcp {0} <name>\n", action),
                        );
                    };
                    return Ok(());
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Lifecycle action \'{0}\' for MCP server \'{1}\' is planned/partial.\n",
                            action,
                            name,
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Currently waiting for full MCP client lifecycle + ApprovalGate integration in Phase 3.8.\n",
                        ),
                    );
                };
            }
            "doctor" => {
                {
                    ::std::io::_print(format_args!("MCP Doctor (Phase 3.7)\n"));
                };
                let mcp_conf_exists = paths.mcp_json_file.exists()
                    || paths.mcp_toml_file.exists();
                {
                    ::std::io::_print(
                        format_args!(
                            "[*] Config paths checked: {0} / {1}\n",
                            paths.mcp_json_file.display(),
                            paths.mcp_toml_file.display(),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "[*] Config exists: {0}\n",
                            if mcp_conf_exists { "yes" } else { "no" },
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "[*] Configured servers: {0}\n",
                            config.mcp_servers.len(),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "[*] Tool catalog path: {0}\n",
                            paths.tool_catalog_file.display(),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "[*] Tool catalog exists: {0}\n",
                            if paths.tool_catalog_file.exists() { "yes" } else { "no" },
                        ),
                    );
                };
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!(
                            "Unknown action \'{0}\'. Expected: status, list, show, start, stop, restart, doctor.\n",
                            action,
                        ),
                    );
                };
            }
        }
        Ok(())
    }
    fn handle_migrate_db(paths: &crate::paths::GoatPaths) -> anyhow::Result<()> {
        use anyhow::Context;
        let legacy = crate::paths::GoatPaths::detect_legacy_db();
        let Some(legacy_path) = legacy else {
            {
                ::std::io::_print(
                    format_args!(
                        "No legacy database found at ./goat_brain.db — nothing to migrate.\n",
                    ),
                );
            };
            return Ok(());
        };
        if paths.db_file.exists() {
            {
                ::std::io::_print(
                    format_args!(
                        "Target database already exists: {0}\n",
                        paths.db_file.display(),
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!(
                        "To replace it, manually delete it first and re-run migrate-db.\n",
                    ),
                );
            };
            return Ok(());
        }
        paths
            .ensure_data_dir()
            .with_context(|| {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "could not create data directory: {0}",
                            paths.data_dir.display(),
                        ),
                    )
                })
            })?;
        std::fs::copy(&legacy_path, &paths.db_file)
            .with_context(|| {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "failed to copy {0} to {1}",
                            legacy_path.display(),
                            paths.db_file.display(),
                        ),
                    )
                })
            })?;
        {
            ::std::io::_print(
                format_args!(
                    "Migration successful: {0} → {1}\n",
                    legacy_path.display(),
                    paths.db_file.display(),
                ),
            );
        };
        {
            ::std::io::_print(
                format_args!(
                    "The original file at {0} was NOT deleted. Remove it manually when ready.\n",
                    legacy_path.display(),
                ),
            );
        };
        Ok(())
    }
    fn handle_models_command(
        config: &crate::config::Config,
        action: &str,
        args: &[String],
    ) -> anyhow::Result<()> {
        use crate::providers::{
            ModelProviderCapability, ModelProviderRegistry, ModelRouteRequest,
        };
        let mut registry = ModelProviderRegistry::new(config.model_routing.clone());
        for (_, p_cfg) in &config.providers {
            registry.register(p_cfg.clone());
        }
        match action {
            "list" => {
                {
                    ::std::io::_print(format_args!("GOAT Available Models\n"));
                };
                {
                    ::std::io::_print(format_args!("{0}\n", "─".repeat(72)));
                };
                for provider in registry.providers.values() {
                    if !provider.enabled {
                        continue;
                    }
                    {
                        ::std::io::_print(
                            format_args!(
                                "Provider: {0} ({1})\n",
                                provider.name,
                                provider.id,
                            ),
                        );
                    };
                    {
                        ::std::io::_print(
                            format_args!(
                                "  Default Model: {0}\n",
                                provider.default_model,
                            ),
                        );
                    };
                    if !provider.available_models.is_empty() {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "  Available Models: {0}\n",
                                    provider.available_models.join(", "),
                                ),
                            );
                        };
                    }
                    {
                        ::std::io::_print(format_args!("\n"));
                    };
                }
            }
            "route" => {
                let task_kind = args.get(0).map(|s| s.as_str()).unwrap_or("general");
                let req = ModelRouteRequest {
                    agent_id: "cli_user".to_string(),
                    task_kind: task_kind.to_string(),
                    required_capabilities: ::alloc::vec::Vec::new(),
                    local_only: false,
                    allow_external: true,
                    preferred_provider: None,
                    preferred_model: None,
                    quality_preference: "balanced".to_string(),
                    latency_preference: "balanced".to_string(),
                    cost_preference: "balanced".to_string(),
                    fallback_allowed: true,
                };
                let decision = registry.route(&req);
                {
                    ::std::io::_print(
                        format_args!("Routing decision for task \'{0}\':\n", task_kind),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("  Provider: {0}\n", decision.provider_id),
                    );
                };
                {
                    ::std::io::_print(format_args!("  Model: {0}\n", decision.model));
                };
                {
                    ::std::io::_print(
                        format_args!("  Local Only: {0}\n", decision.local_only),
                    );
                };
                {
                    ::std::io::_print(format_args!("  Notes: {0}\n", decision.notes));
                };
            }
            _ => {
                {
                    ::std::io::_print(format_args!("Unknown action: {0}\n", action));
                };
                {
                    ::std::io::_print(format_args!("Usage: goat models <list|route>\n"));
                };
            }
        }
        Ok(())
    }
    fn handle_providers_command(
        config: &crate::config::Config,
        action: &str,
    ) -> anyhow::Result<()> {
        use crate::providers::{
            ModelProviderAdapter, ModelProviderRegistry, ModelProviderStatus,
            OpenAiCompatibleAdapter,
        };
        let mut registry = ModelProviderRegistry::new(config.model_routing.clone());
        for (_, p_cfg) in &config.providers {
            registry.register(p_cfg.clone());
        }
        match action {
            "list" => {
                {
                    ::std::io::_print(format_args!("GOAT Model Providers\n"));
                };
                {
                    ::std::io::_print(format_args!("{0}\n", "─".repeat(72)));
                };
                for provider in registry.providers.values() {
                    let status_icon = if provider.enabled { "✓" } else { "✗" };
                    {
                        ::std::io::_print(
                            format_args!(
                                "  {0} {1:15} ({2})\n",
                                status_icon,
                                provider.name,
                                provider.id,
                            ),
                        );
                    };
                }
            }
            "doctor" => {
                {
                    ::std::io::_print(format_args!("GOAT Provider Doctor\n"));
                };
                {
                    ::std::io::_print(format_args!("{0}\n", "─".repeat(72)));
                };
                for provider in registry.providers.values() {
                    if !provider.enabled {
                        continue;
                    }
                    let adapter = OpenAiCompatibleAdapter::new(
                        provider.base_url.clone().unwrap_or_default(),
                        config.provider_api_key(&provider.id),
                        provider.timeout_secs,
                    );
                    let status = adapter.status();
                    let status_str = match status {
                        ModelProviderStatus::Ready => "Ready",
                        ModelProviderStatus::NotConfigured => "Not Configured",
                        ModelProviderStatus::MissingKey => "Missing API Key",
                        ModelProviderStatus::Unreachable => "Unreachable",
                        ModelProviderStatus::Unknown => "Unknown",
                    };
                    let status_icon = if status == ModelProviderStatus::Ready {
                        "✓"
                    } else {
                        "!"
                    };
                    {
                        ::std::io::_print(
                            format_args!(
                                "  {0} {1:15} {2}\n",
                                status_icon,
                                provider.name,
                                status_str,
                            ),
                        );
                    };
                }
            }
            _ => {
                {
                    ::std::io::_print(format_args!("Unknown action: {0}\n", action));
                };
                {
                    ::std::io::_print(
                        format_args!("Usage: goat providers <list|doctor>\n"),
                    );
                };
            }
        }
        Ok(())
    }
    fn handle_project_command(
        paths: &crate::paths::GoatPaths,
        _config: &crate::config::Config,
        action: &str,
    ) -> anyhow::Result<()> {
        use crate::brain::Brain;
        use crate::project::ProjectScanner;
        use std::env;
        let root = env::current_dir().unwrap_or_default();
        let brain = Brain::new(&paths.db_file)?;
        if action == "scan" {
            {
                ::std::io::_print(
                    format_args!("Scanning project at {0}...\n", root.display()),
                );
            };
            let scanner = ProjectScanner::new(root.clone());
            let meta = scanner.scan()?;
            brain.save_project(root.to_string_lossy().as_ref(), &meta)?;
            {
                ::std::io::_print(format_args!("Scan complete.\n"));
            };
            print_project_summary(&meta);
        } else {
            if let Ok(Some(meta)) = brain.get_project(root.to_string_lossy().as_ref()) {
                {
                    ::std::io::_print(
                        format_args!("Project context found for {0}\n", root.display()),
                    );
                };
                print_project_summary(&meta);
            } else {
                {
                    ::std::io::_print(
                        format_args!(
                            "No project context found for {0}\n",
                            root.display(),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Run `goat project scan` to index this directory.\n",
                        ),
                    );
                };
            }
        }
        Ok(())
    }
    fn print_project_summary(meta: &crate::project::ProjectMetadata) {
        {
            ::std::io::_print(format_args!("{0}\n", "─".repeat(60)));
        };
        {
            ::std::io::_print(format_args!("Root: {0}\n", meta.root_path.display()));
        };
        {
            ::std::io::_print(
                format_args!(
                    "Git Repo: {0}\n",
                    if meta.is_git_repo { "Yes" } else { "No" },
                ),
            );
        };
        if !meta.stack.is_empty() {
            {
                ::std::io::_print(format_args!("Stack: {0}\n", meta.stack.join(", ")));
            };
        }
        if !meta.package_files.is_empty() {
            {
                ::std::io::_print(
                    format_args!("Packages: {0}\n", meta.package_files.join(", ")),
                );
            };
        }
        if !meta.detected_commands.is_empty() {
            {
                ::std::io::_print(
                    format_args!("Commands: {0}\n", meta.detected_commands.join(", ")),
                );
            };
        }
        {
            ::std::io::_print(
                format_args!("Ignored directories: {0}\n", meta.ignored_dirs_count),
            );
        };
        {
            ::std::io::_print(format_args!("{0}\n", "─".repeat(60)));
        };
    }
    fn handle_memory_command(
        paths: &crate::paths::GoatPaths,
        config: &crate::config::Config,
        action: &str,
        text: Option<&str>,
    ) -> anyhow::Result<()> {
        use crate::memory::MemoryManager;
        let manager = MemoryManager::new(paths, config.memory.clone());
        match action {
            "status" => {
                let (u_count, u_max, u_warn) = manager.user_budget_status();
                let (m_count, m_max, m_warn) = manager.memory_budget_status();
                {
                    ::std::io::_print(format_args!("[MEMORY] Status:\n"));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  USER.md   : {0}/{1} chars {2}\n",
                            u_count,
                            u_max,
                            if u_warn { "(OVER BUDGET)" } else { "" },
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  MEMORY.md : {0}/{1} chars {2}\n",
                            m_count,
                            m_max,
                            if m_warn { "(OVER BUDGET)" } else { "" },
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("  Enabled   : {0}\n", config.memory.enabled),
                    );
                };
            }
            "show" => {
                {
                    ::std::io::_print(format_args!("--- USER.md ---\n"));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "{0}\n",
                            manager.get_user_content().unwrap_or_default(),
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("--- MEMORY.md ---\n"));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "{0}\n",
                            manager.get_memory_content().unwrap_or_default(),
                        ),
                    );
                };
            }
            "path" => {
                {
                    ::std::io::_print(
                        format_args!("USER.md:   {0}\n", manager.user_file.display()),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("MEMORY.md: {0}\n", manager.memory_file.display()),
                    );
                };
            }
            "edit" => {
                {
                    ::std::io::_print(
                        format_args!(
                            "To edit memory files, open these in your editor:\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("  {0}\n", manager.user_file.display()),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("  {0}\n", manager.memory_file.display()),
                    );
                };
            }
            "add-user" => {
                if let Some(t) = text {
                    manager.add_user(t)?;
                    {
                        ::std::io::_print(format_args!("Added to USER.md\n"));
                    };
                } else {
                    {
                        ::std::io::_print(format_args!("Please provide text to add.\n"));
                    };
                }
            }
            "add-note" => {
                if let Some(t) = text {
                    manager.add_note(t)?;
                    {
                        ::std::io::_print(format_args!("Added to MEMORY.md\n"));
                    };
                } else {
                    {
                        ::std::io::_print(format_args!("Please provide text to add.\n"));
                    };
                }
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!("Unknown memory action: {0}\n", action),
                    );
                };
            }
        }
        Ok(())
    }
    fn handle_recall_command(
        paths: &crate::paths::GoatPaths,
        query: &str,
    ) -> anyhow::Result<()> {
        use crate::brain::Brain;
        let brain = Brain::new(&paths.db_file)?;
        let results = brain.recall_search(query)?;
        if results.is_empty() {
            {
                ::std::io::_print(
                    format_args!("No recall results found for: {0}\n", query),
                );
            };
        } else {
            {
                ::std::io::_print(
                    format_args!(
                        "Found {0} result(s) for \'{1}\':\n",
                        results.len(),
                        query,
                    ),
                );
            };
            for (idx, (session_id, role, content)) in results.iter().enumerate() {
                let snippet = if content.len() > 80 {
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0}...", &content[..77].replace('\n', " ")),
                        )
                    })
                } else {
                    content.replace('\n', " ")
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  {0}. [{1}] {2}: {3}\n",
                            idx + 1,
                            &session_id[..8],
                            role,
                            snippet,
                        ),
                    );
                };
            }
        }
        Ok(())
    }
    async fn handle_skills_command(
        paths: &crate::paths::GoatPaths,
        config: &crate::config::Config,
        action: &str,
        arg: Option<&str>,
        session_id: Option<&str>,
    ) -> anyhow::Result<()> {
        let skill_manager = crate::skills::SkillManager::new(
            paths.clone(),
            config.skills.clone(),
        );
        match action {
            "path" => {
                {
                    ::std::io::_print(
                        format_args!("{0}\n", skill_manager.skills_dir().display()),
                    );
                };
            }
            "list" => {
                let skills = skill_manager.list_skills();
                if skills.is_empty() {
                    {
                        ::std::io::_print(
                            format_args!(
                                "No skills found in {0}\n",
                                skill_manager.skills_dir().display(),
                            ),
                        );
                    };
                    return Ok(());
                }
                {
                    ::std::io::_print(format_args!("Skills ({0}):\n", skills.len()));
                };
                for s in skills {
                    let name = if s.is_suspicious {
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0} [SUSPICIOUS]", s.name),
                            )
                        })
                    } else {
                        s.name
                    };
                    {
                        ::std::io::_print(
                            format_args!("  - {1:<20} {0}\n", s.description, name),
                        );
                    };
                }
            }
            "show" => {
                let name = arg
                    .ok_or_else(|| ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            format_args!("Expected skill name"),
                        );
                        error
                    }))?;
                if let Some(skill) = skill_manager.get_skill(name) {
                    if skill.is_suspicious {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "WARNING: This skill contains suspicious patterns:\n",
                                ),
                            );
                        };
                        for w in &skill.warnings {
                            {
                                ::std::io::_print(format_args!("  - {0}\n", w));
                            };
                        }
                        {
                            ::std::io::_print(format_args!("\n"));
                        };
                    }
                    {
                        ::std::io::_print(format_args!("{0}\n", skill.content));
                    };
                } else {
                    {
                        ::std::io::_print(
                            format_args!("Skill \'{0}\' not found.\n", name),
                        );
                    };
                }
            }
            "create" => {
                let name = arg
                    .ok_or_else(|| ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            format_args!("Expected skill name"),
                        );
                        error
                    }))?;
                let path = skill_manager.create_template(name)?;
                {
                    ::std::io::_print(
                        format_args!("Created skill template at: {0}\n", path.display()),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Edit this file to implement your skill.\n"),
                    );
                };
            }
            "validate" => {
                let skills = skill_manager.list_skills();
                let mut invalid = 0;
                for s in skills {
                    if s.is_suspicious {
                        invalid += 1;
                        {
                            ::std::io::_print(
                                format_args!(
                                    "WARNING: Skill \'{0}\' is suspicious:\n",
                                    s.name,
                                ),
                            );
                        };
                        for w in s.warnings {
                            {
                                ::std::io::_print(format_args!("  - {0}\n", w));
                            };
                        }
                    }
                }
                if invalid == 0 {
                    {
                        ::std::io::_print(
                            format_args!("All skills passed validation.\n"),
                        );
                    };
                } else {
                    {
                        ::std::io::_print(
                            format_args!("{0} skills failed validation.\n", invalid),
                        );
                    };
                }
            }
            "search" => {
                let query = arg
                    .ok_or_else(|| ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            format_args!("Expected search query"),
                        );
                        error
                    }))?;
                let results = skill_manager.search_skills(query);
                if results.is_empty() {
                    {
                        ::std::io::_print(
                            format_args!("No skills found matching \'{0}\'\n", query),
                        );
                    };
                    return Ok(());
                }
                {
                    ::std::io::_print(
                        format_args!("Found {0} matching skills:\n", results.len()),
                    );
                };
                for s in results {
                    {
                        ::std::io::_print(
                            format_args!("  - {0:<20} {1}\n", s.name, s.description),
                        );
                    };
                }
            }
            "create-from-session" => {
                let name = arg
                    .ok_or_else(|| ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            format_args!("Expected skill name"),
                        );
                        error
                    }))?;
                let sid = session_id
                    .ok_or_else(|| {
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!(
                                    "Expected --session <id> for create-from-session",
                                ),
                            );
                            error
                        })
                    })?;
                let brain = crate::brain::Brain::new(&paths.db_file)
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("Could not open brain db: {0}", e),
                            )
                        }),
                    ))?;
                let history = brain.load_session_history(sid)?;
                if history.is_empty() {
                    return ::anyhow::__private::Err(
                        ::anyhow::Error::msg(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("No history found for session {0}", sid),
                                )
                            }),
                        ),
                    );
                }
                let mut history_text = String::new();
                for msg in history {
                    if msg.0 != "system" {
                        history_text
                            .push_str(
                                &::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("{0}: {1}\n", msg.0, msg.1),
                                    )
                                }),
                            );
                    }
                }
                {
                    ::std::io::_print(
                        format_args!(
                            "Extracting skill \'{0}\' from session {1}...\n",
                            name,
                            sid,
                        ),
                    );
                };
                let mut registry = crate::models::ProfileRegistry::from_config(
                    &config.profiles,
                );
                let mut router = crate::llm::LlmRouter::from_config(config);
                let profile_name = &registry.default_profile;
                let (_, chain) = registry.resolve(profile_name);
                let prompt = ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!(
                            "You are a skill curator. The user wants to extract a reusable skill from the following session history.\nGenerate a valid SKILL.md file with the following headers: Name, Description, Triggers, Tools Needed, Procedure, Safety Notes, Verification.\nThe skill name should be: {0}\n\nRules:\n- NEVER include real API keys, passwords, or secrets.\n- Focus on the generalized workflow, not the exact files edited.\n- Output only the Markdown content.\n\nSession History:\n{1}",
                            name,
                            history_text,
                        ),
                    )
                });
                let messages = ::alloc::boxed::box_assume_init_into_vec_unsafe(
                    ::alloc::intrinsics::write_box_via_move(
                        ::alloc::boxed::Box::new_uninit(),
                        [
                            crate::llm::Message {
                                role: "user".to_string(),
                                content: Some(prompt),
                                tool_calls: None,
                                tool_call_id: None,
                            },
                        ],
                    ),
                );
                match router.completion_with_fallback(&chain, messages, None).await {
                    Ok((resp, _)) => {
                        let content = resp.content.unwrap_or_default();
                        let skill_dir = skill_manager.skills_dir().join(name);
                        std::fs::create_dir_all(&skill_dir)?;
                        let skill_file = skill_dir.join("SKILL.md");
                        std::fs::write(&skill_file, content)?;
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Extracted and saved skill \'{0}\' to {1}\n",
                                    name,
                                    skill_file.display(),
                                ),
                            );
                        };
                    }
                    Err(e) => {
                        return ::anyhow::__private::Err(
                            ::anyhow::Error::msg(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("Failed to extract skill from LLM: {0}", e),
                                    )
                                }),
                            ),
                        );
                    }
                }
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!(
                            "Unknown action \'{0}\'. Expected: list, show, path, create, validate, search, create-from-session.\n",
                            action,
                        ),
                    );
                };
            }
        }
        Ok(())
    }
    fn handle_repo_map_command(
        paths: &crate::paths::GoatPaths,
        config: &crate::config::Config,
        action: &str,
    ) -> anyhow::Result<()> {
        use crate::repo_map::{ProjectCommands, RepoMapScanner};
        use std::env;
        let root = env::current_dir().unwrap_or_default();
        let max_chars = config.repo_map.max_chars;
        let include_symbols = config.repo_map.include_symbols;
        match action {
            "show" | "refresh" => {
                if action == "refresh" {
                    {
                        ::std::io::_print(
                            format_args!(
                                "Refreshing repo map for {0}...\n",
                                root.display(),
                            ),
                        );
                    };
                } else {
                    {
                        ::std::io::_print(
                            format_args!("Repo map for {0}:\n", root.display()),
                        );
                    };
                }
                let scanner = if include_symbols {
                    RepoMapScanner::new(root.clone())
                } else {
                    RepoMapScanner::new(root.clone()).with_no_symbols()
                };
                let map = scanner
                    .scan()
                    .map_err(|e| ::anyhow::Error::msg(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("Scan failed: {0}", e))
                        }),
                    ))?;
                {
                    ::std::io::_print(format_args!("{0}\n", "─".repeat(60)));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "{0}\n",
                            map.to_compact_string(max_chars, include_symbols),
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("{0}\n", "─".repeat(60)));
                };
                let cmds = ProjectCommands::detect(&root);
                {
                    ::std::io::_print(format_args!("Detected commands:\n"));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  check  : {0}\n",
                            cmds.check.as_deref().unwrap_or("not detected"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  test   : {0}\n",
                            cmds.test.as_deref().unwrap_or("not detected"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  lint   : {0}\n",
                            cmds.lint.as_deref().unwrap_or("not detected"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  format : {0}\n",
                            cmds.format.as_deref().unwrap_or("not detected"),
                        ),
                    );
                };
                if paths.db_file.exists() {
                    if let Ok(brain) = crate::brain::Brain::new(&paths.db_file) {
                        let meta = crate::project::ProjectScanner::new(root.clone())
                            .scan()
                            .ok();
                        if let Some(meta) = meta {
                            let _ = brain
                                .save_project(root.to_string_lossy().as_ref(), &meta);
                        }
                    }
                }
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!(
                            "Unknown action \'{0}\'. Expected: show, refresh.\n",
                            action,
                        ),
                    );
                };
            }
        }
        Ok(())
    }
    fn handle_dev_command(kind: &str) -> anyhow::Result<()> {
        handle_dev_command_with_args(kind, None)
    }
    fn handle_dev_command_with_args(
        kind: &str,
        extra: Option<&str>,
    ) -> anyhow::Result<()> {
        use std::io::{self, BufRead, Write};
        let root = std::env::current_dir().unwrap_or_default();
        let cmds = crate::repo_map::ProjectCommands::detect(&root);
        let base_cmd = match kind {
            "check" => cmds.check,
            "test" => cmds.test,
            "lint" => cmds.lint,
            "format" => cmds.format,
            _ => None,
        };
        let cmd = match base_cmd {
            Some(c) => {
                if let Some(extra_args) = extra {
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0} {1}", c, extra_args))
                    })
                } else {
                    c
                }
            }
            None => {
                {
                    ::std::io::_print(
                        format_args!(
                            "[DEV] No {0} command detected for this project.\n",
                            kind,
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("[DEV] GOAT scanned: {0}\n", root.display()),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "[DEV] Supported: Rust (Cargo.toml), Node (package.json), Python (pyproject.toml), Go (go.mod).\n",
                        ),
                    );
                };
                return Ok(());
            }
        };
        {
            ::std::io::_print(
                format_args!("[DEV] Detected {0} command: {1}\n", kind, cmd),
            );
        };
        {
            ::std::io::_print(
                format_args!(
                    "[DEV] ⚠ This command will run in your terminal. Confirm to proceed.\n",
                ),
            );
        };
        {
            ::std::io::_print(
                format_args!(
                    "[DEV] (In TUI/headless mode, the ApprovalGate prompt will appear instead.)\n",
                ),
            );
        };
        {
            ::std::io::_print(format_args!("[DEV] Execute \'{0}\' now? [y/N]: ", cmd));
        };
        io::stdout().flush().ok();
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).ok();
        let answer = line.trim().to_lowercase();
        if answer == "y" || answer == "yes" {
            {
                ::std::io::_print(format_args!("[DEV] Running: {0}\n", cmd));
            };
            let status = std::process::Command::new("bash").arg("-c").arg(&cmd).status();
            match status {
                Ok(s) if s.success() => {
                    ::std::io::_print(
                        format_args!("[DEV] ✓ {0} completed successfully.\n", kind),
                    );
                }
                Ok(s) => {
                    ::std::io::_print(
                        format_args!(
                            "[DEV] ✗ {0} exited with code: {1:?}\n",
                            kind,
                            s.code(),
                        ),
                    );
                }
                Err(e) => {
                    ::std::io::_print(
                        format_args!("[DEV] Error running command: {0}\n", e),
                    );
                }
            }
        } else {
            {
                ::std::io::_print(format_args!("[DEV] Cancelled.\n"));
            };
        }
        Ok(())
    }
    fn handle_patch_command(action: &str) {
        match action {
            "show" => {
                {
                    ::std::io::_print(
                        format_args!("[PATCH] No pending patch in current session.\n"),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "[PATCH] Patches are created when GOAT proposes a file write during an agent session.\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "[PATCH] Use /patch in TUI or headless mode to inspect pending diffs.\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "[PATCH] Full patch queue is planned for Phase 2.4.\n",
                        ),
                    );
                };
            }
            "apply" => {
                {
                    ::std::io::_print(
                        format_args!("[PATCH] No pending patch to apply.\n"),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "[PATCH] Start a session and let the agent propose a file write.\n",
                        ),
                    );
                };
            }
            "discard" => {
                {
                    ::std::io::_print(
                        format_args!("[PATCH] No pending patch to discard.\n"),
                    );
                };
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!(
                            "Unknown patch action \'{0}\'. Expected: show, apply, discard.\n",
                            action,
                        ),
                    );
                };
            }
        }
    }
    fn handle_subagents_command(
        paths: &crate::paths::GoatPaths,
        _config: &crate::config::Config,
        action: &str,
        arg: Option<&str>,
    ) -> anyhow::Result<()> {
        let registry = crate::subagents::SubagentRegistry::new();
        match action {
            "list" | "" => {
                let list = registry.list_all();
                {
                    ::std::io::_print(
                        format_args!(
                            "GOAT Subagent Registry ({0} subagents)\n",
                            list.len(),
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("{0:-<80}\n", ""));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "{0:<15} | {1:<15} | {2:<15} | {3}\n",
                            "Name",
                            "Kind",
                            "Risk Level",
                            "Profile",
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("{0:-<80}\n", ""));
                };
                for agent in list {
                    {
                        ::std::io::_print(
                            format_args!(
                                "{0:<15} | {1:<15} | {2:<15} | {3}\n",
                                agent.name,
                                agent.kind.to_string(),
                                agent.risk_level.to_string(),
                                agent.default_model_profile,
                            ),
                        );
                    };
                }
            }
            "show" => {
                if let Some(name) = arg {
                    if let Some(agent) = registry.get(name) {
                        {
                            ::std::io::_print(
                                format_args!("Subagent: {0}\n", agent.name),
                            );
                        };
                        {
                            ::std::io::_print(format_args!("Kind: {0}\n", agent.kind));
                        };
                        {
                            ::std::io::_print(
                                format_args!("Purpose: {0}\n", agent.purpose),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Risk Level: {0}\n", agent.risk_level),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Model Profile: {0}\n",
                                    agent.default_model_profile,
                                ),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Allowed Tools: {0:?}\n", agent.allowed_tools),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Context Budget: {0}\n", agent.context_budget),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Requires Approval: {0}\n",
                                    agent.requires_approval,
                                ),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Can Propose Patches: {0}\n",
                                    agent.can_propose_patches,
                                ),
                            );
                        };
                    } else {
                        {
                            ::std::io::_print(
                                format_args!("Subagent \'{0}\' not found.\n", name),
                            );
                        };
                    }
                } else {
                    {
                        ::std::io::_print(
                            format_args!("Please specify a subagent name to show.\n"),
                        );
                    };
                }
            }
            "audit" => {
                if paths.subagent_audit_log_file.exists() {
                    if let Ok(content) = std::fs::read_to_string(
                        &paths.subagent_audit_log_file,
                    ) {
                        {
                            ::std::io::_print(format_args!("{0}\n", content));
                        };
                    } else {
                        {
                            ::std::io::_print(
                                format_args!("Error reading subagent audit log.\n"),
                            );
                        };
                    }
                } else {
                    {
                        ::std::io::_print(
                            format_args!("No subagent audit log found.\n"),
                        );
                    };
                }
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!(
                            "Unknown action: {0}. Available: list, show, audit.\n",
                            action,
                        ),
                    );
                };
            }
        }
        Ok(())
    }
    async fn handle_ask_agent_command(
        name: &str,
        task: &str,
        rt: &crate::runtime::GoatRuntime,
    ) -> anyhow::Result<()> {
        {
            ::std::io::_print(format_args!("Asking subagent \'{0}\'...\n", name));
        };
        let manager = &rt.subagent_manager;
        let summary = "CLI context summary... (limited repo map)";
        let result = manager
            .ask_agent(name, task, summary, None, None, &rt.llm_router, &rt.model_chain)
            .await?;
        {
            ::std::io::_print(format_args!("\nResponse:\n{0}\n\n", result));
        };
        Ok(())
    }
    fn handle_external_agents_command(
        mut rt: crate::runtime::GoatRuntime,
        action: &str,
        arg: Option<&str>,
    ) {
        let mut ext_mgr = rt.external_agent_manager;
        ext_mgr.detect_all(&rt.config);
        match action {
            "list" => {
                let agents = ext_mgr.registry.list_all();
                {
                    ::std::io::_print(
                        format_args!(
                            "GOAT External Agent Registry ({0} adapters)\n",
                            agents.len(),
                        ),
                    );
                };
                for agent in agents {
                    {
                        ::std::io::_print(
                            format_args!(
                                "  {0:<15} [{1}] - {2}\n",
                                agent.name,
                                agent.command_name,
                                agent.status,
                            ),
                        );
                    };
                }
            }
            "detect" => {
                {
                    ::std::io::_print(format_args!("Detecting external agents...\n"));
                };
                for agent in ext_mgr.registry.list_all() {
                    {
                        ::std::io::_print(
                            format_args!("  {0:<15} - {1}\n", agent.name, agent.status),
                        );
                    };
                }
            }
            "show" => {
                let name = arg.unwrap_or("");
                if let Some(agent) = ext_mgr.registry.get(name) {
                    {
                        ::std::io::_print(format_args!("Name: {0}\n", agent.name));
                    };
                    {
                        ::std::io::_print(
                            format_args!("Command: {0}\n", agent.command_name),
                        );
                    };
                    {
                        ::std::io::_print(format_args!("Status: {0}\n", agent.status));
                    };
                    {
                        ::std::io::_print(format_args!("Risk: {0}\n", agent.risk_level));
                    };
                    {
                        ::std::io::_print(
                            format_args!(
                                "Workspace Behavior: {0}\n",
                                agent.workspace_behavior,
                            ),
                        );
                    };
                    if let Some(ref path) = agent.detected_path {
                        {
                            ::std::io::_print(
                                format_args!("Detected Path: {0}\n", path.display()),
                            );
                        };
                    }
                } else {
                    {
                        ::std::io::_print(
                            format_args!("External agent \'{0}\' not found.\n", name),
                        );
                    };
                }
            }
            "audit" => {
                if rt.paths.external_agent_audit_log_file.exists() {
                    if let Ok(content) = std::fs::read_to_string(
                        &rt.paths.external_agent_audit_log_file,
                    ) {
                        {
                            ::std::io::_print(format_args!("{0}\n", content));
                        };
                    }
                } else {
                    {
                        ::std::io::_print(
                            format_args!("No external agent audit log found.\n"),
                        );
                    };
                }
            }
            "doctor" => {
                let checks = crate::paths::run_doctor(&rt.paths, &rt.config, false);
                crate::paths::print_doctor_results(&checks);
            }
            "runs" => {
                let jsonl_path = rt.paths.data_dir.join("external-agent-runs.jsonl");
                if jsonl_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                        {
                            ::std::io::_print(format_args!("External Agent Runs:\n"));
                        };
                        for line in content.lines() {
                            if let Ok(run) = serde_json::from_str::<
                                crate::external_agents::ExternalAgentRun,
                            >(line) {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "  {0} | Agent: {1:<12} | Mode: {2:<15} | Status: {3}\n",
                                            run.id,
                                            run.agent_name,
                                            run.mode,
                                            if run.success { "Success" } else { "Failed" },
                                        ),
                                    );
                                };
                            }
                        }
                    }
                } else {
                    {
                        ::std::io::_print(format_args!("No runs recorded yet.\n"));
                    };
                }
            }
            "run" => {
                if let Some(run_id) = arg {
                    let jsonl_path = rt.paths.data_dir.join("external-agent-runs.jsonl");
                    let mut found = false;
                    if jsonl_path.exists() {
                        if let Ok(content) = std::fs::read_to_string(&jsonl_path) {
                            for line in content.lines() {
                                if let Ok(run) = serde_json::from_str::<
                                    crate::external_agents::ExternalAgentRun,
                                >(line) {
                                    if run.id == run_id {
                                        {
                                            ::std::io::_print(format_args!("Run ID: {0}\n", run.id));
                                        };
                                        {
                                            ::std::io::_print(
                                                format_args!("Agent: {0}\n", run.agent_name),
                                            );
                                        };
                                        {
                                            ::std::io::_print(
                                                format_args!("Timestamp: {0}\n", run.timestamp),
                                            );
                                        };
                                        {
                                            ::std::io::_print(format_args!("Mode: {0}\n", run.mode));
                                        };
                                        {
                                            ::std::io::_print(
                                                format_args!(
                                                    "Workspace: {0}\n",
                                                    run.workspace_path.display(),
                                                ),
                                            );
                                        };
                                        {
                                            ::std::io::_print(format_args!("Task: {0}\n", run.task));
                                        };
                                        {
                                            ::std::io::_print(
                                                format_args!("Success: {0}\n", run.success),
                                            );
                                        };
                                        {
                                            ::std::io::_print(
                                                format_args!("Duration: {0:?}\n", run.duration),
                                            );
                                        };
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if !found {
                        {
                            ::std::io::_print(
                                format_args!("Run ID \'{0}\' not found.\n", run_id),
                            );
                        };
                    }
                } else {
                    {
                        ::std::io::_print(
                            format_args!("Usage: goat external-agents run <id>\n"),
                        );
                    };
                }
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!("Unknown external-agents action: {0}\n", action),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Valid actions: list, detect, show <name>, audit, doctor, runs, run <id>\n",
                        ),
                    );
                };
            }
        }
    }
    async fn handle_delegate_external_command(
        mut rt: crate::runtime::GoatRuntime,
        name: &str,
        task: &str,
    ) {
        {
            ::std::io::_print(
                format_args!("Delegating task to external agent \'{0}\'...\n", name),
            );
        };
        rt.external_agent_manager.detect_all(&rt.config);
        let action = rt
            .tool_registry
            .evaluate_action("delegate_external_agent", &rt.config.tools);
        if let crate::tool_registry::ToolAction::Deny(reason) = action {
            {
                ::std::io::_print(
                    format_args!("Delegation denied by tool registry: {0}\n", reason),
                );
            };
            return;
        }
        let req = crate::approval::ApprovalRequest {
            tool_name: "delegate_external_agent".to_string(),
            action_summary: ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("agent: {0}, task: {1}", name, task))
            }),
            risk_level: crate::approval::RiskLevel::High,
            explanation: None,
            working_directory: None,
        };
        if let Some(crate::approval::ApprovalDecision::Denied(msg)) = rt
            .approval_gate
            .check_policy(&req)
        {
            match rt.external_agent_manager.delegate(name, task, &rt.config) {
                Ok(res) => {
                    {
                        ::std::io::_print(
                            format_args!(
                                "Execution finished. Success: {0}\n",
                                res.success,
                            ),
                        );
                    };
                    {
                        ::std::io::_print(format_args!("Stdout:\n{0}\n", res.stdout));
                    };
                    if !res.stderr.is_empty() {
                        {
                            ::std::io::_print(
                                format_args!("Stderr:\n{0}\n", res.stderr),
                            );
                        };
                    }
                }
                Err(e) => {
                    ::std::io::_print(format_args!("Error: {0}\n", e));
                }
            }
        }
        match rt.external_agent_manager.delegate(name, task, &rt.config) {
            Ok(res) => {
                {
                    ::std::io::_print(
                        format_args!("Execution finished. Success: {0}\n", res.success),
                    );
                };
                {
                    ::std::io::_print(format_args!("Stdout:\n{0}\n", res.stdout));
                };
                if !res.stderr.is_empty() {
                    {
                        ::std::io::_print(format_args!("Stderr:\n{0}\n", res.stderr));
                    };
                }
            }
            Err(e) => {
                ::std::io::_print(format_args!("Error: {0}\n", e));
            }
        }
    }
    fn handle_hooks_command(
        paths: &crate::paths::GoatPaths,
        config: &crate::config::Config,
        action: &str,
        arg: Option<&str>,
    ) -> anyhow::Result<()> {
        let mut hm = crate::hooks::HooksManager::new(
            config.hooks.clone(),
            paths.clone(),
        );
        match action {
            "list" => {
                let info = hm.list_hooks_info();
                {
                    ::std::io::_print(format_args!("[HOOKS] Registered Hooks:\n"));
                };
                for i in info {
                    {
                        ::std::io::_print(format_args!("  - {0}\n", i));
                    };
                }
            }
            "show" => {
                if let Some(name) = arg {
                    {
                        ::std::io::_print(
                            format_args!(
                                "[HOOKS] Show hook not fully implemented in CLI.\n",
                            ),
                        );
                    };
                } else {
                    {
                        ::std::io::_print(
                            format_args!("Usage: goat hooks show <name>\n"),
                        );
                    };
                }
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!("Unknown hooks action: {0}\n", action),
                    );
                };
            }
        }
        Ok(())
    }
    fn handle_schedule_command(
        paths: &crate::paths::GoatPaths,
        config: &crate::config::Config,
        action: &str,
        args: &[String],
    ) -> anyhow::Result<()> {
        let mut sm = crate::scheduler::SchedulerManager::new(
            config.scheduler.clone(),
            paths.clone(),
        );
        match action {
            "list" => {
                let jobs = sm.list_jobs();
                {
                    ::std::io::_print(format_args!("[SCHEDULE] Scheduled Jobs:\n"));
                };
                for j in jobs {
                    {
                        ::std::io::_print(
                            format_args!(
                                "  - [{0}] {1} (enabled: {2})\n",
                                j.id,
                                j.prompt_or_command,
                                j.enabled,
                            ),
                        );
                    };
                }
            }
            "add" => {
                {
                    ::std::io::_print(
                        format_args!(
                            "[SCHEDULE] Adding jobs via CLI is not fully implemented yet.\n",
                        ),
                    );
                };
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!("Unknown schedule action: {0}\n", action),
                    );
                };
            }
        }
        Ok(())
    }
    fn handle_jobs_command(
        paths: &crate::paths::GoatPaths,
        _config: &crate::config::Config,
        action: &str,
        arg: Option<&str>,
    ) -> anyhow::Result<()> {
        let mut rt = crate::agent_runtime::AgentRuntime::new(
            crate::agent_runtime::AgentRuntimeConfig::default(),
            paths.runtime_dir.clone(),
        )?;
        match action {
            "list" => {
                let jobs = rt.list_jobs();
                {
                    ::std::io::_print(format_args!("[RUNTIME] Active jobs:\n"));
                };
                if jobs.is_empty() {
                    {
                        ::std::io::_print(format_args!("  No jobs found.\n"));
                    };
                } else {
                    for job in jobs {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "  - [{0}] {1} ({2:?}) - {3}\n",
                                    job.id,
                                    job.agent_id,
                                    job.status,
                                    job.input_summary,
                                ),
                            );
                        };
                    }
                }
            }
            "show" => {
                if let Some(id) = arg {
                    if let Some(job) = rt.get_job(id) {
                        {
                            ::std::io::_print(format_args!("Job ID: {0}\n", job.id));
                        };
                        {
                            ::std::io::_print(
                                format_args!("Agent: {0}\n", job.agent_id),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Status: {0:?}\n", job.status),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Task: {0}\n", job.input_summary),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Artifacts: {0:?}\n", job.artifacts),
                            );
                        };
                    } else {
                        {
                            ::std::io::_print(format_args!("Job {0} not found.\n", id));
                        };
                    }
                } else {
                    {
                        ::std::io::_print(format_args!("Usage: goat jobs show <id>\n"));
                    };
                }
            }
            "pause" => {
                if let Some(id) = arg {
                    let _ = rt.pause_job(id);
                    {
                        ::std::io::_print(format_args!("Job {0} paused.\n", id));
                    };
                }
            }
            "resume" => {
                if let Some(id) = arg {
                    let _ = rt.resume_job(id);
                    {
                        ::std::io::_print(format_args!("Job {0} resumed.\n", id));
                    };
                }
            }
            "cancel" => {
                if let Some(id) = arg {
                    let _ = rt.cancel_job(id);
                    {
                        ::std::io::_print(format_args!("Job {0} cancelled.\n", id));
                    };
                }
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!("Unknown jobs action: {0}\n", action),
                    );
                };
            }
        }
        Ok(())
    }
    async fn handle_daemon_command(
        paths: &crate::paths::GoatPaths,
        config: &crate::config::Config,
        action: &str,
    ) -> anyhow::Result<()> {
        match action {
            "start" => {
                let (rt, _) = crate::runtime::GoatRuntime::bootstrap(
                    config.clone(),
                    paths.clone(),
                    ::alloc::vec::Vec::new(),
                    false,
                    None,
                );
                crate::daemon::run(rt).await?;
            }
            "status" => {
                crate::daemon::get_status(paths);
            }
            "doctor" => {
                crate::daemon::print_doctor(paths, config);
            }
            "stop" => {
                {
                    ::std::io::_print(
                        format_args!(
                            "[DAEMON] Stop command is partial. Use Ctrl+C on the start terminal or kill the PID directly for now.\n",
                        ),
                    );
                };
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!(
                            "[DAEMON] Unknown action \'{0}\'. Use start, status, stop, or doctor.\n",
                            action,
                        ),
                    );
                };
            }
        }
        Ok(())
    }
    fn handle_dashboard_command(action: &str) {
        let root = std::env::current_dir().unwrap_or_default();
        let dashboard_dir = root.join("apps").join("dashboard");
        let fallback_dir = root.join("dashboard");
        let active_dir = if dashboard_dir.exists() {
            dashboard_dir
        } else if fallback_dir.exists() {
            fallback_dir
        } else {
            {
                ::std::io::_print(
                    format_args!(
                        "[DASHBOARD] Cannot find dashboard/ or apps/dashboard/ directory.\n",
                    ),
                );
            };
            return;
        };
        match action {
            "path" => {
                {
                    ::std::io::_print(format_args!("{0}\n", active_dir.display()));
                };
            }
            "doctor" => {
                {
                    ::std::io::_print(format_args!("[DASHBOARD DOCTOR]\n"));
                };
                {
                    ::std::io::_print(
                        format_args!("  Dashboard Path: {0}\n", active_dir.display()),
                    );
                };
                let pkg_json = active_dir.join("package.json");
                {
                    ::std::io::_print(
                        format_args!(
                            "  package.json: {0}\n",
                            if pkg_json.exists() { "Found" } else { "Missing" },
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  To run the dashboard, navigate to the path, run `npm install`, then `npm run dev`.\n",
                        ),
                    );
                };
            }
            "dev" => {
                {
                    ::std::io::_print(
                        format_args!(
                            "[DASHBOARD] Dashboard code is at: {0}\n",
                            active_dir.display(),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("  Please run the following in a new terminal:\n"),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("    cd {0}\n", active_dir.display()),
                    );
                };
                {
                    ::std::io::_print(format_args!("    npm install\n"));
                };
                {
                    ::std::io::_print(format_args!("    npm run dev\n"));
                };
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!(
                            "[DASHBOARD] Unknown action \'{0}\'. Use dev, path, or doctor.\n",
                            action,
                        ),
                    );
                };
            }
        }
    }
    fn handle_desktop_command(action: &str) {
        let root = std::env::current_dir().unwrap_or_default();
        let desktop_dir = root.join("apps").join("desktop");
        if !desktop_dir.exists() {
            {
                ::std::io::_print(
                    format_args!("[DESKTOP] Cannot find apps/desktop/ directory.\n"),
                );
            };
            return;
        }
        match action {
            "path" => {
                {
                    ::std::io::_print(format_args!("{0}\n", desktop_dir.display()));
                };
            }
            "doctor" => {
                {
                    ::std::io::_print(format_args!("[DESKTOP DOCTOR]\n"));
                };
                {
                    ::std::io::_print(
                        format_args!("  Desktop Path: {0}\n", desktop_dir.display()),
                    );
                };
                let pkg_json = desktop_dir.join("package.json");
                {
                    ::std::io::_print(
                        format_args!(
                            "  package.json: {0}\n",
                            if pkg_json.exists() { "Found" } else { "Missing" },
                        ),
                    );
                };
                let tauri_conf = desktop_dir.join("src-tauri").join("tauri.conf.json");
                {
                    ::std::io::_print(
                        format_args!(
                            "  tauri.conf.json: {0}\n",
                            if tauri_conf.exists() { "Found" } else { "Missing" },
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "  To run the desktop app, navigate to the path, run `npm install`, then `npm run tauri dev`.\n",
                        ),
                    );
                };
            }
            "run" | "dev" => {
                {
                    ::std::io::_print(
                        format_args!(
                            "[DESKTOP] Desktop app code is at: {0}\n",
                            desktop_dir.display(),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("  Please run the following in a new terminal:\n"),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("    cd {0}\n", desktop_dir.display()),
                    );
                };
                {
                    ::std::io::_print(format_args!("    npm install\n"));
                };
                {
                    ::std::io::_print(format_args!("    npm run dev\n"));
                };
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!(
                            "[DESKTOP] Unknown action \'{0}\'. Use dev, run, path, or doctor.\n",
                            action,
                        ),
                    );
                };
            }
        }
    }
    fn handle_browser_command(
        paths: &crate::paths::GoatPaths,
        config: &crate::config::Config,
        action: &str,
        args: &[String],
    ) -> anyhow::Result<()> {
        use crate::browser_adapter::BrowserAdapterManager;
        use crate::browser_workflows::BrowserWorkflowManager;
        use std::sync::Arc;
        use tokio::sync::Mutex;
        let manager = BrowserWorkflowManager::new(&paths.data_dir);
        let browser_config = config.browser.clone();
        let mut browser_adapter = BrowserAdapterManager::new(browser_config);
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        match action {
            "workflows" => {
                let list = manager.list_workflows()?;
                {
                    ::std::io::_print(
                        format_args!("Browser Workflows ({0}):\n", list.len()),
                    );
                };
                for w in list {
                    {
                        ::std::io::_print(
                            format_args!(
                                "- {0} [{1}] -> Status: {2:?}\n",
                                w.id,
                                w.title,
                                w.status,
                            ),
                        );
                    };
                }
            }
            "screenshot" => {
                let url = args
                    .get(0)
                    .cloned()
                    .unwrap_or_else(|| "http://localhost:3000".to_string());
                {
                    ::std::io::_print(
                        format_args!("Creating workflow for screenshot of {0}\n", url),
                    );
                };
                let w = manager
                    .create_workflow("Screenshot Capture", &url, "screenshot");
                manager.save_workflow(&w)?;
                let updated = rt
                    .block_on(manager.run_workflow(&w.id, &mut browser_adapter))?;
                {
                    ::std::io::_print(
                        format_args!(
                            "Workflow Completed. Status: {0:?}\n",
                            updated.status,
                        ),
                    );
                };
            }
            "inspect" => {
                let url = args
                    .get(0)
                    .cloned()
                    .unwrap_or_else(|| "http://localhost:3000".to_string());
                {
                    ::std::io::_print(
                        format_args!("Creating workflow for inspection of {0}\n", url),
                    );
                };
                let w = manager.create_workflow("DOM Inspection", &url, "inspect");
                manager.save_workflow(&w)?;
                let updated = rt
                    .block_on(manager.run_workflow(&w.id, &mut browser_adapter))?;
                {
                    ::std::io::_print(
                        format_args!(
                            "Workflow Completed. Status: {0:?}\n",
                            updated.status,
                        ),
                    );
                };
            }
            "qa" => {
                let url = args
                    .get(0)
                    .cloned()
                    .unwrap_or_else(|| "http://localhost:3000".to_string());
                {
                    ::std::io::_print(
                        format_args!("Creating workflow for QA of {0}\n", url),
                    );
                };
                let w = manager.create_workflow("UI QA", &url, "ui-qa");
                manager.save_workflow(&w)?;
                let updated = rt
                    .block_on(manager.run_workflow(&w.id, &mut browser_adapter))?;
                {
                    ::std::io::_print(
                        format_args!(
                            "Workflow Completed. Status: {0:?}\n",
                            updated.status,
                        ),
                    );
                };
            }
            "landing-review" => {
                let url = args
                    .get(0)
                    .cloned()
                    .unwrap_or_else(|| "http://localhost:3000".to_string());
                {
                    ::std::io::_print(
                        format_args!(
                            "Creating workflow for Landing Page Review of {0}\n",
                            url,
                        ),
                    );
                };
                let w = manager
                    .create_workflow("Landing Page Review", &url, "landing-review");
                manager.save_workflow(&w)?;
                let updated = rt
                    .block_on(manager.run_workflow(&w.id, &mut browser_adapter))?;
                {
                    ::std::io::_print(
                        format_args!(
                            "Workflow Completed. Status: {0:?}\n",
                            updated.status,
                        ),
                    );
                };
            }
            "dashboard-qa" => {
                {
                    ::std::io::_print(
                        format_args!("Creating workflow for Dashboard QA\n"),
                    );
                };
                let w = manager
                    .create_workflow(
                        "Dashboard QA",
                        "http://localhost:3000",
                        "dashboard-qa",
                    );
                manager.save_workflow(&w)?;
                let updated = rt
                    .block_on(manager.run_workflow(&w.id, &mut browser_adapter))?;
                {
                    ::std::io::_print(
                        format_args!(
                            "Workflow Completed. Status: {0:?}\n",
                            updated.status,
                        ),
                    );
                };
            }
            "health" => {
                let url = args
                    .get(0)
                    .cloned()
                    .unwrap_or_else(|| "http://localhost:3000".to_string());
                {
                    ::std::io::_print(
                        format_args!("Creating workflow for Health Check of {0}\n", url),
                    );
                };
                let w = manager
                    .create_workflow("Web Health Check", &url, "web-health-check");
                manager.save_workflow(&w)?;
                let updated = rt
                    .block_on(manager.run_workflow(&w.id, &mut browser_adapter))?;
                {
                    ::std::io::_print(
                        format_args!(
                            "Workflow Completed. Status: {0:?}\n",
                            updated.status,
                        ),
                    );
                };
            }
            _ => {
                {
                    ::std::io::_print(format_args!("Unknown action: {0}\n", action));
                };
            }
        }
        Ok(())
    }
    fn handle_builder_command(
        paths: &crate::paths::GoatPaths,
        config: &crate::config::Config,
        action: &str,
        args: &[String],
    ) -> anyhow::Result<()> {
        use crate::agents::builder::{BuilderAgent, BuilderInspectionScope};
        use crate::brain_index::BrainIndexManager;
        let agent = BuilderAgent::new()?;
        let brain_mgr = BrainIndexManager::new(
            paths.clone(),
            config.brain_index.clone(),
            &config.embeddings,
        );
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        match action {
            "inspect" => {
                let result = agent
                    .inspect_repo(BuilderInspectionScope {
                        max_depth: 3,
                        include_tests: true,
                    })?;
                {
                    ::std::io::_print(
                        format_args!(
                            "[BUILDER] Inspection complete. Snapshot generated.\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Root: {0}\n", result.snapshot.root_path),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Main Language: {0}\n",
                            result.snapshot.tech_stack.main_language,
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Files scanned: {0}\n", result.snapshot.file_count),
                    );
                };
            }
            "plan" => {
                let goal = args.join(" ");
                if goal.is_empty() {
                    {
                        ::std::io::_print(
                            format_args!("[BUILDER] Please provide a goal.\n"),
                        );
                    };
                    return Ok(());
                }
                let plan = rt.block_on(agent.plan_patch(&goal, &brain_mgr))?;
                {
                    ::std::io::_print(
                        format_args!(
                            "[BUILDER] Patch Plan Generated (ID: {0})\n",
                            plan.id,
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("Goal: {0}\n", plan.goal));
                };
                {
                    ::std::io::_print(
                        format_args!("Risk Level: {0}\n", plan.risk_level),
                    );
                };
            }
            "diff-review" => {
                let plan_id = args.first().map(|s| s.as_str()).unwrap_or("active_plan");
                let review = agent.diff_review(plan_id)?;
                {
                    ::std::io::_print(format_args!("[BUILDER] Diff Review Complete.\n"));
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Overall Severity: {0:?}\n",
                            review.overall_severity,
                        ),
                    );
                };
                for finding in review.findings {
                    {
                        ::std::io::_print(
                            format_args!(
                                "- [{0}]: {1}\n",
                                finding.file_path,
                                finding.issue_description,
                            ),
                        );
                    };
                }
            }
            "test-plan" => {
                let goal = args.join(" ");
                let plan = agent.test_plan(&goal)?;
                {
                    ::std::io::_print(
                        format_args!(
                            "[BUILDER] Test Plan Created (ID: {0})\n",
                            plan.plan_id,
                        ),
                    );
                };
                for cmd in plan.commands {
                    {
                        ::std::io::_print(format_args!("Command: {0}\n", cmd.command));
                    };
                }
            }
            "validate" => {
                let plan_id = args.first().map(|s| s.as_str()).unwrap_or("active_plan");
                let result = agent.validate(plan_id)?;
                {
                    ::std::io::_print(
                        format_args!(
                            "[BUILDER] Validation Finished. Valid: {0}\n",
                            result.is_valid,
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("Logs:\n{0}\n", result.test_logs));
                };
            }
            "rollback-plan" => {
                let plan_id = args.first().map(|s| s.as_str()).unwrap_or("active_plan");
                let rollback = agent.rollback_plan(plan_id)?;
                {
                    ::std::io::_print(
                        format_args!("[BUILDER] Rollback Plan generated.\n"),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Fallback command: {0}\n",
                            rollback.command_fallback,
                        ),
                    );
                };
            }
            _ => {
                {
                    ::std::io::_print(
                        format_args!(
                            "Unknown action: {0}. Use inspect, plan, diff-review, test-plan, validate, rollback-plan\n",
                            action,
                        ),
                    );
                };
            }
        }
        Ok(())
    }
    fn handle_researcher_command(
        _paths: &crate::paths::GoatPaths,
        _config: &crate::config::Config,
        action: &str,
        args: &[String],
    ) -> anyhow::Result<()> {
        match action {
            "projects" => {
                ::std::io::_print(format_args!("[RESEARCHER] Projects list:\n"));
            }
            "new" => {
                let q = args.join(" ");
                {
                    ::std::io::_print(
                        format_args!("[RESEARCHER] Creating project: {0}\n", q),
                    );
                };
            }
            "add-source" => {
                ::std::io::_print(
                    format_args!("[RESEARCHER] Adding source to project\n"),
                );
            }
            "ingest-browser" => {
                ::std::io::_print(
                    format_args!("[RESEARCHER] Ingesting browser artifact\n"),
                );
            }
            "brief" => {
                ::std::io::_print(
                    format_args!("[RESEARCHER] Generating brief for project\n"),
                );
            }
            "competitors" => {
                ::std::io::_print(format_args!("[RESEARCHER] Scanning competitors\n"));
            }
            "compare-tech" => {
                ::std::io::_print(
                    format_args!("[RESEARCHER] Comparing technology options\n"),
                );
            }
            "report" => {
                ::std::io::_print(format_args!("[RESEARCHER] Generating report\n"));
            }
            _ => {
                ::std::io::_print(
                    format_args!(
                        "[RESEARCHER] Unknown action. Use projects, new, add-source, ingest-browser, brief, competitors, compare-tech, report\n",
                    ),
                );
            }
        }
        Ok(())
    }
}
