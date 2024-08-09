#[doc = "Register `SPIBUF` reader"]
pub type R = crate::R<SpibufSpec>;
#[doc = "Register `SPIBUF` writer"]
pub type W = crate::W<SpibufSpec>;
#[doc = "Field `RXDATA` reader - 15:0\\]
SPI Receive Data. This is the received data, transferred from the Receive Shift-Register at the end of a transfer completion. Irrespective of the programmed character length &amp; the direction of shifting, the received data is stored right-justified in the register."]
pub type RxdataR = crate::FieldReader<u16>;
#[doc = "Field `RXDATA` writer - 15:0\\]
SPI Receive Data. This is the received data, transferred from the Receive Shift-Register at the end of a transfer completion. Irrespective of the programmed character length &amp; the direction of shifting, the received data is stored right-justified in the register."]
pub type RxdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LCSNR` reader - 23:16\\]
Last Chip select number. LCSNR in the status field is a copy of CSNR in the corresponding control field. It defines the chip select that has been activated during the last data transfer from the corresponding buffer. This is the copy of CSNR bits from SPIDAT1 latched at the end of a transfer. Note: Effect of NUM_CS_PINS generic on LCSNR bits. Actual number of bits implemented in LCSNR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will read ΓÇÿ0ΓÇÖ always"]
pub type LcsnrR = crate::FieldReader;
#[doc = "Field `LCSNR` writer - 23:16\\]
Last Chip select number. LCSNR in the status field is a copy of CSNR in the corresponding control field. It defines the chip select that has been activated during the last data transfer from the corresponding buffer. This is the copy of CSNR bits from SPIDAT1 latched at the end of a transfer. Note: Effect of NUM_CS_PINS generic on LCSNR bits. Actual number of bits implemented in LCSNR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will read ΓÇÿ0ΓÇÖ always"]
pub type LcsnrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLENERR` reader - 24:24\\]
Data Length Error flag. 1 = A Data Length Error has occured. 0 = No Data Length Error has occured. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read."]
pub type DlenerrR = crate::BitReader;
#[doc = "Field `DLENERR` writer - 24:24\\]
Data Length Error flag. 1 = A Data Length Error has occured. 0 = No Data Length Error has occured. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read."]
pub type DlenerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - 25:25\\]
Time-out due to non-activation of ENA pin. 1 =An ENA signal time-out occurred. The SPI / MibSPI generates a time-out because the slave hasnΓÇÖt responded in time by activating the ENA signal after the chip select signal has been activated. If a time-out condition is detected the corresponding chip select is deactivated immediately and the TIMEOUT flag is set. In addition the TIMOUT flag in the status field of the corresponding buffer and in the SPIFLG register is set. 0 =No ENA-pin time-out occurred. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read. This bit is valid in Master mode only."]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - 25:25\\]
Time-out due to non-activation of ENA pin. 1 =An ENA signal time-out occurred. The SPI / MibSPI generates a time-out because the slave hasnΓÇÖt responded in time by activating the ENA signal after the chip select signal has been activated. If a time-out condition is detected the corresponding chip select is deactivated immediately and the TIMEOUT flag is set. In addition the TIMOUT flag in the status field of the corresponding buffer and in the SPIFLG register is set. 0 =No ENA-pin time-out occurred. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read. This bit is valid in Master mode only."]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITYERR` reader - 26:26\\]
Calculated parity differs from received parity bit. 1 =A parity error occurred. If the parity generator is enabled (can be selected individually for each buffer) an even or odd parity bit is added at the end of a data word (see Section 8.23). During reception of the data word the parity generator calculates the reference parity and compares it to the received parity bit. In the event of a mismatch the PARITYERR flag is set. 0 =No parity error detected. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read."]
pub type ParityerrR = crate::BitReader;
#[doc = "Field `PARITYERR` writer - 26:26\\]
Calculated parity differs from received parity bit. 1 =A parity error occurred. If the parity generator is enabled (can be selected individually for each buffer) an even or odd parity bit is added at the end of a data word (see Section 8.23). During reception of the data word the parity generator calculates the reference parity and compares it to the received parity bit. In the event of a mismatch the PARITYERR flag is set. 0 =No parity error detected. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read."]
pub type ParityerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESYNC` reader - 27:27\\]
De-synchronization of slave device. De-synchronization monitor is active in master mode only. 1 =A slave device is de-synchronized. The master monitors the ENA signal coming from the slave device and sets the DESYNC flag if ENA is deactivated before the last reception point or after the last bit is transmitted plus tT2EDELAY (see Section 8.21). If DESYNCENA is set an interrupt is asserted. De-synchronization can occur if a slave device misses a clock edge coming from the master. 0 =No slave de-synchronization detected. This bit is valid in Master mode only. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read. Note: Possible inconsistency of Desync flag in SPI/Compatibility mode MibSPI Due to the nature of this Error, under some circumstances it is possible for Desync error detected for the previous buffer to be visible in the current buffer. This is due to the fact that Receive Completion flag/interrupt will be generated when the buffer transfer is completed. But Desync will be detected after the buffer transfer is completed. So, if VBUS master reads the received data quickly when an RXINT is detected, then the status flag may not reflect the correct Desync condition. This inconsistency in Desync flag is valid only in SPI or Compatibility mode of MibSPI. In Multibuffer mode, Desync flag is always guaranteed to be for the current buffer."]
pub type DesyncR = crate::BitReader;
#[doc = "Field `DESYNC` writer - 27:27\\]
De-synchronization of slave device. De-synchronization monitor is active in master mode only. 1 =A slave device is de-synchronized. The master monitors the ENA signal coming from the slave device and sets the DESYNC flag if ENA is deactivated before the last reception point or after the last bit is transmitted plus tT2EDELAY (see Section 8.21). If DESYNCENA is set an interrupt is asserted. De-synchronization can occur if a slave device misses a clock edge coming from the master. 0 =No slave de-synchronization detected. This bit is valid in Master mode only. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read. Note: Possible inconsistency of Desync flag in SPI/Compatibility mode MibSPI Due to the nature of this Error, under some circumstances it is possible for Desync error detected for the previous buffer to be visible in the current buffer. This is due to the fact that Receive Completion flag/interrupt will be generated when the buffer transfer is completed. But Desync will be detected after the buffer transfer is completed. So, if VBUS master reads the received data quickly when an RXINT is detected, then the status flag may not reflect the correct Desync condition. This inconsistency in Desync flag is valid only in SPI or Compatibility mode of MibSPI. In Multibuffer mode, Desync flag is always guaranteed to be for the current buffer."]
pub type DesyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITERR` reader - 28:28\\]
Mismatch of internal transmit data and transmitted data. 1 =A bit error occurred. The SPI / MibSPI samples the signal of the transmit pin (master: SIMO, slave: SOMI) at the receive point (half clock cycle after transmit point). If the sampled value differs from the transmitted value a bit error is detected and the flag BITERR is set. A possible reason for a bit error can be noise, a to high bit rate / capacitive load or another master/slave trying to transmit at the same time. 0 =No bit error occurred. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read."]
pub type BiterrR = crate::BitReader;
#[doc = "Field `BITERR` writer - 28:28\\]
Mismatch of internal transmit data and transmitted data. 1 =A bit error occurred. The SPI / MibSPI samples the signal of the transmit pin (master: SIMO, slave: SOMI) at the receive point (half clock cycle after transmit point). If the sampled value differs from the transmitted value a bit error is detected and the flag BITERR is set. A possible reason for a bit error can be noise, a to high bit rate / capacitive load or another master/slave trying to transmit at the same time. 0 =No bit error occurred. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read."]
pub type BiterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFULL` reader - 29:29\\]
Transmit data buffer full. This flag is a read-only flag. Writing into SPIDAT0 or SPIDAT1 field while TX Shift Register is full will automatically set the TXFULL flag. Once the data is copied to the Shift Register the TXFULL flag will be cleared. Writing to SPIDAT0/SPIDAT1 register when both TXBUF &amp; the TX Shift Register are empty does not set the TXFULL flag. 1 =Transmit buffer is full, SPIDAT0/SPIDAT1 is not ready to accept a new data. 0 =Transmit buffer is empty, SPIDAT0/SPIDAT1 is ready to accept a new data."]
pub type TxfullR = crate::BitReader;
#[doc = "Field `TXFULL` writer - 29:29\\]
Transmit data buffer full. This flag is a read-only flag. Writing into SPIDAT0 or SPIDAT1 field while TX Shift Register is full will automatically set the TXFULL flag. Once the data is copied to the Shift Register the TXFULL flag will be cleared. Writing to SPIDAT0/SPIDAT1 register when both TXBUF &amp; the TX Shift Register are empty does not set the TXFULL flag. 1 =Transmit buffer is full, SPIDAT0/SPIDAT1 is not ready to accept a new data. 0 =Transmit buffer is empty, SPIDAT0/SPIDAT1 is ready to accept a new data."]
pub type TxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVR` reader - 30:30\\]
Receive data buffer overrun. When a data transfer is completed and the received data is copied into the RXBUF while it is already full, RXOVR is set. Refer to Figure 1 for a view of internal logic diagram. An Overrun always occurs to the RXBUF, and SPIBUF contents never get Overwritten until after it is read by the VBUSP Master. If enabled, RXOVRN interrupt gets generated when RXBUF is Overwritten, and reading SPIFLG or SPIVECTx register shows the RXOVRN condition. However, two read operations to the SPIBUF register are required to reach the Overrun buffer. 1 =A receive data overrun condition occurred since last time reading the data field. 0 =No receive data overrun condition occurred since last time reading the data field. This bit is cleared to ΓÇÿ0ΓÇÖ under the following conditions: o RXDATA portion of the SPIBUF register is read. o OVRNINTFLG bit in SPIFLG register is write-cleared. When an Overrun occurs, the SPIBUF contents will not be overwritten. Only the RXBUF contents are overwritten. When the SPIBUF is read out , the RXBUF contents get copied to the SPIBUF if RXBUF is full. So, the first data read out will be intact when an Overrun occurs. Note: A special condition under which RXOVR flag gets set. If both SPIBUF &amp; RXBUF are already full and while another buffer receive is underway, if any errors like TIMEOUT, BITERR &amp; DLEN_ERR occur, then RXOVR in RXBUF &amp; SPIFLG registers will be set to indicate that the status flags are getting overwritten by the new transfer. This overrun should be treated like a normal Receiver Overrun."]
pub type RxovrR = crate::BitReader;
#[doc = "Field `RXOVR` writer - 30:30\\]
Receive data buffer overrun. When a data transfer is completed and the received data is copied into the RXBUF while it is already full, RXOVR is set. Refer to Figure 1 for a view of internal logic diagram. An Overrun always occurs to the RXBUF, and SPIBUF contents never get Overwritten until after it is read by the VBUSP Master. If enabled, RXOVRN interrupt gets generated when RXBUF is Overwritten, and reading SPIFLG or SPIVECTx register shows the RXOVRN condition. However, two read operations to the SPIBUF register are required to reach the Overrun buffer. 1 =A receive data overrun condition occurred since last time reading the data field. 0 =No receive data overrun condition occurred since last time reading the data field. This bit is cleared to ΓÇÿ0ΓÇÖ under the following conditions: o RXDATA portion of the SPIBUF register is read. o OVRNINTFLG bit in SPIFLG register is write-cleared. When an Overrun occurs, the SPIBUF contents will not be overwritten. Only the RXBUF contents are overwritten. When the SPIBUF is read out , the RXBUF contents get copied to the SPIBUF if RXBUF is full. So, the first data read out will be intact when an Overrun occurs. Note: A special condition under which RXOVR flag gets set. If both SPIBUF &amp; RXBUF are already full and while another buffer receive is underway, if any errors like TIMEOUT, BITERR &amp; DLEN_ERR occur, then RXOVR in RXBUF &amp; SPIFLG registers will be set to indicate that the status flags are getting overwritten by the new transfer. This overrun should be treated like a normal Receiver Overrun."]
pub type RxovrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEMPTY` reader - 31:31\\]
Receive data buffer empty. When host reads the SPIBUF field or the whole SPIBUF register this will automatically set the RXEMPTY flag. When a data transfer is completed, the received data is copied into SPIBUF, the RXEMPTY flag is cleared. 1 =No data received since last reading of SPIBUF register. 0 =A new Data is received and copied into SPIBUF field. This flag gets set to ΓÇÿ1ΓÇÖ under following conditions: oReading RXDATA portion of the SPIBUF register. oWriting ΓÇÿ1ΓÇÖ to clear the RXINTFLG bit in SPIFLG register. Write-Clearing the RXINTFLG bit before reading the SPIBUF indicates the received data is being ignored. Conversely, RXINTFLG can be cleared by reading the RXDATA portion of the SPIBUF register. So, reading the full of SPIBUF register itself clears the Receiver Full Interrupt Flag."]
pub type RxemptyR = crate::BitReader;
#[doc = "Field `RXEMPTY` writer - 31:31\\]
Receive data buffer empty. When host reads the SPIBUF field or the whole SPIBUF register this will automatically set the RXEMPTY flag. When a data transfer is completed, the received data is copied into SPIBUF, the RXEMPTY flag is cleared. 1 =No data received since last reading of SPIBUF register. 0 =A new Data is received and copied into SPIBUF field. This flag gets set to ΓÇÿ1ΓÇÖ under following conditions: oReading RXDATA portion of the SPIBUF register. oWriting ΓÇÿ1ΓÇÖ to clear the RXINTFLG bit in SPIFLG register. Write-Clearing the RXINTFLG bit before reading the SPIBUF indicates the received data is being ignored. Conversely, RXINTFLG can be cleared by reading the RXDATA portion of the SPIBUF register. So, reading the full of SPIBUF register itself clears the Receiver Full Interrupt Flag."]
pub type RxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
SPI Receive Data. This is the received data, transferred from the Receive Shift-Register at the end of a transfer completion. Irrespective of the programmed character length &amp; the direction of shifting, the received data is stored right-justified in the register."]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Last Chip select number. LCSNR in the status field is a copy of CSNR in the corresponding control field. It defines the chip select that has been activated during the last data transfer from the corresponding buffer. This is the copy of CSNR bits from SPIDAT1 latched at the end of a transfer. Note: Effect of NUM_CS_PINS generic on LCSNR bits. Actual number of bits implemented in LCSNR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will read ΓÇÿ0ΓÇÖ always"]
    #[inline(always)]
    pub fn lcsnr(&self) -> LcsnrR {
        LcsnrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Data Length Error flag. 1 = A Data Length Error has occured. 0 = No Data Length Error has occured. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read."]
    #[inline(always)]
    pub fn dlenerr(&self) -> DlenerrR {
        DlenerrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Time-out due to non-activation of ENA pin. 1 =An ENA signal time-out occurred. The SPI / MibSPI generates a time-out because the slave hasnΓÇÖt responded in time by activating the ENA signal after the chip select signal has been activated. If a time-out condition is detected the corresponding chip select is deactivated immediately and the TIMEOUT flag is set. In addition the TIMOUT flag in the status field of the corresponding buffer and in the SPIFLG register is set. 0 =No ENA-pin time-out occurred. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read. This bit is valid in Master mode only."]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Calculated parity differs from received parity bit. 1 =A parity error occurred. If the parity generator is enabled (can be selected individually for each buffer) an even or odd parity bit is added at the end of a data word (see Section 8.23). During reception of the data word the parity generator calculates the reference parity and compares it to the received parity bit. In the event of a mismatch the PARITYERR flag is set. 0 =No parity error detected. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read."]
    #[inline(always)]
    pub fn parityerr(&self) -> ParityerrR {
        ParityerrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
De-synchronization of slave device. De-synchronization monitor is active in master mode only. 1 =A slave device is de-synchronized. The master monitors the ENA signal coming from the slave device and sets the DESYNC flag if ENA is deactivated before the last reception point or after the last bit is transmitted plus tT2EDELAY (see Section 8.21). If DESYNCENA is set an interrupt is asserted. De-synchronization can occur if a slave device misses a clock edge coming from the master. 0 =No slave de-synchronization detected. This bit is valid in Master mode only. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read. Note: Possible inconsistency of Desync flag in SPI/Compatibility mode MibSPI Due to the nature of this Error, under some circumstances it is possible for Desync error detected for the previous buffer to be visible in the current buffer. This is due to the fact that Receive Completion flag/interrupt will be generated when the buffer transfer is completed. But Desync will be detected after the buffer transfer is completed. So, if VBUS master reads the received data quickly when an RXINT is detected, then the status flag may not reflect the correct Desync condition. This inconsistency in Desync flag is valid only in SPI or Compatibility mode of MibSPI. In Multibuffer mode, Desync flag is always guaranteed to be for the current buffer."]
    #[inline(always)]
    pub fn desync(&self) -> DesyncR {
        DesyncR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Mismatch of internal transmit data and transmitted data. 1 =A bit error occurred. The SPI / MibSPI samples the signal of the transmit pin (master: SIMO, slave: SOMI) at the receive point (half clock cycle after transmit point). If the sampled value differs from the transmitted value a bit error is detected and the flag BITERR is set. A possible reason for a bit error can be noise, a to high bit rate / capacitive load or another master/slave trying to transmit at the same time. 0 =No bit error occurred. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read."]
    #[inline(always)]
    pub fn biterr(&self) -> BiterrR {
        BiterrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Transmit data buffer full. This flag is a read-only flag. Writing into SPIDAT0 or SPIDAT1 field while TX Shift Register is full will automatically set the TXFULL flag. Once the data is copied to the Shift Register the TXFULL flag will be cleared. Writing to SPIDAT0/SPIDAT1 register when both TXBUF &amp; the TX Shift Register are empty does not set the TXFULL flag. 1 =Transmit buffer is full, SPIDAT0/SPIDAT1 is not ready to accept a new data. 0 =Transmit buffer is empty, SPIDAT0/SPIDAT1 is ready to accept a new data."]
    #[inline(always)]
    pub fn txfull(&self) -> TxfullR {
        TxfullR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Receive data buffer overrun. When a data transfer is completed and the received data is copied into the RXBUF while it is already full, RXOVR is set. Refer to Figure 1 for a view of internal logic diagram. An Overrun always occurs to the RXBUF, and SPIBUF contents never get Overwritten until after it is read by the VBUSP Master. If enabled, RXOVRN interrupt gets generated when RXBUF is Overwritten, and reading SPIFLG or SPIVECTx register shows the RXOVRN condition. However, two read operations to the SPIBUF register are required to reach the Overrun buffer. 1 =A receive data overrun condition occurred since last time reading the data field. 0 =No receive data overrun condition occurred since last time reading the data field. This bit is cleared to ΓÇÿ0ΓÇÖ under the following conditions: o RXDATA portion of the SPIBUF register is read. o OVRNINTFLG bit in SPIFLG register is write-cleared. When an Overrun occurs, the SPIBUF contents will not be overwritten. Only the RXBUF contents are overwritten. When the SPIBUF is read out , the RXBUF contents get copied to the SPIBUF if RXBUF is full. So, the first data read out will be intact when an Overrun occurs. Note: A special condition under which RXOVR flag gets set. If both SPIBUF &amp; RXBUF are already full and while another buffer receive is underway, if any errors like TIMEOUT, BITERR &amp; DLEN_ERR occur, then RXOVR in RXBUF &amp; SPIFLG registers will be set to indicate that the status flags are getting overwritten by the new transfer. This overrun should be treated like a normal Receiver Overrun."]
    #[inline(always)]
    pub fn rxovr(&self) -> RxovrR {
        RxovrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Receive data buffer empty. When host reads the SPIBUF field or the whole SPIBUF register this will automatically set the RXEMPTY flag. When a data transfer is completed, the received data is copied into SPIBUF, the RXEMPTY flag is cleared. 1 =No data received since last reading of SPIBUF register. 0 =A new Data is received and copied into SPIBUF field. This flag gets set to ΓÇÿ1ΓÇÖ under following conditions: oReading RXDATA portion of the SPIBUF register. oWriting ΓÇÿ1ΓÇÖ to clear the RXINTFLG bit in SPIFLG register. Write-Clearing the RXINTFLG bit before reading the SPIBUF indicates the received data is being ignored. Conversely, RXINTFLG can be cleared by reading the RXDATA portion of the SPIBUF register. So, reading the full of SPIBUF register itself clears the Receiver Full Interrupt Flag."]
    #[inline(always)]
    pub fn rxempty(&self) -> RxemptyR {
        RxemptyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
SPI Receive Data. This is the received data, transferred from the Receive Shift-Register at the end of a transfer completion. Irrespective of the programmed character length &amp; the direction of shifting, the received data is stored right-justified in the register."]
    #[inline(always)]
    #[must_use]
    pub fn rxdata(&mut self) -> RxdataW<SpibufSpec> {
        RxdataW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Last Chip select number. LCSNR in the status field is a copy of CSNR in the corresponding control field. It defines the chip select that has been activated during the last data transfer from the corresponding buffer. This is the copy of CSNR bits from SPIDAT1 latched at the end of a transfer. Note: Effect of NUM_CS_PINS generic on LCSNR bits. Actual number of bits implemented in LCSNR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will read ΓÇÿ0ΓÇÖ always"]
    #[inline(always)]
    #[must_use]
    pub fn lcsnr(&mut self) -> LcsnrW<SpibufSpec> {
        LcsnrW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Data Length Error flag. 1 = A Data Length Error has occured. 0 = No Data Length Error has occured. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read."]
    #[inline(always)]
    #[must_use]
    pub fn dlenerr(&mut self) -> DlenerrW<SpibufSpec> {
        DlenerrW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Time-out due to non-activation of ENA pin. 1 =An ENA signal time-out occurred. The SPI / MibSPI generates a time-out because the slave hasnΓÇÖt responded in time by activating the ENA signal after the chip select signal has been activated. If a time-out condition is detected the corresponding chip select is deactivated immediately and the TIMEOUT flag is set. In addition the TIMOUT flag in the status field of the corresponding buffer and in the SPIFLG register is set. 0 =No ENA-pin time-out occurred. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read. This bit is valid in Master mode only."]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<SpibufSpec> {
        TimeoutW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Calculated parity differs from received parity bit. 1 =A parity error occurred. If the parity generator is enabled (can be selected individually for each buffer) an even or odd parity bit is added at the end of a data word (see Section 8.23). During reception of the data word the parity generator calculates the reference parity and compares it to the received parity bit. In the event of a mismatch the PARITYERR flag is set. 0 =No parity error detected. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read."]
    #[inline(always)]
    #[must_use]
    pub fn parityerr(&mut self) -> ParityerrW<SpibufSpec> {
        ParityerrW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
De-synchronization of slave device. De-synchronization monitor is active in master mode only. 1 =A slave device is de-synchronized. The master monitors the ENA signal coming from the slave device and sets the DESYNC flag if ENA is deactivated before the last reception point or after the last bit is transmitted plus tT2EDELAY (see Section 8.21). If DESYNCENA is set an interrupt is asserted. De-synchronization can occur if a slave device misses a clock edge coming from the master. 0 =No slave de-synchronization detected. This bit is valid in Master mode only. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read. Note: Possible inconsistency of Desync flag in SPI/Compatibility mode MibSPI Due to the nature of this Error, under some circumstances it is possible for Desync error detected for the previous buffer to be visible in the current buffer. This is due to the fact that Receive Completion flag/interrupt will be generated when the buffer transfer is completed. But Desync will be detected after the buffer transfer is completed. So, if VBUS master reads the received data quickly when an RXINT is detected, then the status flag may not reflect the correct Desync condition. This inconsistency in Desync flag is valid only in SPI or Compatibility mode of MibSPI. In Multibuffer mode, Desync flag is always guaranteed to be for the current buffer."]
    #[inline(always)]
    #[must_use]
    pub fn desync(&mut self) -> DesyncW<SpibufSpec> {
        DesyncW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Mismatch of internal transmit data and transmitted data. 1 =A bit error occurred. The SPI / MibSPI samples the signal of the transmit pin (master: SIMO, slave: SOMI) at the receive point (half clock cycle after transmit point). If the sampled value differs from the transmitted value a bit error is detected and the flag BITERR is set. A possible reason for a bit error can be noise, a to high bit rate / capacitive load or another master/slave trying to transmit at the same time. 0 =No bit error occurred. This flag is cleared to ΓÇÿ0ΓÇÖ when RXDATA portion of the SPIBUF register is read."]
    #[inline(always)]
    #[must_use]
    pub fn biterr(&mut self) -> BiterrW<SpibufSpec> {
        BiterrW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Transmit data buffer full. This flag is a read-only flag. Writing into SPIDAT0 or SPIDAT1 field while TX Shift Register is full will automatically set the TXFULL flag. Once the data is copied to the Shift Register the TXFULL flag will be cleared. Writing to SPIDAT0/SPIDAT1 register when both TXBUF &amp; the TX Shift Register are empty does not set the TXFULL flag. 1 =Transmit buffer is full, SPIDAT0/SPIDAT1 is not ready to accept a new data. 0 =Transmit buffer is empty, SPIDAT0/SPIDAT1 is ready to accept a new data."]
    #[inline(always)]
    #[must_use]
    pub fn txfull(&mut self) -> TxfullW<SpibufSpec> {
        TxfullW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Receive data buffer overrun. When a data transfer is completed and the received data is copied into the RXBUF while it is already full, RXOVR is set. Refer to Figure 1 for a view of internal logic diagram. An Overrun always occurs to the RXBUF, and SPIBUF contents never get Overwritten until after it is read by the VBUSP Master. If enabled, RXOVRN interrupt gets generated when RXBUF is Overwritten, and reading SPIFLG or SPIVECTx register shows the RXOVRN condition. However, two read operations to the SPIBUF register are required to reach the Overrun buffer. 1 =A receive data overrun condition occurred since last time reading the data field. 0 =No receive data overrun condition occurred since last time reading the data field. This bit is cleared to ΓÇÿ0ΓÇÖ under the following conditions: o RXDATA portion of the SPIBUF register is read. o OVRNINTFLG bit in SPIFLG register is write-cleared. When an Overrun occurs, the SPIBUF contents will not be overwritten. Only the RXBUF contents are overwritten. When the SPIBUF is read out , the RXBUF contents get copied to the SPIBUF if RXBUF is full. So, the first data read out will be intact when an Overrun occurs. Note: A special condition under which RXOVR flag gets set. If both SPIBUF &amp; RXBUF are already full and while another buffer receive is underway, if any errors like TIMEOUT, BITERR &amp; DLEN_ERR occur, then RXOVR in RXBUF &amp; SPIFLG registers will be set to indicate that the status flags are getting overwritten by the new transfer. This overrun should be treated like a normal Receiver Overrun."]
    #[inline(always)]
    #[must_use]
    pub fn rxovr(&mut self) -> RxovrW<SpibufSpec> {
        RxovrW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Receive data buffer empty. When host reads the SPIBUF field or the whole SPIBUF register this will automatically set the RXEMPTY flag. When a data transfer is completed, the received data is copied into SPIBUF, the RXEMPTY flag is cleared. 1 =No data received since last reading of SPIBUF register. 0 =A new Data is received and copied into SPIBUF field. This flag gets set to ΓÇÿ1ΓÇÖ under following conditions: oReading RXDATA portion of the SPIBUF register. oWriting ΓÇÿ1ΓÇÖ to clear the RXINTFLG bit in SPIFLG register. Write-Clearing the RXINTFLG bit before reading the SPIBUF indicates the received data is being ignored. Conversely, RXINTFLG can be cleared by reading the RXDATA portion of the SPIBUF register. So, reading the full of SPIBUF register itself clears the Receiver Full Interrupt Flag."]
    #[inline(always)]
    #[must_use]
    pub fn rxempty(&mut self) -> RxemptyW<SpibufSpec> {
        RxemptyW::new(self, 31)
    }
}
#[doc = "SPI / MibSPI Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spibuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spibuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpibufSpec;
impl crate::RegisterSpec for SpibufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spibuf::R`](R) reader structure"]
impl crate::Readable for SpibufSpec {}
#[doc = "`write(|w| ..)` method takes [`spibuf::W`](W) writer structure"]
impl crate::Writable for SpibufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIBUF to value 0"]
impl crate::Resettable for SpibufSpec {
    const RESET_VALUE: u32 = 0;
}
