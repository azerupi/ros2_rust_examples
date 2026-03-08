/// Demonstrates `ParameterBuilder::validate()`.
///
/// The node declares an integer "speed" parameter that must be a positive
/// multiple of 5. Any attempt to set it to an invalid value — whether via
/// `param.set()` or `ros2 param set` — is rejected and the old value is kept.
///
/// Try it:
///   $ ros2 run examples_rclrs_parameter_callback_demo validate_demo
///
///   # In another terminal:
///   $ ros2 param set /validate_demo speed 25    # succeeds
///   $ ros2 param set /validate_demo speed 13    # rejected: not a multiple of 5
///   $ ros2 param set /validate_demo speed -10   # rejected: not positive
use rclrs::*;

fn main() -> Result<(), RclrsError> {
    let mut executor = Context::default_from_env()?.create_basic_executor();
    let node = executor.create_node("validate_demo")?;

    let speed = node
        .declare_parameter("speed")
        .default(50i64)
        .validate(|value: &i64| {
            if *value <= 0 {
                Err("speed must be positive".into())
            } else if *value % 5 != 0 {
                Err("speed must be a multiple of 5".into())
            } else {
                Ok(())
            }
        })
        .mandatory()?;

    println!("speed = {} (initial)", speed.get());

    // Demonstrate programmatic validation
    match speed.set(25) {
        Ok(()) => println!("speed = {} (set to 25 succeeded)", speed.get()),
        Err(e) => println!("set to 25 failed: {e}"),
    }
    match speed.set(13) {
        Ok(()) => println!("speed = {} (set to 13 succeeded)", speed.get()),
        Err(e) => println!("set to 13 failed: {e}"),
    }
    match speed.set(-10) {
        Ok(()) => println!("speed = {} (set to -10 succeeded)", speed.get()),
        Err(e) => println!("set to -10 failed: {e}"),
    }

    println!(
        "\nspeed = {} (unchanged after rejections)\n\n\
        Node is spinning. Try changing the parameter from another terminal:\n  \
        $ ros2 param set /validate_demo speed 25   # accepted\n  \
        $ ros2 param set /validate_demo speed 13   # rejected\n",
        speed.get()
    );

    executor.spin(SpinOptions::default()).first_error()
}
