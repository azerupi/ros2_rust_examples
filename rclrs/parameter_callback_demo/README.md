# Parameter Callback Demo

This package contains examples demonstrating the parameter callback APIs in rclrs.

## Examples

### 1. Parameter Validation (`parameter_validation`)

Demonstrates how to use validation callbacks to enforce constraints on parameter values:

- **Per-parameter validation** using `on_validate()` in the parameter builder
- **Node-level validation** using `add_on_set_parameters_callback()`
- Rejecting invalid parameter changes with meaningful error messages

```bash
ros2 run examples_rclrs_parameter_callback_demo parameter_validation

# In another terminal, try these commands:
ros2 param set parameter_validation max_speed 50.0    # OK
ros2 param set parameter_validation max_speed -10.0   # Fails: negative
ros2 param set parameter_validation max_speed 5.0     # Fails: >50% reduction
ros2 param set parameter_validation robot_name ""     # Fails: empty name
```

### 2. Parameter Change Notification (`parameter_change_notification`)

Demonstrates how to use post-change callbacks to react to parameter updates:

- **Logging parameter changes** with old and new values
- **Multiple callbacks** on the same node
- **Taking action** based on specific parameter changes

```bash
ros2 run examples_rclrs_parameter_callback_demo parameter_change_notification

# In another terminal:
ros2 param set parameter_change_notification speed 50.0
ros2 param set parameter_change_notification mode "manual"
ros2 param set parameter_change_notification debug_enabled true
```

### 3. Async Parameter Subscription (`parameter_async_subscription`)

Demonstrates how to use async subscriptions to react to parameter changes:

- **Async/await pattern** for parameter change notification
- **Non-blocking** reaction to parameter updates
- **Integration with tokio** async runtime

```bash
ros2 run examples_rclrs_parameter_callback_demo parameter_async_subscription

# In another terminal:
ros2 param set parameter_async_subscription target_position 100.0
ros2 param set parameter_async_subscription target_position 50.0
```

## Key Concepts

### Validation Callbacks

Validation callbacks run **before** a parameter change is applied. They can accept or reject the change:

```rust
let param = node
    .declare_parameter("my_param")
    .default(10.0)
    .on_validate(|old_value, new_value| {
        if *new_value < 0.0 {
            Err("Value cannot be negative".into())
        } else {
            Ok(())
        }
    })
    .mandatory()?;
```

### Post-Change Callbacks

Post-change callbacks run **after** a parameter change is applied. They cannot reject changes but can react to them:

```rust
let handle = node.add_post_set_parameters_callback(|name, old, new| {
    println!("Parameter {} changed from {:?} to {:?}", name, old, new);
});
// Callback is automatically unregistered when `handle` is dropped
```

### Async Subscriptions

Async subscriptions provide a way to await parameter changes:

```rust
let mut subscription = param.subscribe();
while let Some(new_value) = subscription.changed().await {
    println!("New value: {}", new_value);
    // Perform async operations here
}
```

## Building

```bash
colcon build --packages-select examples_rclrs_parameter_callback_demo
source install/setup.bash
```
