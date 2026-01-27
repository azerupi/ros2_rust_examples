//! Example: Parameter Change Notification Callbacks
//!
//! This example demonstrates how to use post-set parameter callbacks
//! to react to parameter changes, for example to log changes or
//! update internal state.
//!
//! Run this example:
//!   ros2 run examples_rclrs_parameter_callback_demo parameter_change_notification
//!
//! Try changing parameters and watch the notifications:
//!   ros2 param set parameter_change_notification speed 50.0
//!   ros2 param set parameter_change_notification mode "manual"
//!   ros2 param set parameter_change_notification debug_enabled true

use rclrs::*;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

fn main() -> Result<(), RclrsError> {
    let mut executor = Context::default_from_env()?.create_basic_executor();
    let node = executor.create_node("parameter_change_notification")?;

    // Declare some parameters
    let _speed = node
        .declare_parameter("speed")
        .default(10.0)
        .description("Current speed setting")
        .mandatory()?;

    let _mode: MandatoryParameter<Arc<str>> = node
        .declare_parameter("mode")
        .default("auto".into())
        .description("Operating mode: auto, manual, or emergency")
        .mandatory()?;

    let _debug_enabled = node
        .declare_parameter("debug_enabled")
        .default(false)
        .description("Whether debug logging is enabled")
        .mandatory()?;

    // Counter to track total parameter changes
    let change_count = Arc::new(AtomicU64::new(0));
    let change_count_for_callback = Arc::clone(&change_count);

    // Register a post-change callback that logs all parameter changes
    // The handle is kept alive for the duration of the program
    let _notification_handle =
        node.add_post_set_parameters_callback(move |name, old_value, new_value| {
            let count = change_count_for_callback.fetch_add(1, Ordering::SeqCst) + 1;

            println!(
                "[Change #{}] Parameter '{}' changed:",
                count, name
            );
            println!("    Old value: {:?}", old_value);
            println!("    New value: {:?}", new_value);
            println!();

            // Example: Take action based on specific parameter changes
            match name {
                "speed" => {
                    println!("    -> Updating motor controller with new speed");
                }
                "mode" => {
                    println!("    -> Switching operating mode");
                }
                "debug_enabled" => {
                    println!("    -> Toggling debug logging");
                }
                _ => {}
            }
        });

    // You can also register multiple callbacks
    // This one just counts changes silently
    let change_count_for_stats = Arc::clone(&change_count);
    let _stats_handle = node.add_post_set_parameters_callback(move |_name, _old, _new| {
        // In a real application, you might update metrics, write to a log file, etc.
        let total = change_count_for_stats.load(Ordering::SeqCst);
        if total > 0 && total % 5 == 0 {
            println!("=== Statistics: {} total parameter changes so far ===\n", total);
        }
    });

    println!("Parameter Change Notification Demo");
    println!("===================================\n");
    println!("This demo shows how post-change callbacks can react to parameter updates.\n");
    println!("Available parameters:");
    println!("  - speed (double): Current speed setting");
    println!("  - mode (string): Operating mode");
    println!("  - debug_enabled (bool): Debug logging toggle\n");
    println!("Try changing them:");
    println!("  ros2 param set parameter_change_notification speed 50.0");
    println!("  ros2 param set parameter_change_notification mode \"manual\"");
    println!("  ros2 param set parameter_change_notification debug_enabled true\n");
    println!("Node is spinning... Press Ctrl+C to exit.\n");

    executor.spin(SpinOptions::default()).first_error()
}
