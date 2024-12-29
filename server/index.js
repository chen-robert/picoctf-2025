const express = require('express');
const cors = require('cors');
const morgan = require('morgan');
const dotenv = require('dotenv');
const path = require('path');
const { 
    loadCpuSignals, 
    getBitsValue, 
    setBits, 
    splitBits,
    checkInt,
    serializeCircuit
} = require('./utils');

const { process } = require('./wasm/pkg/verilog_ctf_wasm.js');
const MEM_SIZE = 65536;

// Load environment variables
dotenv.config();

const app = express();
const port = process.env.PORT || 3000;

// Middleware
app.use(cors());
app.use(morgan('dev'));
app.use(express.json());

// Serve static files from the public directory
app.use(express.static(path.join(__dirname, 'public')));

// Routes
app.get('/api/health', (req, res) => {
  res.json({ status: 'healthy' });
});

function runCPU(memory) {
  const state = new Uint8Array(100_000);
  const signals = loadCpuSignals();

  // Reset sequence
  process(state);
  state[signals.reset] = 255;
  process(state);
  state[signals.reset] = 0;
  process(state);

  let flag = false;
  const MAX_CYCLES = 500000;

  for (let cycle = 0; cycle < MAX_CYCLES; cycle++) {
    // Toggle clock
    state[signals.clock] ^= 255;
    process(state);

    // On clock low edge
    if (state[signals.clock] === 0) {
      // Handle memory writes
      if (state[signals.write_enable] === 255) {
        const addr = getBitsValue(state, signals.addr);
        const val = getBitsValue(state, signals.out_val);
        memory[addr] = val & 0xFF;
        memory[addr + 1] = (val >> 8) & 0xFF;
      }

      // Handle memory reads
      const addr = getBitsValue(state, signals.addr);
      const [first_byte, second_byte] = splitBits(signals.inp_val, 8);
      setBits(state, first_byte, memory[addr]);
      setBits(state, second_byte, memory[addr + 1]);

      // Check halted and flag
      if (state[signals.halted] === 255) {
        break;
      }
      if (state[signals.flag] === 255) {
        flag = true;
      }
    }
  }

  return flag;
}

const OUTPUT_START = 0x1000;
const CIRCUIT_STATE = 0x2000;
const CIRCUIT_START = 0x3000;

function doRun(res, memory) {
  const flag = runCPU(memory);

  if (flag) {
    res.status(200).json({ status: 'success', flag: 'TODO' });
  } else {
    res.status(200).json({ status: 'success', flag: 'TODO' });
  }
}

// Add the check endpoint
app.post('/check', (req, res) => {
    const circuit = req.body.circuit;

    if (!Array.isArray(circuit) || 
        !circuit.every(entry => checkInt(entry?.input1) && 
                                checkInt(entry?.input2) && 
                                checkInt(entry?.output))) {
        return res.status(400).end();
    }

    const serialized = serializeCircuit(circuit);
    console.log('Received valid circuit:', circuit);
    console.log('Serialized words:', [...serialized]);
    console.log('Raw bytes:', [...new Uint8Array(serialized.buffer)]);
    res.json({ status: 'received' });
});

// Catch-all route to serve index.html
app.get('*', (req, res) => {
  res.sendFile(path.join(__dirname, 'public', 'index.html'));
});

// Error handling middleware
app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(500).json({ 
    error: 'Something broke!',
    message: err.message 
  });
});

// Start server
app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});

module.exports = {
  runCPU
}; 