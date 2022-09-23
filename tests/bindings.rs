use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

#[test]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};
    use interoptopus_backend_csharp::overloads::{DotNet, Unity};

    let config = Config {
        dll_name: "csharp_interop".to_string(),
        namespace_mappings: NamespaceMappings::new("connorwmackay"),
        ..Config::default()
    };

    Generator::new(config, csharp_interop::my_inventory())
        .add_overload_writer(DotNet::new())
        .write_file("bindings/Interop.cs")?;

    Ok(())
}