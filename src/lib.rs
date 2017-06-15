// Copyright 2017 CoreOS, Inc.
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(allocator)]
#![allocator]

#![no_std]

#![crate_name = "alloc_uefi"]
#![crate_type = "rlib"]

#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    internal_uefi::get_system_table().boot_services().allocate_pool(size)
}

#[no_mangle]
pub extern fn __rust_allocate_zeroed(size: usize, _align: usize) -> *mut u8 {
    let boot_services = internal_uefi::get_system_table().boot_services();
    let out = boot_services.allocate_pool(size);
    boot_services.set_mem(out, 0, size);
    out
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    internal_uefi::get_system_table().boot_services().free_pool(ptr);
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize, _align: usize) -> *mut u8 {
    let boot_services = internal_uefi::get_system_table().boot_services();
    let out = boot_services.allocate_pool(size);
    boot_services.copy_mem(out, ptr, old_size);
    boot_services.free_pool(ptr);
    out
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize, _size: usize, _align: usize) -> usize {
    old_size
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}

pub fn setup_alloc(system_table: *const internal_uefi::SystemTable, mem_type: internal_uefi::MemoryType) {
    internal_uefi::set_system_table(system_table);
    internal_uefi::set_mem_type(mem_type);
}

mod internal_uefi {
	mod table {
        #[repr(C)]
        pub struct TableHeader {
            signature: u64,
            revision: u32,
            header_size: u32,
            crc32: u32,
            reserved: u32
        }
    }
    pub enum CVoid {}
    enum NotYetDef {}

    pub static mut SYSTEM_TABLE: *const SystemTable = 0 as *const SystemTable;
    pub static mut MEM_TYPE: MemoryType = MemoryType::Conventional;

    pub fn set_system_table(table: *const SystemTable) {
        unsafe {
            SYSTEM_TABLE = table;
        }
    }

    pub fn get_system_table() -> &'static SystemTable {
        unsafe {
            &*SYSTEM_TABLE
        }
    }

    pub fn set_mem_type(m: MemoryType) {
        unsafe {
            MEM_TYPE = m;
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    #[allow(dead_code)]
    pub enum MemoryType {
        Reserved = 0,
        LoaderCode = 1,
        LoaderData = 2,
        BootServicesCode = 3,
        BootServicesData = 4,
        RuntimeServicesCode = 5,
        RuntimeServicesData = 6,
        Conventional = 7,
        Unusable = 8,
        AcpiReclaimed = 9,
        AcpiNvs = 10,
        MemoryMappedIo = 11,
        MemoryMappedIoPortSpace = 12,
        PalCode = 13,
    }

    #[repr(C)]
    pub struct SystemTable {
        header: table::TableHeader,
        vendor: *const u16,
        revision: u32,
        con_in_handle: *mut CVoid,
        con_in: *const CVoid,
        con_out_handle: *mut CVoid,
        con_out: *const CVoid,
        std_err_handle: *mut CVoid,
        std_err: *const CVoid,
        runtime_services: *const CVoid,
        boot_services: &'static BootServices
    }
    
    impl SystemTable {
        pub fn boot_services(&self) -> &'static BootServices {
            self.boot_services
        }
    }

    #[repr(C)]
    pub struct BootServices {
        header: table::TableHeader,
        raise_tpl: *const NotYetDef,
        restore_tpl: *const NotYetDef,
        allocate_pages: *const NotYetDef,
        free_pages: *const NotYetDef,
        get_memory_map: *const NotYetDef,
        allocate_pool: unsafe extern "win64" fn(pool_type: MemoryType, size: usize, out: *mut *mut u8),
        free_pool: unsafe extern "win64" fn(*mut CVoid),
        create_event: *const NotYetDef,
        set_timer: *const NotYetDef,
        wait_for_event: *const NotYetDef,
        signal_event: *const NotYetDef,
        close_event: *const NotYetDef,
        check_event: *const NotYetDef,
        install_protocol_interface: *const NotYetDef,
        reinstall_protocol_interface: *const NotYetDef,
        uninstall_protocol_interface: *const NotYetDef,
        handle_protocol: *const NotYetDef,
        __reserved: *const NotYetDef,
        register_protocol_notify: *const NotYetDef,
        locate_handle: *const NotYetDef,
        locate_device_path: *const NotYetDef,
        install_configuration_table: *const NotYetDef,
        load_image: *const NotYetDef,
        start_image: *const NotYetDef,
        exit: *const NotYetDef,
        unload_image: *const NotYetDef,
        exit_boot_services: *const NotYetDef,
        get_next_monotonic_count: *const NotYetDef,
        stall: *const NotYetDef,
        set_watchdog_timer: *const NotYetDef,
        connect_controller: *const NotYetDef,
        disconnect_controller: *const NotYetDef,
        open_protocol: *const NotYetDef,
        close_protocol: *const NotYetDef,
        open_protocol_information: *const NotYetDef,
        protocols_per_handle: *const NotYetDef,
        locate_handle_buffer: *const NotYetDef,
        locate_protocol: *const NotYetDef,
        install_multiple_protocol_interfaces: *const NotYetDef,
        uninstall_multiple_protocol_interfaces: *const NotYetDef,
        calculate_crc32: *const NotYetDef,
        copy_mem: unsafe extern "win64" fn(*mut CVoid, *mut CVoid, usize),
        set_mem: unsafe extern "win64" fn(*mut CVoid, usize, u8),
        create_event_ex: *const NotYetDef,
    }

    impl BootServices {
        pub fn allocate_pool(&self, size: usize) -> *mut u8 {
            let mut ptr: *mut u8 = 0 as *mut u8;
            unsafe { (self.allocate_pool)(MEM_TYPE, size, &mut ptr) };
            ptr
        }

        pub fn free_pool(&self, p: *const u8) {
            unsafe { (self.free_pool)(p as *mut CVoid) };
        }

        pub fn copy_mem(&self, dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
            unsafe { (self.copy_mem)(dest as *mut CVoid, src as *mut CVoid, n) };
            dest
        }

        pub fn set_mem(&self, s: *mut u8, c: u8, n: usize) -> *mut u8 {
            unsafe { (self.set_mem)(s as *mut CVoid, n, c) };
            s
        }
    }
}
