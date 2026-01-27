//! Example: Parameter Validation Callbacks
//!
//! This example demonstrates how to use parameter validation callbacks
//! to enforce constraints on parameter values that go beyond simple ranges.
//!
//! Run this example:
//!   ros2 run examples_rclrs_parameter_callback_demo parameter_validation
//!
//! Try changing the parameters:
//!   ros2 param set parameter_validation max_speed 50.0    # Should succeed
//!   ros2 param set parameter_validation max_speed -10.0   # Should fail (negative)
//!   ros2 param set parameter_validation max_speed 5.0     # Should fail (too big a reduction)
//!
//!   ros2 param set parameter_validation robot_name "my_robot"  # Should succeed
//!   ros2 param set parameter_validation robot_name ""          # Should fail (empty)

use rclrs::*;

fn main() -> Result<(), RclrsError> {
    let mut executor = Context::default_from_env()?.create_basic_executor();
    let node = executor.create_node("parameter_validation")?;

    // Example 1: Per-parameter validation with on_validate()
    // This validates that speed can't be negative and can't be reduced by more than 50%
    let max_speed = node
        .declare_parameter("max_speed")
        .default(100.0)
        .description("Maximum robot speed in m/s")
        .on_validate(|old_value, new_value| {
            // Reject negative values
            if *new_value < 0.0 {
                return Err("Speed cannot be negative".into());
            }

            // Reject reductions of more than 50% (safety feature)
            if *new_value < *old_value * 0.5 {
                return Err(format!(
                    "Cannot reduce speed by more than 50% at once. \
                     Current: {}, requested: {}, minimum allowed: {}",
                    old_value,
                    new_value,
                    old_value * 0.5
                ));
            }

            Ok(())
        })
        .mandatory()?;

    // Example 2: Per-parameter validation for strings
    let robot_name = node
        .declare_parameter("robot_name")
        .default("default_robot".into())
        .description("Name of the robot")
        .on_validate(|_old, new_value: &std::sync::Arc<str>| {
            if new_value.is_empty() {
                return Err("Robot name cannot be empty".into());
            }
            if new_value.len() > 32 {
                return Err("Robot name cannot exceed 32 characters".into());
            }
            if !new_value.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Err("Robot name can only contain alphanumeric characters and underscores".into());
            }
            Ok(())
        })
        .mandatory()?;

    // Example 3: Node-level validation callback for cross-parameter constraints
    // This ensures that certain parameters are only modified when the robot is not active
    let is_active = node
        .declare_parameter("is_active")
        .default(false)
        .description("Whether the robot is currently active")
        .mandatory()?;

    let critical_param = node
        .declare_parameter("critical_param")
        .default(42)
        .description("A critical parameter that can only be changed when robot is inactive")
        .mandatory()?;

    // Keep a reference to prevent callback from being unregistered
    let is_active_clone = is_active.name().to_string();
    let critical_param_name = critical_param.name().to_string();
    let is_active_for_callback = node
        .declare_parameter::<bool>("is_active_shadow")
        .default(false)
        .optional()?;

    let _validation_handle = node.add_on_set_parameters_callback(move |name, _old, _new| {
        // Block changes to critical_param when robot is active
        if name == critical_param_name {
            if is_active_for_callback.get().unwrap_or(false) {
                return Err(format!(
                    "Cannot modify '{}' while robot is active. Set 'is_active' to false first.",
                    name
                ));
            }
        }
        Ok(())
    });

    println!("Parameter Validation Demo");
    println!("=========================\n");
    println!("Current parameters:");
    println!("  max_speed: {}", max_speed.get());
    println!("  robot_name: {}", robot_name.get());
    println!("  is_active: {}", is_active.get());
    println!("  critical_param: {}", critical_param.get());
    println!();
    println!("Try these commands to test validation:");
    println!("  ros2 param set parameter_validation max_speed 50.0    # OK");
    println!("  ros2 param set parameter_validation max_speed -10.0   # Fails: negative");
    println!("  ros2 param set parameter_validation max_speed 5.0     # Fails: >50% reduction");
    println!();
    println!("  ros2 param set parameter_validation robot_name \"my_robot\"  # OK");
    println!("  ros2 param set parameter_validation robot_name \"\"          # Fails: empty");
    println!();
    println!("Node is spinning... Press Ctrl+C to exit.\n");

    executor.spin(SpinOptions::default()).first_error()
}
