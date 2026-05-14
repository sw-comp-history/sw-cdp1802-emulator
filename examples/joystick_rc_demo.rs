//! Runnable CDP1802 joystick resistor-capacitor timing demo.

use std::env;
use std::io::{self, Write};

use sw_cdp1802_asm::{assemble, assemble_intel_hex, assemble_listing};
use sw_cdp1802_emulator::{
    CpuState, JoystickRcBoard, Memory, VIDEO_HEIGHT, VIDEO_SIZE_BYTES, VIDEO_WIDTH, VideoView,
    format_cpu_state, format_hex_dump, step_with_joystick,
};

pub const MAX_STEPS: u64 = 500;
pub const JOYSTICK_VIDEO_BASE: u16 = 0x0000;
const SET_PIXEL: char = '█';
const CLEAR_PIXEL: char = ' ';

pub const DEMO_SOURCE: &str = include_str!("asm/joystick_rc_demo.s");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoOptions {
    pub once: Option<(u8, u8)>,
    pub show_source: bool,
    pub show_listing: bool,
    pub show_hex: bool,
}

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
    let steps = run_one_loop(&mut state, &mut memory, &mut board);

    JoystickFrame {
        steps,
        state,
        memory,
        board,
    }
}

fn run_one_loop(state: &mut CpuState, memory: &mut Memory, board: &mut JoystickRcBoard) -> u64 {
    let start = state.instr_count;
    while state.instr_count - start < MAX_STEPS {
        step_with_joystick(state, memory, Some(board)).expect("step joystick demo");
        if state.instr_count > start && state.pc() == 0 {
            return state.instr_count - start;
        }
    }
    panic!("joystick demo did not complete one loop in {MAX_STEPS} instructions");
}

pub fn render_solid_video(memory: &Memory) -> String {
    VideoView::new(JOYSTICK_VIDEO_BASE)
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
    let options = parse_args(&args[1..]).unwrap_or_else(|message| {
        eprintln!("{message}");
        eprintln!("usage: joystick_rc_demo [--source] [--listing] [--hex] [--once X Y]");
        std::process::exit(2);
    });
    if let Some((x, y)) = options.once {
        print_assembly_artifacts(&options);
        run_and_print(x, y);
        return;
    }

    println!("=== CDP1802 joystick RC timing demo ===");
    print_assembly_artifacts(&options);
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

pub fn parse_args(args: &[String]) -> Result<DemoOptions, String> {
    let mut options = DemoOptions {
        once: None,
        show_source: false,
        show_listing: false,
        show_hex: false,
    };

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--source" => options.show_source = true,
            "--listing" => options.show_listing = true,
            "--hex" => options.show_hex = true,
            "--once" => {
                if options.once.is_some() {
                    return Err("--once may only be provided once".to_string());
                }
                let x = args
                    .get(i + 1)
                    .ok_or_else(|| "--once requires X and Y axis values".to_string())
                    .and_then(|s| parse_axis_arg(s))?;
                let y = args
                    .get(i + 2)
                    .ok_or_else(|| "--once requires X and Y axis values".to_string())
                    .and_then(|s| parse_axis_arg(s))?;
                options.once = Some((x, y));
                i += 2;
            }
            other => return Err(format!("unknown argument `{other}`")),
        }
        i += 1;
    }

    Ok(options)
}

fn print_assembly_artifacts(options: &DemoOptions) {
    if options.show_source {
        println!("--- source ---");
        println!("{DEMO_SOURCE}");
    }
    if options.show_listing {
        println!("--- listing ---");
        print!(
            "{}",
            assemble_listing(DEMO_SOURCE).expect("assemble listing")
        );
    }
    if options.show_hex {
        println!("--- intel hex ---");
        print!(
            "{}",
            assemble_intel_hex(DEMO_SOURCE).expect("assemble Intel HEX")
        );
    }
    if options.show_source || options.show_listing || options.show_hex {
        println!();
    }
}

fn parse_axis_arg(s: &str) -> Result<u8, String> {
    s.parse::<u8>()
        .map_err(|_| format!("axis value must be 0..255, got `{s}`"))
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
    println!("--- final CPU state ---");
    print!("{}", format_cpu_state(&frame.state));
    println!(
        "ball bucket = ({x_bucket}, {y_bucket}); R1 = 0x{:04x}",
        frame.state.read_reg(1)
    );
    println!(
        "--- video RAM 0x{JOYSTICK_VIDEO_BASE:04x}..0x{:04x} ---",
        JOYSTICK_VIDEO_BASE + VIDEO_SIZE_BYTES as u16 - 1
    );
    print!(
        "{}",
        format_hex_dump(
            JOYSTICK_VIDEO_BASE,
            &frame
                .memory
                .read_range(JOYSTICK_VIDEO_BASE, VIDEO_SIZE_BYTES)
        )
    );
    println!("--- video {VIDEO_WIDTH}x{VIDEO_HEIGHT} @ 0x{JOYSTICK_VIDEO_BASE:04x} ---");
    println!("{}", render_solid_video(&frame.memory));
}
