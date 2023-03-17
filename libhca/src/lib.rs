/*
Copyright 2023 The openBCE Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod pci;

/// HCA
pub struct HCA {
    pub description: String,
    pub serial_number: String,
    pub driver: String,
    pub functions: Vec<Function>,
}

/// Function is the interal concept for Node/Port on the host.
pub struct Function {
    pub guid: String,
    pub lid: i32,
}

/// List the HCAs on the host.
pub fn list_hca() -> Vec<HCA> {
    let mut hcas = vec![];

    //     unsafe {
    //         let hca_list = pci::get_hca_list();
    //
    //
    //         pci::free_hca_list(hca_list);
    //     }

    hcas
}
