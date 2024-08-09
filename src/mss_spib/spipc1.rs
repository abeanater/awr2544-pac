#[doc = "Register `SPIPC1` reader"]
pub type R = crate::R<Spipc1Spec>;
#[doc = "Register `SPIPC1` writer"]
pub type W = crate::W<Spipc1Spec>;
#[doc = "Field `SCSDIR` reader - 7:0\\]
SPISCS\\[7:0\\]
direction. Controls the direction of the SPISCSx pins when they are used as a general-purpose I/O pin. Each pins could be configured independently from the others If the SPISCSx is used as a SPI functional pin, the I/O direction is determined by the CLKMOD bit (SPIGCR1.1). 0=SPISCSx pin is an input 1=SPISCSx pin is an output Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSDIR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
pub type ScsdirR = crate::FieldReader;
#[doc = "Field `SCSDIR` writer - 7:0\\]
SPISCS\\[7:0\\]
direction. Controls the direction of the SPISCSx pins when they are used as a general-purpose I/O pin. Each pins could be configured independently from the others If the SPISCSx is used as a SPI functional pin, the I/O direction is determined by the CLKMOD bit (SPIGCR1.1). 0=SPISCSx pin is an input 1=SPISCSx pin is an output Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSDIR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
pub type ScsdirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENADIR` reader - 8:8\\]
SPIENA direction. Controls the direction of the SPIENA pin when it is used as a general-purpose I/O. If the SPIENA pin is used as a functional pin, then the I/O direction is determined by the CLKMOD bit (SPIGCR1.1). 0=SPIENA pin is an input 1=SPIENA pin is an output"]
pub type EnadirR = crate::BitReader;
#[doc = "Field `ENADIR` writer - 8:8\\]
SPIENA direction. Controls the direction of the SPIENA pin when it is used as a general-purpose I/O. If the SPIENA pin is used as a functional pin, then the I/O direction is determined by the CLKMOD bit (SPIGCR1.1). 0=SPIENA pin is an input 1=SPIENA pin is an output"]
pub type EnadirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIR` reader - 9:9\\]
SPICLK direction. Controls the direction of the SPICLK pin when it is used as a general-purpose I/O pin. In functional mode, the I/O direction is determined by the CLKMOD bit. 0=SPICLK pin is an input 1=SPICLK pin is an output"]
pub type ClkdirR = crate::BitReader;
#[doc = "Field `CLKDIR` writer - 9:9\\]
SPICLK direction. Controls the direction of the SPICLK pin when it is used as a general-purpose I/O pin. In functional mode, the I/O direction is determined by the CLKMOD bit. 0=SPICLK pin is an input 1=SPICLK pin is an output"]
pub type ClkdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMODIR0` reader - 10:10\\]
SPISIMO0 direction. Controls the direction of the SPISIMO0 pin when it is used as a general-purpose I/O pin. If the SPISIMO0 pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISIMO0 pin is an input 1=SPISIMO0 pin is an output Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simodir0R = crate::BitReader;
#[doc = "Field `SIMODIR0` writer - 10:10\\]
SPISIMO0 direction. Controls the direction of the SPISIMO0 pin when it is used as a general-purpose I/O pin. If the SPISIMO0 pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISIMO0 pin is an input 1=SPISIMO0 pin is an output Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simodir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOMIDIR0` reader - 11:11\\]
SPISOMI0 direction. Controls the direction of the SPISOMI0 pin when it is used as a general-purpose I/O pin. If the SPISOMI0 pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISOMI0 pin is an input 1=SPISOMI0 pin is an output Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somidir0R = crate::BitReader;
#[doc = "Field `SOMIDIR0` writer - 11:11\\]
SPISOMI0 direction. Controls the direction of the SPISOMI0 pin when it is used as a general-purpose I/O pin. If the SPISOMI0 pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISOMI0 pin is an input 1=SPISOMI0 pin is an output Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somidir0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIMODIR` reader - 23:16\\]
SPISIMOx direction. Controls the direction of the SPISIMOx pin when it is used as a general-purpose I/O pin. If the SPISIMOx pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISIMOx pin is an input 1=SPISIMOx pin is an output"]
pub type SimodirR = crate::FieldReader;
#[doc = "Field `SIMODIR` writer - 23:16\\]
SPISIMOx direction. Controls the direction of the SPISIMOx pin when it is used as a general-purpose I/O pin. If the SPISIMOx pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISIMOx pin is an input 1=SPISIMOx pin is an output"]
pub type SimodirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOMIDIR` reader - 31:24\\]
SPISOMIx direction. Controls the direction of the SPISOMIx pin when it is used as a general-purpose I/O pin. If the SPISOMIx pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISOMIx pin is an input 1=SPISOMIx pin is an output"]
pub type SomidirR = crate::FieldReader;
#[doc = "Field `SOMIDIR` writer - 31:24\\]
SPISOMIx direction. Controls the direction of the SPISOMIx pin when it is used as a general-purpose I/O pin. If the SPISOMIx pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISOMIx pin is an input 1=SPISOMIx pin is an output"]
pub type SomidirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCS\\[7:0\\]
direction. Controls the direction of the SPISCSx pins when they are used as a general-purpose I/O pin. Each pins could be configured independently from the others If the SPISCSx is used as a SPI functional pin, the I/O direction is determined by the CLKMOD bit (SPIGCR1.1). 0=SPISCSx pin is an input 1=SPISCSx pin is an output Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSDIR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    pub fn scsdir(&self) -> ScsdirR {
        ScsdirR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA direction. Controls the direction of the SPIENA pin when it is used as a general-purpose I/O. If the SPIENA pin is used as a functional pin, then the I/O direction is determined by the CLKMOD bit (SPIGCR1.1). 0=SPIENA pin is an input 1=SPIENA pin is an output"]
    #[inline(always)]
    pub fn enadir(&self) -> EnadirR {
        EnadirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
SPICLK direction. Controls the direction of the SPICLK pin when it is used as a general-purpose I/O pin. In functional mode, the I/O direction is determined by the CLKMOD bit. 0=SPICLK pin is an input 1=SPICLK pin is an output"]
    #[inline(always)]
    pub fn clkdir(&self) -> ClkdirR {
        ClkdirR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
SPISIMO0 direction. Controls the direction of the SPISIMO0 pin when it is used as a general-purpose I/O pin. If the SPISIMO0 pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISIMO0 pin is an input 1=SPISIMO0 pin is an output Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    pub fn simodir0(&self) -> Simodir0R {
        Simodir0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
SPISOMI0 direction. Controls the direction of the SPISOMI0 pin when it is used as a general-purpose I/O pin. If the SPISOMI0 pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISOMI0 pin is an input 1=SPISOMI0 pin is an output Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    pub fn somidir0(&self) -> Somidir0R {
        Somidir0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SPISIMOx direction. Controls the direction of the SPISIMOx pin when it is used as a general-purpose I/O pin. If the SPISIMOx pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISIMOx pin is an input 1=SPISIMOx pin is an output"]
    #[inline(always)]
    pub fn simodir(&self) -> SimodirR {
        SimodirR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SPISOMIx direction. Controls the direction of the SPISOMIx pin when it is used as a general-purpose I/O pin. If the SPISOMIx pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISOMIx pin is an input 1=SPISOMIx pin is an output"]
    #[inline(always)]
    pub fn somidir(&self) -> SomidirR {
        SomidirR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCS\\[7:0\\]
direction. Controls the direction of the SPISCSx pins when they are used as a general-purpose I/O pin. Each pins could be configured independently from the others If the SPISCSx is used as a SPI functional pin, the I/O direction is determined by the CLKMOD bit (SPIGCR1.1). 0=SPISCSx pin is an input 1=SPISCSx pin is an output Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSDIR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    #[must_use]
    pub fn scsdir(&mut self) -> ScsdirW<Spipc1Spec> {
        ScsdirW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA direction. Controls the direction of the SPIENA pin when it is used as a general-purpose I/O. If the SPIENA pin is used as a functional pin, then the I/O direction is determined by the CLKMOD bit (SPIGCR1.1). 0=SPIENA pin is an input 1=SPIENA pin is an output"]
    #[inline(always)]
    #[must_use]
    pub fn enadir(&mut self) -> EnadirW<Spipc1Spec> {
        EnadirW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
SPICLK direction. Controls the direction of the SPICLK pin when it is used as a general-purpose I/O pin. In functional mode, the I/O direction is determined by the CLKMOD bit. 0=SPICLK pin is an input 1=SPICLK pin is an output"]
    #[inline(always)]
    #[must_use]
    pub fn clkdir(&mut self) -> ClkdirW<Spipc1Spec> {
        ClkdirW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
SPISIMO0 direction. Controls the direction of the SPISIMO0 pin when it is used as a general-purpose I/O pin. If the SPISIMO0 pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISIMO0 pin is an input 1=SPISIMO0 pin is an output Note: Bit 10 or bit 16 can be used to set the direction for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn simodir0(&mut self) -> Simodir0W<Spipc1Spec> {
        Simodir0W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
SPISOMI0 direction. Controls the direction of the SPISOMI0 pin when it is used as a general-purpose I/O pin. If the SPISOMI0 pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISOMI0 pin is an input 1=SPISOMI0 pin is an output Note: Bit 11 or bit 24 can be used to set the direction for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    #[must_use]
    pub fn somidir0(&mut self) -> Somidir0W<Spipc1Spec> {
        Somidir0W::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Spipc1Spec> {
        NuW::new(self, 12)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SPISIMOx direction. Controls the direction of the SPISIMOx pin when it is used as a general-purpose I/O pin. If the SPISIMOx pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISIMOx pin is an input 1=SPISIMOx pin is an output"]
    #[inline(always)]
    #[must_use]
    pub fn simodir(&mut self) -> SimodirW<Spipc1Spec> {
        SimodirW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SPISOMIx direction. Controls the direction of the SPISOMIx pin when it is used as a general-purpose I/O pin. If the SPISOMIx pin is used as a SPI / MibSPI functional pin, the I/O direction is determined by the MASTER bit. 0=SPISOMIx pin is an input 1=SPISOMIx pin is an output"]
    #[inline(always)]
    #[must_use]
    pub fn somidir(&mut self) -> SomidirW<Spipc1Spec> {
        SomidirW::new(self, 24)
    }
}
#[doc = "SPI / MibSPI Pin Control Register 1 (SPIPC1) - SPIDIR\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipc1Spec;
impl crate::RegisterSpec for Spipc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipc1::R`](R) reader structure"]
impl crate::Readable for Spipc1Spec {}
#[doc = "`write(|w| ..)` method takes [`spipc1::W`](W) writer structure"]
impl crate::Writable for Spipc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIPC1 to value 0"]
impl crate::Resettable for Spipc1Spec {
    const RESET_VALUE: u32 = 0;
}
