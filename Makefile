build:
	cd ./verilog/OnlyNandYosysSynth/OnlyNandYosysSynth/ && yosys ./synth_counter.ys && cp tmp.json ../../output.json
	cargo test 
	cargo r

