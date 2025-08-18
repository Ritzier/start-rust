let project_type = variable::get("project_type");

switch project_type {
    "cli" => {
        file::rename("template-cli/Cargo.toml", "Cargo.toml");
        file::rename("template-cli/src", "src");
    }
    _ => {
        print(`Unsupport project type: ${project_type}`);
    }
}

