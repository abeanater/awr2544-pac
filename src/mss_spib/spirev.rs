#[doc = "Register `SPIREV` reader"]
pub type R = crate::R<SpirevSpec>;
#[doc = "Register `SPIREV` writer"]
pub type W = crate::W<SpirevSpec>;
#[doc = "Field `MINOR` reader - 5:0\\]
Minor Revision number Reads 0x8"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - 5:0\\]
Minor Revision number Reads 0x8"]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Indicates device specific implementation Reads 0x0"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Indicates device specific implementation Reads 0x0"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR` reader - 10:8\\]
Major Revision number Reads 0x3"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - 10:8\\]
Major Revision number Reads 0x3"]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL` reader - 15:11\\]
RTL version number Read value will provide an approximate RTL revision number. The design release version can be obtained from the device specification"]
pub type RtlR = crate::FieldReader;
#[doc = "Field `RTL` writer - 15:11\\]
RTL version number Read value will provide an approximate RTL revision number. The design release version can be obtained from the device specification"]
pub type RtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNC` reader - 27:16\\]
Indicates functionally equivalent module family Reads 0xA05"]
pub type FuncR = crate::FieldReader<u16>;
#[doc = "Field `FUNC` writer - 27:16\\]
Indicates functionally equivalent module family Reads 0xA05"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `NU` reader - 29:28\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - 29:28\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCHEME` reader - 31:30\\]
Identification Scheme Used to distinguish different ID schemes. Reads 0x01"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
Identification Scheme Used to distinguish different ID schemes. Reads 0x01"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor Revision number Reads 0x8"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates device specific implementation Reads 0x0"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision number Reads 0x3"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version number Read value will provide an approximate RTL revision number. The design release version can be obtained from the device specification"]
    #[inline(always)]
    pub fn rtl(&self) -> RtlR {
        RtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Indicates functionally equivalent module family Reads 0xA05"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Identification Scheme Used to distinguish different ID schemes. Reads 0x01"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor Revision number Reads 0x8"]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MinorW<SpirevSpec> {
        MinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates device specific implementation Reads 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<SpirevSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision number Reads 0x3"]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<SpirevSpec> {
        MajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version number Read value will provide an approximate RTL revision number. The design release version can be obtained from the device specification"]
    #[inline(always)]
    #[must_use]
    pub fn rtl(&mut self) -> RtlW<SpirevSpec> {
        RtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Indicates functionally equivalent module family Reads 0xA05"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<SpirevSpec> {
        FuncW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<SpirevSpec> {
        NuW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Identification Scheme Used to distinguish different ID schemes. Reads 0x01"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<SpirevSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "SPI / MibSPI Revision ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spirev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spirev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpirevSpec;
impl crate::RegisterSpec for SpirevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spirev::R`](R) reader structure"]
impl crate::Readable for SpirevSpec {}
#[doc = "`write(|w| ..)` method takes [`spirev::W`](W) writer structure"]
impl crate::Writable for SpirevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIREV to value 0"]
impl crate::Resettable for SpirevSpec {
    const RESET_VALUE: u32 = 0;
}
