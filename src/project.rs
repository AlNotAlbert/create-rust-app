use std::path::PathBuf;
use inflector::Inflector;
use anyhow::Result;
use console::{style};
use dialoguer::{
    Confirm,
    theme::ColorfulTheme
};
use rust_embed::RustEmbed;

use crate::logger::{message, command_msg, error, exit, file_msg, dependency_msg};

#[derive(RustEmbed)]
#[folder = "template"]
struct Asset;

fn add_dependency(project_dir: &std::path::PathBuf, name: &str, version: toml::Value) -> Result<(), std::io::Error> {
  let mut path = std::path::PathBuf::from(project_dir);
  path.push("Cargo.toml");
  
  let toml: String = std::fs::read_to_string(&path)?;

  let mut parsed_toml = toml.parse::<toml::Value>().unwrap();

  // println!("Parsed toml:\n{:#?}", &parsed_toml);

  let root: &mut toml::value::Table = parsed_toml.as_table_mut().unwrap();

  let deps_table: &mut toml::value::Table = root.get_mut("dependencies").unwrap().as_table_mut().unwrap();
  deps_table.insert(name.into(), version);

  let updated_toml = toml::to_string(&parsed_toml).unwrap();

  dependency_msg(name);
  
  std::fs::write(&path, updated_toml)?;
  
  Ok(())
}

/**
 * create-rust-app project generation
 */
pub fn create(opt: crate::Opt) -> Result<()> {
  let mut project_dir: PathBuf = PathBuf::from(opt.target);
    
    if project_dir.exists() {
        message("Directory already exists");

        project_dir = match std::fs::canonicalize(project_dir) {
            Ok(p) => p,
            Err(err) => exit("std::fs::canonicalize():", err)
        };

        let proceed = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Delete directory contents?")
            .default(false)
            .interact()?;

        if proceed {
            match std::fs::remove_dir_all(&project_dir) {
                Ok(_) => {},
                Err(err) => exit("std::fs::remove_dir_all():", err)
            }
        } else {
            std::process::exit(0);
        }
    }
    
    let project_name = project_dir.components().last().unwrap().as_os_str().to_str().unwrap();
        
    message(&format!("Creating project {}", style(project_name).yellow()));
        
    match std::fs::create_dir_all(&project_dir) {
        Ok(_) => {},
        Err(err) => exit("std::fs::create_dir_all():", err)
    }

    command_msg("cargo init");
    
    let cargo_init = std::process::Command::new("cargo")
        .current_dir(&project_dir)
        .arg("init")
        .status()
        .expect("failed to execute process");

    if !cargo_init.success() {
        error("Failed to execute `cargo init`");
        std::process::exit(1);
    }

    add_dependency(&project_dir, "actix-files", toml::Value::String("0.5.0".into()))?;
    add_dependency(&project_dir, "actix-http", toml::Value::String("2.2.0".into()))?;
    add_dependency(&project_dir, "actix-web", toml::Value::String("3.3.2".into()))?;
    add_dependency(&project_dir, "actix-web-httpauth", toml::Value::String("0.5.0".into()))?;
    add_dependency(&project_dir, "anyhow", toml::Value::String("1.0.33".into()))?;
    add_dependency(&project_dir, "chrono", "version = \"0.4.19\"\nfeatures = [\"serde\"]".parse::<toml::Value>().unwrap())?;
    add_dependency(&project_dir, "derive_more", toml::Value::String("0.99.11".into()))?;
    add_dependency(&project_dir, "diesel", "version = \"1.4.5\"\nfeatures = [\"postgres\", \"uuid\", \"r2d2\", \"chrono\", \"32-column-tables\"]\ndefault-features = false".parse::<toml::Value>().unwrap())?;
    add_dependency(&project_dir, "dotenv", toml::Value::String("0.15.0".into()))?;
    add_dependency(&project_dir, "env_logger", toml::Value::String("0.8.1".into()))?;
    add_dependency(&project_dir, "futures-util", toml::Value::String("0.3.8".into()))?;
    add_dependency(&project_dir, "jsonwebtoken", toml::Value::String("7.2.0".into()))?;
    add_dependency(&project_dir, "lettre", toml::Value::String("0.9.5".into()))?;
    add_dependency(&project_dir, "lettre_email", toml::Value::String("0.9.4".into()))?;
    add_dependency(&project_dir, "sentry", toml::Value::String("0.22.0".into()))?;
    add_dependency(&project_dir, "serde", "version = \"1.0.117\"\nfeatures = [\"derive\"]".parse::<toml::Value>().unwrap())?;
    add_dependency(&project_dir, "serde_derive", toml::Value::String("1.0.117".into()))?;
    add_dependency(&project_dir, "serde_json", toml::Value::String("1.0.59".into()))?;
    add_dependency(&project_dir, "tsync", toml::Value::String("1.0.1".into()))?;
    add_dependency(&project_dir, "uuid", "version = \"0.8.1\"\nfeatures = [\"serde\", \"v4\"]".parse::<toml::Value>().unwrap())?;

    /*
        Populate with project files
    */

    for filename in Asset::iter() {
        let file_contents = Asset::get(filename.as_ref()).unwrap();
        let mut file_path = std::path::PathBuf::from(&project_dir);
        file_path.push(filename.as_ref());
        let mut directory_path = std::path::PathBuf::from(&file_path);
        directory_path.pop();

        file_msg(filename.as_ref());
        std::fs::create_dir_all(directory_path)?;
        std::fs::write(file_path, file_contents)?;
    }
    
    /*
        Finalize; create the initial commit.
    */

    command_msg("git init");
    
    let git_init = std::process::Command::new("git")
        .current_dir(&project_dir)
        .arg("init")
        .stdout(std::process::Stdio::null())
        .status()
        .expect("failed to execute process");

    if !git_init.success() {
        error("Failed to execute `git init`");
        std::process::exit(1);
    }

    command_msg("git add -A");
    
    let git_add = std::process::Command::new("git")
        .current_dir(&project_dir)
        .arg("add")
        .arg("-A")
        .stdout(std::process::Stdio::null())
        .status()
        .expect("failed to execute process");

    if !git_add.success() {
        error("Failed to execute `git add -A`");
        std::process::exit(1);
    }

    command_msg("git commit -m Initial commit"); 
    
    let git_commit = std::process::Command::new("git")
        .current_dir(&project_dir)
        .arg("commit")
        .arg("-m")
        .arg("Initial commit")
        .stdout(std::process::Stdio::null())
        .status()
        .expect("failed to execute process");

    if !git_commit.success() {
        error("Failed to execute `git commit`");
        std::process::exit(1);
    }

    command_msg("git branch -M main");
    
    let git_branch = std::process::Command::new("git")
        .current_dir(&project_dir)
        .arg("branch")
        .arg("-M")
        .arg("main")
        .stdout(std::process::Stdio::null())
        .status()
        .expect("failed to execute process");

    if !git_branch.success() {
        error("Failed to execute `git branch -M main`");
        std::process::exit(1);
    }

    message("   ");
    message(&format!("   {}", style("ALL DONE!").underlined()));
    message(&format!("   1. Enable continuous-compilation!"));
    message(&format!("      {}", style("cargo install cargo-watch").cyan()));
    message(&format!("   2. Add dependencies via 'cargo add <dep>':"));
    message(&format!("      {}", style("cargo install cargo-edit").cyan()));
    message(&format!("   3. Get the diesel CLI to manage your database:"));
    message(&format!("      {}", style("cargo install diesel_cli").cyan()));
    message(&format!("   4. Develop! Run the following for continuous compilation:"));
    message(&format!("      (terminal 1) {}", style("cd app && yarn start").cyan()));
    message(&format!("      (terminal 2) {}", style("cargo watch -x run -i app/").cyan()));
    message("   ");
    message("   Enjoy :-)");

    Ok(())
}

