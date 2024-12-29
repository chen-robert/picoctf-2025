build:
	cd ./verilog && yosys ./synth_cpu.ys
	cargo test 
	cargo r --release --bin verilog-ctf
	cargo r --release --bin assembler programs/nand_checker.asm server/programs/nand_checker.bin
	cargo r --release --bin assembler programs/flag.asm server/programs/flag.bin

	cd ./server/wasm && wasm-pack build --target nodejs

