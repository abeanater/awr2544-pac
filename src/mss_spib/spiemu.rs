#[doc = "Register `SPIEMU` reader"]
pub type R = crate::R<SpiemuSpec>;
#[doc = "Register `SPIEMU` writer"]
pub type W = crate::W<SpiemuSpec>;
#[doc = "Field `RXDATA` reader - 15:0\\]
SPI Receive Data. SPI / MibSPI emulation is a mirror of the SPIBUF register. The only difference between SPIEMU and SPIBUF is that a read from SPIEMU does not clear any of the status flags."]
pub type RxdataR = crate::FieldReader<u16>;
#[doc = "Field `RXDATA` writer - 15:0\\]
SPI Receive Data. SPI / MibSPI emulation is a mirror of the SPIBUF register. The only difference between SPIEMU and SPIBUF is that a read from SPIEMU does not clear any of the status flags."]
pub type RxdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LCSNR` reader - 23:16\\]
Last Chip select number. LCSNR in the status field is a copy of CSNR in the corresponding control field. It defines the chip select that has been activated during the last data transfer from the corresponding buffer. This is the copy of CSNR bits from SPIDAT1 latched at the end of a transfer. Note: Effect of NUM_CS_PINS generic on LCSNR bits. Actual number of bits implemented in LCSNR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will read ΓÇÿ0ΓÇÖ always."]
pub type LcsnrR = crate::FieldReader;
#[doc = "Field `LCSNR` writer - 23:16\\]
Last Chip select number. LCSNR in the status field is a copy of CSNR in the corresponding control field. It defines the chip select that has been activated during the last data transfer from the corresponding buffer. This is the copy of CSNR bits from SPIDAT1 latched at the end of a transfer. Note: Effect of NUM_CS_PINS generic on LCSNR bits. Actual number of bits implemented in LCSNR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will read ΓÇÿ0ΓÇÖ always."]
pub type LcsnrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLENERR` reader - 24:24\\]
Data Length Error flag. 1 = A Data Length Error has occured. 0 = No Data Length Error has occured."]
pub type DlenerrR = crate::BitReader;
#[doc = "Field `DLENERR` writer - 24:24\\]
Data Length Error flag. 1 = A Data Length Error has occured. 0 = No Data Length Error has occured."]
pub type DlenerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - 25:25\\]
Time-out due to non-activation of ENA pin. 1 =An ENA signal time-out occurred. The SPI / MibSPI generates a time-out because the slave hasnΓÇÖt responded in time by activating the ENA signal after the chip select signal has been activated. If a time-out condition is detected the corresponding chip select is deactivated immediately and the TIMEOUT flag is set. In addition the TIMOUT flag in the status field of the corresponding buffer and in the SPIFLG register is set. 0 =No ENA-pin time-out occurred. This bit is valid in Master mode only."]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - 25:25\\]
Time-out due to non-activation of ENA pin. 1 =An ENA signal time-out occurred. The SPI / MibSPI generates a time-out because the slave hasnΓÇÖt responded in time by activating the ENA signal after the chip select signal has been activated. If a time-out condition is detected the corresponding chip select is deactivated immediately and the TIMEOUT flag is set. In addition the TIMOUT flag in the status field of the corresponding buffer and in the SPIFLG register is set. 0 =No ENA-pin time-out occurred. This bit is valid in Master mode only."]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITYERR` reader - 26:26\\]
Calculated parity differs from received parity bit. 1 =A parity error occurred. If the parity generator is enabled (can be selected individually for each buffer) an even or odd parity bit is added at the end of a data word (see Section 8.23). During reception of the data word the parity generator calculates the reference parity and compares it to the received parity bit. In the event of a mismatch the PARITYERR flag is set. 0 =No parity error detected."]
pub type ParityerrR = crate::BitReader;
#[doc = "Field `PARITYERR` writer - 26:26\\]
Calculated parity differs from received parity bit. 1 =A parity error occurred. If the parity generator is enabled (can be selected individually for each buffer) an even or odd parity bit is added at the end of a data word (see Section 8.23). During reception of the data word the parity generator calculates the reference parity and compares it to the received parity bit. In the event of a mismatch the PARITYERR flag is set. 0 =No parity error detected."]
pub type ParityerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESYNC` reader - 27:27\\]
De-synchronization of slave device. De-synchronization monitor is active in master mode only. 1 =A slave device is de-synchronized. The master monitors the ENA signal coming from the slave device and sets the DESYNC flag if ENA is deactivated before the last reception point or after the last bit is transmitted plus tT2EDELAY (see Section 8.21). If DESYNCENA is set an interrupt is asserted. De-synchronization can occur if a slave device misses a clock edge coming from the master. 0 =No slave de-synchronization detected. This bit is valid in Master mode only."]
pub type DesyncR = crate::BitReader;
#[doc = "Field `DESYNC` writer - 27:27\\]
De-synchronization of slave device. De-synchronization monitor is active in master mode only. 1 =A slave device is de-synchronized. The master monitors the ENA signal coming from the slave device and sets the DESYNC flag if ENA is deactivated before the last reception point or after the last bit is transmitted plus tT2EDELAY (see Section 8.21). If DESYNCENA is set an interrupt is asserted. De-synchronization can occur if a slave device misses a clock edge coming from the master. 0 =No slave de-synchronization detected. This bit is valid in Master mode only."]
pub type DesyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITERR` reader - 28:28\\]
Mismatch of internal transmit data and transmitted data. 1 =A bit error occurred. The SPI / MibSPI samples the signal of the transmit pin (master: SIMO, slave: SOMI) at the receive point (half clock cycle after transmit point). If the sampled value differs from the transmitted value a bit error is detected and the flag BITERR is set. A possible reason for a bit error can be noise, a to high bit rate / capacitive load or another master/slave trying to transmit at the same time. 0 =No bit error occurred."]
pub type BiterrR = crate::BitReader;
#[doc = "Field `BITERR` writer - 28:28\\]
Mismatch of internal transmit data and transmitted data. 1 =A bit error occurred. The SPI / MibSPI samples the signal of the transmit pin (master: SIMO, slave: SOMI) at the receive point (half clock cycle after transmit point). If the sampled value differs from the transmitted value a bit error is detected and the flag BITERR is set. A possible reason for a bit error can be noise, a to high bit rate / capacitive load or another master/slave trying to transmit at the same time. 0 =No bit error occurred."]
pub type BiterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFULL` reader - 29:29\\]
Transmit data buffer full. 1 =Transmit buffer is full, SPIDAT0/SPIDAT1 is not ready to accept a new data. 0 =Transmit buffer is empty, SPIDAT0/SPIDAT1 is ready to accept a new data."]
pub type TxfullR = crate::BitReader;
#[doc = "Field `TXFULL` writer - 29:29\\]
Transmit data buffer full. 1 =Transmit buffer is full, SPIDAT0/SPIDAT1 is not ready to accept a new data. 0 =Transmit buffer is empty, SPIDAT0/SPIDAT1 is ready to accept a new data."]
pub type TxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVR` reader - 30:30\\]
Receive data buffer overrun. 1 =A receive data overrun condition occurred since last time reading the data field. 0 =No receive data overrun condition occurred since last time reading the data field."]
pub type RxovrR = crate::BitReader;
#[doc = "Field `RXOVR` writer - 30:30\\]
Receive data buffer overrun. 1 =A receive data overrun condition occurred since last time reading the data field. 0 =No receive data overrun condition occurred since last time reading the data field."]
pub type RxovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEMPTY` reader - 31:31\\]
Receive data buffer empty. 1 = No data received since last reading of SPIBUF register. 0 = A new Data is received and copied into SPIBUF field."]
pub type RxemptyR = crate::BitReader;
#[doc = "Field `RXEMPTY` writer - 31:31\\]
Receive data buffer empty. 1 = No data received since last reading of SPIBUF register. 0 = A new Data is received and copied into SPIBUF field."]
pub type RxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
SPI Receive Data. SPI / MibSPI emulation is a mirror of the SPIBUF register. The only difference between SPIEMU and SPIBUF is that a read from SPIEMU does not clear any of the status flags."]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Last Chip select number. LCSNR in the status field is a copy of CSNR in the corresponding control field. It defines the chip select that has been activated during the last data transfer from the corresponding buffer. This is the copy of CSNR bits from SPIDAT1 latched at the end of a transfer. Note: Effect of NUM_CS_PINS generic on LCSNR bits. Actual number of bits implemented in LCSNR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    pub fn lcsnr(&self) -> LcsnrR {
        LcsnrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Data Length Error flag. 1 = A Data Length Error has occured. 0 = No Data Length Error has occured."]
    #[inline(always)]
    pub fn dlenerr(&self) -> DlenerrR {
        DlenerrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Time-out due to non-activation of ENA pin. 1 =An ENA signal time-out occurred. The SPI / MibSPI generates a time-out because the slave hasnΓÇÖt responded in time by activating the ENA signal after the chip select signal has been activated. If a time-out condition is detected the corresponding chip select is deactivated immediately and the TIMEOUT flag is set. In addition the TIMOUT flag in the status field of the corresponding buffer and in the SPIFLG register is set. 0 =No ENA-pin time-out occurred. This bit is valid in Master mode only."]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Calculated parity differs from received parity bit. 1 =A parity error occurred. If the parity generator is enabled (can be selected individually for each buffer) an even or odd parity bit is added at the end of a data word (see Section 8.23). During reception of the data word the parity generator calculates the reference parity and compares it to the received parity bit. In the event of a mismatch the PARITYERR flag is set. 0 =No parity error detected."]
    #[inline(always)]
    pub fn parityerr(&self) -> ParityerrR {
        ParityerrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
De-synchronization of slave device. De-synchronization monitor is active in master mode only. 1 =A slave device is de-synchronized. The master monitors the ENA signal coming from the slave device and sets the DESYNC flag if ENA is deactivated before the last reception point or after the last bit is transmitted plus tT2EDELAY (see Section 8.21). If DESYNCENA is set an interrupt is asserted. De-synchronization can occur if a slave device misses a clock edge coming from the master. 0 =No slave de-synchronization detected. This bit is valid in Master mode only."]
    #[inline(always)]
    pub fn desync(&self) -> DesyncR {
        DesyncR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Mismatch of internal transmit data and transmitted data. 1 =A bit error occurred. The SPI / MibSPI samples the signal of the transmit pin (master: SIMO, slave: SOMI) at the receive point (half clock cycle after transmit point). If the sampled value differs from the transmitted value a bit error is detected and the flag BITERR is set. A possible reason for a bit error can be noise, a to high bit rate / capacitive load or another master/slave trying to transmit at the same time. 0 =No bit error occurred."]
    #[inline(always)]
    pub fn biterr(&self) -> BiterrR {
        BiterrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Transmit data buffer full. 1 =Transmit buffer is full, SPIDAT0/SPIDAT1 is not ready to accept a new data. 0 =Transmit buffer is empty, SPIDAT0/SPIDAT1 is ready to accept a new data."]
    #[inline(always)]
    pub fn txfull(&self) -> TxfullR {
        TxfullR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Receive data buffer overrun. 1 =A receive data overrun condition occurred since last time reading the data field. 0 =No receive data overrun condition occurred since last time reading the data field."]
    #[inline(always)]
    pub fn rxovr(&self) -> RxovrR {
        RxovrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Receive data buffer empty. 1 = No data received since last reading of SPIBUF register. 0 = A new Data is received and copied into SPIBUF field."]
    #[inline(always)]
    pub fn rxempty(&self) -> RxemptyR {
        RxemptyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
SPI Receive Data. SPI / MibSPI emulation is a mirror of the SPIBUF register. The only difference between SPIEMU and SPIBUF is that a read from SPIEMU does not clear any of the status flags."]
    #[inline(always)]
    #[must_use]
    pub fn rxdata(&mut self) -> RxdataW<SpiemuSpec> {
        RxdataW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Last Chip select number. LCSNR in the status field is a copy of CSNR in the corresponding control field. It defines the chip select that has been activated during the last data transfer from the corresponding buffer. This is the copy of CSNR bits from SPIDAT1 latched at the end of a transfer. Note: Effect of NUM_CS_PINS generic on LCSNR bits. Actual number of bits implemented in LCSNR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    #[must_use]
    pub fn lcsnr(&mut self) -> LcsnrW<SpiemuSpec> {
        LcsnrW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Data Length Error flag. 1 = A Data Length Error has occured. 0 = No Data Length Error has occured."]
    #[inline(always)]
    #[must_use]
    pub fn dlenerr(&mut self) -> DlenerrW<SpiemuSpec> {
        DlenerrW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Time-out due to non-activation of ENA pin. 1 =An ENA signal time-out occurred. The SPI / MibSPI generates a time-out because the slave hasnΓÇÖt responded in time by activating the ENA signal after the chip select signal has been activated. If a time-out condition is detected the corresponding chip select is deactivated immediately and the TIMEOUT flag is set. In addition the TIMOUT flag in the status field of the corresponding buffer and in the SPIFLG register is set. 0 =No ENA-pin time-out occurred. This bit is valid in Master mode only."]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<SpiemuSpec> {
        TimeoutW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Calculated parity differs from received parity bit. 1 =A parity error occurred. If the parity generator is enabled (can be selected individually for each buffer) an even or odd parity bit is added at the end of a data word (see Section 8.23). During reception of the data word the parity generator calculates the reference parity and compares it to the received parity bit. In the event of a mismatch the PARITYERR flag is set. 0 =No parity error detected."]
    #[inline(always)]
    #[must_use]
    pub fn parityerr(&mut self) -> ParityerrW<SpiemuSpec> {
        ParityerrW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
De-synchronization of slave device. De-synchronization monitor is active in master mode only. 1 =A slave device is de-synchronized. The master monitors the ENA signal coming from the slave device and sets the DESYNC flag if ENA is deactivated before the last reception point or after the last bit is transmitted plus tT2EDELAY (see Section 8.21). If DESYNCENA is set an interrupt is asserted. De-synchronization can occur if a slave device misses a clock edge coming from the master. 0 =No slave de-synchronization detected. This bit is valid in Master mode only."]
    #[inline(always)]
    #[must_use]
    pub fn desync(&mut self) -> DesyncW<SpiemuSpec> {
        DesyncW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Mismatch of internal transmit data and transmitted data. 1 =A bit error occurred. The SPI / MibSPI samples the signal of the transmit pin (master: SIMO, slave: SOMI) at the receive point (half clock cycle after transmit point). If the sampled value differs from the transmitted value a bit error is detected and the flag BITERR is set. A possible reason for a bit error can be noise, a to high bit rate / capacitive load or another master/slave trying to transmit at the same time. 0 =No bit error occurred."]
    #[inline(always)]
    #[must_use]
    pub fn biterr(&mut self) -> BiterrW<SpiemuSpec> {
        BiterrW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Transmit data buffer full. 1 =Transmit buffer is full, SPIDAT0/SPIDAT1 is not ready to accept a new data. 0 =Transmit buffer is empty, SPIDAT0/SPIDAT1 is ready to accept a new data."]
    #[inline(always)]
    #[must_use]
    pub fn txfull(&mut self) -> TxfullW<SpiemuSpec> {
        TxfullW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Receive data buffer overrun. 1 =A receive data overrun condition occurred since last time reading the data field. 0 =No receive data overrun condition occurred since last time reading the data field."]
    #[inline(always)]
    #[must_use]
    pub fn rxovr(&mut self) -> RxovrW<SpiemuSpec> {
        RxovrW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Receive data buffer empty. 1 = No data received since last reading of SPIBUF register. 0 = A new Data is received and copied into SPIBUF field."]
    #[inline(always)]
    #[must_use]
    pub fn rxempty(&mut self) -> RxemptyW<SpiemuSpec> {
        RxemptyW::new(self, 31)
    }
}
#[doc = "SPI / MibSPI Emulation Register Note: All the fields ot SPIEMU register are Read-Only. Read operation on this register under any mode will not have any impact on the status of this or any other registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`spiemu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spiemu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiemuSpec;
impl crate::RegisterSpec for SpiemuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spiemu::R`](R) reader structure"]
impl crate::Readable for SpiemuSpec {}
#[doc = "`write(|w| ..)` method takes [`spiemu::W`](W) writer structure"]
impl crate::Writable for SpiemuSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIEMU to value 0"]
impl crate::Resettable for SpiemuSpec {
    const RESET_VALUE: u32 = 0;
}
