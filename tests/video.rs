use sw_cdp1802_emulator::{
    Memory, VIDEO_BASE, VIDEO_HEIGHT, VIDEO_SIZE_BYTES, VIDEO_WIDTH, VideoView,
};

#[test]
fn elf_video_view_has_documented_geometry() {
    let view = VideoView::elf_64x32();
    assert_eq!(view.base(), VIDEO_BASE);
    assert_eq!(view.width(), VIDEO_WIDTH);
    assert_eq!(view.height(), VIDEO_HEIGHT);
    assert_eq!(view.size_bytes(), VIDEO_SIZE_BYTES);
    assert_eq!(VIDEO_SIZE_BYTES, 256);
}

#[test]
fn pixel_mapping_is_msb_first_across_rows() {
    let mut mem = Memory::default();
    mem.write_byte(VIDEO_BASE, 0b1000_0001);
    mem.write_byte(VIDEO_BASE + 7, 0b0000_0001);
    mem.write_byte(VIDEO_BASE + 8, 0b0100_0000);
    let view = VideoView::elf_64x32();

    assert_eq!(view.pixel(&mem, 0, 0), Some(true));
    assert_eq!(view.pixel(&mem, 1, 0), Some(false));
    assert_eq!(view.pixel(&mem, 7, 0), Some(true));
    assert_eq!(view.pixel(&mem, 63, 0), Some(true));
    assert_eq!(view.pixel(&mem, 0, 1), Some(false));
    assert_eq!(view.pixel(&mem, 1, 1), Some(true));
    assert_eq!(view.pixel(&mem, 64, 0), None);
    assert_eq!(view.pixel(&mem, 0, 32), None);
}

#[test]
fn render_text_uses_hash_for_set_pixels_and_dot_for_clear_pixels() {
    let mut mem = Memory::default();
    mem.write_byte(VIDEO_BASE, 0b1010_0000);
    mem.write_byte(VIDEO_BASE + 8, 0b0000_0001);
    let rendered = VideoView::elf_64x32().render_text(&mem);
    let lines: Vec<&str> = rendered.lines().collect();

    assert_eq!(lines.len(), 32);
    assert_eq!(lines[0].len(), 64);
    assert_eq!(&lines[0][..8], "#.#.....");
    assert_eq!(&lines[1][..8], ".......#");
    assert_eq!(&lines[1][56..], "........");
    assert!(lines[2..].iter().all(|line| *line == ".".repeat(64)));
}

#[test]
fn render_text_has_no_trailing_newline() {
    let mem = Memory::default();
    let rendered = VideoView::elf_64x32().render_text(&mem);

    assert!(!rendered.ends_with('\n'));
    assert_eq!(rendered.matches('\n').count(), VIDEO_HEIGHT - 1);
}
