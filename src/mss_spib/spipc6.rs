#[doc = "Register `SPIPC6` reader"]
pub type R = crate::R<Spipc6Spec>;
#[doc = "Register `SPIPC6` writer"]
pub type W = crate::W<Spipc6Spec>;
#[doc = "Field `SCSPDR` reader - 7:0\\]
SPISCSx Open drain enable Enables Open drain capability for the pin SPISCSx if the following conditions are met. SCSDIRx = 1 (SPISCS pin configured in GPIO mode as output pin) SCSDOUTx = 1 0 = Output value on SPISCSx pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISCSx is Tri-stated Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSPDR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
pub type ScspdrR = crate::FieldReader;
#[doc = "Field `SCSPDR` writer - 7:0\\]
SPISCSx Open drain enable Enables Open drain capability for the pin SPISCSx if the following conditions are met. SCSDIRx = 1 (SPISCS pin configured in GPIO mode as output pin) SCSDOUTx = 1 0 = Output value on SPISCSx pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISCSx is Tri-stated Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSPDR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
pub type ScspdrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENAPDR` reader - 8:8\\]
SPIENA Open drain enable Enables Open drain capability for the pin SPIENA if the following conditions are met. ENABLEDIR = 1 (SPIENA pin configured in GPIO mode as output pin) ENABLEDOUT = 1 0 = Output value on SPIENA pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPIENA is Tri-stated"]
pub type EnapdrR = crate::BitReader;
#[doc = "Field `ENAPDR` writer - 8:8\\]
SPIENA Open drain enable Enables Open drain capability for the pin SPIENA if the following conditions are met. ENABLEDIR = 1 (SPIENA pin configured in GPIO mode as output pin) ENABLEDOUT = 1 0 = Output value on SPIENA pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPIENA is Tri-stated"]
pub type EnapdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPDR` reader - 9:9\\]
SPICLK Open drain enable Enables Open drain capability for the pin CLK if the following conditions are met. CLKDIR = 1 (SPICLK pin configured in GPIO mode as output pin) CLKDOUT = 1 0 = Output value on SPICLK pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPICLK is Tri-stated"]
pub type ClkpdrR = crate::BitReader;
#[doc = "Field `CLKPDR` writer - 9:9\\]
SPICLK Open drain enable Enables Open drain capability for the pin CLK if the following conditions are met. CLKDIR = 1 (SPICLK pin configured in GPIO mode as output pin) CLKDOUT = 1 0 = Output value on SPICLK pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPICLK is Tri-stated"]
pub type ClkpdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMOPDR0` reader - 10:10\\]
SPISIMO0 Open drain enable Enables Open drain capability for the pin SPISIMO0 if the following conditions are met. SIMODIR0 = 1 (SPISIMO pin configured in GPIO mode as output pin) SIMODOUT0 = 1 0 = Output value on SPISIMO0 pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISIMO0 is Tri-stated Note: Bit 10 or bit 16 can be used to set the direction for pin SPISIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simopdr0R = crate::BitReader;
#[doc = "Field `SIMOPDR0` writer - 10:10\\]
SPISIMO0 Open drain enable Enables Open drain capability for the pin SPISIMO0 if the following conditions are met. SIMODIR0 = 1 (SPISIMO pin configured in GPIO mode as output pin) SIMODOUT0 = 1 0 = Output value on SPISIMO0 pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISIMO0 is Tri-stated Note: Bit 10 or bit 16 can be used to set the direction for pin SPISIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simopdr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOMIPDR0` reader - 11:11\\]
SPISOMI0 Open drain enable Enables Open drain capability for the pin SPISOMI if the following conditions are met. SOMIDIR0 = 1 (SPISOMI0 pin configured in GPIO mode as output pin) SOMIDOUT0 = 1 0 = Output value on SPISOMI0 pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISOMI0 is Tri-stated Note: Bit 11 or bit 24 can be used to set the direction for pin SPISOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somipdr0R = crate::BitReader;
#[doc = "Field `SOMIPDR0` writer - 11:11\\]
SPISOMI0 Open drain enable Enables Open drain capability for the pin SPISOMI if the following conditions are met. SOMIDIR0 = 1 (SPISOMI0 pin configured in GPIO mode as output pin) SOMIDOUT0 = 1 0 = Output value on SPISOMI0 pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISOMI0 is Tri-stated Note: Bit 11 or bit 24 can be used to set the direction for pin SPISOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somipdr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIMOPDR` reader - 23:16\\]
SPISIMOx Open drain enable Enables Open drain capability for the pin SPISIMOx if the following conditions are met. SIMODIRx = 1 (SPISIMOx pin configured in GPIO mode as output pin) SIMODOUTx = 1 0 = Output value on SPISIMOx pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISIMOx is Tri-stated"]
pub type SimopdrR = crate::FieldReader;
#[doc = "Field `SIMOPDR` writer - 23:16\\]
SPISIMOx Open drain enable Enables Open drain capability for the pin SPISIMOx if the following conditions are met. SIMODIRx = 1 (SPISIMOx pin configured in GPIO mode as output pin) SIMODOUTx = 1 0 = Output value on SPISIMOx pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISIMOx is Tri-stated"]
pub type SimopdrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOMIPDR` reader - 31:24\\]
SPISOMIx Open drain enable Enables Open drain capability for the pin SOMIx if the following conditions are met. SOMIDIRx = 1 (SPISOMI0 pin configured in GPIO mode as output pin) SOMIDOUTx = 1 0 = Output value on SPISOMIx pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISOMIx is Tri-stated"]
pub type SomipdrR = crate::FieldReader;
#[doc = "Field `SOMIPDR` writer - 31:24\\]
SPISOMIx Open drain enable Enables Open drain capability for the pin SOMIx if the following conditions are met. SOMIDIRx = 1 (SPISOMI0 pin configured in GPIO mode as output pin) SOMIDOUTx = 1 0 = Output value on SPISOMIx pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISOMIx is Tri-stated"]
pub type SomipdrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCSx Open drain enable Enables Open drain capability for the pin SPISCSx if the following conditions are met. SCSDIRx = 1 (SPISCS pin configured in GPIO mode as output pin) SCSDOUTx = 1 0 = Output value on SPISCSx pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISCSx is Tri-stated Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSPDR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    pub fn scspdr(&self) -> ScspdrR {
        ScspdrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA Open drain enable Enables Open drain capability for the pin SPIENA if the following conditions are met. ENABLEDIR = 1 (SPIENA pin configured in GPIO mode as output pin) ENABLEDOUT = 1 0 = Output value on SPIENA pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPIENA is Tri-stated"]
    #[inline(always)]
    pub fn enapdr(&self) -> EnapdrR {
        EnapdrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
SPICLK Open drain enable Enables Open drain capability for the pin CLK if the following conditions are met. CLKDIR = 1 (SPICLK pin configured in GPIO mode as output pin) CLKDOUT = 1 0 = Output value on SPICLK pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPICLK is Tri-stated"]
    #[inline(always)]
    pub fn clkpdr(&self) -> ClkpdrR {
        ClkpdrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
SPISIMO0 Open drain enable Enables Open drain capability for the pin SPISIMO0 if the following conditions are met. SIMODIR0 = 1 (SPISIMO pin configured in GPIO mode as output pin) SIMODOUT0 = 1 0 = Output value on SPISIMO0 pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISIMO0 is Tri-stated Note: Bit 10 or bit 16 can be used to set the direction for pin SPISIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    pub fn simopdr0(&self) -> Simopdr0R {
        Simopdr0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
SPISOMI0 Open drain enable Enables Open drain capability for the pin SPISOMI if the following conditions are met. SOMIDIR0 = 1 (SPISOMI0 pin configured in GPIO mode as output pin) SOMIDOUT0 = 1 0 = Output value on SPISOMI0 pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISOMI0 is Tri-stated Note: Bit 11 or bit 24 can be used to set the direction for pin SPISOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    pub fn somipdr0(&self) -> Somipdr0R {
        Somipdr0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SPISIMOx Open drain enable Enables Open drain capability for the pin SPISIMOx if the following conditions are met. SIMODIRx = 1 (SPISIMOx pin configured in GPIO mode as output pin) SIMODOUTx = 1 0 = Output value on SPISIMOx pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISIMOx is Tri-stated"]
    #[inline(always)]
    pub fn simopdr(&self) -> SimopdrR {
        SimopdrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SPISOMIx Open drain enable Enables Open drain capability for the pin SOMIx if the following conditions are met. SOMIDIRx = 1 (SPISOMI0 pin configured in GPIO mode as output pin) SOMIDOUTx = 1 0 = Output value on SPISOMIx pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISOMIx is Tri-stated"]
    #[inline(always)]
    pub fn somipdr(&self) -> SomipdrR {
        SomipdrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCSx Open drain enable Enables Open drain capability for the pin SPISCSx if the following conditions are met. SCSDIRx = 1 (SPISCS pin configured in GPIO mode as output pin) SCSDOUTx = 1 0 = Output value on SPISCSx pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISCSx is Tri-stated Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSPDR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    #[must_use]
    pub fn scspdr(&mut self) -> ScspdrW<Spipc6Spec> {
        ScspdrW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA Open drain enable Enables Open drain capability for the pin SPIENA if the following conditions are met. ENABLEDIR = 1 (SPIENA pin configured in GPIO mode as output pin) ENABLEDOUT = 1 0 = Output value on SPIENA pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPIENA is Tri-stated"]
    #[inline(always)]
    #[must_use]
    pub fn enapdr(&mut self) -> EnapdrW<Spipc6Spec> {
        EnapdrW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
SPICLK Open drain enable Enables Open drain capability for the pin CLK if the following conditions are met. CLKDIR = 1 (SPICLK pin configured in GPIO mode as output pin) CLKDOUT = 1 0 = Output value on SPICLK pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPICLK is Tri-stated"]
    #[inline(always)]
    #[must_use]
    pub fn clkpdr(&mut self) -> ClkpdrW<Spipc6Spec> {
        ClkpdrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
SPISIMO0 Open drain enable Enables Open drain capability for the pin SPISIMO0 if the following conditions are met. SIMODIR0 = 1 (SPISIMO pin configured in GPIO mode as output pin) SIMODOUT0 = 1 0 = Output value on SPISIMO0 pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISIMO0 is Tri-stated Note: Bit 10 or bit 16 can be used to set the direction for pin SPISIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn simopdr0(&mut self) -> Simopdr0W<Spipc6Spec> {
        Simopdr0W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
SPISOMI0 Open drain enable Enables Open drain capability for the pin SPISOMI if the following conditions are met. SOMIDIR0 = 1 (SPISOMI0 pin configured in GPIO mode as output pin) SOMIDOUT0 = 1 0 = Output value on SPISOMI0 pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISOMI0 is Tri-stated Note: Bit 11 or bit 24 can be used to set the direction for pin SPISOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    #[must_use]
    pub fn somipdr0(&mut self) -> Somipdr0W<Spipc6Spec> {
        Somipdr0W::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Spipc6Spec> {
        NuW::new(self, 12)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SPISIMOx Open drain enable Enables Open drain capability for the pin SPISIMOx if the following conditions are met. SIMODIRx = 1 (SPISIMOx pin configured in GPIO mode as output pin) SIMODOUTx = 1 0 = Output value on SPISIMOx pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISIMOx is Tri-stated"]
    #[inline(always)]
    #[must_use]
    pub fn simopdr(&mut self) -> SimopdrW<Spipc6Spec> {
        SimopdrW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SPISOMIx Open drain enable Enables Open drain capability for the pin SOMIx if the following conditions are met. SOMIDIRx = 1 (SPISOMI0 pin configured in GPIO mode as output pin) SOMIDOUTx = 1 0 = Output value on SPISOMIx pin is logic ΓÇÿ1ΓÇÖ 1 = Output pin SPISOMIx is Tri-stated"]
    #[inline(always)]
    #[must_use]
    pub fn somipdr(&mut self) -> SomipdrW<Spipc6Spec> {
        SomipdrW::new(self, 24)
    }
}
#[doc = "SPI / MibSPI Pin Control Register 6 (SPIPC6) - SPIPDR\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipc6Spec;
impl crate::RegisterSpec for Spipc6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipc6::R`](R) reader structure"]
impl crate::Readable for Spipc6Spec {}
#[doc = "`write(|w| ..)` method takes [`spipc6::W`](W) writer structure"]
impl crate::Writable for Spipc6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIPC6 to value 0"]
impl crate::Resettable for Spipc6Spec {
    const RESET_VALUE: u32 = 0;
}
