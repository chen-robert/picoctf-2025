module rom (
    input wire [3:0] addr,  // 4-bit address
    output reg [7:0] data   // 8-bit data output
);
    // ROM memory array
    reg [7:0] memory [15:0]; // 16x8 ROM

    // Initialize ROM contents
    initial begin
        memory[0] = 8'h00;
        memory[1] = 8'h11;
        memory[2] = 8'h22;
        memory[3] = 8'h33;
        memory[4] = 8'h44;
        memory[5] = 8'h55;
        memory[6] = 8'h66;
        memory[7] = 8'h77;
        memory[8] = 8'h88;
        memory[9] = 8'h99;
        memory[10] = 8'hAA;
        memory[11] = 8'hBB;
        memory[12] = 8'hCC;
        memory[13] = 8'hDD;
        memory[14] = 8'hEE;
        memory[15] = 8'hFF;
    end

    // Output data based on address
    always @(*) begin
        data = memory[addr];
    end
endmodule
