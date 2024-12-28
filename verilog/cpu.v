module cpu(
	clock,
  addr,
  inp_val,
  out_val,
  program_counter,
  state,
  reset,
  write_enable,
  halted,
  flag
);

input clock, reset;
input [15:0] inp_val;
output reg [15:0] addr;
output reg [15:0] out_val;
output reg write_enable;
output reg state;

output reg halted, flag;

// Registers
output reg [15:0] program_counter;		// Program Counter

// Internal buses
wire [15:0] instruction = inp_val;  // Connect instruction to input
wire [3:0] opcode	   = instruction[3:0];
wire [2:0] reg_dest    = instruction[6:4];  // Changed to 4 bits for 16 registers
wire [2:0] reg_src     = instruction[10:8];  // Changed to 4 bits for 16 registers
wire [7:0] immediate   = instruction[15:8];

wire [2:0] reg_src2   = instruction[14:12]; // Source register 2 for GT

// Register file
reg [15:0] registers [0:7]; // Changed to 16 16-bit registers
reg [2:0] load_dest;        // Changed to 4 bits for LOAD instruction destination register
reg should_load;            // Flag to track if we should load in state 2

// Initialize registers to 0
initial begin
    registers[0] <= 0;
    registers[1] <= 0; 
    registers[2] <= 0;
    registers[3] <= 0;
    registers[4] <= 0;
    registers[5] <= 0;
    registers[6] <= 0;
    registers[7] <= 0;

    load_dest <= 0;
    should_load <= 0;
    halted <= 0;
    flag <= 0;
    out_val <= 0;
    state <= 0;
    program_counter <= 0;
    addr <= 0;
    write_enable <= 0;
end

always @(posedge clock or posedge reset) begin
  if (reset) begin
    out_val <= 0;
    state <= 0;
    program_counter <= 0;
    load_dest <= 0;
    write_enable <= 0;
    should_load <= 0;
    halted <= 0;
    flag <= 0;

    // reset registers
    registers[0] <= 0;
    registers[1] <= 0; 
    registers[2] <= 0;
    registers[3] <= 0;
    registers[4] <= 0;
    registers[5] <= 0;
    registers[6] <= 0;
    registers[7] <= 0;

    addr <= 0;
  end else begin
    if (!halted) begin
      case (state)
        0: begin 
          // Load PC
          addr <= program_counter;
          state <= 1;
          if (should_load) begin
            registers[load_dest] <= inp_val;
            should_load <= 0;
          end
          write_enable <= 0; // Reset write_enable at start of instruction
        end
        1: begin
          // Execute opcode
          program_counter <= program_counter + 2;

          case (opcode)
            4'b0000: begin // NOP
              // Do nothing
            end
            4'b0001: begin // ADD
              registers[reg_dest] <= registers[reg_dest] + registers[reg_src];
            end
            4'b0100: begin // ADDI
              registers[reg_dest] <= registers[reg_dest] + immediate;
            end
            4'b0110: begin // NAND
              registers[reg_dest] <= ~(registers[reg_dest] & registers[reg_src]);
            end
            4'b0111: begin // GT
              registers[reg_dest] <= (registers[reg_src] > registers[reg_src2]) ? 1 : 0;
            end
            4'b1000: begin // LOADI
              registers[reg_dest] <= immediate;
            end
            4'b1001: begin // STORE
              addr <= registers[reg_dest];  // Write to memory address in reg_src
              out_val <= registers[reg_src]; // Value to write from reg_dest
              write_enable <= 1;  // Enable memory write
            end
            4'b1011: begin // LOAD
              load_dest <= reg_dest;  // Latch the destination register
              addr <= registers[reg_src];  // Read from address in reg_src
              should_load <= 1;
            end
            4'b1100: begin // JZ
              if (registers[reg_dest] == 0) begin
                program_counter <= immediate; // Jump to address (multiply by 2 since instructions are 2 bytes)
              end
            end
            4'b1101: begin // LOADW
              load_dest <= reg_dest;  // Latch the destination register
              addr <= program_counter + 2;  // Read next word
              should_load <= 1;
              program_counter <= program_counter + 4;
            end
            4'b1110: begin // FLAG
              flag <= registers[0][15:0] == "os" && 
                     registers[1][15:0] == "ec" && 
                     registers[2][15:0] == ".i" && 
                     registers[3][15:0] == "o\0";
            end
            default: begin
              halted <= 1;
            end
          endcase
          state <= 0;
        end
      endcase
    end
  end
end

endmodule
