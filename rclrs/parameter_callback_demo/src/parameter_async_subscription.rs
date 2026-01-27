//! Example: Async Parameter Subscription
//!
//! This example demonstrates how to use the async subscription feature
//! to react to parameter changes in async code. This is particularly
//! useful when you need to perform async operations in response to
//! parameter changes.
//!
//! Run this example:
//!   ros2 run examples_rclrs_parameter_callback_demo parameter_async_subscription
//!
//! Try changing the parameter:
//!   ros2 param set parameter_async_subscription target_position 100.0
//!   ros2 param set parameter_async_subscription target_position 50.0

use rclrs::*;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), RclrsError> {
    let context = Context::default_from_env()?;
    let executor = context.create_basic_executor();
    let node = executor.create_node("parameter_async_subscription")?;

    // Declare a parameter that we'll subscribe to
    let target_position = Arc::new(
        node.declare_parameter("target_position")
            .default(0.0)
            .description("Target position for the robot to move to")
            .mandatory()?,
    );

    // Clone for the async task
    let target_position_for_task = Arc::clone(&target_position);

    // Spawn an async task that reacts to parameter changes
    let subscriber_task = tokio::spawn(async move {
        let mut subscription = target_position_for_task.subscribe();

        println!("Async subscriber started. Waiting for parameter changes...\n");

        loop {
            // Wait for the parameter to change
            match subscription.changed().await {
                Some(new_value) => {
                    println!("Received new target_position: {}", new_value);
                    println!("  -> Starting simulated async move operation...");

                    // Simulate an async operation (e.g., commanding a motor)
                    // In a real application, this could be an async I/O operation
                    for progress in [25, 50, 75, 100] {
                        sleep(Duration::from_millis(250)).await;
                        println!("  -> Move progress: {}%", progress);
                    }

                    println!("  -> Move complete! Now at position {}\n", new_value);
                }
                None => {
                    println!("Parameter was dropped, exiting subscriber task");
                    break;
                }
            }
        }
    });

    println!("Async Parameter Subscription Demo");
    println!("==================================\n");
    println!("This demo shows how to use async subscriptions to react to parameter changes.\n");
    println!("The async subscriber will simulate moving to a new position when the");
    println!("'target_position' parameter changes.\n");
    println!("Current target_position: {}\n", target_position.get());
    println!("Try changing it:");
    println!("  ros2 param set parameter_async_subscription target_position 100.0");
    println!("  ros2 param set parameter_async_subscription target_position 50.0\n");
    println!("Watch the async move operation execute in response to changes.\n");
    println!("Node is spinning... Press Ctrl+C to exit.\n");

    // Spin the executor in another task
    let spin_task = tokio::spawn(async move {
        // Use spin_async to run the executor without blocking
        let (_, errors) = executor.spin_async(SpinOptions::default()).await;
        if let Some(error) = errors.into_iter().next() {
            eprintln!("Executor error: {:?}", error);
        }
    });

    // Wait for both tasks (they run until Ctrl+C)
    tokio::select! {
        _ = subscriber_task => {},
        _ = spin_task => {},
    }

    Ok(())
}
