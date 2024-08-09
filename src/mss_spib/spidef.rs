#[doc = "Register `SPIDEF` reader"]
pub type R = crate::R<SpidefSpec>;
#[doc = "Register `SPIDEF` writer"]
pub type W = crate::W<SpidefSpec>;
#[doc = "Field `CSDEF0` reader - 7:0\\]
Chip select default pattern. Master mode behavior. The CSDEFx bits are output to the chip select pins when no transmission are currently performed. It allows the user to set a chip select pattern which deselects all the SPI slaves. 1 =If CSDEFx is set to ΓÇ£1ΓÇ¥ the corresponding chip select is set to ΓÇ£1ΓÇ¥ while SPI/MibSPI is IDLE. 0 =If CSDEFx is set to ΓÇ£0ΓÇ¥ the corresponding chip select is set to ΓÇ£0ΓÇ¥ while SPI/MibSPI is IDLE. Note: Effect of NUM_CS_PINS generic on CSDEF bits. Actual number of bits implemented in CSDEF\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be treated as reserved, read-only and will read ΓÇÿ0ΓÇÖ always."]
pub type Csdef0R = crate::FieldReader;
#[doc = "Field `CSDEF0` writer - 7:0\\]
Chip select default pattern. Master mode behavior. The CSDEFx bits are output to the chip select pins when no transmission are currently performed. It allows the user to set a chip select pattern which deselects all the SPI slaves. 1 =If CSDEFx is set to ΓÇ£1ΓÇ¥ the corresponding chip select is set to ΓÇ£1ΓÇ¥ while SPI/MibSPI is IDLE. 0 =If CSDEFx is set to ΓÇ£0ΓÇ¥ the corresponding chip select is set to ΓÇ£0ΓÇ¥ while SPI/MibSPI is IDLE. Note: Effect of NUM_CS_PINS generic on CSDEF bits. Actual number of bits implemented in CSDEF\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be treated as reserved, read-only and will read ΓÇÿ0ΓÇÖ always."]
pub type Csdef0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU` reader - 31:8\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:8\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Chip select default pattern. Master mode behavior. The CSDEFx bits are output to the chip select pins when no transmission are currently performed. It allows the user to set a chip select pattern which deselects all the SPI slaves. 1 =If CSDEFx is set to ΓÇ£1ΓÇ¥ the corresponding chip select is set to ΓÇ£1ΓÇ¥ while SPI/MibSPI is IDLE. 0 =If CSDEFx is set to ΓÇ£0ΓÇ¥ the corresponding chip select is set to ΓÇ£0ΓÇ¥ while SPI/MibSPI is IDLE. Note: Effect of NUM_CS_PINS generic on CSDEF bits. Actual number of bits implemented in CSDEF\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be treated as reserved, read-only and will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    pub fn csdef0(&self) -> Csdef0R {
        Csdef0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Chip select default pattern. Master mode behavior. The CSDEFx bits are output to the chip select pins when no transmission are currently performed. It allows the user to set a chip select pattern which deselects all the SPI slaves. 1 =If CSDEFx is set to ΓÇ£1ΓÇ¥ the corresponding chip select is set to ΓÇ£1ΓÇ¥ while SPI/MibSPI is IDLE. 0 =If CSDEFx is set to ΓÇ£0ΓÇ¥ the corresponding chip select is set to ΓÇ£0ΓÇ¥ while SPI/MibSPI is IDLE. Note: Effect of NUM_CS_PINS generic on CSDEF bits. Actual number of bits implemented in CSDEF\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be treated as reserved, read-only and will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    #[must_use]
    pub fn csdef0(&mut self) -> Csdef0W<SpidefSpec> {
        Csdef0W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<SpidefSpec> {
        NuW::new(self, 8)
    }
}
#[doc = "SPI / MibSPI Default Chip select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spidef::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spidef::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpidefSpec;
impl crate::RegisterSpec for SpidefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spidef::R`](R) reader structure"]
impl crate::Readable for SpidefSpec {}
#[doc = "`write(|w| ..)` method takes [`spidef::W`](W) writer structure"]
impl crate::Writable for SpidefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIDEF to value 0"]
impl crate::Resettable for SpidefSpec {
    const RESET_VALUE: u32 = 0;
}
