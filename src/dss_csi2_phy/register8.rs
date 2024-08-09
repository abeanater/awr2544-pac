#[doc = "Register `REGISTER8` reader"]
pub type R = crate::R<Register8Spec>;
#[doc = "Register `REGISTER8` writer"]
pub type W = crate::W<Register8Spec>;
#[doc = "Field `BIASGENCODE` reader - 4:0\\]
11111: Min current 10000: Nominal current 00000: Max current"]
pub type BiasgencodeR = crate::FieldReader;
#[doc = "Field `BIASGENCODE` writer - 4:0\\]
11111: Min current 10000: Nominal current 00000: Max current"]
pub type BiasgencodeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDEFUSEBIASGEN` reader - 5:5\\]
1: Override EFUSE bits 0: Use EFUSE bits"]
pub type OvrrdefusebiasgenR = crate::BitReader;
#[doc = "Field `OVRRDEFUSEBIASGEN` writer - 5:5\\]
1: Override EFUSE bits 0: Use EFUSE bits"]
pub type OvrrdefusebiasgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHSTXTERMRES` reader - 10:6\\]
11111: Max resistance 10000: Nominal resistance 00000: Min resistance"]
pub type ReghstxtermresR = crate::FieldReader;
#[doc = "Field `REGHSTXTERMRES` writer - 10:6\\]
11111: Max resistance 10000: Nominal resistance 00000: Min resistance"]
pub type ReghstxtermresW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDHSTXTERMRES` reader - 11:11\\]
1: Override EFUSE bits 0: Use EFUSE bits"]
pub type OvrrdhstxtermresR = crate::BitReader;
#[doc = "Field `OVRRDHSTXTERMRES` writer - 11:11\\]
1: Override EFUSE bits 0: Use EFUSE bits"]
pub type OvrrdhstxtermresW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
11111: Min current 10000: Nominal current 00000: Max current"]
    #[inline(always)]
    pub fn biasgencode(&self) -> BiasgencodeR {
        BiasgencodeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
1: Override EFUSE bits 0: Use EFUSE bits"]
    #[inline(always)]
    pub fn ovrrdefusebiasgen(&self) -> OvrrdefusebiasgenR {
        OvrrdefusebiasgenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - 10:6\\]
11111: Max resistance 10000: Nominal resistance 00000: Min resistance"]
    #[inline(always)]
    pub fn reghstxtermres(&self) -> ReghstxtermresR {
        ReghstxtermresR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
1: Override EFUSE bits 0: Use EFUSE bits"]
    #[inline(always)]
    pub fn ovrrdhstxtermres(&self) -> OvrrdhstxtermresR {
        OvrrdhstxtermresR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
11111: Min current 10000: Nominal current 00000: Max current"]
    #[inline(always)]
    #[must_use]
    pub fn biasgencode(&mut self) -> BiasgencodeW<Register8Spec> {
        BiasgencodeW::new(self, 0)
    }
    #[doc = "Bit 5 - 5:5\\]
1: Override EFUSE bits 0: Use EFUSE bits"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdefusebiasgen(&mut self) -> OvrrdefusebiasgenW<Register8Spec> {
        OvrrdefusebiasgenW::new(self, 5)
    }
    #[doc = "Bits 6:10 - 10:6\\]
11111: Max resistance 10000: Nominal resistance 00000: Min resistance"]
    #[inline(always)]
    #[must_use]
    pub fn reghstxtermres(&mut self) -> ReghstxtermresW<Register8Spec> {
        ReghstxtermresW::new(self, 6)
    }
    #[doc = "Bit 11 - 11:11\\]
1: Override EFUSE bits 0: Use EFUSE bits"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdhstxtermres(&mut self) -> OvrrdhstxtermresW<Register8Spec> {
        OvrrdhstxtermresW::new(self, 11)
    }
}
#[doc = "REGISTER8\n\nYou can [`read`](crate::Reg::read) this register and get [`register8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register8Spec;
impl crate::RegisterSpec for Register8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register8::R`](R) reader structure"]
impl crate::Readable for Register8Spec {}
#[doc = "`write(|w| ..)` method takes [`register8::W`](W) writer structure"]
impl crate::Writable for Register8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER8 to value 0"]
impl crate::Resettable for Register8Spec {
    const RESET_VALUE: u32 = 0;
}
