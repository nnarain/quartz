extern crate libchip8;

use libchip8::*;

/// Helper function to help test
fn run(vm: &mut Chip8, memory: Vec<u8>, should_panic: bool) {
    vm.load_memory(memory);

    loop {
        match vm.step(1) {
            Ok(_) => continue,
            Err(e) => {
                if should_panic {
                    panic!("{:?}", e)
                }
                else {
                    break;
                }
            }
        }
    }
}

#[test]
#[should_panic]
fn test_invalid_opcode() {
    let mut vm = Chip8::new();

    let program = vec![
        0xFF,
        0xFF
    ];

    run(&mut vm, program, true);
}

#[test]
fn test_jump() {
    let mut vm = Chip8::new();

    let program = vec![
        0x14, 0x50,
        0xFF,
        0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_pc(), 0x452u16);
}

#[test]
fn test_call() {
    let mut vm = Chip8::new();

    let program = vec![
        0x22, 0x50,
        0xFF,
        0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_pc(), 0x252u16);
    assert_eq!(vm.get_sp(), 1);
    assert_eq!(vm.get_stack(0), 0x0202);
}

#[test]
fn test_load_vx() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0x01, // LD V0, 0x01
        0x61, 0x02, // LD V1, 0x02
        0x62, 0x03, // LD V2, 0x03
        0x63, 0x04, // LD V3, 0x04
        0xFF,       // stop
        0xFF        //
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(0), 1);
    assert_eq!(vm.get_register(1), 2);
    assert_eq!(vm.get_register(2), 3);
    assert_eq!(vm.get_register(3), 4);
}

#[test]
fn test_load_vx_vy() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0x01, // LD V0, 0x01
        0x61, 0x02, // LD V1, 0x02
        0x80, 0x10, // LD V0, V1
        0xFF,       // stop
        0xFF        //
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(0), 2);
    assert_eq!(vm.get_register(1), 2);
}

#[test]
fn test_skip_if_vx_equals_kk() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0xDE, // LD V0, $DE
        0x61, 0xAD, // LD V1, $AD
        0x30, 0xDE, // SE V0, $DE
        0x60, 0xFF, // LD V0, $FF ; should skip
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_pc(), 0x20A);
    assert_eq!(vm.get_register(0), 0xDE);
    assert_eq!(vm.get_register(1), 0xAD);
}

#[test]
fn test_skip_if_vx_equals_vy() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0xDE, // LD V0, $DE
        0x61, 0xDE, // LD V1, $DE
        0x50, 0x1E, // SE V0, V1
        0x60, 0xFF, // LD V0, $FF ; should skip
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_pc(), 0x20A);
    assert_eq!(vm.get_register(0), 0xDE);
    assert_eq!(vm.get_register(1), 0xDE);
}

#[test]
fn test_skip_if_vx_not_equals_kk() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0xDE, // LD V0, $DE
        0x40, 0x1E, // SNE V0, $1E
        0x60, 0xFF, // LD V0, $FF ; should skip
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_pc(), 0x208);
    assert_eq!(vm.get_register(0), 0xDE);
}

#[test]
fn test_skip_if_vx_not_equals_vy() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0xF0, // LD V0, $F0
        0x61, 0x0F, // LD V0, $0F
        0x90, 0x10, // SNE V0, VY
        0x60, 0x00,
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_pc(), 0x20A);
}

#[test]
fn test_add_vx_kk() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0x05, // LD V0, $05
        0x70, 0x05, // SNE V0, $1E
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(0), 0x0A);
}

#[test]
fn test_or_vx_vy() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0xF0, // LD V0, $05
        0x61, 0x0F, // LD V0, $05
        0x80, 0x11, // OR V0, VY
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(0), 0xFF);
}

#[test]
fn test_and_vx_vy() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0xF0, // LD V0, $F0
        0x61, 0x0F, // LD V0, $0F
        0x80, 0x12, // AND V0, VY
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(0), 0x00);
}

#[test]
fn test_xor_vx_vy() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0x66, // LD V0, $66
        0x61, 0xFF, // LD V0, $FF
        0x80, 0x13, // XOR V0, VY
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(0), 0x99);
}

#[test]
fn test_addc_vx_vy() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0xFF, // LD V0, $FF
        0x61, 0x01, // LD V0, $01
        0x80, 0x14, // ADDC V0, V1
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(0), 0x00);
    assert_eq!(vm.get_register(15), 1);
}

#[test]
fn test_sub_vx_vy() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0x04, // LD V0, $04
        0x61, 0x05, // LD V0, $05
        0x80, 0x15, // SUB V0, V1
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(0), 0xFF);
    assert_eq!(vm.get_register(15), 0);
}

#[test]
fn test_subn_vx_vy() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0x04, // LD V0, $04
        0x61, 0x05, // LD V0, $05
        0x80, 0x17, // SUB V0, V1
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(0), 1);
    assert_eq!(vm.get_register(15), 1);
}

#[test]
fn test_shl_vx() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0x81, // LD V0, $81
        0x80, 0x0E, // SHL V0
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(0), 0x2);
    assert_eq!(vm.get_register(15), 1);
}

#[test]
fn test_shr_vx() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0x05, // LD V0, $05
        0x80, 0x06, // SHL V0
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(0), 0x2);
    assert_eq!(vm.get_register(15), 1);
}

#[test]
fn test_load_i() {
    let mut vm = Chip8::new();

    let program = vec![
        0xAF, 0xFF, // LD I, $FFF
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_i(), 0xFFF);
}

#[test]
fn test_jump_relative() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0x01, // LD V0, $01
        0xB2, 0x50, // JR $250
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_pc(), 0x253);
}

#[test]
fn test_skip_if_key_pressed() {
    let mut vm = Chip8::new();
    vm.key(0, true);

    let program = vec![
        0x60, 0x00, // LD V0, $00
        0x61, 0x01, // LD V1, $01
        0xE0, 0x9E, // SKP V0
        0x61, 0x04, // LD V1, $04; skipped
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(1), 1);
}

#[test]
fn test_skip_if_key_not_pressed() {
    let mut vm = Chip8::new();
    vm.key(0, false);

    let program = vec![
        0x60, 0x00, // LD V0, $00
        0x61, 0x01, // LD V1, $01
        0xE0, 0xA1, // SKNP V0
        0x61, 0x04, // LD V1, $04; skipped
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(1), 1);
}

#[test]
fn test_key_wait() {
    let mut vm = Chip8::new();
    vm.set_key_wait(Box::new(|| {
        4
    }));

    let program = vec![
        0x60, 0x00, // LD V0, $00
        0xF0, 0x0A, // LD Vx, K
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_register(0), 4);
}

#[test]
fn test_ld_f() {
    let mut vm = Chip8::new();

    let program = vec![
        0x60, 0x04, // LD V0, $00
        0xF0, 0x29, // LD F, V0
        0xFF, 0xFF  // stop
    ];

    run(&mut vm, program, false);

    assert_eq!(vm.get_i(), 20);
}
