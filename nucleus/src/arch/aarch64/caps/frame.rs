/*
 * SPDX-License-Identifier: BlueOak-1.0.0
 * Copyright (c) Berkus Decker <berkus+vesper@metta.systems>
 */

use {
    crate::{
        capdef,
        caps::{CapError, Capability},
    },
    core::convert::TryFrom,
    paste::paste,
    register::{register_bitfields, LocalRegisterCopy},
};

//=====================
// Cap definition
//=====================

register_bitfields! {
    u128,
    FrameCap [
        Type OFFSET(0) NUMBITS(6) [
            value = 1
        ],
        Size OFFSET(6) NUMBITS(2) [],
        VMRights OFFSET(8) NUMBITS(2) [],
        IsDevice OFFSET(10) NUMBITS(1) [],
        BasePtr OFFSET(16) NUMBITS(48) [], // PhysAddr
        MappedAddress OFFSET(64) NUMBITS(48) [], // VirtAddr
        MappedASID OFFSET(112) NUMBITS(16) [],
    ]
}

capdef! { Frame }

//=====================
// Cap implementation
//=====================
