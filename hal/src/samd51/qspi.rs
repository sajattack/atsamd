use crate::{
    target_device::{QSPI, MCLK},
    gpio::{Pa8, Pa9, Pa10, Pa11, Pb10, Pb11, Input, Floating, PfH, Port},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    /// The command you selected cannot be performed by this function
    CommandFunctionMismatch
}

pub struct Qspi {
    qspi: QSPI,
    _sck: Pb10<PfH>,
    _cs:  Pb11<PfH>,
    _io0: Pa8<PfH>,
    _io1: Pa9<PfH>,
    _io2: Pa10<PfH>,
    _io3: Pa11<PfH>,
}

impl Qspi {
   pub fn new(
        mclk: &mut MCLK,
        port: &mut Port,
        qspi: QSPI,
        _sck: Pb10<Input<Floating>>,
        _cs:  Pb11<Input<Floating>>,
        _io0: Pa8<Input<Floating>>,
        _io1: Pa9<Input<Floating>>,
        _io2: Pa10<Input<Floating>>,
        _io3: Pa11<Input<Floating>>,
    ) -> Qspi {
        let _sck = _sck.into_function_h(port);
        let _cs = _cs.into_function_h(port);
        let _io0 = _io0.into_function_h(port);
        let _io1 = _io1.into_function_h(port);
        let _io2 = _io2.into_function_h(port);
        let _io3 = _io3.into_function_h(port);

        mclk.apbcmask.modify(|_, w| w.qspi_().set_bit());
        // Enable the clocks for the qspi peripheral in single data rate mode.
        mclk.ahbmask.modify(|_, w| {
            w.qspi_().set_bit(); 
            w.qspi_2x_().clear_bit()
        });

        qspi.ctrla.modify(|_, w| w.swrst().set_bit());
        qspi.baud.modify(|_, w| {
            unsafe { w.baud().bits(14) }; //120MHz / (14+1) = 8MHz

            // SPI MODE 0
            w.cpol().clear_bit();
            w.cpha().clear_bit()
        });

        qspi.ctrlb.modify(|_, w| {
            w.mode().memory();
            w.csmode().noreload();
            w.csmode().lastxfer();
            w.datalen()._8bits()
        });

        qspi.ctrla.modify(|_, w| w.enable().set_bit());

        Qspi {
            qspi,
            _sck,
            _cs,
            _io0,
            _io1,
            _io2,
            _io3,
        }
    }

    unsafe fn run_write_instruction(
        &self,
        command: Command,
        addr: u32,
        buf: &[u8]
    ) {
        if command == Command::EraseSector || command == Command::EraseBlock {
            self.qspi.instraddr.write(|w| { w.addr().bits(addr) });
        }
        self.qspi.instrctrl.write(|w| w.instr().bits(command.bits()));
        let _ = self.qspi.instrframe.read().bits();
        if buf.len() > 0 {
            core::ptr::copy(buf.as_ptr(), (QSPI_AHB + addr) as *mut u8, buf.len());
        }

        self.qspi.ctrla.write(|w| {
            w.enable().set_bit();
            w.lastxfer().set_bit()
        });

        while self.qspi.intflag.read().instrend().bit_is_clear() {}
        self.qspi.intflag.modify(|_, w| w.instrend().set_bit());
    }

    unsafe fn run_read_instruction(
        &self,
        command: Command,
        addr: u32,
        buf: &mut [u8]
    ) {
        self.qspi.instrctrl.write(|w| w.instr().bits(command.bits()));
        let _ = self.qspi.instrframe.read().bits();
        if buf.len() > 0 {
            core::ptr::copy((QSPI_AHB + addr) as *mut u8, buf.as_mut_ptr(), buf.len());
        }

        self.qspi.ctrla.write(|w| {
            w.enable().set_bit();
            w.lastxfer().set_bit()
        });

        while self.qspi.intflag.read().instrend().bit_is_clear() {}
        self.qspi.intflag.modify(|_, w| w.instrend().set_bit());
    }

    /// Run a generic command that neither takes nor receives data
    pub fn run_command(&self, command: Command) -> Result<(), Error> {
        match command {
            //TODO verify this list of commands
            Command::WriteEnable |
            Command::WriteDisable |
            Command::Reset |
            Command::EnableReset  => (),
            _ => { return Err(Error::CommandFunctionMismatch) }
        }

        self.qspi.instrframe.write(|w| {
            w.width().single_bit_spi();
            w.addrlen()._24bits();
            w.tfrtype().read();
            w.instren().set_bit()
        });
        unsafe { self.run_read_instruction(command, 0, &mut[]); }
        Ok(())
    }

