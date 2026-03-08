/// Demonstrates `MandatoryParameter::on_change()` and `OptionalParameter::on_change()`.
///
/// The node declares two parameters:
///   - "speed" (mandatory i64) — on_change logs the new value
///   - "label" (optional string) — on_change logs the new value or "unset"
///
/// Try it:
///   $ ros2 run examples_rclrs_parameter_callback_demo on_change_demo
///
///   # In another terminal:
///   $ ros2 param set /on_change_demo speed 99
///   $ ros2 param set /on_change_demo label "turbo"
///   $ ros2 param set /on_change_demo label ""   # resets to empty, not unset
use rclrs::*;
use std::sync::Arc;

fn main() -> Result<(), RclrsError> {
    let mut executor = Context::default_from_env()?.create_basic_executor();
    let node = executor.create_node("on_change_demo")?;

    let speed = node
        .declare_parameter("speed")
        .default(50i64)
        .mandatory()?;

    let label: OptionalParameter<Arc<str>> = node
        .declare_parameter("label")
        .optional()?;

    speed.on_change(|value: &i64| {
        println!("[on_change] speed changed to {value}");
    });

    label.on_change(|value: Option<&Arc<str>>| {
        match value {
            Some(s) => println!("[on_change] label changed to \"{s}\""),
            None => println!("[on_change] label was unset"),
        }
    });

    println!(
        "speed = {}, label = {:?}\n\n\
        Node is spinning. Change parameters from another terminal:\n  \
        $ ros2 param set /on_change_demo speed 99\n  \
        $ ros2 param set /on_change_demo label \"turbo\"\n",
        speed.get(),
        label.get(),
    );

    executor.spin(SpinOptions::default()).first_error()
}
