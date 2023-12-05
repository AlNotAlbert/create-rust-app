use std::path::PathBuf;
use inflector::Inflector;
use crate::utils::fs::ensure_directory;
use crate::utils::logger;

fn get_migration_number() -> usize {
    let migrations_dir = PathBuf::from("backend/prisma/migrations");

    if !migrations_dir.is_dir() {
        logger::message("Migrations directory does not exist, create it?");
    }

    let v = migrations_dir.read_dir().unwrap();
    v.count()
}

pub fn create(name: &str) {
    let migration_number = get_migration_number();
    let mut migrations_dir = PathBuf::from("backend/prisma/migrations");
    let migration_dir_name = format!("{:0>14}_{}", migration_number, name.to_snake_case());

    migrations_dir.push(&migration_dir_name);
    ensure_directory(&migrations_dir, false).unwrap();

    todo!("create up.sql and down.sql files for prisma");
}