pub fn create_resource(opt: crate::Opt) -> Result<()> {
    let arg: String = opt.target;
    let args: Vec<&str> = arg.split_whitespace().collect();

    if args.len() == 0 {
        error("No args given");
        std::process::exit(1);
    }
    
    let model_name = args[0].to_pascal_case();
    let file_name = model_name.to_snake_case();
    let table_name = model_name.to_table_case();

    message(&format!("Creating resource '{}'", model_name));
    
    add_rust_file(
        "src/models",
        file_name.as_str(),
        crate::model::generate_model(model_name.as_str(), table_name.as_str())        
    )?;

    add_rust_file(
        "src/services",
        file_name.as_str(),
        crate::service::generate_service(file_name.as_str(), model_name.as_str(), table_name.as_str())
    )?;

    register_service(file_name.as_str())?;
    
    Ok(())
}

fn register_service(service_file_name: &str) -> Result<()> {
    message(&format!("Registering service {}", service_file_name));
    
    let main_file_path = PathBuf::from("src/main.rs");
    if main_file_path.exists() && main_file_path.is_file() {
        let mut main_file_contents = std::fs::read_to_string(&main_file_path)?;
        
        main_file_contents = main_file_contents.replace("web::scope(\"/api\")", "web::scope(\"/api\")\n                    .service(services::{}::endpoints(web::scope(\"/{}\")))");
        std::fs::write(main_file_path, main_file_contents)?;
    }

    Ok(())
}

fn add_rust_file(file_directory: &str, file_name: &str, file_contents: String) -> Result<()> {
    let file_path = PathBuf::from(format!("{}/{}.rs", file_directory, file_name));
    let mod_file_path = PathBuf::from(format!("{}/mod.rs", file_directory));
    let file_directory = PathBuf::from(file_directory);
    
    if !file_directory.exists() {
        let proceed = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Directory does not exist, create '{:#?}' directory?", &file_directory))
            .default(false)
            .interact()?;

        if proceed {
            match std::fs::create_dir_all(file_directory) {
                Ok(_) => {},
                Err(err) => exit("std::fs::create_dir_all(file_directory):", err)
            }
        } else {
            return Ok(());
        }
    }

    file_msg(&format!("{:#?}", &file_path));

    let mut mod_file_contents: String;
    
    if mod_file_path.exists() {
        mod_file_contents = std::fs::read_to_string(&mod_file_path)?;
        mod_file_contents.push('\n');
    } else {
        mod_file_contents = String::new();
    }
    
    mod_file_contents.push_str("pub mod ");
    mod_file_contents.push_str(file_name);
    mod_file_contents.push_str(";\n");
    std::fs::write(mod_file_path, mod_file_contents)?;
    
    std::fs::write(
        file_path,
        file_contents
    )?;

    Ok(())
}