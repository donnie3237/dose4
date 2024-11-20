use inquire::{Select, Text}; // For interactive user input
use std::process::Command;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use std::fs;
use clap::{Command as ClapCommand, Arg, ArgAction}; // Updated clap usage

fn main() {
    // Define the CLI arguments with clap
    let matches = ClapCommand::new("dose")
        .version("1.0")
        .about("")
        .arg(Arg::new("check")
            .long("check")
            .action(ArgAction::SetTrue)
            .help("Check if Git and Node.js are installed"))
        .get_matches();

    // If --check argument is passed, check for git and node installations
    if *matches.get_one::<bool>("check").unwrap_or(&false) {
        // Check Git version
        match Command::new("git").arg("--version").output() {
            Ok(output) => {
                if output.status.success() {
                    println!("✅ Git is installed: {}", String::from_utf8_lossy(&output.stdout).trim());
                } else {
                    eprintln!("❌ Git is installed but not working properly.");
                }
            }
            Err(_) => eprintln!("❌ Git is not installed. Please install Git first."),
        }

        // Check Node.js version
        match Command::new("node").arg("--version").output() {
            Ok(output) => {
                if output.status.success() {
                    println!("✅ Node.js is installed: {}", String::from_utf8_lossy(&output.stdout).trim());
                } else {
                    eprintln!("❌ Node.js is installed but not working properly.");
                }
            }
            Err(_) => eprintln!("❌ Node.js is not installed. Please install Node.js first."),
        }

        return; // Exit after checking
    }

    // Prompt user for their name
    let name = Text::new("What's your name?")
        .prompt()
        .expect("Failed to read name");

    // Greet the user
    println!("Hello, {}!", name);

    // Define a list of frameworks and their corresponding Git repositories
    let github_repos: Vec<(&str, &str)> = vec![
        ("react", "https://github.com/donnie3237/React-template2.git"),
        ("svelte", "https://github.com/donnie3237/svelte-template.git"),
        ("express", "https://github.com/donnie3237/ExpressJS-Template.git"),
        ("hono", "https://github.com/donnie3237/Hono-template.git"),
        ("nextjs", "https://github.com/donnie3237/Nextjs-template.git"),
        ("astro", "https://github.com/donnie3237/Astro-template.git"),
        ("tauri", "https://github.com/donnie3237/Tauri-template.git"),
        ("typescript", "https://github.com/JKTheRipperTH/vue-dose3-Template.git"),
    ];

    // Extract only the names for display in the menu
    let framework_names: Vec<&str> = github_repos.iter().map(|(name, _)| *name).collect();

    // Prompt the user to select a framework
    let selected_framework = Select::new("Select a framework to clone:", framework_names)
        .prompt()
        .expect("Failed to select a framework");

    // Find the corresponding repository URL
    if let Some((_, repo_url)) = github_repos.iter().find(|(name, _)| name == &selected_framework) {
        println!("You selected: {}. Repository: {}", selected_framework, repo_url);

        // Use the user's name as the target directory
        let target_directory = name.clone();

        // Check if git is installed
        match Command::new("git").arg("--version").output() {
            Ok(output) => {
                if output.status.success() {
                    println!("Git is installed: {}", String::from_utf8_lossy(&output.stdout).trim());

                    // Initialize the spinner
                    let pb = ProgressBar::new_spinner();
                    pb.set_message("Cloning repository...");
                    pb.enable_steady_tick(Duration::from_millis(100)); // Use correct Duration
                    pb.set_style(
                        ProgressStyle::default_spinner()
                            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                            .template("{spinner} {msg}")
                            .unwrap(),
                    );

                    // Clone the repository into the target directory without showing output
                    let clone_result = Command::new("git")
                        .arg("clone")
                        .arg(repo_url)
                        .arg(&target_directory)
                        .stdout(std::process::Stdio::null())
                        .stderr(std::process::Stdio::null())
                        .status();

                    // Stop the spinner
                    pb.finish_and_clear();

                    match clone_result {
                        Ok(status) => {
                            if status.success() {
                                println!(
                                    "✅ Successfully cloned repository into folder: {}",
                                    target_directory
                                );

                                // Remove the .git folder to avoid it being cloned
                                let git_folder_path = format!("{}/.git", target_directory);
                                if fs::remove_dir_all(&git_folder_path).is_ok() {
                                    println!("✅ Removed .git folder.");

                                    // Ask the user what to do next
                                    let action = Select::new(
                                        "What would you like to do next?",
                                        vec!["Finish", "Open in VSCode"],
                                    )
                                    .prompt()
                                    .expect("Failed to select an action");

                                    if action == "Open in VSCode" {
                                        // Platform detection: Check if the OS is Windows or Unix-like
                                        let is_windows = std::env::consts::OS == "windows";

                                        // Run the appropriate command based on the platform
                                        let result = if is_windows {
                                            // On Windows, use cmd to cd and open VSCode
                                            Command::new("cmd")
                                                .args(&["/C", &format!("cd {} && code .", target_directory)])
                                                .status()
                                        } else {
                                            // On Unix-like systems, use bash to cd and open VSCode
                                            Command::new("bash")
                                                .arg("-c")
                                                .arg(format!("cd {} && code .", target_directory))
                                                .status()
                                        };

                                        match result {
                                            Ok(status) => {
                                                if status.success() {
                                                    println!("✅ Opened in VSCode.");
                                                } else {
                                                    eprintln!("⚠️ Failed to open VSCode.");
                                                }
                                            }
                                            Err(e) => eprintln!("❌ Failed to execute command: {}", e),
                                        }
                                    } else {
                                        println!("✅ Process completed. Have a nice day!");
                                    }
                                } else {
                                    eprintln!("⚠️ Failed to remove .git folder.");
                                }
                            } else {
                                eprintln!("❌ Failed to clone repository: {}", repo_url);
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to run git clone: {}", e),
                    }
                } else {
                    eprintln!("⚠️  Git is installed but not working properly.");
                }
            }
            Err(_) => {
                eprintln!("❌ Git is not installed. Please install Git first.");
            }
        }
    } else {
        eprintln!("❌ Selected framework not found in the list!");
    }
}
