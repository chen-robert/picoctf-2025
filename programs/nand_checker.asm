LOADW r4 0x3000 ; circuit base
LOADW r5 0x1000 ; expected output base
LOADW r6 0x2000 ; circuit state

LOADI r0 0
ADD r0 r4
LOADW r2 0x1000

; IMPORTANT: assumes that the input ends with a 0 and no input past that

check_start:
    LOAD r1 r0
    ADDI r0 2

    JZ r1 start

    GT r1 r2 r1
    JZ r1 end

    LOADI r1 0
    JZ r1 check_start

start:
    LOAD r0 r4
    ADDI r4 2
    LOAD r1 r4
    ADDI r4 2
    LOAD r2 r4
    ADDI r4 2

    ; if (r0 == 0 || r1 == 0 || r2 == 0) jmp end
    JZ r0 end
    JZ r1 end
    JZ r2 end

    ; double them
    ADD r0 r0
    ADD r1 r1
    ADD r2 r2

    ; mem[r2] = nand(mem[r0], mem[r1])
    ADD r0 r6
    ADD r1 r6
    ADD r2 r6

    LOAD r0 r0 
    LOAD r1 r1 

    NAND r0 r1

    STORE r2 r0

    LOADI r7 0 
    JZ r7 start

end:
    LOAD r0 r5
    
    LOADW r1 0xffff
    LOADI r2 2
    LOADI r7 0


finish_start:
    ADD r5 r2
    ADD r6 r2

    LOAD r3 r5
    LOAD r4 r6

    GT r3 r3 r7
    GT r4 r4 r7

    ADD r3 r4 ; 0 or 2 -> jump to check_nxt
    JZ r3 check_nxt

    GT r3 r2 r3 ; if !(2 > r3) -> jump to check_nxt
    JZ r3 check_nxt
    JZ r7 lose

check_nxt:
    ADD r0 r1
    JZ r0 win
    JZ r7 finish_start

lose:
    LOADW r0 0x3333
    LOADW r5 0x1000 ; expected output base
    STORE r5 r0
    HLT
win:
    LOADW r0 0x1337
    LOADW r5 0x1000 ; expected output base
    STORE r5 r0
    HLT 