use std::error::Error;
use verilog_macro::json_to_println;
use crate::err;

pub struct State<'a> {
    pub data: &'a mut [u8],
    pub updates: u64,
    pub total_updates: u64
}

impl State<'_> {
    pub fn tick(&mut self) -> Result<(), Box<dyn Error>> {
        self.updates = 0;

        loop {
            json_to_println!("./verilog/cpu.json");

            if self.updates == 0 {
                break;
            }
            self.updates = 0;
        }
        Ok(())
    }

    pub fn nand(&mut self, a: usize, b: usize, y: usize) -> Result<(), Box<dyn Error>> {
        let nxt = !(self.data[a] & self.data[b]);
        assert!(nxt == 0 || nxt == 255);

        if self.data[y] != nxt {
            self.data[y] = nxt;
            self.updates += 1;
            self.total_updates += 1;
        }


        Ok(())
    }

    pub fn flip<I>(&mut self, idx: I) -> Result<(), Box<dyn Error>>
    where 
        I: TryInto<usize> 
    {
        let idx = idx.try_into().map_err(|_| err!("Index conversion failed"))?;
        self.data[idx] = !self.data[idx];
        Ok(())
    }

    pub fn print<'a, I>(&self, mut idxs: I) -> Result<(), Box<dyn Error>>
    where
        I: Iterator<Item = &'a i32> + Clone
    {
        let idxs_clone = idxs.clone();

        let start = *idxs.next().ok_or_else(|| err!("Empty iterator"))?;
        let last = idxs.last().ok_or_else(|| err!("Empty iterator"))?;

        let ret = self.get(idxs_clone)?;
        println!("{}:{} \t{:b}", start, last, ret);
        Ok(())
    }

    pub fn get<'a, I>(&self, indices: I) -> Result<u64, Box<dyn Error>>
    where
        I: Iterator<Item = &'a i32>
    {
        let mut ret = 0;

        for (cnt, i) in indices.enumerate() {
            let i: usize = (*i as i64).try_into().map_err(|_| err!("Index conversion failed"))?;

            let nxt = (self.data[i] >> 7) as u64;
            assert!(nxt == 0 || nxt == 1);

            ret = ret | (nxt << cnt);
        }
        Ok(ret)
    }

    pub fn set<'a, I, N>(&mut self, indices: I, val: N) -> Result<(), Box<dyn Error>>
    where 
        I: Iterator<Item = &'a i32>,
        N: Into<u64>
    {
        let mut val = val.into();
        let mut count = 0;
        for i in indices {
            self.set_bit(*i as i64, val & 1 == 1)?;
            val >>= 1;
            count += 1;
        }
        assert!(count == 8, "Expected length of 8 bits");
        Ok(())
    }

    pub fn set_bit<I>(&mut self, idx: I, on: bool) -> Result<(), Box<dyn Error>>
    where 
        I: TryInto<usize> 
    {
        let idx = idx.try_into().map_err(|_| err!("Index conversion failed"))?;
        self.data[idx] = if on { 255 } else { 0 };
        Ok(())
    }
} 