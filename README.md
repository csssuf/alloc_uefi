# alloc\_uefi
Rust allocator for UEFI environments.

## Usage
Add alloc\_uefi as a dependency, and call the following function as part of your
application's UEFI initialization:
```rust
pub fn setup_alloc(system_table: *const internal_uefi::SystemTable, mem_type: MemoryType)
```

The `mem_type` argument should be whatever the ImageDataType value is of the
LoadedImageProtocol struct from your application's image handle.
