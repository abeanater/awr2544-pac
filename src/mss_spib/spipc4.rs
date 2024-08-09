#[doc = "Register `SPIPC4` reader"]
pub type R = crate::R<Spipc4Spec>;
#[doc = "Register `SPIPC4` writer"]
pub type W = crate::W<Spipc4Spec>;
#[doc = "Field `SCSSET` reader - 7:0\\]
SPISCS\\[7:0\\]
dataout set. Only active when the SPISCSx pins are configured as a general-purpose output pins. A value of ΓÇÿ1ΓÇÖ written to these bits set the corresponding SCSDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISCSx pin if it is in General Purpose O/P mode Read: 0= Current value on SCSDOUTx is 0. 1= Current value on SCSDOUTx is 1. Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSSET\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always. Note: Register Read Read of SPIPC4 register gives out contents of the SPIPC3 register."]
pub type ScssetR = crate::FieldReader;
#[doc = "Field `SCSSET` writer - 7:0\\]
SPISCS\\[7:0\\]
dataout set. Only active when the SPISCSx pins are configured as a general-purpose output pins. A value of ΓÇÿ1ΓÇÖ written to these bits set the corresponding SCSDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISCSx pin if it is in General Purpose O/P mode Read: 0= Current value on SCSDOUTx is 0. 1= Current value on SCSDOUTx is 1. Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSSET\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always. Note: Register Read Read of SPIPC4 register gives out contents of the SPIPC3 register."]
pub type ScssetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENASET` reader - 8:8\\]
SPIENA dataout set. Only active when the SPIENA pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding ENABLEDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPIENA pin if it is in General Purpose O/P mode Read: 0= Current value on ENADOUT is 0. 1= Current value on ENADOUT is 1"]
pub type EnasetR = crate::BitReader;
#[doc = "Field `ENASET` writer - 8:8\\]
SPIENA dataout set. Only active when the SPIENA pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding ENABLEDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPIENA pin if it is in General Purpose O/P mode Read: 0= Current value on ENADOUT is 0. 1= Current value on ENADOUT is 1"]
pub type EnasetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSET` reader - 9:9\\]
SPICLK dataout set. Only active when the SPICLK pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding CLKDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPICLK pin if it is in General Purpose O/P mode Read: 0= Current value on CLKDOUT pin is logic 0. 1= Current value on CLKDOUT pin is logic 1"]
pub type ClksetR = crate::BitReader;
#[doc = "Field `CLKSET` writer - 9:9\\]
SPICLK dataout set. Only active when the SPICLK pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding CLKDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPICLK pin if it is in General Purpose O/P mode Read: 0= Current value on CLKDOUT pin is logic 0. 1= Current value on CLKDOUT pin is logic 1"]
pub type ClksetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMOSET0` reader - 10:10\\]
SPISIMO0 dataout set. Only active when the SPISIMO0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISIMODOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISIMO0 pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUT0 is 0. 1= Current value on SIMODOUT0 is 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simoset0R = crate::BitReader;
#[doc = "Field `SIMOSET0` writer - 10:10\\]
SPISIMO0 dataout set. Only active when the SPISIMO0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISIMODOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISIMO0 pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUT0 is 0. 1= Current value on SIMODOUT0 is 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simoset0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOMISET0` reader - 11:11\\]
SPISOMI0 dataout set. Only active when the SPISOMI0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISOMIDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISOMI0 pin if it is in General Purpose O/P mode Read: 0= Current value on SOMIDOUT0 is 0. 1= Current value on SOMIDOUT0 is 1. Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somiset0R = crate::BitReader;
#[doc = "Field `SOMISET0` writer - 11:11\\]
SPISOMI0 dataout set. Only active when the SPISOMI0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISOMIDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISOMI0 pin if it is in General Purpose O/P mode Read: 0= Current value on SOMIDOUT0 is 0. 1= Current value on SOMIDOUT0 is 1. Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somiset0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIMOSET` reader - 23:16\\]
SPISIMOx dataout set. Only active when the SPISIMOx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISIMODOUTx bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISIMOx pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUTx is 0. 1= Current value on SIMODOUTx is 1"]
pub type SimosetR = crate::FieldReader;
#[doc = "Field `SIMOSET` writer - 23:16\\]
SPISIMOx dataout set. Only active when the SPISIMOx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISIMODOUTx bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISIMOx pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUTx is 0. 1= Current value on SIMODOUTx is 1"]
pub type SimosetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOMISET` reader - 31:24\\]
SPISOMIx dataout set. Only active when the SPISOMIx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISOMIDOUTx bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISOMIx pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUTx is 0. 1= Current value on SOMIDOUTx is 1."]
pub type SomisetR = crate::FieldReader;
#[doc = "Field `SOMISET` writer - 31:24\\]
SPISOMIx dataout set. Only active when the SPISOMIx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISOMIDOUTx bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISOMIx pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUTx is 0. 1= Current value on SOMIDOUTx is 1."]
pub type SomisetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCS\\[7:0\\]
dataout set. Only active when the SPISCSx pins are configured as a general-purpose output pins. A value of ΓÇÿ1ΓÇÖ written to these bits set the corresponding SCSDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISCSx pin if it is in General Purpose O/P mode Read: 0= Current value on SCSDOUTx is 0. 1= Current value on SCSDOUTx is 1. Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSSET\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always. Note: Register Read Read of SPIPC4 register gives out contents of the SPIPC3 register."]
    #[inline(always)]
    pub fn scsset(&self) -> ScssetR {
        ScssetR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA dataout set. Only active when the SPIENA pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding ENABLEDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPIENA pin if it is in General Purpose O/P mode Read: 0= Current value on ENADOUT is 0. 1= Current value on ENADOUT is 1"]
    #[inline(always)]
    pub fn enaset(&self) -> EnasetR {
        EnasetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
SPICLK dataout set. Only active when the SPICLK pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding CLKDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPICLK pin if it is in General Purpose O/P mode Read: 0= Current value on CLKDOUT pin is logic 0. 1= Current value on CLKDOUT pin is logic 1"]
    #[inline(always)]
    pub fn clkset(&self) -> ClksetR {
        ClksetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
SPISIMO0 dataout set. Only active when the SPISIMO0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISIMODOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISIMO0 pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUT0 is 0. 1= Current value on SIMODOUT0 is 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    pub fn simoset0(&self) -> Simoset0R {
        Simoset0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
SPISOMI0 dataout set. Only active when the SPISOMI0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISOMIDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISOMI0 pin if it is in General Purpose O/P mode Read: 0= Current value on SOMIDOUT0 is 0. 1= Current value on SOMIDOUT0 is 1. Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    pub fn somiset0(&self) -> Somiset0R {
        Somiset0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SPISIMOx dataout set. Only active when the SPISIMOx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISIMODOUTx bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISIMOx pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUTx is 0. 1= Current value on SIMODOUTx is 1"]
    #[inline(always)]
    pub fn simoset(&self) -> SimosetR {
        SimosetR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SPISOMIx dataout set. Only active when the SPISOMIx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISOMIDOUTx bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISOMIx pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUTx is 0. 1= Current value on SOMIDOUTx is 1."]
    #[inline(always)]
    pub fn somiset(&self) -> SomisetR {
        SomisetR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCS\\[7:0\\]
dataout set. Only active when the SPISCSx pins are configured as a general-purpose output pins. A value of ΓÇÿ1ΓÇÖ written to these bits set the corresponding SCSDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISCSx pin if it is in General Purpose O/P mode Read: 0= Current value on SCSDOUTx is 0. 1= Current value on SCSDOUTx is 1. Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSSET\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always. Note: Register Read Read of SPIPC4 register gives out contents of the SPIPC3 register."]
    #[inline(always)]
    #[must_use]
    pub fn scsset(&mut self) -> ScssetW<Spipc4Spec> {
        ScssetW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA dataout set. Only active when the SPIENA pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding ENABLEDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPIENA pin if it is in General Purpose O/P mode Read: 0= Current value on ENADOUT is 0. 1= Current value on ENADOUT is 1"]
    #[inline(always)]
    #[must_use]
    pub fn enaset(&mut self) -> EnasetW<Spipc4Spec> {
        EnasetW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
SPICLK dataout set. Only active when the SPICLK pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding CLKDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPICLK pin if it is in General Purpose O/P mode Read: 0= Current value on CLKDOUT pin is logic 0. 1= Current value on CLKDOUT pin is logic 1"]
    #[inline(always)]
    #[must_use]
    pub fn clkset(&mut self) -> ClksetW<Spipc4Spec> {
        ClksetW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
SPISIMO0 dataout set. Only active when the SPISIMO0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISIMODOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISIMO0 pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUT0 is 0. 1= Current value on SIMODOUT0 is 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn simoset0(&mut self) -> Simoset0W<Spipc4Spec> {
        Simoset0W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
SPISOMI0 dataout set. Only active when the SPISOMI0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISOMIDOUT bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISOMI0 pin if it is in General Purpose O/P mode Read: 0= Current value on SOMIDOUT0 is 0. 1= Current value on SOMIDOUT0 is 1. Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    #[must_use]
    pub fn somiset0(&mut self) -> Somiset0W<Spipc4Spec> {
        Somiset0W::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Spipc4Spec> {
        NuW::new(self, 12)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SPISIMOx dataout set. Only active when the SPISIMOx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISIMODOUTx bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISIMOx pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUTx is 0. 1= Current value on SIMODOUTx is 1"]
    #[inline(always)]
    #[must_use]
    pub fn simoset(&mut self) -> SimosetW<Spipc4Spec> {
        SimosetW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SPISOMIx dataout set. Only active when the SPISOMIx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit sets the corresponding SPISOMIDOUTx bit to ΓÇÿ1ΓÇÖ. Write: 0= Has no effect 1= Logic 1 placed on SPISOMIx pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUTx is 0. 1= Current value on SOMIDOUTx is 1."]
    #[inline(always)]
    #[must_use]
    pub fn somiset(&mut self) -> SomisetW<Spipc4Spec> {
        SomisetW::new(self, 24)
    }
}
#[doc = "SPI / MibSPI Pin Control Register 4 (SPIPC4) - SPIDSET\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipc4Spec;
impl crate::RegisterSpec for Spipc4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipc4::R`](R) reader structure"]
impl crate::Readable for Spipc4Spec {}
#[doc = "`write(|w| ..)` method takes [`spipc4::W`](W) writer structure"]
impl crate::Writable for Spipc4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIPC4 to value 0"]
impl crate::Resettable for Spipc4Spec {
    const RESET_VALUE: u32 = 0;
}
