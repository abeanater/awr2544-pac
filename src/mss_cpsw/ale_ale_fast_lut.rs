#[doc = "Register `ALE_ALE_FAST_LUT` reader"]
pub type R = crate::R<AleAleFastLutSpec>;
#[doc = "Register `ALE_ALE_FAST_LUT` writer"]
pub type W = crate::W<AleAleFastLutSpec>;
#[doc = "Field `THE_ FAST_LUT_FIELD` reader - 1:0\\]
The ~Fast_LUT field alows any port to be Fast_LUT mode, which will cause all lookup operations to start based on DA/SA and VLAN only. That is any data beyong the first 32 are not used in the lookup process."]
pub type TheFastLutFieldR = crate::FieldReader;
#[doc = "Field `THE_ FAST_LUT_FIELD` writer - 1:0\\]
The ~Fast_LUT field alows any port to be Fast_LUT mode, which will cause all lookup operations to start based on DA/SA and VLAN only. That is any data beyong the first 32 are not used in the lookup process."]
pub type TheFastLutFieldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
The ~Fast_LUT field alows any port to be Fast_LUT mode, which will cause all lookup operations to start based on DA/SA and VLAN only. That is any data beyong the first 32 are not used in the lookup process."]
    #[inline(always)]
    pub fn the_fast_lut_field(&self) -> TheFastLutFieldR {
        TheFastLutFieldR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
The ~Fast_LUT field alows any port to be Fast_LUT mode, which will cause all lookup operations to start based on DA/SA and VLAN only. That is any data beyong the first 32 are not used in the lookup process."]
    #[inline(always)]
    #[must_use]
    pub fn the_fast_lut_field(&mut self) -> TheFastLutFieldW<AleAleFastLutSpec> {
        TheFastLutFieldW::new(self, 0)
    }
}
#[doc = "The Fast LUT registers allows the ports to be placed in Fast LUT mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_fast_lut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_fast_lut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleFastLutSpec;
impl crate::RegisterSpec for AleAleFastLutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_fast_lut::R`](R) reader structure"]
impl crate::Readable for AleAleFastLutSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_fast_lut::W`](W) writer structure"]
impl crate::Writable for AleAleFastLutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_FAST_LUT to value 0"]
impl crate::Resettable for AleAleFastLutSpec {
    const RESET_VALUE: u32 = 0;
}
