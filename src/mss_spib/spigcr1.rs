#[doc = "Register `SPIGCR1` reader"]
pub type R = crate::R<Spigcr1Spec>;
#[doc = "Register `SPIGCR1` writer"]
pub type W = crate::W<Spigcr1Spec>;
#[doc = "Field `MASTER` reader - 0:0\\]
MASTER: SPISIMO/SPISOMI pin direction determination. Determines the direction of the SPISIMO and SPISOMI pins. This bit determines whether the SPI/MibSPI is in Master mode or Slave mode. This bit also controls the Master-only features like the C2T/T2C counters, C2E/T2E timers, most of the Error conditions specific to Master mode. 0=SPISIMO pin an input, SPISOMI pin an output 1=SPISOMI pin an input, SPISIMO pin an output Note: Although there are two different bits which control the Master/Slave mode functions, only two of their combinations are valid. For compatibility reasons both the bits are retained. For Master mode of operation: MASTER = ΓÇÿ1ΓÇÖ, CLKMOD = ΓÇÿ1ΓÇÖ For Slave mode of operation: MASTER = ΓÇÿ0ΓÇÖ, CLKMOD = ΓÇÿ0ΓÇÖ Any other combinations of these two bits may not yield any desirable operation of the module."]
pub type MasterR = crate::BitReader;
#[doc = "Field `MASTER` writer - 0:0\\]
MASTER: SPISIMO/SPISOMI pin direction determination. Determines the direction of the SPISIMO and SPISOMI pins. This bit determines whether the SPI/MibSPI is in Master mode or Slave mode. This bit also controls the Master-only features like the C2T/T2C counters, C2E/T2E timers, most of the Error conditions specific to Master mode. 0=SPISIMO pin an input, SPISOMI pin an output 1=SPISOMI pin an input, SPISIMO pin an output Note: Although there are two different bits which control the Master/Slave mode functions, only two of their combinations are valid. For compatibility reasons both the bits are retained. For Master mode of operation: MASTER = ΓÇÿ1ΓÇÖ, CLKMOD = ΓÇÿ1ΓÇÖ For Slave mode of operation: MASTER = ΓÇÿ0ΓÇÖ, CLKMOD = ΓÇÿ0ΓÇÖ Any other combinations of these two bits may not yield any desirable operation of the module."]
pub type MasterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKMOD` reader - 1:1\\]
CLKMOD. Clock mode Selects either an internal or external clock source. This bit also determines the I/O direction of the SPIENA and SPISCS\\[7:0\\]
pins in functional mode. 0=Clock is external 1=Clock is internal"]
pub type ClkmodR = crate::BitReader;
#[doc = "Field `CLKMOD` writer - 1:1\\]
CLKMOD. Clock mode Selects either an internal or external clock source. This bit also determines the I/O direction of the SPIENA and SPISCS\\[7:0\\]
pins in functional mode. 0=Clock is external 1=Clock is internal"]
pub type ClkmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 7:2\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 7:2\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `POWERDOWN` reader - 8:8\\]
POWERDOWN. When active, the SPI / MibSPI state machines enter a powerdown state. 0=MibSPI in active mode 1=MibSPI in powerdown mode"]
pub type PowerdownR = crate::BitReader;
#[doc = "Field `POWERDOWN` writer - 8:8\\]
POWERDOWN. When active, the SPI / MibSPI state machines enter a powerdown state. 0=MibSPI in active mode 1=MibSPI in powerdown mode"]
pub type PowerdownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 15:9\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 15:9\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LOOPBACK` reader - 16:16\\]
LOOP BACK. Internal loop-back test mode. The internal self-test option can be enabled by setting this bit. If the SPISIMO and SPISOMI pins are configured with SPI functionality, then the SPISIMO pin is internally connected to the SPISOMI pin. The transmit data is looped back as receive data and is stored in the receive field of the concerned buffer. Externally, during loop-back operation, the SPICLK pin outputs an inactive value and SPISOMI remains in high-impedance state. The SPI / MibSPI has to be initialized in master mode before the loop-back can be selected. If the SPI / MibSPI is initialized in slave mode or a data transfer is ongoing, errors may result. 1 =Internal loop-back test mode enabled. 0 =Internal loop-back test mode disabled. This loopback mode can be used only in Master mode. This automatically selects digital loopback path. When this Loopback mode is selected, CLKMOD bit should be set to ΓÇÿ1ΓÇÖ, meaning that SPICLK can only be internal."]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - 16:16\\]
LOOP BACK. Internal loop-back test mode. The internal self-test option can be enabled by setting this bit. If the SPISIMO and SPISOMI pins are configured with SPI functionality, then the SPISIMO pin is internally connected to the SPISOMI pin. The transmit data is looped back as receive data and is stored in the receive field of the concerned buffer. Externally, during loop-back operation, the SPICLK pin outputs an inactive value and SPISOMI remains in high-impedance state. The SPI / MibSPI has to be initialized in master mode before the loop-back can be selected. If the SPI / MibSPI is initialized in slave mode or a data transfer is ongoing, errors may result. 1 =Internal loop-back test mode enabled. 0 =Internal loop-back test mode disabled. This loopback mode can be used only in Master mode. This automatically selects digital loopback path. When this Loopback mode is selected, CLKMOD bit should be set to ΓÇÿ1ΓÇÖ, meaning that SPICLK can only be internal."]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - 23:17\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3R = crate::FieldReader;
#[doc = "Field `NU3` writer - 23:17\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SPIEN` reader - 24:24\\]
SPI enable. This bit enables the SPI/MibSPI transfers. This bit must be set to 1 after all other SPI / MibSPI configuration bits have been written. When SPIEN bit is 0 or cleared to 0, the following SPI/MibSPI registers get forced to their default states (to ΓÇÿ0ΓÇÖs except for RXEMPTY bit in SPIBUF): - Both TX &amp; RX Shift Registers - The TXDATA fields of SPIDAT0 and SPIDAT1 registers - All the fields of SPIFLG register - Contents of SPIBUF &amp; the internal RXBUF registers 0=SPI / MibSPI is not activated for transfers. 1=Activates SPI / MibSPI"]
pub type SpienR = crate::BitReader;
#[doc = "Field `SPIEN` writer - 24:24\\]
SPI enable. This bit enables the SPI/MibSPI transfers. This bit must be set to 1 after all other SPI / MibSPI configuration bits have been written. When SPIEN bit is 0 or cleared to 0, the following SPI/MibSPI registers get forced to their default states (to ΓÇÿ0ΓÇÖs except for RXEMPTY bit in SPIBUF): - Both TX &amp; RX Shift Registers - The TXDATA fields of SPIDAT0 and SPIDAT1 registers - All the fields of SPIFLG register - Contents of SPIBUF &amp; the internal RXBUF registers 0=SPI / MibSPI is not activated for transfers. 1=Activates SPI / MibSPI"]
pub type SpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU4` reader - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4R = crate::FieldReader;
#[doc = "Field `NU4` writer - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MASTER: SPISIMO/SPISOMI pin direction determination. Determines the direction of the SPISIMO and SPISOMI pins. This bit determines whether the SPI/MibSPI is in Master mode or Slave mode. This bit also controls the Master-only features like the C2T/T2C counters, C2E/T2E timers, most of the Error conditions specific to Master mode. 0=SPISIMO pin an input, SPISOMI pin an output 1=SPISOMI pin an input, SPISIMO pin an output Note: Although there are two different bits which control the Master/Slave mode functions, only two of their combinations are valid. For compatibility reasons both the bits are retained. For Master mode of operation: MASTER = ΓÇÿ1ΓÇÖ, CLKMOD = ΓÇÿ1ΓÇÖ For Slave mode of operation: MASTER = ΓÇÿ0ΓÇÖ, CLKMOD = ΓÇÿ0ΓÇÖ Any other combinations of these two bits may not yield any desirable operation of the module."]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CLKMOD. Clock mode Selects either an internal or external clock source. This bit also determines the I/O direction of the SPIENA and SPISCS\\[7:0\\]
pins in functional mode. 0=Clock is external 1=Clock is internal"]
    #[inline(always)]
    pub fn clkmod(&self) -> ClkmodR {
        ClkmodR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
POWERDOWN. When active, the SPI / MibSPI state machines enter a powerdown state. 0=MibSPI in active mode 1=MibSPI in powerdown mode"]
    #[inline(always)]
    pub fn powerdown(&self) -> PowerdownR {
        PowerdownR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
LOOP BACK. Internal loop-back test mode. The internal self-test option can be enabled by setting this bit. If the SPISIMO and SPISOMI pins are configured with SPI functionality, then the SPISIMO pin is internally connected to the SPISOMI pin. The transmit data is looped back as receive data and is stored in the receive field of the concerned buffer. Externally, during loop-back operation, the SPICLK pin outputs an inactive value and SPISOMI remains in high-impedance state. The SPI / MibSPI has to be initialized in master mode before the loop-back can be selected. If the SPI / MibSPI is initialized in slave mode or a data transfer is ongoing, errors may result. 1 =Internal loop-back test mode enabled. 0 =Internal loop-back test mode disabled. This loopback mode can be used only in Master mode. This automatically selects digital loopback path. When this Loopback mode is selected, CLKMOD bit should be set to ΓÇÿ1ΓÇÖ, meaning that SPICLK can only be internal."]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
SPI enable. This bit enables the SPI/MibSPI transfers. This bit must be set to 1 after all other SPI / MibSPI configuration bits have been written. When SPIEN bit is 0 or cleared to 0, the following SPI/MibSPI registers get forced to their default states (to ΓÇÿ0ΓÇÖs except for RXEMPTY bit in SPIBUF): - Both TX &amp; RX Shift Registers - The TXDATA fields of SPIDAT0 and SPIDAT1 registers - All the fields of SPIFLG register - Contents of SPIBUF &amp; the internal RXBUF registers 0=SPI / MibSPI is not activated for transfers. 1=Activates SPI / MibSPI"]
    #[inline(always)]
    pub fn spien(&self) -> SpienR {
        SpienR::new(((self.bits >> 24) & 1) != 0)
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
MASTER: SPISIMO/SPISOMI pin direction determination. Determines the direction of the SPISIMO and SPISOMI pins. This bit determines whether the SPI/MibSPI is in Master mode or Slave mode. This bit also controls the Master-only features like the C2T/T2C counters, C2E/T2E timers, most of the Error conditions specific to Master mode. 0=SPISIMO pin an input, SPISOMI pin an output 1=SPISOMI pin an input, SPISIMO pin an output Note: Although there are two different bits which control the Master/Slave mode functions, only two of their combinations are valid. For compatibility reasons both the bits are retained. For Master mode of operation: MASTER = ΓÇÿ1ΓÇÖ, CLKMOD = ΓÇÿ1ΓÇÖ For Slave mode of operation: MASTER = ΓÇÿ0ΓÇÖ, CLKMOD = ΓÇÿ0ΓÇÖ Any other combinations of these two bits may not yield any desirable operation of the module."]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MasterW<Spigcr1Spec> {
        MasterW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CLKMOD. Clock mode Selects either an internal or external clock source. This bit also determines the I/O direction of the SPIENA and SPISCS\\[7:0\\]
pins in functional mode. 0=Clock is external 1=Clock is internal"]
    #[inline(always)]
    #[must_use]
    pub fn clkmod(&mut self) -> ClkmodW<Spigcr1Spec> {
        ClkmodW::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Spigcr1Spec> {
        Nu1W::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
POWERDOWN. When active, the SPI / MibSPI state machines enter a powerdown state. 0=MibSPI in active mode 1=MibSPI in powerdown mode"]
    #[inline(always)]
    #[must_use]
    pub fn powerdown(&mut self) -> PowerdownW<Spigcr1Spec> {
        PowerdownW::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Spigcr1Spec> {
        Nu2W::new(self, 9)
    }
    #[doc = "Bit 16 - 16:16\\]
LOOP BACK. Internal loop-back test mode. The internal self-test option can be enabled by setting this bit. If the SPISIMO and SPISOMI pins are configured with SPI functionality, then the SPISIMO pin is internally connected to the SPISOMI pin. The transmit data is looped back as receive data and is stored in the receive field of the concerned buffer. Externally, during loop-back operation, the SPICLK pin outputs an inactive value and SPISOMI remains in high-impedance state. The SPI / MibSPI has to be initialized in master mode before the loop-back can be selected. If the SPI / MibSPI is initialized in slave mode or a data transfer is ongoing, errors may result. 1 =Internal loop-back test mode enabled. 0 =Internal loop-back test mode disabled. This loopback mode can be used only in Master mode. This automatically selects digital loopback path. When this Loopback mode is selected, CLKMOD bit should be set to ΓÇÿ1ΓÇÖ, meaning that SPICLK can only be internal."]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LoopbackW<Spigcr1Spec> {
        LoopbackW::new(self, 16)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<Spigcr1Spec> {
        Nu3W::new(self, 17)
    }
    #[doc = "Bit 24 - 24:24\\]
SPI enable. This bit enables the SPI/MibSPI transfers. This bit must be set to 1 after all other SPI / MibSPI configuration bits have been written. When SPIEN bit is 0 or cleared to 0, the following SPI/MibSPI registers get forced to their default states (to ΓÇÿ0ΓÇÖs except for RXEMPTY bit in SPIBUF): - Both TX &amp; RX Shift Registers - The TXDATA fields of SPIDAT0 and SPIDAT1 registers - All the fields of SPIFLG register - Contents of SPIBUF &amp; the internal RXBUF registers 0=SPI / MibSPI is not activated for transfers. 1=Activates SPI / MibSPI"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SpienW<Spigcr1Spec> {
        SpienW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<Spigcr1Spec> {
        Nu4W::new(self, 25)
    }
}
#[doc = "SPI / MibSPI Global control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`spigcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spigcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spigcr1Spec;
impl crate::RegisterSpec for Spigcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spigcr1::R`](R) reader structure"]
impl crate::Readable for Spigcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`spigcr1::W`](W) writer structure"]
impl crate::Writable for Spigcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIGCR1 to value 0"]
impl crate::Resettable for Spigcr1Spec {
    const RESET_VALUE: u32 = 0;
}
