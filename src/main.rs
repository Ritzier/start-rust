use {{project-name}}::{Error, trace};

fn main() -> Result<(), Error> {
    trace::setup_tracing();

    Ok(())
}
