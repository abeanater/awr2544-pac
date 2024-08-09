#[doc = "Register `STCTPR` reader"]
pub type R = crate::R<StctprSpec>;
#[doc = "Register `STCTPR` writer"]
pub type W = crate::W<StctprSpec>;
#[doc = "Field `TO_PRELOAD` reader - 31:0\\]
Self test time out preload (RWP - Read, Priviledge Mode Write only) This register contains the total number of STC clock cycles it will take before a self-test timeout error will be triggered after the initiation of the self-test run. This is a fail safe feature to avoid system hang-up situation on account of any run away self test issues. This register should be loaded with a meaningful count value for this feature to be effective. This register value (preload count value) gets loaded into the self test timeout down counter whenever a self test run is initiated (ST_ENA is enabled). and gets disabled on completion of a self test run."]
pub type ToPreloadR = crate::FieldReader<u32>;
#[doc = "Field `TO_PRELOAD` writer - 31:0\\]
Self test time out preload (RWP - Read, Priviledge Mode Write only) This register contains the total number of STC clock cycles it will take before a self-test timeout error will be triggered after the initiation of the self-test run. This is a fail safe feature to avoid system hang-up situation on account of any run away self test issues. This register should be loaded with a meaningful count value for this feature to be effective. This register value (preload count value) gets loaded into the self test timeout down counter whenever a self test run is initiated (ST_ENA is enabled). and gets disabled on completion of a self test run."]
pub type ToPreloadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Self test time out preload (RWP - Read, Priviledge Mode Write only) This register contains the total number of STC clock cycles it will take before a self-test timeout error will be triggered after the initiation of the self-test run. This is a fail safe feature to avoid system hang-up situation on account of any run away self test issues. This register should be loaded with a meaningful count value for this feature to be effective. This register value (preload count value) gets loaded into the self test timeout down counter whenever a self test run is initiated (ST_ENA is enabled). and gets disabled on completion of a self test run."]
    #[inline(always)]
    pub fn to_preload(&self) -> ToPreloadR {
        ToPreloadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Self test time out preload (RWP - Read, Priviledge Mode Write only) This register contains the total number of STC clock cycles it will take before a self-test timeout error will be triggered after the initiation of the self-test run. This is a fail safe feature to avoid system hang-up situation on account of any run away self test issues. This register should be loaded with a meaningful count value for this feature to be effective. This register value (preload count value) gets loaded into the self test timeout down counter whenever a self test run is initiated (ST_ENA is enabled). and gets disabled on completion of a self test run."]
    #[inline(always)]
    #[must_use]
    pub fn to_preload(&mut self) -> ToPreloadW<StctprSpec> {
        ToPreloadW::new(self, 0)
    }
}
#[doc = "Time out counter preload register\n\nYou can [`read`](crate::Reg::read) this register and get [`stctpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stctpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StctprSpec;
impl crate::RegisterSpec for StctprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stctpr::R`](R) reader structure"]
impl crate::Readable for StctprSpec {}
#[doc = "`write(|w| ..)` method takes [`stctpr::W`](W) writer structure"]
impl crate::Writable for StctprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCTPR to value 0"]
impl crate::Resettable for StctprSpec {
    const RESET_VALUE: u32 = 0;
}
