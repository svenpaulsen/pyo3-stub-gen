use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    pyo3::Python::initialize();

    env_logger::init();
    let stub = empty_super_module::stub_info()?;
    stub.generate()?;
    Ok(())
}
