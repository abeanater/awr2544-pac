#[doc = "Register `CPSW_HW_TRIG_VAL` reader"]
pub type R = crate::R<CpswHwTrigValSpec>;
#[doc = "Register `CPSW_HW_TRIG_VAL` writer"]
pub type W = crate::W<CpswHwTrigValSpec>;
#[doc = "Field `CPSW_HW_TRIG_VAL` reader - 7:0\\]
Trigger"]
pub type CpswHwTrigValR = crate::FieldReader;
#[doc = "Field `CPSW_HW_TRIG_VAL` writer - 7:0\\]
Trigger"]
pub type CpswHwTrigValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Trigger"]
    #[inline(always)]
    pub fn cpsw_hw_trig_val(&self) -> CpswHwTrigValR {
        CpswHwTrigValR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn cpsw_hw_trig_val(&mut self) -> CpswHwTrigValW<CpswHwTrigValSpec> {
        CpswHwTrigValW::new(self, 0)
    }
}
#[doc = "CPSW_HW_TRIG_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_hw_trig_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_hw_trig_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswHwTrigValSpec;
impl crate::RegisterSpec for CpswHwTrigValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_hw_trig_val::R`](R) reader structure"]
impl crate::Readable for CpswHwTrigValSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_hw_trig_val::W`](W) writer structure"]
impl crate::Writable for CpswHwTrigValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_HW_TRIG_VAL to value 0"]
impl crate::Resettable for CpswHwTrigValSpec {
    const RESET_VALUE: u32 = 0;
}
