use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    pyo3::Python::initialize();

    let stub = test_dash_package::stub_info()?;
    stub.generate()?;
    Ok(())
}
