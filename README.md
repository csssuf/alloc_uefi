# alloc\_uefi
Rust allocator for UEFI environments.

## Usage
Add alloc\_uefi as a dependency, and provide the following function as your
application's entry point:
```rust
pub extern fn efi_main(sys_table: *const internal_uefi::SystemTable, image_handle: *mut internal_uefi::CVoid) -> isize;
```
