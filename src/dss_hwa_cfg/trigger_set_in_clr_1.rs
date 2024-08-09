#[doc = "Register `TRIGGER_SET_IN_CLR_1` reader"]
pub type R = crate::R<TriggerSetInClr1Spec>;
#[doc = "Register `TRIGGER_SET_IN_CLR_1` writer"]
pub type W = crate::W<TriggerSetInClr1Spec>;
#[doc = "Field `trigger_set_in_clr_1` reader - 0:0\\]
Clear trigger_set_status : This register-bit when set clears the trigger status register TRIGGER_SET_STATUS_1 described above It s a Self clearing bit"]
pub type TriggerSetInClr1R = crate::BitReader;
#[doc = "Field `trigger_set_in_clr_1` writer - 0:0\\]
Clear trigger_set_status : This register-bit when set clears the trigger status register TRIGGER_SET_STATUS_1 described above It s a Self clearing bit"]
pub type TriggerSetInClr1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear trigger_set_status : This register-bit when set clears the trigger status register TRIGGER_SET_STATUS_1 described above It s a Self clearing bit"]
    #[inline(always)]
    pub fn trigger_set_in_clr_1(&self) -> TriggerSetInClr1R {
        TriggerSetInClr1R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear trigger_set_status : This register-bit when set clears the trigger status register TRIGGER_SET_STATUS_1 described above It s a Self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn trigger_set_in_clr_1(&mut self) -> TriggerSetInClr1W<TriggerSetInClr1Spec> {
        TriggerSetInClr1W::new(self, 0)
    }
}
#[doc = "TRIGGER_SET_IN_CLR_1\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger_set_in_clr_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger_set_in_clr_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TriggerSetInClr1Spec;
impl crate::RegisterSpec for TriggerSetInClr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger_set_in_clr_1::R`](R) reader structure"]
impl crate::Readable for TriggerSetInClr1Spec {}
#[doc = "`write(|w| ..)` method takes [`trigger_set_in_clr_1::W`](W) writer structure"]
impl crate::Writable for TriggerSetInClr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIGGER_SET_IN_CLR_1 to value 0"]
impl crate::Resettable for TriggerSetInClr1Spec {
    const RESET_VALUE: u32 = 0;
}
