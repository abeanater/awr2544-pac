#[doc = "Register `SPIPC2` reader"]
pub type R = crate::R<Spipc2Spec>;
#[doc = "Register `SPIPC2` writer"]
pub type W = crate::W<Spipc2Spec>;
#[doc = "Field `SCSDIN` reader - 7:0\\]
SPISCS\\[7:0\\]
data in. Reflects the value of the SPISCSx pins. 0=Current value on SPISCSx pin is logic 0. 1=Current value on SPISCSx pin is logic 1 Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSDIN\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will read ΓÇÿ0ΓÇÖ always."]
pub type ScsdinR = crate::FieldReader;
#[doc = "Field `SCSDIN` writer - 7:0\\]
SPISCS\\[7:0\\]
data in. Reflects the value of the SPISCSx pins. 0=Current value on SPISCSx pin is logic 0. 1=Current value on SPISCSx pin is logic 1 Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSDIN\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will read ΓÇÿ0ΓÇÖ always."]
pub type ScsdinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENADIN` reader - 8:8\\]
SPIENA data in. Reflects the value of the SPIENA pin. 0=Current value on SPIENA pin is logic 0. 1=Current value on SPIENA pin is logic 1"]
pub type EnadinR = crate::BitReader;
#[doc = "Field `ENADIN` writer - 8:8\\]
SPIENA data in. Reflects the value of the SPIENA pin. 0=Current value on SPIENA pin is logic 0. 1=Current value on SPIENA pin is logic 1"]
pub type EnadinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIN` reader - 9:9\\]
Clock data in. Reflects the value of the SPICLK pin. 0=Current value on SPICLK pin is logic 0. 1=Current value on SPICLK pin is logic 1"]
pub type ClkdinR = crate::BitReader;
#[doc = "Field `CLKDIN` writer - 9:9\\]
Clock data in. Reflects the value of the SPICLK pin. 0=Current value on SPICLK pin is logic 0. 1=Current value on SPICLK pin is logic 1"]
pub type ClkdinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMODIN0` reader - 10:10\\]
SPISIMO0 data in. Reflects the value of the SPISIMO0 pin. 0=Current value on SPISIMO0 pin is logic 0. 1=Current value on SPISIMO0 pin is logic 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simodin0R = crate::BitReader;
#[doc = "Field `SIMODIN0` writer - 10:10\\]
SPISIMO0 data in. Reflects the value of the SPISIMO0 pin. 0=Current value on SPISIMO0 pin is logic 0. 1=Current value on SPISIMO0 pin is logic 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simodin0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOMIDIN0` reader - 11:11\\]
SPISOMI0 data in. Reflects the value of the SPISOMI0 pin. 0=Current value on SPISOMI0 pin is logic 0. 1=Current value on SPISOMI0 pin is logic 1 Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somidin0R = crate::BitReader;
#[doc = "Field `SOMIDIN0` writer - 11:11\\]
SPISOMI0 data in. Reflects the value of the SPISOMI0 pin. 0=Current value on SPISOMI0 pin is logic 0. 1=Current value on SPISOMI0 pin is logic 1 Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somidin0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIMODIN` reader - 23:16\\]
SPISIMOx data in. Reflects the value of the SPISIMOx pin. 0=Current value on SPISIMOx pin is logic 0. 1=Current value on SPISIMOx pin is logic 1"]
pub type SimodinR = crate::FieldReader;
#[doc = "Field `SIMODIN` writer - 23:16\\]
SPISIMOx data in. Reflects the value of the SPISIMOx pin. 0=Current value on SPISIMOx pin is logic 0. 1=Current value on SPISIMOx pin is logic 1"]
pub type SimodinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOMIDIN` reader - 31:24\\]
SPISOMIx data in. Reflects the value of the SPISOMIx pin. 0=Current value on SPISOMIx pin is logic 0. 1=Current value on SPISOMIx pin is logic 1"]
pub type SomidinR = crate::FieldReader;
#[doc = "Field `SOMIDIN` writer - 31:24\\]
SPISOMIx data in. Reflects the value of the SPISOMIx pin. 0=Current value on SPISOMIx pin is logic 0. 1=Current value on SPISOMIx pin is logic 1"]
pub type SomidinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCS\\[7:0\\]
data in. Reflects the value of the SPISCSx pins. 0=Current value on SPISCSx pin is logic 0. 1=Current value on SPISCSx pin is logic 1 Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSDIN\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    pub fn scsdin(&self) -> ScsdinR {
        ScsdinR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA data in. Reflects the value of the SPIENA pin. 0=Current value on SPIENA pin is logic 0. 1=Current value on SPIENA pin is logic 1"]
    #[inline(always)]
    pub fn enadin(&self) -> EnadinR {
        EnadinR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Clock data in. Reflects the value of the SPICLK pin. 0=Current value on SPICLK pin is logic 0. 1=Current value on SPICLK pin is logic 1"]
    #[inline(always)]
    pub fn clkdin(&self) -> ClkdinR {
        ClkdinR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
SPISIMO0 data in. Reflects the value of the SPISIMO0 pin. 0=Current value on SPISIMO0 pin is logic 0. 1=Current value on SPISIMO0 pin is logic 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    pub fn simodin0(&self) -> Simodin0R {
        Simodin0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
SPISOMI0 data in. Reflects the value of the SPISOMI0 pin. 0=Current value on SPISOMI0 pin is logic 0. 1=Current value on SPISOMI0 pin is logic 1 Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    pub fn somidin0(&self) -> Somidin0R {
        Somidin0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SPISIMOx data in. Reflects the value of the SPISIMOx pin. 0=Current value on SPISIMOx pin is logic 0. 1=Current value on SPISIMOx pin is logic 1"]
    #[inline(always)]
    pub fn simodin(&self) -> SimodinR {
        SimodinR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SPISOMIx data in. Reflects the value of the SPISOMIx pin. 0=Current value on SPISOMIx pin is logic 0. 1=Current value on SPISOMIx pin is logic 1"]
    #[inline(always)]
    pub fn somidin(&self) -> SomidinR {
        SomidinR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCS\\[7:0\\]
data in. Reflects the value of the SPISCSx pins. 0=Current value on SPISCSx pin is logic 0. 1=Current value on SPISCSx pin is logic 1 Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSDIN\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    #[must_use]
    pub fn scsdin(&mut self) -> ScsdinW<Spipc2Spec> {
        ScsdinW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA data in. Reflects the value of the SPIENA pin. 0=Current value on SPIENA pin is logic 0. 1=Current value on SPIENA pin is logic 1"]
    #[inline(always)]
    #[must_use]
    pub fn enadin(&mut self) -> EnadinW<Spipc2Spec> {
        EnadinW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Clock data in. Reflects the value of the SPICLK pin. 0=Current value on SPICLK pin is logic 0. 1=Current value on SPICLK pin is logic 1"]
    #[inline(always)]
    #[must_use]
    pub fn clkdin(&mut self) -> ClkdinW<Spipc2Spec> {
        ClkdinW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
SPISIMO0 data in. Reflects the value of the SPISIMO0 pin. 0=Current value on SPISIMO0 pin is logic 0. 1=Current value on SPISIMO0 pin is logic 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn simodin0(&mut self) -> Simodin0W<Spipc2Spec> {
        Simodin0W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
SPISOMI0 data in. Reflects the value of the SPISOMI0 pin. 0=Current value on SPISOMI0 pin is logic 0. 1=Current value on SPISOMI0 pin is logic 1 Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    #[must_use]
    pub fn somidin0(&mut self) -> Somidin0W<Spipc2Spec> {
        Somidin0W::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Spipc2Spec> {
        NuW::new(self, 12)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SPISIMOx data in. Reflects the value of the SPISIMOx pin. 0=Current value on SPISIMOx pin is logic 0. 1=Current value on SPISIMOx pin is logic 1"]
    #[inline(always)]
    #[must_use]
    pub fn simodin(&mut self) -> SimodinW<Spipc2Spec> {
        SimodinW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SPISOMIx data in. Reflects the value of the SPISOMIx pin. 0=Current value on SPISOMIx pin is logic 0. 1=Current value on SPISOMIx pin is logic 1"]
    #[inline(always)]
    #[must_use]
    pub fn somidin(&mut self) -> SomidinW<Spipc2Spec> {
        SomidinW::new(self, 24)
    }
}
#[doc = "SPI / MibSPI Pin Control Register 2 (SPIPC2) - SPIDIN\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipc2Spec;
impl crate::RegisterSpec for Spipc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipc2::R`](R) reader structure"]
impl crate::Readable for Spipc2Spec {}
#[doc = "`write(|w| ..)` method takes [`spipc2::W`](W) writer structure"]
impl crate::Writable for Spipc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIPC2 to value 0"]
impl crate::Resettable for Spipc2Spec {
    const RESET_VALUE: u32 = 0;
}
