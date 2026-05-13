Saga: demo output cleanup

Step 1: complete-post-demo-dumps
Goal: improve every runnable demo post-run section so it clearly shows a complete CPU state dump with all modeled state and R0..RF, plus relevant memory buffers. Ensure the output labels say CPU state, not just registers, and add/adjust regression tests where practical.

Step 2: joystick-demo-output-cleanup
Goal: improve the joystick RC demo output for human demo use. Hide Intel HEX by default or move it behind an explicit flag, make the full CPU state plainly visible, add a raw video RAM dump for 0x2000..0x20ff or the relevant frame slice, and keep the rendered 64x32 grid. Update README/docs with the new flags and expected output shape.