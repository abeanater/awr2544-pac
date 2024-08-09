#[doc = "Register `SPIINT0` reader"]
pub type R = crate::R<Spiint0Spec>;
#[doc = "Register `SPIINT0` writer"]
pub type W = crate::W<Spiint0Spec>;
#[doc = "Field `DLENERRENA` reader - 0:0\\]
Data Length Error interrupt Enable. 1 = Enables an interrupt when Data Length Error occurs. 0 = No interrupt is generated upon Data Length Error. A Data Length Error occurs under the following conditions. Master: In a 4-pin with SPIENA mode or 5-pin mode, if the SPIENA pin from the slave is deasserted before the Master has completed its transfer, the Data Length Error is set. That is, if the Character Length counter has not completed counting while SPIENA deassertion is detected, then it means that the Slave has neither received full data from the Master nor has it transmitted complete data. Slave: In a 4-pin with ChipSelects mode or 5-pin mode, if the incoming valid SPISCS pin is de-activated before the Character Length counter completes counting, then Data Length Error is set."]
pub type DlenerrenaR = crate::BitReader;
#[doc = "Field `DLENERRENA` writer - 0:0\\]
Data Length Error interrupt Enable. 1 = Enables an interrupt when Data Length Error occurs. 0 = No interrupt is generated upon Data Length Error. A Data Length Error occurs under the following conditions. Master: In a 4-pin with SPIENA mode or 5-pin mode, if the SPIENA pin from the slave is deasserted before the Master has completed its transfer, the Data Length Error is set. That is, if the Character Length counter has not completed counting while SPIENA deassertion is detected, then it means that the Slave has neither received full data from the Master nor has it transmitted complete data. Slave: In a 4-pin with ChipSelects mode or 5-pin mode, if the incoming valid SPISCS pin is de-activated before the Character Length counter completes counting, then Data Length Error is set."]
pub type DlenerrenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTENA` reader - 1:1\\]
Enables interrupt on ENA signal time-out. 1 =Enables an interrupt on a time-out of the ENA signal (TIMEOUT = 1). 0 =No interrupt asserted upon ENA signal time-out."]
pub type TimeoutenaR = crate::BitReader;
#[doc = "Field `TIMEOUTENA` writer - 1:1\\]
Enables interrupt on ENA signal time-out. 1 =Enables an interrupt on a time-out of the ENA signal (TIMEOUT = 1). 0 =No interrupt asserted upon ENA signal time-out."]
pub type TimeoutenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARERRENA` reader - 2:2\\]
Enables interrupt on parity error. 1 =Enables an interrupt on a parity error (PARITYERR = 1). 0 =No interrupt asserted upon parity error."]
pub type ParerrenaR = crate::BitReader;
#[doc = "Field `PARERRENA` writer - 2:2\\]
Enables interrupt on parity error. 1 =Enables an interrupt on a parity error (PARITYERR = 1). 0 =No interrupt asserted upon parity error."]
pub type ParerrenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESYNCENA` reader - 3:3\\]
Enables interrupt on de-synchronized slave. DESYNCENA is used in master mode only. 1 =Enables an interrupt on de-synchronization of the slave (DESYNC = 1). 0 =No interrupt asserted upon de-synchronization error."]
pub type DesyncenaR = crate::BitReader;
#[doc = "Field `DESYNCENA` writer - 3:3\\]
Enables interrupt on de-synchronized slave. DESYNCENA is used in master mode only. 1 =Enables an interrupt on de-synchronization of the slave (DESYNC = 1). 0 =No interrupt asserted upon de-synchronization error."]
pub type DesyncenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITERRENA` reader - 4:4\\]
Enables interrupt on bit error. 1 =Enables an interrupt on a bit error (BITERR = 1). 0 =No interrupt asserted upon bit error."]
pub type BiterrenaR = crate::BitReader;
#[doc = "Field `BITERRENA` writer - 4:4\\]
Enables interrupt on bit error. 1 =Enables an interrupt on a bit error (BITERR = 1). 0 =No interrupt asserted upon bit error."]
pub type BiterrenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 5:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1R = crate::BitReader;
#[doc = "Field `NU1` writer - 5:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRNINTENA` reader - 6:6\\]
Overrun interrupt enable. An interrupt is to be generated when the RCVR OVRN flag bit (SPIFLG.6) is set by hardware. Otherwise, no interrupt will be generated. 0=Overrun interrupt will not be generated 1=Overrun interrupt will be generated"]
pub type OvrnintenaR = crate::BitReader;
#[doc = "Field `OVRNINTENA` writer - 6:6\\]
Overrun interrupt enable. An interrupt is to be generated when the RCVR OVRN flag bit (SPIFLG.6) is set by hardware. Otherwise, no interrupt will be generated. 0=Overrun interrupt will not be generated 1=Overrun interrupt will be generated"]
pub type OvrnintenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2R = crate::BitReader;
#[doc = "Field `NU2` writer - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINTENA` reader - 8:8\\]
An interrupt is to be generated when the RXINTFLAG bit (SPIFLG.8) is set by hardware. Otherwise, no interrupt will be generated. 0=Interrupt will not be generated 1=Interrupt will be generated Both Transmitter Empty &amp; Receiver Full interrupts are valid in SPI or Compatibility mode of MibSPI only. In Multibuffered mode these interrupts will not be generated even if enabled."]
pub type RxintenaR = crate::BitReader;
#[doc = "Field `RXINTENA` writer - 8:8\\]
An interrupt is to be generated when the RXINTFLAG bit (SPIFLG.8) is set by hardware. Otherwise, no interrupt will be generated. 0=Interrupt will not be generated 1=Interrupt will be generated Both Transmitter Empty &amp; Receiver Full interrupts are valid in SPI or Compatibility mode of MibSPI only. In Multibuffered mode these interrupts will not be generated even if enabled."]
pub type RxintenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINTENA` reader - 9:9\\]
An interrupt is to be generated everytime data is written to the ΓÇ£Shift RegisterΓÇ¥, so that a new data can be written to TXBUF. Setting this bit will generate an interrupt if TXINTFLG bit (SPIFLG.9) is set to ΓÇÿ1ΓÇÖ. 0=No interrupt will be generated upon TXINTFLG getting set to ΓÇÿ1ΓÇÖ. 1=Interrupt will be generated upon TXINTFLG getting set to ΓÇÿ1ΓÇÖ. An interrupt request will be generated as soon as this bit is set to ΓÇÿ1ΓÇÖ. By default it will be generated on INT0 line. SPILVL register can be programmed before-hand to change this default."]
pub type TxintenaR = crate::BitReader;
#[doc = "Field `TXINTENA` writer - 9:9\\]
An interrupt is to be generated everytime data is written to the ΓÇ£Shift RegisterΓÇ¥, so that a new data can be written to TXBUF. Setting this bit will generate an interrupt if TXINTFLG bit (SPIFLG.9) is set to ΓÇÿ1ΓÇÖ. 0=No interrupt will be generated upon TXINTFLG getting set to ΓÇÿ1ΓÇÖ. 1=Interrupt will be generated upon TXINTFLG getting set to ΓÇÿ1ΓÇÖ. An interrupt request will be generated as soon as this bit is set to ΓÇÿ1ΓÇÖ. By default it will be generated on INT0 line. SPILVL register can be programmed before-hand to change this default."]
pub type TxintenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - 15:10\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3R = crate::FieldReader;
#[doc = "Field `NU3` writer - 15:10\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DMAREQEN` reader - 16:16\\]
DMA request enable. Enables the DMA request signal to be generated for both receive and transmit channels. Enable DMA REQ only after setting the SPIEN bit to ΓÇÿ1ΓÇÖ. 0=DMA is not used 1=DMA Requests will be generated. A DMA request will be generated on TX DMA REQ line each time a transmit data is copied to the Shift Register either from TXBUF or directly from SPIDAT0/SPIDAT1 writes. A DMA request will be generated on RX DMA REQ line each time a received data is copied to SPIBUF register either from RXBUF or directly from the Shift Register."]
pub type DmareqenR = crate::BitReader;
#[doc = "Field `DMAREQEN` writer - 16:16\\]
DMA request enable. Enables the DMA request signal to be generated for both receive and transmit channels. Enable DMA REQ only after setting the SPIEN bit to ΓÇÿ1ΓÇÖ. 0=DMA is not used 1=DMA Requests will be generated. A DMA request will be generated on TX DMA REQ line each time a transmit data is copied to the Shift Register either from TXBUF or directly from SPIDAT0/SPIDAT1 writes. A DMA request will be generated on RX DMA REQ line each time a received data is copied to SPIBUF register either from RXBUF or directly from the Shift Register."]
pub type DmareqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU4` reader - 23:17\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4R = crate::FieldReader;
#[doc = "Field `NU4` writer - 23:17\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ENABLEHIGHZ` reader - 24:24\\]
SPIENA pin high-z enable. When active, the SPIENA pin (when it is configured as a WAIT functional output signal in a slave SPI) is forced to place it is output in high-z when not driving a low signal. If inactive, then the pin will output both a high and a low signal. 0=SPIENA pin is pulled high when not active. 1=SPIENA pin remains in high-z when not active."]
pub type EnablehighzR = crate::BitReader;
#[doc = "Field `ENABLEHIGHZ` writer - 24:24\\]
SPIENA pin high-z enable. When active, the SPIENA pin (when it is configured as a WAIT functional output signal in a slave SPI) is forced to place it is output in high-z when not driving a low signal. If inactive, then the pin will output both a high and a low signal. 0=SPIENA pin is pulled high when not active. 1=SPIENA pin remains in high-z when not active."]
pub type EnablehighzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU5` reader - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu5R = crate::FieldReader;
#[doc = "Field `NU5` writer - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data Length Error interrupt Enable. 1 = Enables an interrupt when Data Length Error occurs. 0 = No interrupt is generated upon Data Length Error. A Data Length Error occurs under the following conditions. Master: In a 4-pin with SPIENA mode or 5-pin mode, if the SPIENA pin from the slave is deasserted before the Master has completed its transfer, the Data Length Error is set. That is, if the Character Length counter has not completed counting while SPIENA deassertion is detected, then it means that the Slave has neither received full data from the Master nor has it transmitted complete data. Slave: In a 4-pin with ChipSelects mode or 5-pin mode, if the incoming valid SPISCS pin is de-activated before the Character Length counter completes counting, then Data Length Error is set."]
    #[inline(always)]
    pub fn dlenerrena(&self) -> DlenerrenaR {
        DlenerrenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables interrupt on ENA signal time-out. 1 =Enables an interrupt on a time-out of the ENA signal (TIMEOUT = 1). 0 =No interrupt asserted upon ENA signal time-out."]
    #[inline(always)]
    pub fn timeoutena(&self) -> TimeoutenaR {
        TimeoutenaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables interrupt on parity error. 1 =Enables an interrupt on a parity error (PARITYERR = 1). 0 =No interrupt asserted upon parity error."]
    #[inline(always)]
    pub fn parerrena(&self) -> ParerrenaR {
        ParerrenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables interrupt on de-synchronized slave. DESYNCENA is used in master mode only. 1 =Enables an interrupt on de-synchronization of the slave (DESYNC = 1). 0 =No interrupt asserted upon de-synchronization error."]
    #[inline(always)]
    pub fn desyncena(&self) -> DesyncenaR {
        DesyncenaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables interrupt on bit error. 1 =Enables an interrupt on a bit error (BITERR = 1). 0 =No interrupt asserted upon bit error."]
    #[inline(always)]
    pub fn biterrena(&self) -> BiterrenaR {
        BiterrenaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Overrun interrupt enable. An interrupt is to be generated when the RCVR OVRN flag bit (SPIFLG.6) is set by hardware. Otherwise, no interrupt will be generated. 0=Overrun interrupt will not be generated 1=Overrun interrupt will be generated"]
    #[inline(always)]
    pub fn ovrnintena(&self) -> OvrnintenaR {
        OvrnintenaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
An interrupt is to be generated when the RXINTFLAG bit (SPIFLG.8) is set by hardware. Otherwise, no interrupt will be generated. 0=Interrupt will not be generated 1=Interrupt will be generated Both Transmitter Empty &amp; Receiver Full interrupts are valid in SPI or Compatibility mode of MibSPI only. In Multibuffered mode these interrupts will not be generated even if enabled."]
    #[inline(always)]
    pub fn rxintena(&self) -> RxintenaR {
        RxintenaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
An interrupt is to be generated everytime data is written to the ΓÇ£Shift RegisterΓÇ¥, so that a new data can be written to TXBUF. Setting this bit will generate an interrupt if TXINTFLG bit (SPIFLG.9) is set to ΓÇÿ1ΓÇÖ. 0=No interrupt will be generated upon TXINTFLG getting set to ΓÇÿ1ΓÇÖ. 1=Interrupt will be generated upon TXINTFLG getting set to ΓÇÿ1ΓÇÖ. An interrupt request will be generated as soon as this bit is set to ΓÇÿ1ΓÇÖ. By default it will be generated on INT0 line. SPILVL register can be programmed before-hand to change this default."]
    #[inline(always)]
    pub fn txintena(&self) -> TxintenaR {
        TxintenaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
DMA request enable. Enables the DMA request signal to be generated for both receive and transmit channels. Enable DMA REQ only after setting the SPIEN bit to ΓÇÿ1ΓÇÖ. 0=DMA is not used 1=DMA Requests will be generated. A DMA request will be generated on TX DMA REQ line each time a transmit data is copied to the Shift Register either from TXBUF or directly from SPIDAT0/SPIDAT1 writes. A DMA request will be generated on RX DMA REQ line each time a received data is copied to SPIBUF register either from RXBUF or directly from the Shift Register."]
    #[inline(always)]
    pub fn dmareqen(&self) -> DmareqenR {
        DmareqenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
SPIENA pin high-z enable. When active, the SPIENA pin (when it is configured as a WAIT functional output signal in a slave SPI) is forced to place it is output in high-z when not driving a low signal. If inactive, then the pin will output both a high and a low signal. 0=SPIENA pin is pulled high when not active. 1=SPIENA pin remains in high-z when not active."]
    #[inline(always)]
    pub fn enablehighz(&self) -> EnablehighzR {
        EnablehighzR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu5(&self) -> Nu5R {
        Nu5R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data Length Error interrupt Enable. 1 = Enables an interrupt when Data Length Error occurs. 0 = No interrupt is generated upon Data Length Error. A Data Length Error occurs under the following conditions. Master: In a 4-pin with SPIENA mode or 5-pin mode, if the SPIENA pin from the slave is deasserted before the Master has completed its transfer, the Data Length Error is set. That is, if the Character Length counter has not completed counting while SPIENA deassertion is detected, then it means that the Slave has neither received full data from the Master nor has it transmitted complete data. Slave: In a 4-pin with ChipSelects mode or 5-pin mode, if the incoming valid SPISCS pin is de-activated before the Character Length counter completes counting, then Data Length Error is set."]
    #[inline(always)]
    #[must_use]
    pub fn dlenerrena(&mut self) -> DlenerrenaW<Spiint0Spec> {
        DlenerrenaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables interrupt on ENA signal time-out. 1 =Enables an interrupt on a time-out of the ENA signal (TIMEOUT = 1). 0 =No interrupt asserted upon ENA signal time-out."]
    #[inline(always)]
    #[must_use]
    pub fn timeoutena(&mut self) -> TimeoutenaW<Spiint0Spec> {
        TimeoutenaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables interrupt on parity error. 1 =Enables an interrupt on a parity error (PARITYERR = 1). 0 =No interrupt asserted upon parity error."]
    #[inline(always)]
    #[must_use]
    pub fn parerrena(&mut self) -> ParerrenaW<Spiint0Spec> {
        ParerrenaW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables interrupt on de-synchronized slave. DESYNCENA is used in master mode only. 1 =Enables an interrupt on de-synchronization of the slave (DESYNC = 1). 0 =No interrupt asserted upon de-synchronization error."]
    #[inline(always)]
    #[must_use]
    pub fn desyncena(&mut self) -> DesyncenaW<Spiint0Spec> {
        DesyncenaW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables interrupt on bit error. 1 =Enables an interrupt on a bit error (BITERR = 1). 0 =No interrupt asserted upon bit error."]
    #[inline(always)]
    #[must_use]
    pub fn biterrena(&mut self) -> BiterrenaW<Spiint0Spec> {
        BiterrenaW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Spiint0Spec> {
        Nu1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Overrun interrupt enable. An interrupt is to be generated when the RCVR OVRN flag bit (SPIFLG.6) is set by hardware. Otherwise, no interrupt will be generated. 0=Overrun interrupt will not be generated 1=Overrun interrupt will be generated"]
    #[inline(always)]
    #[must_use]
    pub fn ovrnintena(&mut self) -> OvrnintenaW<Spiint0Spec> {
        OvrnintenaW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Spiint0Spec> {
        Nu2W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
An interrupt is to be generated when the RXINTFLAG bit (SPIFLG.8) is set by hardware. Otherwise, no interrupt will be generated. 0=Interrupt will not be generated 1=Interrupt will be generated Both Transmitter Empty &amp; Receiver Full interrupts are valid in SPI or Compatibility mode of MibSPI only. In Multibuffered mode these interrupts will not be generated even if enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxintena(&mut self) -> RxintenaW<Spiint0Spec> {
        RxintenaW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
An interrupt is to be generated everytime data is written to the ΓÇ£Shift RegisterΓÇ¥, so that a new data can be written to TXBUF. Setting this bit will generate an interrupt if TXINTFLG bit (SPIFLG.9) is set to ΓÇÿ1ΓÇÖ. 0=No interrupt will be generated upon TXINTFLG getting set to ΓÇÿ1ΓÇÖ. 1=Interrupt will be generated upon TXINTFLG getting set to ΓÇÿ1ΓÇÖ. An interrupt request will be generated as soon as this bit is set to ΓÇÿ1ΓÇÖ. By default it will be generated on INT0 line. SPILVL register can be programmed before-hand to change this default."]
    #[inline(always)]
    #[must_use]
    pub fn txintena(&mut self) -> TxintenaW<Spiint0Spec> {
        TxintenaW::new(self, 9)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<Spiint0Spec> {
        Nu3W::new(self, 10)
    }
    #[doc = "Bit 16 - 16:16\\]
DMA request enable. Enables the DMA request signal to be generated for both receive and transmit channels. Enable DMA REQ only after setting the SPIEN bit to ΓÇÿ1ΓÇÖ. 0=DMA is not used 1=DMA Requests will be generated. A DMA request will be generated on TX DMA REQ line each time a transmit data is copied to the Shift Register either from TXBUF or directly from SPIDAT0/SPIDAT1 writes. A DMA request will be generated on RX DMA REQ line each time a received data is copied to SPIBUF register either from RXBUF or directly from the Shift Register."]
    #[inline(always)]
    #[must_use]
    pub fn dmareqen(&mut self) -> DmareqenW<Spiint0Spec> {
        DmareqenW::new(self, 16)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<Spiint0Spec> {
        Nu4W::new(self, 17)
    }
    #[doc = "Bit 24 - 24:24\\]
SPIENA pin high-z enable. When active, the SPIENA pin (when it is configured as a WAIT functional output signal in a slave SPI) is forced to place it is output in high-z when not driving a low signal. If inactive, then the pin will output both a high and a low signal. 0=SPIENA pin is pulled high when not active. 1=SPIENA pin remains in high-z when not active."]
    #[inline(always)]
    #[must_use]
    pub fn enablehighz(&mut self) -> EnablehighzW<Spiint0Spec> {
        EnablehighzW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu5(&mut self) -> Nu5W<Spiint0Spec> {
        Nu5W::new(self, 25)
    }
}
#[doc = "SPI / MibSPI Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spiint0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spiint0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spiint0Spec;
impl crate::RegisterSpec for Spiint0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spiint0::R`](R) reader structure"]
impl crate::Readable for Spiint0Spec {}
#[doc = "`write(|w| ..)` method takes [`spiint0::W`](W) writer structure"]
impl crate::Writable for Spiint0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIINT0 to value 0"]
impl crate::Resettable for Spiint0Spec {
    const RESET_VALUE: u32 = 0;
}
