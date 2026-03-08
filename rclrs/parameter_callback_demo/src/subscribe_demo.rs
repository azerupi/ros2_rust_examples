/// Demonstrates `MandatoryParameter::subscribe()` for async parameter watching.
///
/// The node declares a "speed" parameter and spawns an async task that
/// reacts to every change. It also shows `wait_for()` to block until the
/// parameter reaches a threshold.
///
/// Try it:
///   $ ros2 run examples_rclrs_parameter_callback_demo subscribe_demo
///
///   # In another terminal:
///   $ ros2 param set /subscribe_demo speed 20
///   $ ros2 param set /subscribe_demo speed 75
///   $ ros2 param set /subscribe_demo speed 100   # triggers the wait_for threshold
use rclrs::*;

fn main() -> Result<(), RclrsError> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut executor = Context::default_from_env()?.create_basic_executor();
    let node = executor.create_node("subscribe_demo")?;

    let speed = node
        .declare_parameter("speed")
        .default(10i64)
        .mandatory()?;

    // Subscriber 1: log every change
    let mut changes_sub = speed.subscribe();
    runtime.spawn(async move {
        while let Some(value) = changes_sub.changed().await {
            println!("[subscriber] speed changed to {value}");
        }
        println!("[subscriber] parameter was dropped, shutting down");
    });

    // Subscriber 2: wait for a specific condition
    let mut threshold_sub = speed.subscribe();
    let _threshold_handle = runtime.spawn(async move {
        println!("[wait_for] waiting for speed >= 100 ...");
        match threshold_sub.wait_for(|v| *v >= 100).await {
            Some(value) => println!("[wait_for] speed reached {value}, threshold met!"),
            None => println!("[wait_for] parameter was dropped before threshold was met"),
        }
    });

    println!(
        "speed = {} (initial)\n\n\
        Node is spinning. Change the parameter from another terminal:\n  \
        $ ros2 param set /subscribe_demo speed 20\n  \
        $ ros2 param set /subscribe_demo speed 75\n  \
        $ ros2 param set /subscribe_demo speed 100  # triggers the wait_for threshold\n",
        speed.get(),
    );

    // Spin the executor on the current thread.
    // The async subscribers run on the tokio runtime in the background.
    executor.spin(SpinOptions::default()).first_error()
}
