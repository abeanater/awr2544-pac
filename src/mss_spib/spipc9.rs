#[doc = "Register `SPIPC9` reader"]
pub type R = crate::R<Spipc9Spec>;
#[doc = "Register `SPIPC9` writer"]
pub type W = crate::W<Spipc9Spec>;
#[doc = "Field `SCSSRS` reader - 7:0\\]
Each of these 7 bits controls the slew rate for the corresponding SPISCSx pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select. Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSSRS\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
pub type ScssrsR = crate::FieldReader;
#[doc = "Field `SCSSRS` writer - 7:0\\]
Each of these 7 bits controls the slew rate for the corresponding SPISCSx pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select. Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSSRS\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
pub type ScssrsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENASRS` reader - 8:8\\]
This bit controls the slew rate for SPIENA pin. 0 =Fast Buffer Select. 1 =Slow Buffer Select."]
pub type EnasrsR = crate::BitReader;
#[doc = "Field `ENASRS` writer - 8:8\\]
This bit controls the slew rate for SPIENA pin. 0 =Fast Buffer Select. 1 =Slow Buffer Select."]
pub type EnasrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSRS` reader - 9:9\\]
This bit controls the slew rate for SPICLK pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select."]
pub type ClksrsR = crate::BitReader;
#[doc = "Field `CLKSRS` writer - 9:9\\]
This bit controls the slew rate for SPICLK pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select."]
pub type ClksrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMOSRS0` reader - 10:10\\]
This bit controls the slew rate for SPISIMO0 pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select. Note: Bit 10 or bit 16 can be used to control the slew rate for SPISIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simosrs0R = crate::BitReader;
#[doc = "Field `SIMOSRS0` writer - 10:10\\]
This bit controls the slew rate for SPISIMO0 pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select. Note: Bit 10 or bit 16 can be used to control the slew rate for SPISIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
pub type Simosrs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOMISRS0` reader - 11:11\\]
This bit controls the slew rate for SPISOMI0 pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select. Note: Bit 11 or bit 24 can be used to control the slew rate for SPISOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somisrs0R = crate::BitReader;
#[doc = "Field `SOMISRS0` writer - 11:11\\]
This bit controls the slew rate for SPISOMI0 pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select. Note: Bit 11 or bit 24 can be used to control the slew rate for SPISOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
pub type Somisrs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIMOSRS7` reader - 23:16\\]
Each of these 7 bits controls the slew rate for the corresponding SPISIMOx pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select."]
pub type Simosrs7R = crate::FieldReader;
#[doc = "Field `SIMOSRS7` writer - 23:16\\]
Each of these 7 bits controls the slew rate for the corresponding SPISIMOx pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select."]
pub type Simosrs7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOMISRS7` reader - 31:24\\]
Each of these 7 bits controls the slew rate for the corresponding SPISOMIx pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select."]
pub type Somisrs7R = crate::FieldReader;
#[doc = "Field `SOMISRS7` writer - 31:24\\]
Each of these 7 bits controls the slew rate for the corresponding SPISOMIx pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select."]
pub type Somisrs7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Each of these 7 bits controls the slew rate for the corresponding SPISCSx pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select. Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSSRS\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    pub fn scssrs(&self) -> ScssrsR {
        ScssrsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
This bit controls the slew rate for SPIENA pin. 0 =Fast Buffer Select. 1 =Slow Buffer Select."]
    #[inline(always)]
    pub fn enasrs(&self) -> EnasrsR {
        EnasrsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
This bit controls the slew rate for SPICLK pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select."]
    #[inline(always)]
    pub fn clksrs(&self) -> ClksrsR {
        ClksrsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
This bit controls the slew rate for SPISIMO0 pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select. Note: Bit 10 or bit 16 can be used to control the slew rate for SPISIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    pub fn simosrs0(&self) -> Simosrs0R {
        Simosrs0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
This bit controls the slew rate for SPISOMI0 pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select. Note: Bit 11 or bit 24 can be used to control the slew rate for SPISOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    pub fn somisrs0(&self) -> Somisrs0R {
        Somisrs0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Each of these 7 bits controls the slew rate for the corresponding SPISIMOx pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select."]
    #[inline(always)]
    pub fn simosrs7(&self) -> Simosrs7R {
        Simosrs7R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Each of these 7 bits controls the slew rate for the corresponding SPISOMIx pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select."]
    #[inline(always)]
    pub fn somisrs7(&self) -> Somisrs7R {
        Somisrs7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Each of these 7 bits controls the slew rate for the corresponding SPISCSx pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select. Note: Effect of NUM_CS_PINS generic on ChipSelect bits. Actual number of bits implemented in SCSSRS\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always."]
    #[inline(always)]
    #[must_use]
    pub fn scssrs(&mut self) -> ScssrsW<Spipc9Spec> {
        ScssrsW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
This bit controls the slew rate for SPIENA pin. 0 =Fast Buffer Select. 1 =Slow Buffer Select."]
    #[inline(always)]
    #[must_use]
    pub fn enasrs(&mut self) -> EnasrsW<Spipc9Spec> {
        EnasrsW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
This bit controls the slew rate for SPICLK pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select."]
    #[inline(always)]
    #[must_use]
    pub fn clksrs(&mut self) -> ClksrsW<Spipc9Spec> {
        ClksrsW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
This bit controls the slew rate for SPISIMO0 pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select. Note: Bit 10 or bit 16 can be used to control the slew rate for SPISIMO0. If a 32 bit write is performed, bit 10 will have priority over bit 16."]
    #[inline(always)]
    #[must_use]
    pub fn simosrs0(&mut self) -> Simosrs0W<Spipc9Spec> {
        Simosrs0W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
This bit controls the slew rate for SPISOMI0 pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select. Note: Bit 11 or bit 24 can be used to control the slew rate for SPISOMI0. If a 32 bit write is performed, bit 11 will have priority over bit 24."]
    #[inline(always)]
    #[must_use]
    pub fn somisrs0(&mut self) -> Somisrs0W<Spipc9Spec> {
        Somisrs0W::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Spipc9Spec> {
        NuW::new(self, 12)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Each of these 7 bits controls the slew rate for the corresponding SPISIMOx pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select."]
    #[inline(always)]
    #[must_use]
    pub fn simosrs7(&mut self) -> Simosrs7W<Spipc9Spec> {
        Simosrs7W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Each of these 7 bits controls the slew rate for the corresponding SPISOMIx pin. 0 =Normal Buffer Select. 1 =Slow Buffer Select."]
    #[inline(always)]
    #[must_use]
    pub fn somisrs7(&mut self) -> Somisrs7W<Spipc9Spec> {
        Somisrs7W::new(self, 24)
    }
}
#[doc = "SPI/MibSPI Pin Control Register 9 (SPIPC9) - SPISRSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`spipc9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipc9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipc9Spec;
impl crate::RegisterSpec for Spipc9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipc9::R`](R) reader structure"]
impl crate::Readable for Spipc9Spec {}
#[doc = "`write(|w| ..)` method takes [`spipc9::W`](W) writer structure"]
impl crate::Writable for Spipc9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIPC9 to value 0"]
impl crate::Resettable for Spipc9Spec {
    const RESET_VALUE: u32 = 0;
}
