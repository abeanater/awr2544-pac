#[doc = "Register `TRIGGER_SET_IN_CLR_0` reader"]
pub type R = crate::R<TriggerSetInClr0Spec>;
#[doc = "Register `TRIGGER_SET_IN_CLR_0` writer"]
pub type W = crate::W<TriggerSetInClr0Spec>;
#[doc = "Field `trigger_set_in_clr_0` reader - 0:0\\]
Clear trigger_set_status : This register-bit when set clears the trigger status register TRIGGER_SET_STATUS_0 described above It s a Self clearing bit"]
pub type TriggerSetInClr0R = crate::BitReader;
#[doc = "Field `trigger_set_in_clr_0` writer - 0:0\\]
Clear trigger_set_status : This register-bit when set clears the trigger status register TRIGGER_SET_STATUS_0 described above It s a Self clearing bit"]
pub type TriggerSetInClr0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear trigger_set_status : This register-bit when set clears the trigger status register TRIGGER_SET_STATUS_0 described above It s a Self clearing bit"]
    #[inline(always)]
    pub fn trigger_set_in_clr_0(&self) -> TriggerSetInClr0R {
        TriggerSetInClr0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear trigger_set_status : This register-bit when set clears the trigger status register TRIGGER_SET_STATUS_0 described above It s a Self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn trigger_set_in_clr_0(&mut self) -> TriggerSetInClr0W<TriggerSetInClr0Spec> {
        TriggerSetInClr0W::new(self, 0)
    }
}
#[doc = "TRIGGER_SET_IN_CLR_0\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger_set_in_clr_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger_set_in_clr_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TriggerSetInClr0Spec;
impl crate::RegisterSpec for TriggerSetInClr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger_set_in_clr_0::R`](R) reader structure"]
impl crate::Readable for TriggerSetInClr0Spec {}
#[doc = "`write(|w| ..)` method takes [`trigger_set_in_clr_0::W`](W) writer structure"]
impl crate::Writable for TriggerSetInClr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIGGER_SET_IN_CLR_0 to value 0"]
impl crate::Resettable for TriggerSetInClr0Spec {
    const RESET_VALUE: u32 = 0;
}
