#[doc = "Register `SPIPC0` reader"]
pub type R = crate::R<Spipc0Spec>;
#[doc = "Register `SPIPC0` writer"]
pub type W = crate::W<Spipc0Spec>;
#[doc = "Field `SCSFUN` reader - 7:0\\]
SPISCS\\[7:0\\]
function. Determines whether the SPISCSx pins are to be used as a general-purpose I/O pins or as SPI functional pins. If the slave SPISCSx pins are in functional mode and receive an inactive high signal, the slave SPI will place it is output in high-z and disable shifting. 0=SPISCSx pin is a GPIO 1=SPISCSx pin is a SPI functional pin Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSFUN\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
pub type ScsfunR = crate::FieldReader;
#[doc = "Field `SCSFUN` writer - 7:0\\]
SPISCS\\[7:0\\]
function. Determines whether the SPISCSx pins are to be used as a general-purpose I/O pins or as SPI functional pins. If the slave SPISCSx pins are in functional mode and receive an inactive high signal, the slave SPI will place it is output in high-z and disable shifting. 0=SPISCSx pin is a GPIO 1=SPISCSx pin is a SPI functional pin Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSFUN\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
pub type ScsfunW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENAFUN` reader - 8:8\\]
SPIENA function. Determines whether the SPIENA pin is to be used as a general-purpose I/O pin, or as a SPI / MibSPI functional pin. 0=SPIENA pin is a GPIO 1=SPIENA pin is a SPI / MibSPI functional pin"]
pub type EnafunR = crate::BitReader;
#[doc = "Field `ENAFUN` writer - 8:8\\]
SPIENA function. Determines whether the SPIENA pin is to be used as a general-purpose I/O pin, or as a SPI / MibSPI functional pin. 0=SPIENA pin is a GPIO 1=SPIENA pin is a SPI / MibSPI functional pin"]
pub type EnafunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKFUN` reader - 9:9\\]
SPI / MibSPI clock function. Determines whether the SPICLK pin is to be used as a general-purpose I/O pin, or as a SPI / MibSPI functional pin. 0=SPICLK pin is a GPIO 1=SPICLK pin is a SPI / MibSPI functional pin"]
pub type ClkfunR = crate::BitReader;
#[doc = "Field `CLKFUN` writer - 9:9\\]
SPI / MibSPI clock function. Determines whether the SPICLK pin is to be used as a general-purpose I/O pin, or as a SPI / MibSPI functional pin. 0=SPICLK pin is a GPIO 1=SPICLK pin is a SPI / MibSPI functional pin"]
pub type ClkfunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMOFUN0` reader - 10:10\\]
Slave in, master out function. Determines whether the SPISIMO0 pin is to be used as a general-purpose I/O pin, or as a SPI / MibSPI functional pin. 0=SPISIMO0 pin is a GPIO 1=SPISIMO0 pin is a SPI / MibSPI functional pin Note: Bit 10 or bit 16 can be used to set the function mode for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simofun0R = crate::BitReader;
#[doc = "Field `SIMOFUN0` writer - 10:10\\]
Slave in, master out function. Determines whether the SPISIMO0 pin is to be used as a general-purpose I/O pin, or as a SPI / MibSPI functional pin. 0=SPISIMO0 pin is a GPIO 1=SPISIMO0 pin is a SPI / MibSPI functional pin Note: Bit 10 or bit 16 can be used to set the function mode for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simofun0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOMIFUN0` reader - 11:11\\]
Slave out, master in function. Determines whether the SPISOMI0 pin is to be used as a general-purpose I/O pin or as a SPI / MibSPI functional pin. 0=SPISOMI0 pin is a GPIO 1=SPISOMI0 pin is a SPI / MibSPI functional pin Note: Bit 11 or bit 24 can be used to set the function mode for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somifun0R = crate::BitReader;
#[doc = "Field `SOMIFUN0` writer - 11:11\\]
Slave out, master in function. Determines whether the SPISOMI0 pin is to be used as a general-purpose I/O pin or as a SPI / MibSPI functional pin. 0=SPISOMI0 pin is a GPIO 1=SPISOMI0 pin is a SPI / MibSPI functional pin Note: Bit 11 or bit 24 can be used to set the function mode for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somifun0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIMOFUN` reader - 23:16\\]
Slave in, master out function. Determines whether the SPISIMOx pin is to be used as a general-purpose I/O pin or as a SPI / MibSPI functional pin. 0=SPISIMOx pin is a GPIO 1=SPISIMOx pin is a SPI / MibSPI functional pin Note: Generic based bit implementation Register bits 31 to 24 and 23 to 16 of SPIPC0 to SPIPC9 are implemented depending upon the generic parameter NUM_PARLL_PINS which determines the number of SIMO/SOMI data lines to be supported. Only if 8 dataline support is selected at the time of logic synthesis, bits 31 to 16 are implemented. Un-implemented bits return ΓÇÿ0ΓÇÖ upon read and are not writable."]
pub type SimofunR = crate::FieldReader;
#[doc = "Field `SIMOFUN` writer - 23:16\\]
Slave in, master out function. Determines whether the SPISIMOx pin is to be used as a general-purpose I/O pin or as a SPI / MibSPI functional pin. 0=SPISIMOx pin is a GPIO 1=SPISIMOx pin is a SPI / MibSPI functional pin Note: Generic based bit implementation Register bits 31 to 24 and 23 to 16 of SPIPC0 to SPIPC9 are implemented depending upon the generic parameter NUM_PARLL_PINS which determines the number of SIMO/SOMI data lines to be supported. Only if 8 dataline support is selected at the time of logic synthesis, bits 31 to 16 are implemented. Un-implemented bits return ΓÇÿ0ΓÇÖ upon read and are not writable."]
pub type SimofunW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOMIFUN` reader - 31:24\\]
Slave out, master in function. Determines whether the SPISOMIx pins are to be used as a general-purpose I/O pin or as a SPI / MibSPI functional pin. 0=SPISOMIx pin is a GPIO 1=SPISOMIx pin is a SPI / MibSPI functional pin"]
pub type SomifunR = crate::FieldReader;
#[doc = "Field `SOMIFUN` writer - 31:24\\]
Slave out, master in function. Determines whether the SPISOMIx pins are to be used as a general-purpose I/O pin or as a SPI / MibSPI functional pin. 0=SPISOMIx pin is a GPIO 1=SPISOMIx pin is a SPI / MibSPI functional pin"]
pub type SomifunW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCS\\[7:0\\]
function. Determines whether the SPISCSx pins are to be used as a general-purpose I/O pins or as SPI functional pins. If the slave SPISCSx pins are in functional mode and receive an inactive high signal, the slave SPI will place it is output in high-z and disable shifting. 0=SPISCSx pin is a GPIO 1=SPISCSx pin is a SPI functional pin Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSFUN\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    pub fn scsfun(&self) -> ScsfunR {
        ScsfunR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA function. Determines whether the SPIENA pin is to be used as a general-purpose I/O pin, or as a SPI / MibSPI functional pin. 0=SPIENA pin is a GPIO 1=SPIENA pin is a SPI / MibSPI functional pin"]
    #[inline(always)]
    pub fn enafun(&self) -> EnafunR {
        EnafunR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
SPI / MibSPI clock function. Determines whether the SPICLK pin is to be used as a general-purpose I/O pin, or as a SPI / MibSPI functional pin. 0=SPICLK pin is a GPIO 1=SPICLK pin is a SPI / MibSPI functional pin"]
    #[inline(always)]
    pub fn clkfun(&self) -> ClkfunR {
        ClkfunR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Slave in, master out function. Determines whether the SPISIMO0 pin is to be used as a general-purpose I/O pin, or as a SPI / MibSPI functional pin. 0=SPISIMO0 pin is a GPIO 1=SPISIMO0 pin is a SPI / MibSPI functional pin Note: Bit 10 or bit 16 can be used to set the function mode for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    pub fn simofun0(&self) -> Simofun0R {
        Simofun0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Slave out, master in function. Determines whether the SPISOMI0 pin is to be used as a general-purpose I/O pin or as a SPI / MibSPI functional pin. 0=SPISOMI0 pin is a GPIO 1=SPISOMI0 pin is a SPI / MibSPI functional pin Note: Bit 11 or bit 24 can be used to set the function mode for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    pub fn somifun0(&self) -> Somifun0R {
        Somifun0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Slave in, master out function. Determines whether the SPISIMOx pin is to be used as a general-purpose I/O pin or as a SPI / MibSPI functional pin. 0=SPISIMOx pin is a GPIO 1=SPISIMOx pin is a SPI / MibSPI functional pin Note: Generic based bit implementation Register bits 31 to 24 and 23 to 16 of SPIPC0 to SPIPC9 are implemented depending upon the generic parameter NUM_PARLL_PINS which determines the number of SIMO/SOMI data lines to be supported. Only if 8 dataline support is selected at the time of logic synthesis, bits 31 to 16 are implemented. Un-implemented bits return ΓÇÿ0ΓÇÖ upon read and are not writable."]
    #[inline(always)]
    pub fn simofun(&self) -> SimofunR {
        SimofunR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Slave out, master in function. Determines whether the SPISOMIx pins are to be used as a general-purpose I/O pin or as a SPI / MibSPI functional pin. 0=SPISOMIx pin is a GPIO 1=SPISOMIx pin is a SPI / MibSPI functional pin"]
    #[inline(always)]
    pub fn somifun(&self) -> SomifunR {
        SomifunR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
SPISCS\\[7:0\\]
function. Determines whether the SPISCSx pins are to be used as a general-purpose I/O pins or as SPI functional pins. If the slave SPISCSx pins are in functional mode and receive an inactive high signal, the slave SPI will place it is output in high-z and disable shifting. 0=SPISCSx pin is a GPIO 1=SPISCSx pin is a SPI functional pin Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSFUN\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    #[must_use]
    pub fn scsfun(&mut self) -> ScsfunW<Spipc0Spec> {
        ScsfunW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SPIENA function. Determines whether the SPIENA pin is to be used as a general-purpose I/O pin, or as a SPI / MibSPI functional pin. 0=SPIENA pin is a GPIO 1=SPIENA pin is a SPI / MibSPI functional pin"]
    #[inline(always)]
    #[must_use]
    pub fn enafun(&mut self) -> EnafunW<Spipc0Spec> {
        EnafunW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
SPI / MibSPI clock function. Determines whether the SPICLK pin is to be used as a general-purpose I/O pin, or as a SPI / MibSPI functional pin. 0=SPICLK pin is a GPIO 1=SPICLK pin is a SPI / MibSPI functional pin"]
    #[inline(always)]
    #[must_use]
    pub fn clkfun(&mut self) -> ClkfunW<Spipc0Spec> {
        ClkfunW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Slave in, master out function. Determines whether the SPISIMO0 pin is to be used as a general-purpose I/O pin, or as a SPI / MibSPI functional pin. 0=SPISIMO0 pin is a GPIO 1=SPISIMO0 pin is a SPI / MibSPI functional pin Note: Bit 10 or bit 16 can be used to set the function mode for pin SIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn simofun0(&mut self) -> Simofun0W<Spipc0Spec> {
        Simofun0W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Slave out, master in function. Determines whether the SPISOMI0 pin is to be used as a general-purpose I/O pin or as a SPI / MibSPI functional pin. 0=SPISOMI0 pin is a GPIO 1=SPISOMI0 pin is a SPI / MibSPI functional pin Note: Bit 11 or bit 24 can be used to set the function mode for pin SOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    #[must_use]
    pub fn somifun0(&mut self) -> Somifun0W<Spipc0Spec> {
        Somifun0W::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Spipc0Spec> {
        NuW::new(self, 12)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Slave in, master out function. Determines whether the SPISIMOx pin is to be used as a general-purpose I/O pin or as a SPI / MibSPI functional pin. 0=SPISIMOx pin is a GPIO 1=SPISIMOx pin is a SPI / MibSPI functional pin Note: Generic based bit implementation Register bits 31 to 24 and 23 to 16 of SPIPC0 to SPIPC9 are implemented depending upon the generic parameter NUM_PARLL_PINS which determines the number of SIMO/SOMI data lines to be supported. Only if 8 dataline support is selected at the time of logic synthesis, bits 31 to 16 are implemented. Un-implemented bits return ΓÇÿ0ΓÇÖ upon read and are not writable."]
    #[inline(always)]
    #[must_use]
    pub fn simofun(&mut self) -> SimofunW<Spipc0Spec> {
        SimofunW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Slave out, master in function. Determines whether the SPISOMIx pins are to be used as a general-purpose I/O pin or as a SPI / MibSPI functional pin. 0=SPISOMIx pin is a GPIO 1=SPISOMIx pin is a SPI / MibSPI functional pin"]
    #[inline(always)]
    #[must_use]
    pub fn somifun(&mut self) -> SomifunW<Spipc0Spec> {
        SomifunW::new(self, 24)
    }
}
#[doc = "SPI / MibSPI Pin Control Register 0 (SPIPC0) - SPIFUN Note: Duplicate Control Bits for SIMO0 &amp; SOMI0 Bit 24 is not physically implemented. it is a mirror of Bit11. Any write to Bit 24 will be reflected on Bit11 and when Bit 24 &amp; Bit 11 simultaneously written, the value of Bit11 will control the SOMI pin. Read value of Bit 24 always reflects the Bit 11 value. This is true for the Bit 24 &amp; Bit 11 of all of SPIPC0 to SPIPC9 registers. Same is true for SIMO pin with Bit16 &amp; Bit 10 of SPIPC0 to SPIPC9 registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipc0Spec;
impl crate::RegisterSpec for Spipc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipc0::R`](R) reader structure"]
impl crate::Readable for Spipc0Spec {}
#[doc = "`write(|w| ..)` method takes [`spipc0::W`](W) writer structure"]
impl crate::Writable for Spipc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIPC0 to value 0"]
impl crate::Resettable for Spipc0Spec {
    const RESET_VALUE: u32 = 0;
}
