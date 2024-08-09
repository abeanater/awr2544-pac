#[doc = "Register `GIOPULDISF` reader"]
pub type R = crate::R<GiopuldisfSpec>;
#[doc = "Register `GIOPULDISF` writer"]
pub type W = crate::W<GiopuldisfSpec>;
#[doc = "Field `GIOPULDISF` reader - 7:0\\]
GIO pull disable for port F"]
pub type GiopuldisfR = crate::FieldReader;
#[doc = "Field `GIOPULDISF` writer - 7:0\\]
GIO pull disable for port F"]
pub type GiopuldisfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU40` reader - 31:8\\]
Reserved"]
pub type Nu40R = crate::FieldReader<u32>;
#[doc = "Field `NU40` writer - 31:8\\]
Reserved"]
pub type Nu40W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull disable for port F"]
    #[inline(always)]
    pub fn giopuldisf(&self) -> GiopuldisfR {
        GiopuldisfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu40(&self) -> Nu40R {
        Nu40R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull disable for port F"]
    #[inline(always)]
    #[must_use]
    pub fn giopuldisf(&mut self) -> GiopuldisfW<GiopuldisfSpec> {
        GiopuldisfW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu40(&mut self) -> Nu40W<GiopuldisfSpec> {
        Nu40W::new(self, 8)
    }
}
#[doc = "GIO pul disable for port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giopuldisf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopuldisf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopuldisfSpec;
impl crate::RegisterSpec for GiopuldisfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopuldisf::R`](R) reader structure"]
impl crate::Readable for GiopuldisfSpec {}
#[doc = "`write(|w| ..)` method takes [`giopuldisf::W`](W) writer structure"]
impl crate::Writable for GiopuldisfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPULDISF to value 0"]
impl crate::Resettable for GiopuldisfSpec {
    const RESET_VALUE: u32 = 0;
}
