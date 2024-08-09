#[doc = "Register `SPIPC5` reader"]
pub type R = crate::R<Spipc5Spec>;
#[doc = "Register `SPIPC5` writer"]
pub type W = crate::W<Spipc5Spec>;
#[doc = "Field `SCSCLR` reader - 7:0\\]
SPISCS\\[7:0\\]
dataout clear. Only active when the SPISCSx pins are configured as a general-purpose output pins. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SCSDOUT bit to ΓÇÿ0.. Write: 0= Has no effect 1= Logic 0 placed on SPISCSx pin if it is in General Purpose O/P mode Read: 0= Current value on SCSDOUTx is 0. 1= Current value on SCSDOUTx is 1 Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSCLR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always. Note: Register Read Read of SPIPC5 register gives out contents of the SPIPC3 register."]
pub type ScsclrR = crate::FieldReader;
#[doc = "Field `SCSCLR` writer - 7:0\\]
SPISCS\\[7:0\\]
dataout clear. Only active when the SPISCSx pins are configured as a general-purpose output pins. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SCSDOUT bit to ΓÇÿ0.. Write: 0= Has no effect 1= Logic 0 placed on SPISCSx pin if it is in General Purpose O/P mode Read: 0= Current value on SCSDOUTx is 0. 1= Current value on SCSDOUTx is 1 Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSCLR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always. Note: Register Read Read of SPIPC5 register gives out contents of the SPIPC3 register."]
pub type ScsclrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENACLR` reader - 8:8\\]
SPIENA dataout clear. Only active when the SPIENA pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding ENABLEDOUT bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPIENA pin if it is in General Purpose O/P mode Read: 0= Current value on ENADOUT is 0. 1= Current value on ENADOUT is 1."]
pub type EnaclrR = crate::BitReader;
#[doc = "Field `ENACLR` writer - 8:8\\]
SPIENA dataout clear. Only active when the SPIENA pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding ENABLEDOUT bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPIENA pin if it is in General Purpose O/P mode Read: 0= Current value on ENADOUT is 0. 1= Current value on ENADOUT is 1."]
pub type EnaclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKCLR` reader - 9:9\\]
SPICLK dataout clear. Only active when the SPICLK pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding CLKDOUT bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPICLK pin if it is in General Purpose O/P mode Read: 0= Current value on CLKDOUT is 0. 1= Current value on CLKDOUT is 1."]
pub type ClkclrR = crate::BitReader;
#[doc = "Field `CLKCLR` writer - 9:9\\]
SPICLK dataout clear. Only active when the SPICLK pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding CLKDOUT bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPICLK pin if it is in General Purpose O/P mode Read: 0= Current value on CLKDOUT is 0. 1= Current value on CLKDOUT is 1."]
pub type ClkclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMOCLR0` reader - 10:10\\]
SPISIMO0 dataout clear. Only active when the SPISIMO0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SIMODOUT0 bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISIMO0 pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUT0 is 0. 1= Current value on SIMODOUT0 is 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simoclr0R = crate::BitReader;
#[doc = "Field `SIMOCLR0` writer - 10:10\\]
SPISIMO0 dataout clear. Only active when the SPISIMO0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SIMODOUT0 bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISIMO0 pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUT0 is 0. 1= Current value on SIMODOUT0 is 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simoclr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOMICLR0` reader - 11:11\\]
SPISOMI0 dataout clear. Only active when the SPISOMI0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SPISOMIDOUT bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISOMI0 pin if it is in General Purpose O/P mode Read: 0= Current value on SOMIDOUT0 is 0. 1= Current value on SOMIDOUT0 is 1. Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somiclr0R = crate::BitReader;
#[doc = "Field `SOMICLR0` writer - 11:11\\]
SPISOMI0 dataout clear. Only active when the SPISOMI0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SPISOMIDOUT bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISOMI0 pin if it is in General Purpose O/P mode Read: 0= Current value on SOMIDOUT0 is 0. 1= Current value on SOMIDOUT0 is 1. Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somiclr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIMOCLR` reader - 23:16\\]
SPISIMOx dataout clear. Only active when the SPISIMOx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SPISIMODOUTx bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISIMOx pin if it is in General Purpose O/P mode Read: 0=Current value on SIMODOUTx is 0. 1=Current value on SIMODOUTx is 1."]
pub type SimoclrR = crate::FieldReader;
#[doc = "Field `SIMOCLR` writer - 23:16\\]
SPISIMOx dataout clear. Only active when the SPISIMOx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SPISIMODOUTx bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISIMOx pin if it is in General Purpose O/P mode Read: 0=Current value on SIMODOUTx is 0. 1=Current value on SIMODOUTx is 1."]
pub type SimoclrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOMICLR` reader - 31:24\\]
SPISOMIx dataout clear. Only active when the SPISOMIx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SPISOMIDOUTx bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISOMIx pin if it is in General Purpose O/P mode Read: 0= Current value on SOMIDOUTx is 0. 1= Current value on SOMIDOUTx is 1."]
pub type SomiclrR = crate::FieldReader;
#[doc = "Field `SOMICLR` writer - 31:24\\]
SPISOMIx dataout clear. Only active when the SPISOMIx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SPISOMIDOUTx bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISOMIx pin if it is in General Purpose O/P mode Read: 0= Current value on SOMIDOUTx is 0. 1= Current value on SOMIDOUTx is 1."]
pub type SomiclrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCS\\[7:0\\]
dataout clear. Only active when the SPISCSx pins are configured as a general-purpose output pins. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SCSDOUT bit to ΓÇÿ0.. Write: 0= Has no effect 1= Logic 0 placed on SPISCSx pin if it is in General Purpose O/P mode Read: 0= Current value on SCSDOUTx is 0. 1= Current value on SCSDOUTx is 1 Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSCLR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always. Note: Register Read Read of SPIPC5 register gives out contents of the SPIPC3 register."]
    #[inline(always)]
    pub fn scsclr(&self) -> ScsclrR {
        ScsclrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA dataout clear. Only active when the SPIENA pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding ENABLEDOUT bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPIENA pin if it is in General Purpose O/P mode Read: 0= Current value on ENADOUT is 0. 1= Current value on ENADOUT is 1."]
    #[inline(always)]
    pub fn enaclr(&self) -> EnaclrR {
        EnaclrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
SPICLK dataout clear. Only active when the SPICLK pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding CLKDOUT bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPICLK pin if it is in General Purpose O/P mode Read: 0= Current value on CLKDOUT is 0. 1= Current value on CLKDOUT is 1."]
    #[inline(always)]
    pub fn clkclr(&self) -> ClkclrR {
        ClkclrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
SPISIMO0 dataout clear. Only active when the SPISIMO0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SIMODOUT0 bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISIMO0 pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUT0 is 0. 1= Current value on SIMODOUT0 is 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    pub fn simoclr0(&self) -> Simoclr0R {
        Simoclr0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
SPISOMI0 dataout clear. Only active when the SPISOMI0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SPISOMIDOUT bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISOMI0 pin if it is in General Purpose O/P mode Read: 0= Current value on SOMIDOUT0 is 0. 1= Current value on SOMIDOUT0 is 1. Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    pub fn somiclr0(&self) -> Somiclr0R {
        Somiclr0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SPISIMOx dataout clear. Only active when the SPISIMOx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SPISIMODOUTx bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISIMOx pin if it is in General Purpose O/P mode Read: 0=Current value on SIMODOUTx is 0. 1=Current value on SIMODOUTx is 1."]
    #[inline(always)]
    pub fn simoclr(&self) -> SimoclrR {
        SimoclrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SPISOMIx dataout clear. Only active when the SPISOMIx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SPISOMIDOUTx bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISOMIx pin if it is in General Purpose O/P mode Read: 0= Current value on SOMIDOUTx is 0. 1= Current value on SOMIDOUTx is 1."]
    #[inline(always)]
    pub fn somiclr(&self) -> SomiclrR {
        SomiclrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCS\\[7:0\\]
dataout clear. Only active when the SPISCSx pins are configured as a general-purpose output pins. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SCSDOUT bit to ΓÇÿ0.. Write: 0= Has no effect 1= Logic 0 placed on SPISCSx pin if it is in General Purpose O/P mode Read: 0= Current value on SCSDOUTx is 0. 1= Current value on SCSDOUTx is 1 Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSCLR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always. Note: Register Read Read of SPIPC5 register gives out contents of the SPIPC3 register."]
    #[inline(always)]
    #[must_use]
    pub fn scsclr(&mut self) -> ScsclrW<Spipc5Spec> {
        ScsclrW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA dataout clear. Only active when the SPIENA pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding ENABLEDOUT bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPIENA pin if it is in General Purpose O/P mode Read: 0= Current value on ENADOUT is 0. 1= Current value on ENADOUT is 1."]
    #[inline(always)]
    #[must_use]
    pub fn enaclr(&mut self) -> EnaclrW<Spipc5Spec> {
        EnaclrW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
SPICLK dataout clear. Only active when the SPICLK pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding CLKDOUT bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPICLK pin if it is in General Purpose O/P mode Read: 0= Current value on CLKDOUT is 0. 1= Current value on CLKDOUT is 1."]
    #[inline(always)]
    #[must_use]
    pub fn clkclr(&mut self) -> ClkclrW<Spipc5Spec> {
        ClkclrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
SPISIMO0 dataout clear. Only active when the SPISIMO0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SIMODOUT0 bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISIMO0 pin if it is in General Purpose O/P mode Read: 0= Current value on SIMODOUT0 is 0. 1= Current value on SIMODOUT0 is 1. Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn simoclr0(&mut self) -> Simoclr0W<Spipc5Spec> {
        Simoclr0W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
SPISOMI0 dataout clear. Only active when the SPISOMI0 pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SPISOMIDOUT bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISOMI0 pin if it is in General Purpose O/P mode Read: 0= Current value on SOMIDOUT0 is 0. 1= Current value on SOMIDOUT0 is 1. Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    #[must_use]
    pub fn somiclr0(&mut self) -> Somiclr0W<Spipc5Spec> {
        Somiclr0W::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Spipc5Spec> {
        NuW::new(self, 12)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SPISIMOx dataout clear. Only active when the SPISIMOx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SPISIMODOUTx bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISIMOx pin if it is in General Purpose O/P mode Read: 0=Current value on SIMODOUTx is 0. 1=Current value on SIMODOUTx is 1."]
    #[inline(always)]
    #[must_use]
    pub fn simoclr(&mut self) -> SimoclrW<Spipc5Spec> {
        SimoclrW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SPISOMIx dataout clear. Only active when the SPISOMIx pin is configured as a general-purpose output pin. A value of ΓÇÿ1ΓÇÖ written to this bit clears the corresponding SPISOMIDOUTx bit to ΓÇÿ0ΓÇÖ. Write: 0= Has no effect 1= Logic 0 placed on SPISOMIx pin if it is in General Purpose O/P mode Read: 0= Current value on SOMIDOUTx is 0. 1= Current value on SOMIDOUTx is 1."]
    #[inline(always)]
    #[must_use]
    pub fn somiclr(&mut self) -> SomiclrW<Spipc5Spec> {
        SomiclrW::new(self, 24)
    }
}
#[doc = "SPI / MibSPI Pin Control Register 5 (SPIPC5) - SPIDCLR\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipc5Spec;
impl crate::RegisterSpec for Spipc5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipc5::R`](R) reader structure"]
impl crate::Readable for Spipc5Spec {}
#[doc = "`write(|w| ..)` method takes [`spipc5::W`](W) writer structure"]
impl crate::Writable for Spipc5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIPC5 to value 0"]
impl crate::Resettable for Spipc5Spec {
    const RESET_VALUE: u32 = 0;
}
