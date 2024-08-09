#[doc = "Register `SPIPC3` reader"]
pub type R = crate::R<Spipc3Spec>;
#[doc = "Register `SPIPC3` writer"]
pub type W = crate::W<Spipc3Spec>;
#[doc = "Field `SCSDOUT` reader - 7:0\\]
SPISCS\\[7:0\\]
dataout write. Only active when the SPISCSx pins are configured as a general-purpose I/O pins and configured as an output pins. The value of these bit indicates the value sent to the pins. 0=Current value on SPISCSx pin is logic 0. 1=Current value on SPISCSx pin is logic 1 Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSDOUT\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
pub type ScsdoutR = crate::FieldReader;
#[doc = "Field `SCSDOUT` writer - 7:0\\]
SPISCS\\[7:0\\]
dataout write. Only active when the SPISCSx pins are configured as a general-purpose I/O pins and configured as an output pins. The value of these bit indicates the value sent to the pins. 0=Current value on SPISCSx pin is logic 0. 1=Current value on SPISCSx pin is logic 1 Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSDOUT\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
pub type ScsdoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENADOUT` reader - 8:8\\]
SPIENA dataout write. Only active when the SPIENA pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPIENA pin is logic 0. 1=Current value on SPIENA pin is logic 1"]
pub type EnadoutR = crate::BitReader;
#[doc = "Field `ENADOUT` writer - 8:8\\]
SPIENA dataout write. Only active when the SPIENA pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPIENA pin is logic 0. 1=Current value on SPIENA pin is logic 1"]
pub type EnadoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDOUT` reader - 9:9\\]
SPICLK dataout write. Only active when the SPICLK pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPICLK pin is logic 0. 1=Current value on SPICLK pin is logic 1"]
pub type ClkdoutR = crate::BitReader;
#[doc = "Field `CLKDOUT` writer - 9:9\\]
SPICLK dataout write. Only active when the SPICLK pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPICLK pin is logic 0. 1=Current value on SPICLK pin is logic 1"]
pub type ClkdoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMODOUT0` reader - 10:10\\]
SPISIMO0 dataout write. Only active when the SPISIMO0 pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISIMO0 pin is logic 0. 1=Current value on SPISIMO0 pin is logic 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simodout0R = crate::BitReader;
#[doc = "Field `SIMODOUT0` writer - 10:10\\]
SPISIMO0 dataout write. Only active when the SPISIMO0 pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISIMO0 pin is logic 0. 1=Current value on SPISIMO0 pin is logic 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simodout0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOMIDOUT0` reader - 11:11\\]
SPISOMI0 dataout write. Only active when the SPISOMI0 pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISOMI0 pin is logic 0. 1=Current value on SPISOMI0 pin is logic 1. Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somidout0R = crate::BitReader;
#[doc = "Field `SOMIDOUT0` writer - 11:11\\]
SPISOMI0 dataout write. Only active when the SPISOMI0 pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISOMI0 pin is logic 0. 1=Current value on SPISOMI0 pin is logic 1. Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somidout0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIMODOUT` reader - 23:16\\]
SPISIMOx dataout write. Only active when the SPISIMOx pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISIMOx pin is logic 0. 1=Current value on SPISIMOx pin is logic 1"]
pub type SimodoutR = crate::FieldReader;
#[doc = "Field `SIMODOUT` writer - 23:16\\]
SPISIMOx dataout write. Only active when the SPISIMOx pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISIMOx pin is logic 0. 1=Current value on SPISIMOx pin is logic 1"]
pub type SimodoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOMIDOUT` reader - 31:24\\]
SPISOMIx dataout write. Only active when the SPISOMIx pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISOMIx pin is logic 0. 1=Current value on SPISOMIx pin is logic 1"]
pub type SomidoutR = crate::FieldReader;
#[doc = "Field `SOMIDOUT` writer - 31:24\\]
SPISOMIx dataout write. Only active when the SPISOMIx pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISOMIx pin is logic 0. 1=Current value on SPISOMIx pin is logic 1"]
pub type SomidoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCS\\[7:0\\]
dataout write. Only active when the SPISCSx pins are configured as a general-purpose I/O pins and configured as an output pins. The value of these bit indicates the value sent to the pins. 0=Current value on SPISCSx pin is logic 0. 1=Current value on SPISCSx pin is logic 1 Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSDOUT\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    pub fn scsdout(&self) -> ScsdoutR {
        ScsdoutR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA dataout write. Only active when the SPIENA pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPIENA pin is logic 0. 1=Current value on SPIENA pin is logic 1"]
    #[inline(always)]
    pub fn enadout(&self) -> EnadoutR {
        EnadoutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
SPICLK dataout write. Only active when the SPICLK pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPICLK pin is logic 0. 1=Current value on SPICLK pin is logic 1"]
    #[inline(always)]
    pub fn clkdout(&self) -> ClkdoutR {
        ClkdoutR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
SPISIMO0 dataout write. Only active when the SPISIMO0 pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISIMO0 pin is logic 0. 1=Current value on SPISIMO0 pin is logic 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    pub fn simodout0(&self) -> Simodout0R {
        Simodout0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
SPISOMI0 dataout write. Only active when the SPISOMI0 pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISOMI0 pin is logic 0. 1=Current value on SPISOMI0 pin is logic 1. Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    pub fn somidout0(&self) -> Somidout0R {
        Somidout0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SPISIMOx dataout write. Only active when the SPISIMOx pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISIMOx pin is logic 0. 1=Current value on SPISIMOx pin is logic 1"]
    #[inline(always)]
    pub fn simodout(&self) -> SimodoutR {
        SimodoutR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SPISOMIx dataout write. Only active when the SPISOMIx pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISOMIx pin is logic 0. 1=Current value on SPISOMIx pin is logic 1"]
    #[inline(always)]
    pub fn somidout(&self) -> SomidoutR {
        SomidoutR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCS\\[7:0\\]
dataout write. Only active when the SPISCSx pins are configured as a general-purpose I/O pins and configured as an output pins. The value of these bit indicates the value sent to the pins. 0=Current value on SPISCSx pin is logic 0. 1=Current value on SPISCSx pin is logic 1 Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSDOUT\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    #[must_use]
    pub fn scsdout(&mut self) -> ScsdoutW<Spipc3Spec> {
        ScsdoutW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA dataout write. Only active when the SPIENA pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPIENA pin is logic 0. 1=Current value on SPIENA pin is logic 1"]
    #[inline(always)]
    #[must_use]
    pub fn enadout(&mut self) -> EnadoutW<Spipc3Spec> {
        EnadoutW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
SPICLK dataout write. Only active when the SPICLK pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPICLK pin is logic 0. 1=Current value on SPICLK pin is logic 1"]
    #[inline(always)]
    #[must_use]
    pub fn clkdout(&mut self) -> ClkdoutW<Spipc3Spec> {
        ClkdoutW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
SPISIMO0 dataout write. Only active when the SPISIMO0 pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISIMO0 pin is logic 0. 1=Current value on SPISIMO0 pin is logic 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn simodout0(&mut self) -> Simodout0W<Spipc3Spec> {
        Simodout0W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
SPISOMI0 dataout write. Only active when the SPISOMI0 pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISOMI0 pin is logic 0. 1=Current value on SPISOMI0 pin is logic 1. Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    #[must_use]
    pub fn somidout0(&mut self) -> Somidout0W<Spipc3Spec> {
        Somidout0W::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Spipc3Spec> {
        NuW::new(self, 12)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SPISIMOx dataout write. Only active when the SPISIMOx pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISIMOx pin is logic 0. 1=Current value on SPISIMOx pin is logic 1"]
    #[inline(always)]
    #[must_use]
    pub fn simodout(&mut self) -> SimodoutW<Spipc3Spec> {
        SimodoutW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SPISOMIx dataout write. Only active when the SPISOMIx pin is configured as a general-purpose I/O pin and configured as an output pin. The value of this bit indicates the value sent to the pin. 0=Current value on SPISOMIx pin is logic 0. 1=Current value on SPISOMIx pin is logic 1"]
    #[inline(always)]
    #[must_use]
    pub fn somidout(&mut self) -> SomidoutW<Spipc3Spec> {
        SomidoutW::new(self, 24)
    }
}
#[doc = "SPI / MibSPI Pin Control Register 3 (SPIPC3) - SPIDOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipc3Spec;
impl crate::RegisterSpec for Spipc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipc3::R`](R) reader structure"]
impl crate::Readable for Spipc3Spec {}
#[doc = "`write(|w| ..)` method takes [`spipc3::W`](W) writer structure"]
impl crate::Writable for Spipc3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIPC3 to value 0"]
impl crate::Resettable for Spipc3Spec {
    const RESET_VALUE: u32 = 0;
}
