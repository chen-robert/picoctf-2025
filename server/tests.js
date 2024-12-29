const { execFile } = require('child_process');
const { promisify } = require('util');
const execFileAsync = promisify(execFile);
const fs = require('fs').promises;
const assert = require('assert');
const { runCPU } = require('./cpu.js');

async function assembleProgram(program) {
    await fs.writeFile('temp.asm', program);
    await execFileAsync('../target/release/assembler', ['temp.asm', 'temp.bin']);
    const binary = await fs.readFile('temp.bin');
    await fs.unlink('temp.asm');
    await fs.unlink('temp.bin');
    return binary;
}

async function testFlag() {
    const program = `
        ; Test storing and loading from address 0
        LOADI r0 42     ; R0 = 42
        LOADI r1 0      ; R1 = 0 (address)
        STORE r1 r0     ; Store 42 at address 0
        LOAD r2 r1      ; Load from address 0 into R2
        
        ; Test storing and loading from address 0x100
        LOADW r1 0x100  ; R1 = 0x100 (address)
        LOADI r0 123    ; R0 = 123
        STORE r1 r0     ; Store 123 at address 0x100
        LOAD r3 r1      ; Load from address 0x100 into R3
        
        ; Test storing at max address 0xFFFF
        LOADW r1 0xFFFF ; R1 = 0xFFFF (max address)
        LOADI r0 255    ; R0 = 255
        STORE r1 r0     ; Store 255 at max address
        
        HLT             ; Stop execution
    `;

    const binary = await assembleProgram(program);
    console.log('Assembled binary:', [...binary]);
    
    const memory = new Uint8Array(65536);
    memory.set(binary);
    
    const flag = runCPU(memory);
    
    // Check memory values
    assert.strictEqual(memory[0], 42, 'Memory at address 0 should be 42');
    assert.strictEqual(memory[0x100], 123, 'Memory at address 0x100 should be 123');
    assert.strictEqual(memory[0xFFFF], 255, 'Memory at max address should be 255');
    
    
    console.log('Memory operations test passed!');
}

testFlag(); 