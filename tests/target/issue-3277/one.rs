// rustfmt-version: One

// Original issue case
#![allow(
    clippy::needless_pass_by_value,
    clippy::new_ret_no_self,
    clippy::new_without_default_derive,
)]

// Different cases with commas between parenthesis
#[cfg(feature1 = "std1", feature2 = "std2",)]
#[cfg(all(feature1 = "std1", feature2 = "std2",))]
#[cfg(all(feature1 = "std1", feature2 = "std2",),)]
#[cfg(all(all(feature1 = "std1", feature2 = "std2",),),)]
#[cfg(all(all(feature1 = "std1", feature2 = "std2",)),)]
#[cfg(all(all(feature1 = "std1", feature2 = "std2",)))]
type Os = NoSource;

// Comma at the end of the last list items
#[live_prop_test(
    precondition = "inputs.input_flows.len() == self.num_inputs()",
    postcondition = "result.len() == self.num_outputs()",
    postcondition = "output_times_valid(inputs, &result)",
)]
fn output_flows(
    &self,
    inputs: MachineObservedInputs,
    future: &Self::Future,
) -> Inputs<Option<MaterialFlow>> {
    inputs![]
}

#[cfg(all(feature1 = "std1", feature2 = "std2",))]
#[cfg(all(
    feature1 = "long textttttttttttttttttttttttttttttttttttttttt",
    feature2 = "long textttttttttttttttttttttttttttttttttttttttt",
))]
#[cfg(not(all(
    feature1 = "long textttttttttttttttttttttttttttttttttttttttt",
    feature2 = "long textttttttttttttttttttttttttttttttttttttttt",
)))]
type Os = NoSource;

#[cfg(not(all(feature = "std", any(target_os = "linux", target_os = "android",))))]
#[cfg(not(all(
    feature = "std",
    any(
        target_os = "linuxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        target_os = "android",
    )
)))]
#[cfg(not(all(any(target_os = "linux", target_os = "android",), feature = "std",)))]
#[cfg(any(target_os = "linux", target_os = "android",))]
#[cfg(target_os = "linux", target_os = "android",)]
#[cfg(not(all(target_os = "linux", target_os = "android",), feature = "std",))]
type Os = NoSource;

#[cfg(any(
    target_os = "linux",
    all(target_os = "android", target_os = "windows",)
))]
#[cfg(any(
    target_os = "linux",
    all(target_os = "android", target_os = "windows",),
    target_os = "unix",
))]
type Os = NoSource;

// Tests with no comma at the end of the last list items
#[live_prop_test(
    precondition = "inputs.input_flows.len() == self.num_inputs()",
    postcondition = "result.len() == self.num_outputs()",
    postcondition = "output_times_valid(inputs, &result)"
)]
fn output_flows(
    &self,
    inputs: MachineObservedInputs,
    future: &Self::Future,
) -> Inputs<Option<MaterialFlow>> {
    inputs![]
}

#[cfg(all(
    feature1 = "long textttttttttttttttttttttttttttttttttttttttt",
    feature2 = "long textttttttttttttttttttttttttttttttttttttttt"
))]
#[cfg(not(all(
    feature1 = "long textttttttttttttttttttttttttttttttttttttttt",
    feature2 = "long textttttttttttttttttttttttttttttttttttttttt"
)))]
type Os = NoSource;

#[cfg(not(all(feature = "std", any(target_os = "linux", target_os = "android"))))]
#[cfg(not(all(any(target_os = "linux", target_os = "android"), feature = "std")))]
#[cfg(any(target_os = "linux", target_os = "android"))]
#[cfg(target_os = "linux", target_os = "android")]
type Os = NoSource;

#[cfg(any(target_os = "linux", all(target_os = "android", target_os = "windows")))]
#[cfg(any(
    target_os = "linux",
    all(target_os = "android", target_os = "windows"),
    target_os = "unix"
))]
type Os = NoSource;
