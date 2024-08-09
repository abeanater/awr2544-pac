#[doc = "Register `SPIFMT2` reader"]
pub type R = crate::R<Spifmt2Spec>;
#[doc = "Register `SPIFMT2` writer"]
pub type W = crate::W<Spifmt2Spec>;
#[doc = "Field `CHARLEN` reader - 4:0\\]
SPI data format x data word length. CHARLENx defines the word length of data format x. Legal values are 0x02 (data word length = 2 bit) to 0x10 (data word length = 16). Illegal values, such as 0x00 or 0x1F are not detected and their effect is indeterminate."]
pub type CharlenR = crate::FieldReader;
#[doc = "Field `CHARLEN` writer - 4:0\\]
SPI data format x data word length. CHARLENx defines the word length of data format x. Legal values are 0x02 (data word length = 2 bit) to 0x10 (data word length = 16). Illegal values, such as 0x00 or 0x1F are not detected and their effect is indeterminate."]
pub type CharlenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU` reader - 7:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - 7:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRESCALE` reader - 15:8\\]
SPI data format x prescaler. PRESCALEx can be modified in privilege mode only. PRESCALEx determines the bit transfer rate of data format x if the SPI is the network master. PRESCALEx is directly derived from VBUSPCLK. If the SPI / MibSPI is configured as slave, PRESCALEx DOES NOT NEED to be configured. The clock rate for data format x can be calculated as BRFormat = VBUSPCLCK/(PRESCALEx+1) When PRESCALEx is set to zero (0), the SPI clock rate defaults to VBUSPCLK/2. Any write to this field will update EPRESCALE_FMTx field of EPRESCALEy (y=1,2) registers with EPRESCALE_FMTx(11:8) bits rounded off to ΓÇÿ000ΓÇÖ. Any write to EPRESCALE_FMTx field of the EXTENDED_PRESCALEy (y=1,2) register will cause its lower 8bits to be reflected in this field as well."]
pub type PrescaleR = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - 15:8\\]
SPI data format x prescaler. PRESCALEx can be modified in privilege mode only. PRESCALEx determines the bit transfer rate of data format x if the SPI is the network master. PRESCALEx is directly derived from VBUSPCLK. If the SPI / MibSPI is configured as slave, PRESCALEx DOES NOT NEED to be configured. The clock rate for data format x can be calculated as BRFormat = VBUSPCLCK/(PRESCALEx+1) When PRESCALEx is set to zero (0), the SPI clock rate defaults to VBUSPCLK/2. Any write to this field will update EPRESCALE_FMTx field of EPRESCALEy (y=1,2) registers with EPRESCALE_FMTx(11:8) bits rounded off to ΓÇÿ000ΓÇÖ. Any write to EPRESCALE_FMTx field of the EXTENDED_PRESCALEy (y=1,2) register will cause its lower 8bits to be reflected in this field as well."]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHASE` reader - 16:16\\]
SPI Data format x clock delay. PHASEx defines the clock delay of data format x. PHASEx can be modified in privilege mode only. 1 =If PHASEx is set to ΓÇ£1ΓÇ¥ the SPI clock signal is delayed by a half SPI clock cycle versus the transmit / receive data stream. The first transmit bit has to output prior to the first clock edge. Master and slave receive the first bit with the first edge 0 =If PHASEx is set to ΓÇ£0ΓÇ¥ the SPI clock signal is not delayed versus the transmit / receive data stream. The first data bit is transmitted with the first clock edge and the first bit is received with the second (inverse) clock edge Note: Restriction on SPICLK Phase/Polarity change in Slave Mode In Slave mode if Phase and/or Polarity of SPICLK has to be changed, the following sequence should be used. oClear the GCR1.SPIEN bit to ΓÇÿ0ΓÇÖ. oSet the required Phase/Polarity values in SPIFMTx registers. oSet the GCR1.SPIEN bit back to ΓÇÿ1ΓÇÖ. The setting of GCR1.SPIEN bit in Slave SPI/MibSPI to ΓÇÿ1ΓÇÖ should be done only after the Polarity of the incoming SPICLK signal changes (if POLARITYx bit was changed in the configuration)."]
pub type PhaseR = crate::BitReader;
#[doc = "Field `PHASE` writer - 16:16\\]
SPI Data format x clock delay. PHASEx defines the clock delay of data format x. PHASEx can be modified in privilege mode only. 1 =If PHASEx is set to ΓÇ£1ΓÇ¥ the SPI clock signal is delayed by a half SPI clock cycle versus the transmit / receive data stream. The first transmit bit has to output prior to the first clock edge. Master and slave receive the first bit with the first edge 0 =If PHASEx is set to ΓÇ£0ΓÇ¥ the SPI clock signal is not delayed versus the transmit / receive data stream. The first data bit is transmitted with the first clock edge and the first bit is received with the second (inverse) clock edge Note: Restriction on SPICLK Phase/Polarity change in Slave Mode In Slave mode if Phase and/or Polarity of SPICLK has to be changed, the following sequence should be used. oClear the GCR1.SPIEN bit to ΓÇÿ0ΓÇÖ. oSet the required Phase/Polarity values in SPIFMTx registers. oSet the GCR1.SPIEN bit back to ΓÇÿ1ΓÇÖ. The setting of GCR1.SPIEN bit in Slave SPI/MibSPI to ΓÇÿ1ΓÇÖ should be done only after the Polarity of the incoming SPICLK signal changes (if POLARITYx bit was changed in the configuration)."]
pub type PhaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLARITY` reader - 17:17\\]
SPI data format x clock polarity. POLARITYx defines the clock polarity of data format x. POLARITYx can be modified in privilege mode only. 1 =If POLARITYx is set to ΓÇ£1ΓÇ¥ the SPI clock signal is high-inactive, i.e. before and after data transfer the clock signal is high. 0 =If POLARITYx is set to ΓÇ£0ΓÇ¥ the SPI clock signal is low-inactive, i.e. before and after data transfer the clock signal is low."]
pub type PolarityR = crate::BitReader;
#[doc = "Field `POLARITY` writer - 17:17\\]
SPI data format x clock polarity. POLARITYx defines the clock polarity of data format x. POLARITYx can be modified in privilege mode only. 1 =If POLARITYx is set to ΓÇ£1ΓÇ¥ the SPI clock signal is high-inactive, i.e. before and after data transfer the clock signal is high. 0 =If POLARITYx is set to ΓÇ£0ΓÇ¥ the SPI clock signal is low-inactive, i.e. before and after data transfer the clock signal is low."]
pub type PolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCSTIMERS` reader - 18:18\\]
Disable Chipselect Timers for this format register. The C2TDELAY &amp; T2CDELAY timers are by default enabled for all the Data Format registers. Using this bit, these timers can be disabled for particular Data Format if not required. When a Master is handling multiple Slaves, with varied set-up hold requirement, the application can selectively choose to include or not include the ChipSelect Delay timers for any Slaves. 0 = Both C2TDELAY &amp; T2CDELAY counts are inserted for the ChipSelects. 1 = No C2TDELAY or T2CDELAY is inserted in the ChipSelect timings."]
pub type DiscstimersR = crate::BitReader;
#[doc = "Field `DISCSTIMERS` writer - 18:18\\]
Disable Chipselect Timers for this format register. The C2TDELAY &amp; T2CDELAY timers are by default enabled for all the Data Format registers. Using this bit, these timers can be disabled for particular Data Format if not required. When a Master is handling multiple Slaves, with varied set-up hold requirement, the application can selectively choose to include or not include the ChipSelect Delay timers for any Slaves. 0 = Both C2TDELAY &amp; T2CDELAY counts are inserted for the ChipSelects. 1 = No C2TDELAY or T2CDELAY is inserted in the ChipSelect timings."]
pub type DiscstimersW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDUPLEX_ENA` reader - 19:19\\]
Half Duplex transfer mode enable for Data Format x. This bit controls the I/O function of SOMI/SIMO lines for a specific requirement where in the case of Master mode, TX pin - SIMO will act as an RX pin, and in the case of Slave mode, RX pin - SIMO will act as a TX pin. 0 = Normal Full Duplex transfer. 1 = If MASTER = ΓÇÿ1ΓÇÖ, SIMO pin will act as an RX pin (No TX possible) If MASTER = ΓÇÿ0ΓÇÖ, SIMO pin will act as a TX pin (No RX possible). For all normal operations, HDUPLEX_ENAx bits should always remain ΓÇÿ0ΓÇÖ. It is intended for the usage when the SIMO pin is used for both TX &amp; RX operations at different times."]
pub type HduplexEnaR = crate::BitReader;
#[doc = "Field `HDUPLEX_ENA` writer - 19:19\\]
Half Duplex transfer mode enable for Data Format x. This bit controls the I/O function of SOMI/SIMO lines for a specific requirement where in the case of Master mode, TX pin - SIMO will act as an RX pin, and in the case of Slave mode, RX pin - SIMO will act as a TX pin. 0 = Normal Full Duplex transfer. 1 = If MASTER = ΓÇÿ1ΓÇÖ, SIMO pin will act as an RX pin (No TX possible) If MASTER = ΓÇÿ0ΓÇÖ, SIMO pin will act as a TX pin (No RX possible). For all normal operations, HDUPLEX_ENAx bits should always remain ΓÇÿ0ΓÇÖ. It is intended for the usage when the SIMO pin is used for both TX &amp; RX operations at different times."]
pub type HduplexEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHIFTDIR` reader - 20:20\\]
Shift direction for data format x. With bit SHIFTDIRx the shift direction for data format x (x=0,1,2,3) can be selected. 1 =Data format x shift direction: Least significant bit is shifted out first. 0 =Data format x shift direction: Most significant bit is shifted out first."]
pub type ShiftdirR = crate::BitReader;
#[doc = "Field `SHIFTDIR` writer - 20:20\\]
Shift direction for data format x. With bit SHIFTDIRx the shift direction for data format x (x=0,1,2,3) can be selected. 1 =Data format x shift direction: Least significant bit is shifted out first. 0 =Data format x shift direction: Most significant bit is shifted out first."]
pub type ShiftdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITENA` reader - 21:21\\]
Master waits for ENA signal from slave for data format x. WAITENA is considered in master mode only. In slave mode this bit has no meaning. WAITENA enables a flexible SPI network where slaves with ENA signal and slaves without ENA signal can be mixed. WAITENA defines for each buffer whether the addressed slave generates the ENA signal or does not. 1= Before the SPI / MibSPI starts the data transfer it waits for the ENA signal to become low. If the ENA signal is not pulled down by the addressed slave before the internal time-out counter (C2EDELAY) overflows, then the Master aborts the transfer and sets the TIMEOUT error flag. 0= The SPI / MibSPI does not wait for the ENA signal from the slaves and directly starts the transfer."]
pub type WaitenaR = crate::BitReader;
#[doc = "Field `WAITENA` writer - 21:21\\]
Master waits for ENA signal from slave for data format x. WAITENA is considered in master mode only. In slave mode this bit has no meaning. WAITENA enables a flexible SPI network where slaves with ENA signal and slaves without ENA signal can be mixed. WAITENA defines for each buffer whether the addressed slave generates the ENA signal or does not. 1= Before the SPI / MibSPI starts the data transfer it waits for the ENA signal to become low. If the ENA signal is not pulled down by the addressed slave before the internal time-out counter (C2EDELAY) overflows, then the Master aborts the transfer and sets the TIMEOUT error flag. 0= The SPI / MibSPI does not wait for the ENA signal from the slaves and directly starts the transfer."]
pub type WaitenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITYENA` reader - 22:22\\]
Parity enable for data format x. 1= A parity is transmitted at the end of each transmit data stream. At the end of a transfer the parity generator compares the received parity bit with the locally calculated parity flag. If the parity bits do not match the RXERR flag is set in the corresponding control field. The parity type (even or odd) can be selected via the PARPOL bit. 0= No parity generation/ verification is performed for this data format. If an Uncorrectable Error Flag is set in a Slave mode MibSPI, then wrong parity bit will be transmitted to indicate to the master that there has been some issue with the data parity. The SOMI pin will be forced to transmit all ΓÇÿ0ΓÇÖs. And parity bit will be transmitted as ΓÇÿ1ΓÇÖ if even parity is selected and as ΓÇÿ0ΓÇÖ if odd parity is selected(using the PARPOLx bit of this register). This behavior will be irrespective of an UPE on either TXRAM or RXRAM."]
pub type ParityenaR = crate::BitReader;
#[doc = "Field `PARITYENA` writer - 22:22\\]
Parity enable for data format x. 1= A parity is transmitted at the end of each transmit data stream. At the end of a transfer the parity generator compares the received parity bit with the locally calculated parity flag. If the parity bits do not match the RXERR flag is set in the corresponding control field. The parity type (even or odd) can be selected via the PARPOL bit. 0= No parity generation/ verification is performed for this data format. If an Uncorrectable Error Flag is set in a Slave mode MibSPI, then wrong parity bit will be transmitted to indicate to the master that there has been some issue with the data parity. The SOMI pin will be forced to transmit all ΓÇÿ0ΓÇÖs. And parity bit will be transmitted as ΓÇÿ1ΓÇÖ if even parity is selected and as ΓÇÿ0ΓÇÖ if odd parity is selected(using the PARPOLx bit of this register). This behavior will be irrespective of an UPE on either TXRAM or RXRAM."]
pub type ParityenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARPOL` reader - 23:23\\]
Parity polarity: even or odd. PARPOLx can be modified in privilege mode only. It can be used for data format x (x= 0,1,2,3). 1 =An odd parity flag is added at the end of the transmit data stream. 0 =An even parity flag is added at the end of the transmit data stream."]
pub type ParpolR = crate::BitReader;
#[doc = "Field `PARPOL` writer - 23:23\\]
Parity polarity: even or odd. PARPOLx can be modified in privilege mode only. It can be used for data format x (x= 0,1,2,3). 1 =An odd parity flag is added at the end of the transmit data stream. 0 =An even parity flag is added at the end of the transmit data stream."]
pub type ParpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDELAY` reader - 31:24\\]
Delay in between transmissions for data format x (x= 0,1,2,3). Idle time that will be applied at the end of the current transmission if the bit WDEL is set in the current buffer. The delay to be applied is equal to: WDELAY * PVBUSPCLK + 2 * PVBUSPCLK. PVBUSPCLK -> Period of VBUSPCLK."]
pub type WdelayR = crate::FieldReader;
#[doc = "Field `WDELAY` writer - 31:24\\]
Delay in between transmissions for data format x (x= 0,1,2,3). Idle time that will be applied at the end of the current transmission if the bit WDEL is set in the current buffer. The delay to be applied is equal to: WDELAY * PVBUSPCLK + 2 * PVBUSPCLK. PVBUSPCLK -> Period of VBUSPCLK."]
pub type WdelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
SPI data format x data word length. CHARLENx defines the word length of data format x. Legal values are 0x02 (data word length = 2 bit) to 0x10 (data word length = 16). Illegal values, such as 0x00 or 0x1F are not detected and their effect is indeterminate."]
    #[inline(always)]
    pub fn charlen(&self) -> CharlenR {
        CharlenR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
SPI data format x prescaler. PRESCALEx can be modified in privilege mode only. PRESCALEx determines the bit transfer rate of data format x if the SPI is the network master. PRESCALEx is directly derived from VBUSPCLK. If the SPI / MibSPI is configured as slave, PRESCALEx DOES NOT NEED to be configured. The clock rate for data format x can be calculated as BRFormat = VBUSPCLCK/(PRESCALEx+1) When PRESCALEx is set to zero (0), the SPI clock rate defaults to VBUSPCLK/2. Any write to this field will update EPRESCALE_FMTx field of EPRESCALEy (y=1,2) registers with EPRESCALE_FMTx(11:8) bits rounded off to ΓÇÿ000ΓÇÖ. Any write to EPRESCALE_FMTx field of the EXTENDED_PRESCALEy (y=1,2) register will cause its lower 8bits to be reflected in this field as well."]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
SPI Data format x clock delay. PHASEx defines the clock delay of data format x. PHASEx can be modified in privilege mode only. 1 =If PHASEx is set to ΓÇ£1ΓÇ¥ the SPI clock signal is delayed by a half SPI clock cycle versus the transmit / receive data stream. The first transmit bit has to output prior to the first clock edge. Master and slave receive the first bit with the first edge 0 =If PHASEx is set to ΓÇ£0ΓÇ¥ the SPI clock signal is not delayed versus the transmit / receive data stream. The first data bit is transmitted with the first clock edge and the first bit is received with the second (inverse) clock edge Note: Restriction on SPICLK Phase/Polarity change in Slave Mode In Slave mode if Phase and/or Polarity of SPICLK has to be changed, the following sequence should be used. oClear the GCR1.SPIEN bit to ΓÇÿ0ΓÇÖ. oSet the required Phase/Polarity values in SPIFMTx registers. oSet the GCR1.SPIEN bit back to ΓÇÿ1ΓÇÖ. The setting of GCR1.SPIEN bit in Slave SPI/MibSPI to ΓÇÿ1ΓÇÖ should be done only after the Polarity of the incoming SPICLK signal changes (if POLARITYx bit was changed in the configuration)."]
    #[inline(always)]
    pub fn phase(&self) -> PhaseR {
        PhaseR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
SPI data format x clock polarity. POLARITYx defines the clock polarity of data format x. POLARITYx can be modified in privilege mode only. 1 =If POLARITYx is set to ΓÇ£1ΓÇ¥ the SPI clock signal is high-inactive, i.e. before and after data transfer the clock signal is high. 0 =If POLARITYx is set to ΓÇ£0ΓÇ¥ the SPI clock signal is low-inactive, i.e. before and after data transfer the clock signal is low."]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Disable Chipselect Timers for this format register. The C2TDELAY &amp; T2CDELAY timers are by default enabled for all the Data Format registers. Using this bit, these timers can be disabled for particular Data Format if not required. When a Master is handling multiple Slaves, with varied set-up hold requirement, the application can selectively choose to include or not include the ChipSelect Delay timers for any Slaves. 0 = Both C2TDELAY &amp; T2CDELAY counts are inserted for the ChipSelects. 1 = No C2TDELAY or T2CDELAY is inserted in the ChipSelect timings."]
    #[inline(always)]
    pub fn discstimers(&self) -> DiscstimersR {
        DiscstimersR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Half Duplex transfer mode enable for Data Format x. This bit controls the I/O function of SOMI/SIMO lines for a specific requirement where in the case of Master mode, TX pin - SIMO will act as an RX pin, and in the case of Slave mode, RX pin - SIMO will act as a TX pin. 0 = Normal Full Duplex transfer. 1 = If MASTER = ΓÇÿ1ΓÇÖ, SIMO pin will act as an RX pin (No TX possible) If MASTER = ΓÇÿ0ΓÇÖ, SIMO pin will act as a TX pin (No RX possible). For all normal operations, HDUPLEX_ENAx bits should always remain ΓÇÿ0ΓÇÖ. It is intended for the usage when the SIMO pin is used for both TX &amp; RX operations at different times."]
    #[inline(always)]
    pub fn hduplex_ena(&self) -> HduplexEnaR {
        HduplexEnaR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Shift direction for data format x. With bit SHIFTDIRx the shift direction for data format x (x=0,1,2,3) can be selected. 1 =Data format x shift direction: Least significant bit is shifted out first. 0 =Data format x shift direction: Most significant bit is shifted out first."]
    #[inline(always)]
    pub fn shiftdir(&self) -> ShiftdirR {
        ShiftdirR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Master waits for ENA signal from slave for data format x. WAITENA is considered in master mode only. In slave mode this bit has no meaning. WAITENA enables a flexible SPI network where slaves with ENA signal and slaves without ENA signal can be mixed. WAITENA defines for each buffer whether the addressed slave generates the ENA signal or does not. 1= Before the SPI / MibSPI starts the data transfer it waits for the ENA signal to become low. If the ENA signal is not pulled down by the addressed slave before the internal time-out counter (C2EDELAY) overflows, then the Master aborts the transfer and sets the TIMEOUT error flag. 0= The SPI / MibSPI does not wait for the ENA signal from the slaves and directly starts the transfer."]
    #[inline(always)]
    pub fn waitena(&self) -> WaitenaR {
        WaitenaR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Parity enable for data format x. 1= A parity is transmitted at the end of each transmit data stream. At the end of a transfer the parity generator compares the received parity bit with the locally calculated parity flag. If the parity bits do not match the RXERR flag is set in the corresponding control field. The parity type (even or odd) can be selected via the PARPOL bit. 0= No parity generation/ verification is performed for this data format. If an Uncorrectable Error Flag is set in a Slave mode MibSPI, then wrong parity bit will be transmitted to indicate to the master that there has been some issue with the data parity. The SOMI pin will be forced to transmit all ΓÇÿ0ΓÇÖs. And parity bit will be transmitted as ΓÇÿ1ΓÇÖ if even parity is selected and as ΓÇÿ0ΓÇÖ if odd parity is selected(using the PARPOLx bit of this register). This behavior will be irrespective of an UPE on either TXRAM or RXRAM."]
    #[inline(always)]
    pub fn parityena(&self) -> ParityenaR {
        ParityenaR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Parity polarity: even or odd. PARPOLx can be modified in privilege mode only. It can be used for data format x (x= 0,1,2,3). 1 =An odd parity flag is added at the end of the transmit data stream. 0 =An even parity flag is added at the end of the transmit data stream."]
    #[inline(always)]
    pub fn parpol(&self) -> ParpolR {
        ParpolR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Delay in between transmissions for data format x (x= 0,1,2,3). Idle time that will be applied at the end of the current transmission if the bit WDEL is set in the current buffer. The delay to be applied is equal to: WDELAY * PVBUSPCLK + 2 * PVBUSPCLK. PVBUSPCLK -> Period of VBUSPCLK."]
    #[inline(always)]
    pub fn wdelay(&self) -> WdelayR {
        WdelayR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
SPI data format x data word length. CHARLENx defines the word length of data format x. Legal values are 0x02 (data word length = 2 bit) to 0x10 (data word length = 16). Illegal values, such as 0x00 or 0x1F are not detected and their effect is indeterminate."]
    #[inline(always)]
    #[must_use]
    pub fn charlen(&mut self) -> CharlenW<Spifmt2Spec> {
        CharlenW::new(self, 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Spifmt2Spec> {
        NuW::new(self, 5)
    }
    #[doc = "Bits 8:15 - 15:8\\]
SPI data format x prescaler. PRESCALEx can be modified in privilege mode only. PRESCALEx determines the bit transfer rate of data format x if the SPI is the network master. PRESCALEx is directly derived from VBUSPCLK. If the SPI / MibSPI is configured as slave, PRESCALEx DOES NOT NEED to be configured. The clock rate for data format x can be calculated as BRFormat = VBUSPCLCK/(PRESCALEx+1) When PRESCALEx is set to zero (0), the SPI clock rate defaults to VBUSPCLK/2. Any write to this field will update EPRESCALE_FMTx field of EPRESCALEy (y=1,2) registers with EPRESCALE_FMTx(11:8) bits rounded off to ΓÇÿ000ΓÇÖ. Any write to EPRESCALE_FMTx field of the EXTENDED_PRESCALEy (y=1,2) register will cause its lower 8bits to be reflected in this field as well."]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PrescaleW<Spifmt2Spec> {
        PrescaleW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
SPI Data format x clock delay. PHASEx defines the clock delay of data format x. PHASEx can be modified in privilege mode only. 1 =If PHASEx is set to ΓÇ£1ΓÇ¥ the SPI clock signal is delayed by a half SPI clock cycle versus the transmit / receive data stream. The first transmit bit has to output prior to the first clock edge. Master and slave receive the first bit with the first edge 0 =If PHASEx is set to ΓÇ£0ΓÇ¥ the SPI clock signal is not delayed versus the transmit / receive data stream. The first data bit is transmitted with the first clock edge and the first bit is received with the second (inverse) clock edge Note: Restriction on SPICLK Phase/Polarity change in Slave Mode In Slave mode if Phase and/or Polarity of SPICLK has to be changed, the following sequence should be used. oClear the GCR1.SPIEN bit to ΓÇÿ0ΓÇÖ. oSet the required Phase/Polarity values in SPIFMTx registers. oSet the GCR1.SPIEN bit back to ΓÇÿ1ΓÇÖ. The setting of GCR1.SPIEN bit in Slave SPI/MibSPI to ΓÇÿ1ΓÇÖ should be done only after the Polarity of the incoming SPICLK signal changes (if POLARITYx bit was changed in the configuration)."]
    #[inline(always)]
    #[must_use]
    pub fn phase(&mut self) -> PhaseW<Spifmt2Spec> {
        PhaseW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
SPI data format x clock polarity. POLARITYx defines the clock polarity of data format x. POLARITYx can be modified in privilege mode only. 1 =If POLARITYx is set to ΓÇ£1ΓÇ¥ the SPI clock signal is high-inactive, i.e. before and after data transfer the clock signal is high. 0 =If POLARITYx is set to ΓÇ£0ΓÇ¥ the SPI clock signal is low-inactive, i.e. before and after data transfer the clock signal is low."]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> PolarityW<Spifmt2Spec> {
        PolarityW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Disable Chipselect Timers for this format register. The C2TDELAY &amp; T2CDELAY timers are by default enabled for all the Data Format registers. Using this bit, these timers can be disabled for particular Data Format if not required. When a Master is handling multiple Slaves, with varied set-up hold requirement, the application can selectively choose to include or not include the ChipSelect Delay timers for any Slaves. 0 = Both C2TDELAY &amp; T2CDELAY counts are inserted for the ChipSelects. 1 = No C2TDELAY or T2CDELAY is inserted in the ChipSelect timings."]
    #[inline(always)]
    #[must_use]
    pub fn discstimers(&mut self) -> DiscstimersW<Spifmt2Spec> {
        DiscstimersW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Half Duplex transfer mode enable for Data Format x. This bit controls the I/O function of SOMI/SIMO lines for a specific requirement where in the case of Master mode, TX pin - SIMO will act as an RX pin, and in the case of Slave mode, RX pin - SIMO will act as a TX pin. 0 = Normal Full Duplex transfer. 1 = If MASTER = ΓÇÿ1ΓÇÖ, SIMO pin will act as an RX pin (No TX possible) If MASTER = ΓÇÿ0ΓÇÖ, SIMO pin will act as a TX pin (No RX possible). For all normal operations, HDUPLEX_ENAx bits should always remain ΓÇÿ0ΓÇÖ. It is intended for the usage when the SIMO pin is used for both TX &amp; RX operations at different times."]
    #[inline(always)]
    #[must_use]
    pub fn hduplex_ena(&mut self) -> HduplexEnaW<Spifmt2Spec> {
        HduplexEnaW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Shift direction for data format x. With bit SHIFTDIRx the shift direction for data format x (x=0,1,2,3) can be selected. 1 =Data format x shift direction: Least significant bit is shifted out first. 0 =Data format x shift direction: Most significant bit is shifted out first."]
    #[inline(always)]
    #[must_use]
    pub fn shiftdir(&mut self) -> ShiftdirW<Spifmt2Spec> {
        ShiftdirW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Master waits for ENA signal from slave for data format x. WAITENA is considered in master mode only. In slave mode this bit has no meaning. WAITENA enables a flexible SPI network where slaves with ENA signal and slaves without ENA signal can be mixed. WAITENA defines for each buffer whether the addressed slave generates the ENA signal or does not. 1= Before the SPI / MibSPI starts the data transfer it waits for the ENA signal to become low. If the ENA signal is not pulled down by the addressed slave before the internal time-out counter (C2EDELAY) overflows, then the Master aborts the transfer and sets the TIMEOUT error flag. 0= The SPI / MibSPI does not wait for the ENA signal from the slaves and directly starts the transfer."]
    #[inline(always)]
    #[must_use]
    pub fn waitena(&mut self) -> WaitenaW<Spifmt2Spec> {
        WaitenaW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Parity enable for data format x. 1= A parity is transmitted at the end of each transmit data stream. At the end of a transfer the parity generator compares the received parity bit with the locally calculated parity flag. If the parity bits do not match the RXERR flag is set in the corresponding control field. The parity type (even or odd) can be selected via the PARPOL bit. 0= No parity generation/ verification is performed for this data format. If an Uncorrectable Error Flag is set in a Slave mode MibSPI, then wrong parity bit will be transmitted to indicate to the master that there has been some issue with the data parity. The SOMI pin will be forced to transmit all ΓÇÿ0ΓÇÖs. And parity bit will be transmitted as ΓÇÿ1ΓÇÖ if even parity is selected and as ΓÇÿ0ΓÇÖ if odd parity is selected(using the PARPOLx bit of this register). This behavior will be irrespective of an UPE on either TXRAM or RXRAM."]
    #[inline(always)]
    #[must_use]
    pub fn parityena(&mut self) -> ParityenaW<Spifmt2Spec> {
        ParityenaW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Parity polarity: even or odd. PARPOLx can be modified in privilege mode only. It can be used for data format x (x= 0,1,2,3). 1 =An odd parity flag is added at the end of the transmit data stream. 0 =An even parity flag is added at the end of the transmit data stream."]
    #[inline(always)]
    #[must_use]
    pub fn parpol(&mut self) -> ParpolW<Spifmt2Spec> {
        ParpolW::new(self, 23)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Delay in between transmissions for data format x (x= 0,1,2,3). Idle time that will be applied at the end of the current transmission if the bit WDEL is set in the current buffer. The delay to be applied is equal to: WDELAY * PVBUSPCLK + 2 * PVBUSPCLK. PVBUSPCLK -> Period of VBUSPCLK."]
    #[inline(always)]
    #[must_use]
    pub fn wdelay(&mut self) -> WdelayW<Spifmt2Spec> {
        WdelayW::new(self, 24)
    }
}
#[doc = "SPI / MibSPI Data Format Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`spifmt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spifmt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spifmt2Spec;
impl crate::RegisterSpec for Spifmt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spifmt2::R`](R) reader structure"]
impl crate::Readable for Spifmt2Spec {}
#[doc = "`write(|w| ..)` method takes [`spifmt2::W`](W) writer structure"]
impl crate::Writable for Spifmt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIFMT2 to value 0"]
impl crate::Resettable for Spifmt2Spec {
    const RESET_VALUE: u32 = 0;
}
