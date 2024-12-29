build:
	cd ./verilog && yosys ./synth_cpu.ys
	cargo test 
	cargo r --release --bin assembler programs/nand_checker.asm server/programs/nand_checker.bin
	cargo r --release --bin assembler programs/flag.asm server/programs/flag.bin

	cd ./server/wasm && wasm-pack build --target nodejs

	./create_server_archive.sh
	rm -rf ./archive-tmp
	mkdir ./archive-tmp
	cp ./server.tar.gz ./archive-tmp/server.tar.gz
	cd ./archive-tmp && tar xzf server.tar.gz


	docker build -f server/Dockerfile . -q | xargs -I {} docker run --rm -d --network host {} > .docker_id
	sleep 1
	cargo r --release --bin verilog-ctf || true && \
	./test_circuits.sh && \
	docker stop $$(cat .docker_id)
	rm -f .docker_id