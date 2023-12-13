use crate::error::*;
use embassy_stm32::usart::{Config, UartTx, UartRx};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use defmt::info;

/// Replacement for std::io::Read + byteorder::ReadBytesExt in no_std envs
pub trait Read {
    fn read_u8(&mut self) -> Result<u8, MessageReadError>;

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), MessageReadError> {
        for byte in buf {
            *byte = self.read_u8()?;
        }

        Ok(())
    }
}

// eww wtf this was blocking before
impl<R: embedded_hal::serial::Read<u8>> Read for R {
    fn read_u8(&mut self) -> Result<u8, MessageReadError> {
        nb::block!(self.read()).map_err(|_| MessageReadError::Io)
    }
}

pub trait ReadEmbassyAsync {
    async fn read_u8(&mut self) -> Result<u8, MessageReadError>; 

    // async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), MessageReadError>;
    async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), MessageReadError> {
        for byte in buf {
            *byte = self.read_u8().await.unwrap();
        }

        Ok(())
    }
}


impl ReadEmbassyAsync for UartRx<'static, peripherals::UART7, peripherals::DMA1_CH3> {
    async fn read_u8(&mut self) -> Result<u8, MessageReadError> {//Result<u8, usart::Error> {
        let mut msg: [u8; 1] = [0; 1];
        let res = self.read(&mut msg).await;

        info!("{}", msg[0]);

        match res {
            Ok(v) => {
                return Ok(msg[0]);
            },
            Err(e) => {
                return Err(MessageReadError::Io);
            }
        }
    }

    // async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), MessageReadError> {
    //     let res = self.read(buf).await;
    //     match res {
    //         Ok(v) => {
    //             return Ok(());
    //         },
    //         Err(e) => {
    //             return Err(MessageReadError::Io);
    //         }
    //     }
    // }
}





/// Replacement for std::io::Write + byteorder::WriteBytesExt in no_std envs
pub trait Write {
    fn write_all(&mut self, buf: &[u8]) -> Result<(), MessageWriteError>;
}

impl<W: embedded_hal::serial::Write<u8>> Write for W {
    fn write_all(&mut self, buf: &[u8]) -> Result<(), MessageWriteError> {
        for byte in buf {
            // nb::block!(self.write(*byte)).map_err(|_| MessageWriteError::Io)?;
            self.write(*byte).map_err(|_| MessageWriteError::Io)?;
        }
        Ok(())
    }
}

pub trait WriteEmbassyAsync {
    fn blocking_write(&mut self, buffer: &[u8]);
}

impl WriteEmbassyAsync for UartTx<'static, embassy_stm32::peripherals::UART7> {
    fn blocking_write(&mut self, buffer: &[u8]) {
        for byte in buffer {
            self.blocking_write(&[*byte]).unwrap();
        }
    }
}
