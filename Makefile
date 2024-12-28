build:
	cd ./verilog && yosys ./synth_cpu.ys
	cargo test 
	cargo r