    /// Run one of the read commands
    pub fn read_command(
        &self, 
        command: Command,
        response: &mut [u8]
    ) -> Result<(), Error> {
        match command {
            //TODO verify this list of commands
            Command::Read |
            Command::QuadRead |
            Command::ReadId |
            Command::ReadStatus |
            Command::ReadStatus2 => (),
            _ => { return Err(Error::CommandFunctionMismatch) }
        }

        self.qspi.instrframe.write(|w| {
            w.width().single_bit_spi();
            w.addrlen()._24bits();
            w.tfrtype().read();
            w.instren().set_bit();
            w.dataen().set_bit()
        });
        unsafe { self.run_read_instruction(command, 0, response); }
        Ok(())
    }

    /// Run one of the write commands
    pub fn write_command(&self, command: Command, data: &[u8]) -> Result<(), Error> {
        match command {
            //TODO verify this list of commands
            Command::PageProgram |
            Command::QuadPageProgram |
            Command::WriteStatus |
            Command::WriteStatus2  => (),
            _ => { return Err(Error::CommandFunctionMismatch) }
        }

        self.qspi.instrframe.write(|w| {
            w.width().single_bit_spi();
            w.addrlen()._24bits();
            w.tfrtype().write();
            w.instren().set_bit();
            if data.len() > 0 {
                w.dataen().set_bit()
            } else {
                w.dataen().clear_bit()
            }
        });

        unsafe { self.run_write_instruction(command, 0, data); }
        Ok(())
    }

    /// Run one of the erase commands
    pub fn erase_command(&self, command: Command, address: u32) -> Result<(), Error> {
        match command {
            //TODO verify this list of commands
            Command::EraseSector |
            Command::EraseBlock |
            Command::EraseChip  => (),
            _ => { return Err(Error::CommandFunctionMismatch) }
        }

        self.qspi.instrframe.write(|w| {
            w.width().single_bit_spi();
            w.addrlen()._24bits();
            w.tfrtype().write();
            w.instren().set_bit();
            w.addren().set_bit()
        });
        unsafe { self.run_write_instruction(command, address, &[]); }
        Ok(())
    }

    /// Read a sequential block of memory to buf
    pub fn read_memory(&self, addr: u32, buf: &mut [u8]) {
        self.qspi.instrframe.write(|w| {
            w.width().quad_output();
            w.addrlen()._24bits();
            w.tfrtype().readmemory();
            w.instren().set_bit();
            w.dataen().set_bit();
            w.addren().set_bit();
            unsafe{ w.dummylen().bits(8) }
        });
        unsafe { self.run_read_instruction(Command::QuadRead, addr, buf) };
    }

    /// Write a sequential block of memory to addr
    pub fn write_memory(&self, addr: u32, buf: &[u8]) {
        self.qspi.instrframe.write(|w| {
            w.width().quad_output();
            w.addrlen()._24bits();
            w.tfrtype().writememory();
            w.instren().set_bit();
            w.dataen().set_bit();
            w.addren().set_bit()
        });
        unsafe { self.run_write_instruction(Command::QuadPageProgram, addr, buf) };
    }

    /// Set the clock divider, relative to the main clock
    pub fn set_clk_divider(&self, value: u8) {
        // The baud register is divisor - 1
        self.qspi.baud.write(|w| unsafe { w.baud().bits(value.saturating_sub(1)) });
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Command {
    Read = 0x03,
    QuadRead = 0x6B,
    ReadId = 0x9F,
    PageProgram = 0x02,
    QuadPageProgram = 0x32,
    ReadStatus = 0x05,
    ReadStatus2 = 0x35,
    WriteStatus = 0x01,
    WriteStatus2 = 0x31,
    EnableReset = 0x66,
    Reset = 0x99,
    WriteEnable = 0x06,
    WriteDisable = 0x04,
    EraseSector = 0x20,
    EraseBlock = 0xD8,
    EraseChip = 0xC7
}

impl Command {
    fn bits(self) -> u8 {
        self as u8
    }
}

const QSPI_AHB: u32 = 0x04000000;