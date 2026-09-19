use std::path::PathBuf;

use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;

const RL_STYLES: Styles = Styles::styled()
    .header(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Yellow.on_default())
    .error(AnsiColor::Red.on_default().effects(Effects::BOLD))
    .valid(AnsiColor::Green.on_default())
    .invalid(AnsiColor::Red.on_default());

#[derive(Parser)]
#[command(
    name = "rldocs",
    version,
    about = "Documentation viewer for rl-lang",
    styles = RL_STYLES,
    after_help = "EXAMPLES:\n    \
                   rldocs\n    \
                   rldocs io\n    \
                   rldocs --json\n    \
                   rldocs --tui"
)]
struct Cli {
    /// Topic to look up (stdlib module, concept, or tutorial name)
    topic: Option<String>,

    /// Output as JSON instead of Markdown
    #[arg(long)]
    json: bool,

    /// Show only stdlib entries
    #[arg(long)]
    stdlib: bool,

    /// Show only concept entries
    #[arg(long)]
    concept: bool,

    /// Show only tutorial entries
    #[arg(long)]
    tutorial: bool,

    /// Write output to a file instead of stdout
    #[arg(short, long)]
    output: bool,

    /// Specify output file path
    #[arg(long, value_name = "PATH")]
    out_file: Option<PathBuf>,

    /// Open the docs TUI viewer
    #[arg(long)]
    tui: bool,
}

fn main() {
    let cli = Cli::parse();

    let std_entries = rl_docs::entries::stdlib_entries();
    let concept_entries = rl_docs::entries::concept_entries();
    let tutorial_entries = rl_docs::entries::tutorial_entries();

    let any_category = cli.stdlib || cli.concept || cli.tutorial;
    let want_std = !any_category || cli.stdlib;
    let want_concept = !any_category || cli.concept;
    let want_tutorial = !any_category || cli.tutorial;

    let (matched_std, matched_concepts, matched_tutorial): (
        Vec<&rl_docs::entry::StdEntry>,
        Vec<&rl_docs::entry::ConceptEntry>,
        Vec<&rl_docs::entry::ConceptEntry>,
    ) = match cli.topic.as_deref() {
        None => (
            if want_std {
                std_entries.to_vec()
            } else {
                Vec::new()
            },
            if want_concept {
                concept_entries.to_vec()
            } else {
                Vec::new()
            },
            if want_tutorial {
                tutorial_entries.to_vec()
            } else {
                Vec::new()
            },
        ),
        Some(query) => {
            let matched_std = if want_std {
                std_entries
                    .iter()
                    .copied()
                    .filter(|e| {
                        e.name.contains(query)
                            || e.functions
                                .iter()
                                .any(|f| func_name(f).contains(query))
                    })
                    .collect()
            } else {
                Vec::new()
            };

            let matched_concepts = if want_concept {
                concept_entries
                    .iter()
                    .copied()
                    .filter(|e| e.name.contains(query))
                    .collect()
            } else {
                Vec::new()
            };

            let matched_tutorial = if want_tutorial {
                tutorial_entries
                    .iter()
                    .copied()
                    .filter(|e| e.name.contains(query))
                    .collect()
            } else {
                Vec::new()
            };

            if matched_std.is_empty() && matched_concepts.is_empty() && matched_tutorial.is_empty()
            {
                eprintln!("no docs found for '{}'", query);
                std::process::exit(1);
            }

            (matched_std, matched_concepts, matched_tutorial)
        }
    };

    if cli.tui {
        #[cfg(feature = "tui")]
        {
            if let Err(e) = rl_docs::tui::run_docs_tui(
                &matched_std,
                &matched_concepts,
                &matched_tutorial,
                cli.topic.as_deref(),
            ) {
                eprintln!("error: docs tui failed: {}", e);
                std::process::exit(1);
            }
            return;
        }
        #[cfg(not(feature = "tui"))]
        {
            eprintln!("error: this build of rldocs was compiled without --tui support (missing 'tui' feature)");
            std::process::exit(1);
        }
    }

    let rendered = if cli.json {
        match rl_docs::docs_to_json(&matched_std, &matched_concepts, &matched_tutorial) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error: failed to serialize docs to json: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        let mut out = String::new();
        if !matched_std.is_empty() {
            if let Some(ref query) = cli.topic {
                // Check if any function names match the query
                let has_func_match = matched_std
                    .iter()
                    .any(|e| {
                        e.name != *query
                            && e.functions.iter().any(|f| func_name(f).contains(query.as_str()))
                    });

                if has_func_match {
                    // Render individual matching functions
                    for entry in &matched_std {
                        if entry.name.contains(query.as_str()) {
                            // Module name matches - render whole module
                            out.push_str(&rl_docs::std_to_markdown(std::slice::from_ref(entry)));
                        } else {
                            // Render only matching functions
                            for func in entry.functions {
                                if func_name(func).contains(query.as_str()) {
                                    out.push_str(&render_function(entry, func));
                                }
                            }
                        }
                    }
                } else {
                    out.push_str(&rl_docs::std_to_markdown(&matched_std));
                }
            } else {
                out.push_str(&rl_docs::std_to_markdown(&matched_std));
            }
        }
        if !matched_concepts.is_empty() {
            out.push_str(&rl_docs::concept_to_markdown(&matched_concepts));
        }
        if !matched_tutorial.is_empty() {
            out.push_str(&rl_docs::tutorial_to_markdown(&matched_tutorial));
        }
        out
    };

    let write_to_file = cli.output || cli.out_file.is_some();

    if write_to_file {
        let ext = if cli.json { "json" } else { "md" };
        let filename = cli
            .out_file
            .unwrap_or_else(|| PathBuf::from(format!("docs_output.{}", ext)));
        if let Err(e) = std::fs::write(&filename, &rendered) {
            eprintln!("error: failed to write '{}': {}", filename.display(), e);
            std::process::exit(1);
        }
        println!("docs written to '{}'", filename.display());
    } else {
        println!("{}", rendered);
    }
}

fn func_name(func: &rl_docs::entry::FnEntry) -> &str {
    func.signature.split('(').next().unwrap_or(func.signature)
}

fn render_function(module: &rl_docs::entry::StdEntry, func: &rl_docs::entry::FnEntry) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# std::{}::{}\n\n",
        module.name,
        func_name(func)
    ));
    if let Some(since) = func.since {
        out.push_str(&format!("*since {}*", since));
        if let Some(updated) = func.updated {
            out.push_str(&format!(" *updated {}*", updated));
        }
        out.push_str("\n\n");
    }
    if let Some(deprecated) = func.deprecated {
        out.push_str(&format!("**Deprecated:** {}\n\n", deprecated));
    }
    out.push_str(&format!("{}\n\n", func.description));
    out.push_str(&format!("**Returns:** {}\n\n", func.returns));
    if let Some(errors) = func.errors {
        out.push_str(&format!("**Errors:** {}\n\n", errors));
    }
    out.push_str(&format!("```\n{}\n```\n\n", func.example));
    if let Some(expected) = func.expected_output {
        out.push_str(&format!("output:\n```\n{}\n```\n\n", expected));
    }
    if !func.see_also.is_empty() {
        out.push_str("**See also:** ");
        out.push_str(&func.see_also.join(", "));
        out.push_str("\n\n");
    }
    out
}
