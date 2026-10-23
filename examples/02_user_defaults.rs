use foundation::prelude::*;

const SUITE: &str = "rs.doom-fish.foundation.example";

fn main() -> Result<(), FoundationError> {
    let standard = UserDefaults::standard();
    standard.register(&[
        ("ExampleSmoothing", DefaultValue::Integer(0)),
        ("ExampleGreeting", DefaultValue::from("hello")),
    ])?;
    println!("registered integer: {}", standard.integer("ExampleSmoothing")?);
    println!("registered string:  {:?}", standard.string("ExampleGreeting")?);

    let suite = UserDefaults::with_suite_name(SUITE)?;
    suite.set("ExampleScale", &DefaultValue::Double(1.5))?;
    println!("stored double:      {}", suite.double("ExampleScale")?);
    suite.remove("ExampleScale")?;
    println!("after remove:       {}", suite.contains("ExampleScale")?);
    Ok(())
}
