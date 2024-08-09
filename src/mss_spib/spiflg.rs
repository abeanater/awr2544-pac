#[doc = "Register `SPIFLG` reader"]
pub type R = crate::R<SpiflgSpec>;
#[doc = "Register `SPIFLG` writer"]
pub type W = crate::W<SpiflgSpec>;
#[doc = "Field `DLENERRFLG` reader - 0:0\\]
Data Length Error Flag. 1 = A Data Length Error has occured. 0 = No Data Length Error has occured. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ. A Data Length Error occurs under the following conditions. Master: In a 4-pin with SPIENA mode or 5-pin mode, if the SPIENA pin from the slave is deasserted before the Master has completed its transfer, the Data Length Error is set. That is, if the Character Length counter has not completed counting while SPIENA pin deassertion is detected, then it means that the Slave has neither received full data from the Master nor has it transmitted complete data. Slave: In a 4-pin with ChipSelects mode or 5-pin mode, if the incoming valid SPISCS pin is de-activated before the Character Length counter completes counting, then Data Length Error is set. Note: Clearing of Transmission Error Flags in SPIBUF during Error conditions Whenever any Transmission Errors (TIMEOUT, BITERR, DLEN_ERR, PARITY_ERR, DESYNC) are detected, and the Error Flag are cleared by writing to the Error bit in SPIFLG register, the corresponding Error flag in SPIBUF does not get cleared. Software needs to read the SPIBUF until it becomes empty before proceeding. This ensures that all the older status bits in SPIBUF are cleared before starting the next transfer."]
pub type DlenerrflgR = crate::BitReader;
#[doc = "Field `DLENERRFLG` writer - 0:0\\]
Data Length Error Flag. 1 = A Data Length Error has occured. 0 = No Data Length Error has occured. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ. A Data Length Error occurs under the following conditions. Master: In a 4-pin with SPIENA mode or 5-pin mode, if the SPIENA pin from the slave is deasserted before the Master has completed its transfer, the Data Length Error is set. That is, if the Character Length counter has not completed counting while SPIENA pin deassertion is detected, then it means that the Slave has neither received full data from the Master nor has it transmitted complete data. Slave: In a 4-pin with ChipSelects mode or 5-pin mode, if the incoming valid SPISCS pin is de-activated before the Character Length counter completes counting, then Data Length Error is set. Note: Clearing of Transmission Error Flags in SPIBUF during Error conditions Whenever any Transmission Errors (TIMEOUT, BITERR, DLEN_ERR, PARITY_ERR, DESYNC) are detected, and the Error Flag are cleared by writing to the Error bit in SPIFLG register, the corresponding Error flag in SPIBUF does not get cleared. Software needs to read the SPIBUF until it becomes empty before proceeding. This ensures that all the older status bits in SPIBUF are cleared before starting the next transfer."]
pub type DlenerrflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTFLG` reader - 1:1\\]
Time-out due to non-activation of ENA signal. 1 =An ENA signal time-out occurred. The SPI / MibSPI generates a time-out because the slave hasnΓÇÖt responded in time by activating the ENA signal after the chip select signal has been activated. If a time-out condition is detected the corresponding chip select is deactivated immediately and the TIMEOUT flag is set. In addition the TIMOUT flag in the status field of the corresponding buffer is set. The transmit request of the concerned buffer is cleared, i.e. the SPI / MibSPI doesnΓÇÖt re-start a data transfer from this buffer. 0 =No ENA-signal time-out occurred. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ."]
pub type TimeoutflgR = crate::BitReader;
#[doc = "Field `TIMEOUTFLG` writer - 1:1\\]
Time-out due to non-activation of ENA signal. 1 =An ENA signal time-out occurred. The SPI / MibSPI generates a time-out because the slave hasnΓÇÖt responded in time by activating the ENA signal after the chip select signal has been activated. If a time-out condition is detected the corresponding chip select is deactivated immediately and the TIMEOUT flag is set. In addition the TIMOUT flag in the status field of the corresponding buffer is set. The transmit request of the concerned buffer is cleared, i.e. the SPI / MibSPI doesnΓÇÖt re-start a data transfer from this buffer. 0 =No ENA-signal time-out occurred. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ."]
pub type TimeoutflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARERRFLG` reader - 2:2\\]
Calculated parity differs from received parity bit. 1 =A parity error occurred. If the parity generator is enabled (can be selected individually for each buffer) an even or odd parity bit is added at the end of a data word (see Section 8.23). During reception of the data word the parity generator calculates the reference parity and compares it to the received parity bit. In the event of a mismatch the PARITYERR flag is set and an interrupt is asserted if PARERRENA is set. 0 =No parity error detected. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ."]
pub type ParerrflgR = crate::BitReader;
#[doc = "Field `PARERRFLG` writer - 2:2\\]
Calculated parity differs from received parity bit. 1 =A parity error occurred. If the parity generator is enabled (can be selected individually for each buffer) an even or odd parity bit is added at the end of a data word (see Section 8.23). During reception of the data word the parity generator calculates the reference parity and compares it to the received parity bit. In the event of a mismatch the PARITYERR flag is set and an interrupt is asserted if PARERRENA is set. 0 =No parity error detected. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ."]
pub type ParerrflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESYNCFLG` reader - 3:3\\]
De-synchronization of slave device. De-synchronization monitor is active in master mode only. 1 = A slave device is de-synchronized. The master monitors the ENAble signal coming from the slave device and sets the DESYNC flag after the last bit is transmitted plus tT2EDELAY (see Section 8.21). If DESYNCENA is set an interrupt is asserted. De-synchronization can occur if a slave device misses a clock edge coming from the master. 0 =No slave de-synchronization detected. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ. Note: Inconsistency of Desync flag in SPI/Compatibility mode MibSPI Due to the nature of this Error, under some circumstances it is possible for Desync error detected for the previous buffer to be visible in the current buffer. This is due to the fact that Receive Completion flag/interrupt will be generated when the buffer transfer is completed. But Desync will be detected after the buffer transfer is completed. So, if VBUS master reads the received data quickly when an RXINT is detected, then the status flag may not reflect the correct Desync condition. This inconsistency in Desync flag is valid only in SPI or Compatibility mode of MibSPI. In Multibuffer mode, Desync flag is always guaranteed to be for the current buffer."]
pub type DesyncflgR = crate::BitReader;
#[doc = "Field `DESYNCFLG` writer - 3:3\\]
De-synchronization of slave device. De-synchronization monitor is active in master mode only. 1 = A slave device is de-synchronized. The master monitors the ENAble signal coming from the slave device and sets the DESYNC flag after the last bit is transmitted plus tT2EDELAY (see Section 8.21). If DESYNCENA is set an interrupt is asserted. De-synchronization can occur if a slave device misses a clock edge coming from the master. 0 =No slave de-synchronization detected. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ. Note: Inconsistency of Desync flag in SPI/Compatibility mode MibSPI Due to the nature of this Error, under some circumstances it is possible for Desync error detected for the previous buffer to be visible in the current buffer. This is due to the fact that Receive Completion flag/interrupt will be generated when the buffer transfer is completed. But Desync will be detected after the buffer transfer is completed. So, if VBUS master reads the received data quickly when an RXINT is detected, then the status flag may not reflect the correct Desync condition. This inconsistency in Desync flag is valid only in SPI or Compatibility mode of MibSPI. In Multibuffer mode, Desync flag is always guaranteed to be for the current buffer."]
pub type DesyncflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITERRFLG` reader - 4:4\\]
Mismatch of internal transmit data and transmitted data. 1 =A bit error occurred. The SPI / MibSPI samples the signal of the transmit pin (master: SIMO, slave: SOMI) at the receive point (half clock cycle after transmit point). If the sampled value differs from the transmitted value a bit error is detected and the Flag BITERR is set. If BITERRENA is set an interrupt is asserted. A possible reason for a bit error can be a to high bit rate / capacitive load or another master/slave trying to transmit at the same time. 0 =No bit error occurred. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ."]
pub type BiterrflgR = crate::BitReader;
#[doc = "Field `BITERRFLG` writer - 4:4\\]
Mismatch of internal transmit data and transmitted data. 1 =A bit error occurred. The SPI / MibSPI samples the signal of the transmit pin (master: SIMO, slave: SOMI) at the receive point (half clock cycle after transmit point). If the sampled value differs from the transmitted value a bit error is detected and the Flag BITERR is set. If BITERRENA is set an interrupt is asserted. A possible reason for a bit error can be a to high bit rate / capacitive load or another master/slave trying to transmit at the same time. 0 =No bit error occurred. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ."]
pub type BiterrflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 5:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1R = crate::BitReader;
#[doc = "Field `NU1` writer - 5:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRNINTFLG` reader - 6:6\\]
Receiver overrun flag. The SPI / MibSPI hardware sets this bit when a receive operation completes before the previous character has been read from the receive buffer. The bit indicates that the last received character has been overwritten and therefore lost. The SPI / MibSPI will generate an interrupt request if this bit is set and the OVRN INTEN bit (SPIINT0.6) is set high. 0 = Overrun condition did not occur 1 = Overrun condition has occurred In SPI or Compatibility mode of MibSPI, this bit is cleared under the following conditions: Reading TGINTVECT0 or TGINTVECT1 register when there is a ΓÇ£Receive Buffer OverrunΓÇ¥ interrupt Writing a ΓÇÿ1ΓÇÖ to OVRNINTFLG in SPIFLG register itself Reading SPIBUF register does not clear this OVRNINTFLG bit. If an RXOVRN interrupt is detected, then the SPIBUF may need to be read twice to get to the Overrun buffer. This is due to the fact that the Overrun will always occur to the internal RXBUF. Each read to the SPIBUF will result in RXBUF contents (if it is full) getting copied to SPIBUF. Note: A special condition under which OVRNINTFLG flag gets set. If both SPIBUF &amp; RXBUF are already full and while another buffer receive is underway, if any errors like TIMEOUT, BITERR &amp; DLEN_ERR occur, then RXOVR in RXBUF &amp; OVRNINTFLG in SPIFLG registers will be set to indicate that the status flags are getting overwritten by the new transfer. This overrun should be treated like a normal Receiver Overrun. In Multibuffer mode of MibSPI, this bit is cleared under the following conditions. Reading the RXOVRN_BUF_ADDR register Writing a ΓÇÿ1ΓÇÖ to OVRNINTFLG in SPIFLG register itself In Multibuffer mode, if OVRNINTFLG is set, then the address of the buffer which experienced the Overrun is available in RXOVRN_BUF_ADDR."]
pub type OvrnintflgR = crate::BitReader;
#[doc = "Field `OVRNINTFLG` writer - 6:6\\]
Receiver overrun flag. The SPI / MibSPI hardware sets this bit when a receive operation completes before the previous character has been read from the receive buffer. The bit indicates that the last received character has been overwritten and therefore lost. The SPI / MibSPI will generate an interrupt request if this bit is set and the OVRN INTEN bit (SPIINT0.6) is set high. 0 = Overrun condition did not occur 1 = Overrun condition has occurred In SPI or Compatibility mode of MibSPI, this bit is cleared under the following conditions: Reading TGINTVECT0 or TGINTVECT1 register when there is a ΓÇ£Receive Buffer OverrunΓÇ¥ interrupt Writing a ΓÇÿ1ΓÇÖ to OVRNINTFLG in SPIFLG register itself Reading SPIBUF register does not clear this OVRNINTFLG bit. If an RXOVRN interrupt is detected, then the SPIBUF may need to be read twice to get to the Overrun buffer. This is due to the fact that the Overrun will always occur to the internal RXBUF. Each read to the SPIBUF will result in RXBUF contents (if it is full) getting copied to SPIBUF. Note: A special condition under which OVRNINTFLG flag gets set. If both SPIBUF &amp; RXBUF are already full and while another buffer receive is underway, if any errors like TIMEOUT, BITERR &amp; DLEN_ERR occur, then RXOVR in RXBUF &amp; OVRNINTFLG in SPIFLG registers will be set to indicate that the status flags are getting overwritten by the new transfer. This overrun should be treated like a normal Receiver Overrun. In Multibuffer mode of MibSPI, this bit is cleared under the following conditions. Reading the RXOVRN_BUF_ADDR register Writing a ΓÇÿ1ΓÇÖ to OVRNINTFLG in SPIFLG register itself In Multibuffer mode, if OVRNINTFLG is set, then the address of the buffer which experienced the Overrun is available in RXOVRN_BUF_ADDR."]
pub type OvrnintflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2R = crate::BitReader;
#[doc = "Field `NU2` writer - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINTFLG` reader - 8:8\\]
Receiver Full Interrupt Flag. This flag is set when a word is received and copied into the buffer register (SPIBUF). If RXINTEN is enabled, an interrupt is also generated. During emulation mode, however, a read to the emulation register (SPIEMU) does not clear this flag bit. This bit is cleared under the following ways: Reading the SPIBUF register Reading TGINTVECT0 or TGINTVECT1 register when there is a ΓÇ£Receive Buffer FullΓÇ¥ interrupt Writing a ΓÇÿ1ΓÇÖ to this bit Writing a ΓÇÿ0ΓÇÖ to SPIEN (SPIGCR1.24) System reset 0= No new received data pending. Receive buffer is Empty. 1= A newly received data is ready to be read. Receive buffer is full. Note: Exception for clearing of RXINT If both SPIBUF and RXBUF (internal buffer) are full, then, reading TGINTVECT0 or TGINTVECT1 register (while it shows 10010) does not clear the RXINTFLG in SPIFLG register. In this case, only way to clear the Interrupt is to read the SPIBUF (twice) and clear all the received data. Note: Side effects of Write Clear to RXINTFLG Clearing RXINTFLG bit by writing a ΓÇÿ1ΓÇÖ before reading the SPIBUF sets the RXEMPTY bit of the SPIBUF register too. This way, one can ignore a received data. However, if the internal RXBUF is already full, the data from RXBUF will be copied to SPIBUF and RXEMPTY bit will be cleared again. SPIBUF contents should be read first if this situation needs to be avoided."]
pub type RxintflgR = crate::BitReader;
#[doc = "Field `RXINTFLG` writer - 8:8\\]
Receiver Full Interrupt Flag. This flag is set when a word is received and copied into the buffer register (SPIBUF). If RXINTEN is enabled, an interrupt is also generated. During emulation mode, however, a read to the emulation register (SPIEMU) does not clear this flag bit. This bit is cleared under the following ways: Reading the SPIBUF register Reading TGINTVECT0 or TGINTVECT1 register when there is a ΓÇ£Receive Buffer FullΓÇ¥ interrupt Writing a ΓÇÿ1ΓÇÖ to this bit Writing a ΓÇÿ0ΓÇÖ to SPIEN (SPIGCR1.24) System reset 0= No new received data pending. Receive buffer is Empty. 1= A newly received data is ready to be read. Receive buffer is full. Note: Exception for clearing of RXINT If both SPIBUF and RXBUF (internal buffer) are full, then, reading TGINTVECT0 or TGINTVECT1 register (while it shows 10010) does not clear the RXINTFLG in SPIFLG register. In this case, only way to clear the Interrupt is to read the SPIBUF (twice) and clear all the received data. Note: Side effects of Write Clear to RXINTFLG Clearing RXINTFLG bit by writing a ΓÇÿ1ΓÇÖ before reading the SPIBUF sets the RXEMPTY bit of the SPIBUF register too. This way, one can ignore a received data. However, if the internal RXBUF is already full, the data from RXBUF will be copied to SPIBUF and RXEMPTY bit will be cleared again. SPIBUF contents should be read first if this situation needs to be avoided."]
pub type RxintflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINTFLG` reader - 9:9\\]
Transmitter Empty Interrupt Flag. Serves as an interrupt flag indicating that Transmit Buffer (TXBUF) is empty and a new data can be written to it. This flag is set when a data is copied to the ΓÇ£Shift RegisterΓÇ¥ either directly or from the TXBUF register. This bit is cleared by one of following ways: Writing a new data to either SPIDAT0 or SPIDAT1 Writing a ΓÇÿ0ΓÇÖ to SPIEN (SPIGCR1.24) 0= Transmit Buffer is now full. No interrupt pending for Transmitter Empty 1= Transmit Buffer is empty. An interrupt is pending to fill the transmitter."]
pub type TxintflgR = crate::BitReader;
#[doc = "Field `TXINTFLG` writer - 9:9\\]
Transmitter Empty Interrupt Flag. Serves as an interrupt flag indicating that Transmit Buffer (TXBUF) is empty and a new data can be written to it. This flag is set when a data is copied to the ΓÇ£Shift RegisterΓÇ¥ either directly or from the TXBUF register. This bit is cleared by one of following ways: Writing a new data to either SPIDAT0 or SPIDAT1 Writing a ΓÇÿ0ΓÇÖ to SPIEN (SPIGCR1.24) 0= Transmit Buffer is now full. No interrupt pending for Transmitter Empty 1= Transmit Buffer is empty. An interrupt is pending to fill the transmitter."]
pub type TxintflgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - 23:10\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3R = crate::FieldReader<u16>;
#[doc = "Field `NU3` writer - 23:10\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `BUFINITACTIVE` reader - 24:24\\]
Indicates the status of Multibuffer initialization process. Software should poll this bit to determine if it can proceed with the configuration of Multibuffer mode registers or Multibuffer RAM handling. Refer to Section 3.10.7 for details on Initialization of Multibuffer RAM. 1 = Multibuffer RAM is still being initialized. Do not attempt to write to either Multibuffer RAM or any Multibuffer mode registers. Refer to Section 3.3 for a classification of registers into compatibility mode and Multibuffer mode. 0 = Multibuffer RAM initialization is complete. This bit will show a value of ΓÇÿ1ΓÇÖ as long as the nRESET bit is ΓÇÿ0ΓÇÖ, but does not really indicate that Buffer initialization is underway. Internal automatic buffer initialization starts only when the nRESET bit is set to ΓÇÿ1ΓÇÖ. For SPI, this bit reads ΓÇÿ1ΓÇÖ always. For MibSPI, BUF INIT ACTIVE bit will show up as ΓÇÿ1ΓÇÖ for a maximum of 128/256 (will vary depending upon the actual size of the Multibuffer RAM implemented) VCLK cycles after the nRESET bit in GCR0 is set to ΓÇÿ1ΓÇÖ and then settle to ΓÇÿ0ΓÇÖ. If Auto Memory Initialization is triggered through System (MEM_AUTO_INIT pulse), then BUF INIT ACTIVE bit will show up as ΓÇÿ1ΓÇÖ for a maximum of 128/256 (will vary depending upon the actual size of the Multibuffer RAM implemented) VCLK cycles and then settle to ΓÇÿ0ΓÇÖ."]
pub type BufinitactiveR = crate::BitReader;
#[doc = "Field `BUFINITACTIVE` writer - 24:24\\]
Indicates the status of Multibuffer initialization process. Software should poll this bit to determine if it can proceed with the configuration of Multibuffer mode registers or Multibuffer RAM handling. Refer to Section 3.10.7 for details on Initialization of Multibuffer RAM. 1 = Multibuffer RAM is still being initialized. Do not attempt to write to either Multibuffer RAM or any Multibuffer mode registers. Refer to Section 3.3 for a classification of registers into compatibility mode and Multibuffer mode. 0 = Multibuffer RAM initialization is complete. This bit will show a value of ΓÇÿ1ΓÇÖ as long as the nRESET bit is ΓÇÿ0ΓÇÖ, but does not really indicate that Buffer initialization is underway. Internal automatic buffer initialization starts only when the nRESET bit is set to ΓÇÿ1ΓÇÖ. For SPI, this bit reads ΓÇÿ1ΓÇÖ always. For MibSPI, BUF INIT ACTIVE bit will show up as ΓÇÿ1ΓÇÖ for a maximum of 128/256 (will vary depending upon the actual size of the Multibuffer RAM implemented) VCLK cycles after the nRESET bit in GCR0 is set to ΓÇÿ1ΓÇÖ and then settle to ΓÇÿ0ΓÇÖ. If Auto Memory Initialization is triggered through System (MEM_AUTO_INIT pulse), then BUF INIT ACTIVE bit will show up as ΓÇÿ1ΓÇÖ for a maximum of 128/256 (will vary depending upon the actual size of the Multibuffer RAM implemented) VCLK cycles and then settle to ΓÇÿ0ΓÇÖ."]
pub type BufinitactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU4` reader - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4R = crate::FieldReader;
#[doc = "Field `NU4` writer - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data Length Error Flag. 1 = A Data Length Error has occured. 0 = No Data Length Error has occured. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ. A Data Length Error occurs under the following conditions. Master: In a 4-pin with SPIENA mode or 5-pin mode, if the SPIENA pin from the slave is deasserted before the Master has completed its transfer, the Data Length Error is set. That is, if the Character Length counter has not completed counting while SPIENA pin deassertion is detected, then it means that the Slave has neither received full data from the Master nor has it transmitted complete data. Slave: In a 4-pin with ChipSelects mode or 5-pin mode, if the incoming valid SPISCS pin is de-activated before the Character Length counter completes counting, then Data Length Error is set. Note: Clearing of Transmission Error Flags in SPIBUF during Error conditions Whenever any Transmission Errors (TIMEOUT, BITERR, DLEN_ERR, PARITY_ERR, DESYNC) are detected, and the Error Flag are cleared by writing to the Error bit in SPIFLG register, the corresponding Error flag in SPIBUF does not get cleared. Software needs to read the SPIBUF until it becomes empty before proceeding. This ensures that all the older status bits in SPIBUF are cleared before starting the next transfer."]
    #[inline(always)]
    pub fn dlenerrflg(&self) -> DlenerrflgR {
        DlenerrflgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Time-out due to non-activation of ENA signal. 1 =An ENA signal time-out occurred. The SPI / MibSPI generates a time-out because the slave hasnΓÇÖt responded in time by activating the ENA signal after the chip select signal has been activated. If a time-out condition is detected the corresponding chip select is deactivated immediately and the TIMEOUT flag is set. In addition the TIMOUT flag in the status field of the corresponding buffer is set. The transmit request of the concerned buffer is cleared, i.e. the SPI / MibSPI doesnΓÇÖt re-start a data transfer from this buffer. 0 =No ENA-signal time-out occurred. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    pub fn timeoutflg(&self) -> TimeoutflgR {
        TimeoutflgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Calculated parity differs from received parity bit. 1 =A parity error occurred. If the parity generator is enabled (can be selected individually for each buffer) an even or odd parity bit is added at the end of a data word (see Section 8.23). During reception of the data word the parity generator calculates the reference parity and compares it to the received parity bit. In the event of a mismatch the PARITYERR flag is set and an interrupt is asserted if PARERRENA is set. 0 =No parity error detected. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    pub fn parerrflg(&self) -> ParerrflgR {
        ParerrflgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
De-synchronization of slave device. De-synchronization monitor is active in master mode only. 1 = A slave device is de-synchronized. The master monitors the ENAble signal coming from the slave device and sets the DESYNC flag after the last bit is transmitted plus tT2EDELAY (see Section 8.21). If DESYNCENA is set an interrupt is asserted. De-synchronization can occur if a slave device misses a clock edge coming from the master. 0 =No slave de-synchronization detected. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ. Note: Inconsistency of Desync flag in SPI/Compatibility mode MibSPI Due to the nature of this Error, under some circumstances it is possible for Desync error detected for the previous buffer to be visible in the current buffer. This is due to the fact that Receive Completion flag/interrupt will be generated when the buffer transfer is completed. But Desync will be detected after the buffer transfer is completed. So, if VBUS master reads the received data quickly when an RXINT is detected, then the status flag may not reflect the correct Desync condition. This inconsistency in Desync flag is valid only in SPI or Compatibility mode of MibSPI. In Multibuffer mode, Desync flag is always guaranteed to be for the current buffer."]
    #[inline(always)]
    pub fn desyncflg(&self) -> DesyncflgR {
        DesyncflgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Mismatch of internal transmit data and transmitted data. 1 =A bit error occurred. The SPI / MibSPI samples the signal of the transmit pin (master: SIMO, slave: SOMI) at the receive point (half clock cycle after transmit point). If the sampled value differs from the transmitted value a bit error is detected and the Flag BITERR is set. If BITERRENA is set an interrupt is asserted. A possible reason for a bit error can be a to high bit rate / capacitive load or another master/slave trying to transmit at the same time. 0 =No bit error occurred. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    pub fn biterrflg(&self) -> BiterrflgR {
        BiterrflgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Receiver overrun flag. The SPI / MibSPI hardware sets this bit when a receive operation completes before the previous character has been read from the receive buffer. The bit indicates that the last received character has been overwritten and therefore lost. The SPI / MibSPI will generate an interrupt request if this bit is set and the OVRN INTEN bit (SPIINT0.6) is set high. 0 = Overrun condition did not occur 1 = Overrun condition has occurred In SPI or Compatibility mode of MibSPI, this bit is cleared under the following conditions: Reading TGINTVECT0 or TGINTVECT1 register when there is a ΓÇ£Receive Buffer OverrunΓÇ¥ interrupt Writing a ΓÇÿ1ΓÇÖ to OVRNINTFLG in SPIFLG register itself Reading SPIBUF register does not clear this OVRNINTFLG bit. If an RXOVRN interrupt is detected, then the SPIBUF may need to be read twice to get to the Overrun buffer. This is due to the fact that the Overrun will always occur to the internal RXBUF. Each read to the SPIBUF will result in RXBUF contents (if it is full) getting copied to SPIBUF. Note: A special condition under which OVRNINTFLG flag gets set. If both SPIBUF &amp; RXBUF are already full and while another buffer receive is underway, if any errors like TIMEOUT, BITERR &amp; DLEN_ERR occur, then RXOVR in RXBUF &amp; OVRNINTFLG in SPIFLG registers will be set to indicate that the status flags are getting overwritten by the new transfer. This overrun should be treated like a normal Receiver Overrun. In Multibuffer mode of MibSPI, this bit is cleared under the following conditions. Reading the RXOVRN_BUF_ADDR register Writing a ΓÇÿ1ΓÇÖ to OVRNINTFLG in SPIFLG register itself In Multibuffer mode, if OVRNINTFLG is set, then the address of the buffer which experienced the Overrun is available in RXOVRN_BUF_ADDR."]
    #[inline(always)]
    pub fn ovrnintflg(&self) -> OvrnintflgR {
        OvrnintflgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Receiver Full Interrupt Flag. This flag is set when a word is received and copied into the buffer register (SPIBUF). If RXINTEN is enabled, an interrupt is also generated. During emulation mode, however, a read to the emulation register (SPIEMU) does not clear this flag bit. This bit is cleared under the following ways: Reading the SPIBUF register Reading TGINTVECT0 or TGINTVECT1 register when there is a ΓÇ£Receive Buffer FullΓÇ¥ interrupt Writing a ΓÇÿ1ΓÇÖ to this bit Writing a ΓÇÿ0ΓÇÖ to SPIEN (SPIGCR1.24) System reset 0= No new received data pending. Receive buffer is Empty. 1= A newly received data is ready to be read. Receive buffer is full. Note: Exception for clearing of RXINT If both SPIBUF and RXBUF (internal buffer) are full, then, reading TGINTVECT0 or TGINTVECT1 register (while it shows 10010) does not clear the RXINTFLG in SPIFLG register. In this case, only way to clear the Interrupt is to read the SPIBUF (twice) and clear all the received data. Note: Side effects of Write Clear to RXINTFLG Clearing RXINTFLG bit by writing a ΓÇÿ1ΓÇÖ before reading the SPIBUF sets the RXEMPTY bit of the SPIBUF register too. This way, one can ignore a received data. However, if the internal RXBUF is already full, the data from RXBUF will be copied to SPIBUF and RXEMPTY bit will be cleared again. SPIBUF contents should be read first if this situation needs to be avoided."]
    #[inline(always)]
    pub fn rxintflg(&self) -> RxintflgR {
        RxintflgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmitter Empty Interrupt Flag. Serves as an interrupt flag indicating that Transmit Buffer (TXBUF) is empty and a new data can be written to it. This flag is set when a data is copied to the ΓÇ£Shift RegisterΓÇ¥ either directly or from the TXBUF register. This bit is cleared by one of following ways: Writing a new data to either SPIDAT0 or SPIDAT1 Writing a ΓÇÿ0ΓÇÖ to SPIEN (SPIGCR1.24) 0= Transmit Buffer is now full. No interrupt pending for Transmitter Empty 1= Transmit Buffer is empty. An interrupt is pending to fill the transmitter."]
    #[inline(always)]
    pub fn txintflg(&self) -> TxintflgR {
        TxintflgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:23 - 23:10\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 10) & 0x3fff) as u16)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates the status of Multibuffer initialization process. Software should poll this bit to determine if it can proceed with the configuration of Multibuffer mode registers or Multibuffer RAM handling. Refer to Section 3.10.7 for details on Initialization of Multibuffer RAM. 1 = Multibuffer RAM is still being initialized. Do not attempt to write to either Multibuffer RAM or any Multibuffer mode registers. Refer to Section 3.3 for a classification of registers into compatibility mode and Multibuffer mode. 0 = Multibuffer RAM initialization is complete. This bit will show a value of ΓÇÿ1ΓÇÖ as long as the nRESET bit is ΓÇÿ0ΓÇÖ, but does not really indicate that Buffer initialization is underway. Internal automatic buffer initialization starts only when the nRESET bit is set to ΓÇÿ1ΓÇÖ. For SPI, this bit reads ΓÇÿ1ΓÇÖ always. For MibSPI, BUF INIT ACTIVE bit will show up as ΓÇÿ1ΓÇÖ for a maximum of 128/256 (will vary depending upon the actual size of the Multibuffer RAM implemented) VCLK cycles after the nRESET bit in GCR0 is set to ΓÇÿ1ΓÇÖ and then settle to ΓÇÿ0ΓÇÖ. If Auto Memory Initialization is triggered through System (MEM_AUTO_INIT pulse), then BUF INIT ACTIVE bit will show up as ΓÇÿ1ΓÇÖ for a maximum of 128/256 (will vary depending upon the actual size of the Multibuffer RAM implemented) VCLK cycles and then settle to ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    pub fn bufinitactive(&self) -> BufinitactiveR {
        BufinitactiveR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data Length Error Flag. 1 = A Data Length Error has occured. 0 = No Data Length Error has occured. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ. A Data Length Error occurs under the following conditions. Master: In a 4-pin with SPIENA mode or 5-pin mode, if the SPIENA pin from the slave is deasserted before the Master has completed its transfer, the Data Length Error is set. That is, if the Character Length counter has not completed counting while SPIENA pin deassertion is detected, then it means that the Slave has neither received full data from the Master nor has it transmitted complete data. Slave: In a 4-pin with ChipSelects mode or 5-pin mode, if the incoming valid SPISCS pin is de-activated before the Character Length counter completes counting, then Data Length Error is set. Note: Clearing of Transmission Error Flags in SPIBUF during Error conditions Whenever any Transmission Errors (TIMEOUT, BITERR, DLEN_ERR, PARITY_ERR, DESYNC) are detected, and the Error Flag are cleared by writing to the Error bit in SPIFLG register, the corresponding Error flag in SPIBUF does not get cleared. Software needs to read the SPIBUF until it becomes empty before proceeding. This ensures that all the older status bits in SPIBUF are cleared before starting the next transfer."]
    #[inline(always)]
    #[must_use]
    pub fn dlenerrflg(&mut self) -> DlenerrflgW<SpiflgSpec> {
        DlenerrflgW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Time-out due to non-activation of ENA signal. 1 =An ENA signal time-out occurred. The SPI / MibSPI generates a time-out because the slave hasnΓÇÖt responded in time by activating the ENA signal after the chip select signal has been activated. If a time-out condition is detected the corresponding chip select is deactivated immediately and the TIMEOUT flag is set. In addition the TIMOUT flag in the status field of the corresponding buffer is set. The transmit request of the concerned buffer is cleared, i.e. the SPI / MibSPI doesnΓÇÖt re-start a data transfer from this buffer. 0 =No ENA-signal time-out occurred. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    #[must_use]
    pub fn timeoutflg(&mut self) -> TimeoutflgW<SpiflgSpec> {
        TimeoutflgW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Calculated parity differs from received parity bit. 1 =A parity error occurred. If the parity generator is enabled (can be selected individually for each buffer) an even or odd parity bit is added at the end of a data word (see Section 8.23). During reception of the data word the parity generator calculates the reference parity and compares it to the received parity bit. In the event of a mismatch the PARITYERR flag is set and an interrupt is asserted if PARERRENA is set. 0 =No parity error detected. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    #[must_use]
    pub fn parerrflg(&mut self) -> ParerrflgW<SpiflgSpec> {
        ParerrflgW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
De-synchronization of slave device. De-synchronization monitor is active in master mode only. 1 = A slave device is de-synchronized. The master monitors the ENAble signal coming from the slave device and sets the DESYNC flag after the last bit is transmitted plus tT2EDELAY (see Section 8.21). If DESYNCENA is set an interrupt is asserted. De-synchronization can occur if a slave device misses a clock edge coming from the master. 0 =No slave de-synchronization detected. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ. Note: Inconsistency of Desync flag in SPI/Compatibility mode MibSPI Due to the nature of this Error, under some circumstances it is possible for Desync error detected for the previous buffer to be visible in the current buffer. This is due to the fact that Receive Completion flag/interrupt will be generated when the buffer transfer is completed. But Desync will be detected after the buffer transfer is completed. So, if VBUS master reads the received data quickly when an RXINT is detected, then the status flag may not reflect the correct Desync condition. This inconsistency in Desync flag is valid only in SPI or Compatibility mode of MibSPI. In Multibuffer mode, Desync flag is always guaranteed to be for the current buffer."]
    #[inline(always)]
    #[must_use]
    pub fn desyncflg(&mut self) -> DesyncflgW<SpiflgSpec> {
        DesyncflgW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Mismatch of internal transmit data and transmitted data. 1 =A bit error occurred. The SPI / MibSPI samples the signal of the transmit pin (master: SIMO, slave: SOMI) at the receive point (half clock cycle after transmit point). If the sampled value differs from the transmitted value a bit error is detected and the Flag BITERR is set. If BITERRENA is set an interrupt is asserted. A possible reason for a bit error can be a to high bit rate / capacitive load or another master/slave trying to transmit at the same time. 0 =No bit error occurred. This flag can be cleared by one of the following ways. Write a ΓÇÿ1ΓÇÖ to this bit. Set SPIEN bit to ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    #[must_use]
    pub fn biterrflg(&mut self) -> BiterrflgW<SpiflgSpec> {
        BiterrflgW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<SpiflgSpec> {
        Nu1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Receiver overrun flag. The SPI / MibSPI hardware sets this bit when a receive operation completes before the previous character has been read from the receive buffer. The bit indicates that the last received character has been overwritten and therefore lost. The SPI / MibSPI will generate an interrupt request if this bit is set and the OVRN INTEN bit (SPIINT0.6) is set high. 0 = Overrun condition did not occur 1 = Overrun condition has occurred In SPI or Compatibility mode of MibSPI, this bit is cleared under the following conditions: Reading TGINTVECT0 or TGINTVECT1 register when there is a ΓÇ£Receive Buffer OverrunΓÇ¥ interrupt Writing a ΓÇÿ1ΓÇÖ to OVRNINTFLG in SPIFLG register itself Reading SPIBUF register does not clear this OVRNINTFLG bit. If an RXOVRN interrupt is detected, then the SPIBUF may need to be read twice to get to the Overrun buffer. This is due to the fact that the Overrun will always occur to the internal RXBUF. Each read to the SPIBUF will result in RXBUF contents (if it is full) getting copied to SPIBUF. Note: A special condition under which OVRNINTFLG flag gets set. If both SPIBUF &amp; RXBUF are already full and while another buffer receive is underway, if any errors like TIMEOUT, BITERR &amp; DLEN_ERR occur, then RXOVR in RXBUF &amp; OVRNINTFLG in SPIFLG registers will be set to indicate that the status flags are getting overwritten by the new transfer. This overrun should be treated like a normal Receiver Overrun. In Multibuffer mode of MibSPI, this bit is cleared under the following conditions. Reading the RXOVRN_BUF_ADDR register Writing a ΓÇÿ1ΓÇÖ to OVRNINTFLG in SPIFLG register itself In Multibuffer mode, if OVRNINTFLG is set, then the address of the buffer which experienced the Overrun is available in RXOVRN_BUF_ADDR."]
    #[inline(always)]
    #[must_use]
    pub fn ovrnintflg(&mut self) -> OvrnintflgW<SpiflgSpec> {
        OvrnintflgW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<SpiflgSpec> {
        Nu2W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Receiver Full Interrupt Flag. This flag is set when a word is received and copied into the buffer register (SPIBUF). If RXINTEN is enabled, an interrupt is also generated. During emulation mode, however, a read to the emulation register (SPIEMU) does not clear this flag bit. This bit is cleared under the following ways: Reading the SPIBUF register Reading TGINTVECT0 or TGINTVECT1 register when there is a ΓÇ£Receive Buffer FullΓÇ¥ interrupt Writing a ΓÇÿ1ΓÇÖ to this bit Writing a ΓÇÿ0ΓÇÖ to SPIEN (SPIGCR1.24) System reset 0= No new received data pending. Receive buffer is Empty. 1= A newly received data is ready to be read. Receive buffer is full. Note: Exception for clearing of RXINT If both SPIBUF and RXBUF (internal buffer) are full, then, reading TGINTVECT0 or TGINTVECT1 register (while it shows 10010) does not clear the RXINTFLG in SPIFLG register. In this case, only way to clear the Interrupt is to read the SPIBUF (twice) and clear all the received data. Note: Side effects of Write Clear to RXINTFLG Clearing RXINTFLG bit by writing a ΓÇÿ1ΓÇÖ before reading the SPIBUF sets the RXEMPTY bit of the SPIBUF register too. This way, one can ignore a received data. However, if the internal RXBUF is already full, the data from RXBUF will be copied to SPIBUF and RXEMPTY bit will be cleared again. SPIBUF contents should be read first if this situation needs to be avoided."]
    #[inline(always)]
    #[must_use]
    pub fn rxintflg(&mut self) -> RxintflgW<SpiflgSpec> {
        RxintflgW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmitter Empty Interrupt Flag. Serves as an interrupt flag indicating that Transmit Buffer (TXBUF) is empty and a new data can be written to it. This flag is set when a data is copied to the ΓÇ£Shift RegisterΓÇ¥ either directly or from the TXBUF register. This bit is cleared by one of following ways: Writing a new data to either SPIDAT0 or SPIDAT1 Writing a ΓÇÿ0ΓÇÖ to SPIEN (SPIGCR1.24) 0= Transmit Buffer is now full. No interrupt pending for Transmitter Empty 1= Transmit Buffer is empty. An interrupt is pending to fill the transmitter."]
    #[inline(always)]
    #[must_use]
    pub fn txintflg(&mut self) -> TxintflgW<SpiflgSpec> {
        TxintflgW::new(self, 9)
    }
    #[doc = "Bits 10:23 - 23:10\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<SpiflgSpec> {
        Nu3W::new(self, 10)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates the status of Multibuffer initialization process. Software should poll this bit to determine if it can proceed with the configuration of Multibuffer mode registers or Multibuffer RAM handling. Refer to Section 3.10.7 for details on Initialization of Multibuffer RAM. 1 = Multibuffer RAM is still being initialized. Do not attempt to write to either Multibuffer RAM or any Multibuffer mode registers. Refer to Section 3.3 for a classification of registers into compatibility mode and Multibuffer mode. 0 = Multibuffer RAM initialization is complete. This bit will show a value of ΓÇÿ1ΓÇÖ as long as the nRESET bit is ΓÇÿ0ΓÇÖ, but does not really indicate that Buffer initialization is underway. Internal automatic buffer initialization starts only when the nRESET bit is set to ΓÇÿ1ΓÇÖ. For SPI, this bit reads ΓÇÿ1ΓÇÖ always. For MibSPI, BUF INIT ACTIVE bit will show up as ΓÇÿ1ΓÇÖ for a maximum of 128/256 (will vary depending upon the actual size of the Multibuffer RAM implemented) VCLK cycles after the nRESET bit in GCR0 is set to ΓÇÿ1ΓÇÖ and then settle to ΓÇÿ0ΓÇÖ. If Auto Memory Initialization is triggered through System (MEM_AUTO_INIT pulse), then BUF INIT ACTIVE bit will show up as ΓÇÿ1ΓÇÖ for a maximum of 128/256 (will vary depending upon the actual size of the Multibuffer RAM implemented) VCLK cycles and then settle to ΓÇÿ0ΓÇÖ."]
    #[inline(always)]
    #[must_use]
    pub fn bufinitactive(&mut self) -> BufinitactiveW<SpiflgSpec> {
        BufinitactiveW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<SpiflgSpec> {
        Nu4W::new(self, 25)
    }
}
#[doc = "SPI / MibSPI Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spiflg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spiflg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiflgSpec;
impl crate::RegisterSpec for SpiflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spiflg::R`](R) reader structure"]
impl crate::Readable for SpiflgSpec {}
#[doc = "`write(|w| ..)` method takes [`spiflg::W`](W) writer structure"]
impl crate::Writable for SpiflgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIFLG to value 0"]
impl crate::Resettable for SpiflgSpec {
    const RESET_VALUE: u32 = 0;
}
