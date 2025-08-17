let project_type = variable::get("project_type");
print("Generating " + project_type);

if project_type == "cli" {
    // Copy template-cli contents to root
    file::rename("template-cli/Cargo.toml", "Cargo.toml");
    file::rename("template-cli/src", "src");

    // Remove other template
    file::delete("template-tui");
    file::delete("template-cli");
} else if project_type == "tui" {
    // Copy template-tui contents to root
    file::rename("template-tui/Cargo.toml", "Cargo.toml");
    file::rename("template-tui/src", "src");

    // Remove other template
    file::delete("template-cli");
    file::delete("template-tui");
}

