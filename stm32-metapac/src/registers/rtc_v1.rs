
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Rtc",
        extends: None,
        description: Some("Real-time clock"),
        items: &[
            BlockItem {
                name: "crh",
                description: Some("Control Register High"),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Crh"),
                }),
            },
            BlockItem {
                name: "crl",
                description: Some("Control Register Low"),
                array: None,
                byte_offset: 4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Crl"),
                }),
            },
            BlockItem {
                name: "prlh",
                description: Some("Prescaler Load Register High"),
                array: None,
                byte_offset: 8,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Prlh"),
                }),
            },
            BlockItem {
                name: "prll",
                description: Some("Prescaler Load Register Low"),
                array: None,
                byte_offset: 12,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Prll"),
                }),
            },
            BlockItem {
                name: "divh",
                description: Some("Prescaler Divider Register High"),
                array: None,
                byte_offset: 16,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Divh"),
                }),
            },
            BlockItem {
                name: "divl",
                description: Some("Prescaler Divider Register Low"),
                array: None,
                byte_offset: 20,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Divl"),
                }),
            },
            BlockItem {
                name: "cnth",
                description: Some("Counter Register High"),
                array: None,
                byte_offset: 24,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cnth"),
                }),
            },
            BlockItem {
                name: "cntl",
                description: Some("Counter Register Low"),
                array: None,
                byte_offset: 28,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cntl"),
                }),
            },
            BlockItem {
                name: "alrh",
                description: Some("Alarm Register High"),
                array: None,
                byte_offset: 32,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Alrh"),
                }),
            },
            BlockItem {
                name: "alrl",
                description: Some("Alarm Register Low"),
                array: None,
                byte_offset: 36,
                inner: BlockItemInner::Register(Register {
                    access: Access::Write,
                    bit_size: 32,
                    fieldset: Some("Alrl"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Alrh",
            extends: None,
            description: Some("Alarm Register High"),
            bit_size: 32,
            fields: &[Field {
                name: "alrh",
                description: Some("Alarm register high"),
                bit_offset: 0,
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Alrl",
            extends: None,
            description: Some("Alarm Register Low"),
            bit_size: 32,
            fields: &[Field {
                name: "alrl",
                description: Some("Alarm register low"),
                bit_offset: 0,
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Cnth",
            extends: None,
            description: Some("Counter Register High"),
            bit_size: 32,
            fields: &[Field {
                name: "cnth",
                description: Some("Counter register high"),
                bit_offset: 0,
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Cntl",
            extends: None,
            description: Some("Counter Register Low"),
            bit_size: 32,
            fields: &[Field {
                name: "cntl",
                description: Some("Counter register low"),
                bit_offset: 0,
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Crh",
            extends: None,
            description: Some("Control Register High"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secie",
                    description: Some("Second interrupt enable"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "alrie",
                    description: Some("Alarm interrupt enable"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "owie",
                    description: Some("Overflow interrupt enable"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Crl",
            extends: None,
            description: Some("Control Register Low"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secf",
                    description: Some("Second flag"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "alrf",
                    description: Some("Alarm flag"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "owf",
                    description: Some("Overflow flag"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsf",
                    description: Some("Registers synchronized flag"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnf",
                    description: Some("Enter configuration mode"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtoff",
                    description: Some("RTC operation OFF"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Rtoff"),
                },
            ],
        },
        FieldSet {
            name: "Divh",
            extends: None,
            description: Some("Prescaler Divider Register High"),
            bit_size: 32,
            fields: &[Field {
                name: "divh",
                description: Some("Prescaler divider register high"),
                bit_offset: 0,
                bit_size: 4,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Divl",
            extends: None,
            description: Some("Prescaler Divider Register Low"),
            bit_size: 32,
            fields: &[Field {
                name: "divl",
                description: Some("Prescaler divider register low"),
                bit_offset: 0,
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Prlh",
            extends: None,
            description: Some("Prescaler Load Register High"),
            bit_size: 32,
            fields: &[Field {
                name: "prlh",
                description: Some("Prescaler load register high"),
                bit_offset: 0,
                bit_size: 4,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Prll",
            extends: None,
            description: Some("Prescaler Load Register Low"),
            bit_size: 32,
            fields: &[Field {
                name: "prll",
                description: Some("Prescaler divider register low"),
                bit_offset: 0,
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
    ],
    enums: &[Enum {
        name: "Rtoff",
        description: None,
        bit_size: 1,
        variants: &[
            EnumVariant {
                name: "ONGOING",
                description: Some("Last write operation on RTC registers is still ongoing"),
                value: 0,
            },
            EnumVariant {
                name: "TERMINATED",
                description: Some("Last write operation on RTC registers terminated"),
                value: 1,
            },
        ],
    }],
};
