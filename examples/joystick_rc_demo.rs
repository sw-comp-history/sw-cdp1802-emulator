//! Runnable CDP1802 joystick resistor-capacitor timing demo.

use std::env;
use std::io::{self, Write};

use sw_cdp1802_asm::assemble;
use sw_cdp1802_emulator::{
    CpuState, JoystickRcBoard, Memory, VIDEO_BASE, VIDEO_HEIGHT, VIDEO_WIDTH, VideoView,
    run_with_joystick,
};

pub const MAX_STEPS: u64 = 500;
const SET_PIXEL: char = '█';
const CLEAR_PIXEL: char = ' ';

pub const DEMO_SOURCE: &str = r#"
        ORG 0x0000
        OUT 3
        B4 Y0
        B4 Y1
        B4 Y2
        BR Y3

Y0:     OUT 2
        B4 Y0X0
        B4 Y0X1
        B4 Y0X2
        BR Y0X3
Y1:     OUT 2
        B4 Y1X0
        B4 Y1X1
        B4 Y1X2
        BR Y1X3
Y2:     OUT 2
        B4 Y2X0
        B4 Y2X1
        B4 Y2X2
        BR Y2X3
Y3:     OUT 2
        B4 Y3X0
        B4 Y3X1
        B4 Y3X2
        BR Y3X3

Y0X0:   LDI 0x20
        PHI R1
        LDI 0x00
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y0X1:   LDI 0x20
        PHI R1
        LDI 0x02
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y0X2:   LDI 0x20
        PHI R1
        LDI 0x04
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y0X3:   LDI 0x20
        PHI R1
        LDI 0x06
        PLO R1
        LDI 0x80
        STR R1
        BR DONE

Y1X0:   LDI 0x20
        PHI R1
        LDI 0x40
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y1X1:   LDI 0x20
        PHI R1
        LDI 0x42
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y1X2:   LDI 0x20
        PHI R1
        LDI 0x44
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y1X3:   LDI 0x20
        PHI R1
        LDI 0x46
        PLO R1
        LDI 0x80
        STR R1
        BR DONE

Y2X0:   LDI 0x20
        PHI R1
        LDI 0x80
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y2X1:   LDI 0x20
        PHI R1
        LDI 0x82
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y2X2:   LDI 0x20
        PHI R1
        LDI 0x84
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y2X3:   LDI 0x20
        PHI R1
        LDI 0x86
        PLO R1
        LDI 0x80
        STR R1
        BR DONE

Y3X0:   LDI 0x20
        PHI R1
        LDI 0xc0
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y3X1:   LDI 0x20
        PHI R1
        LDI 0xc2
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y3X2:   LDI 0x20
        PHI R1
        LDI 0xc4
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y3X3:   LDI 0x20
        PHI R1
        LDI 0xc6
        PLO R1
        LDI 0x80
        STR R1

DONE:   IDL
"#;

pub struct JoystickFrame {
    pub steps: u64,
    pub state: CpuState,
    pub memory: Memory,
    pub board: JoystickRcBoard,
}

pub fn run_frame(x: u8, y: u8) -> JoystickFrame {
    let asm = assemble(DEMO_SOURCE).expect("assemble joystick demo");
    let mut memory = Memory::default();
    memory.load_bytes(0, &asm.bytes);
    let mut state = CpuState::new();
    state.x = 15;
    let mut board = JoystickRcBoard::new(x, y);
    let steps = run_with_joystick(&mut state, &mut memory, &mut board, MAX_STEPS)
        .expect("run joystick demo");

    JoystickFrame {
        steps,
        state,
        memory,
        board,
    }
}

pub fn render_solid_video(memory: &Memory) -> String {
    VideoView::elf_64x32()
        .render_text(memory)
        .chars()
        .map(|c| match c {
            '#' => SET_PIXEL,
            '.' => CLEAR_PIXEL,
            other => other,
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 && args[1] == "--once" {
        let x = parse_axis_arg(&args[2]);
        let y = parse_axis_arg(&args[3]);
        run_and_print(x, y);
        return;
    }

    println!("=== CDP1802 joystick RC timing demo ===");
    println!("Enter joystick X/Y values from 0..255, or blank X to quit.");
    loop {
        let Some(x) = prompt_axis("X") else {
            break;
        };
        let Some(y) = prompt_axis("Y") else {
            break;
        };
        run_and_print(x, y);
    }
}

fn parse_axis_arg(s: &str) -> u8 {
    s.parse::<u8>()
        .unwrap_or_else(|_| panic!("axis value must be 0..255, got `{s}`"))
}

fn prompt_axis(name: &str) -> Option<u8> {
    print!("{name}> ");
    io::stdout().flush().expect("flush prompt");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read line");
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    match trimmed.parse::<u8>() {
        Ok(value) => Some(value),
        Err(_) => {
            println!("expected 0..255");
            prompt_axis(name)
        }
    }
}

fn run_and_print(x: u8, y: u8) {
    let asm = assemble(DEMO_SOURCE).expect("assemble joystick demo");
    let frame = run_frame(x, y);
    let x_bucket = frame
        .board
        .delay_for_axis(sw_cdp1802_emulator::JoystickAxis::X);
    let y_bucket = frame
        .board
        .delay_for_axis(sw_cdp1802_emulator::JoystickAxis::Y);

    println!("--- joystick ---");
    println!("x = {x} -> delay bucket {x_bucket}");
    println!("y = {y} -> delay bucket {y_bucket}");
    println!("--- assembled ({} bytes) ---", asm.bytes.len());
    println!(
        "--- ran {} instructions; halted = {} ---",
        frame.steps, frame.state.halted
    );
    println!(
        "ball bucket = ({x_bucket}, {y_bucket}); R1 = 0x{:04x}",
        frame.state.read_reg(1)
    );
    println!("--- video {VIDEO_WIDTH}x{VIDEO_HEIGHT} @ 0x{VIDEO_BASE:04x} ---");
    println!("{}", render_solid_video(&frame.memory));
}
