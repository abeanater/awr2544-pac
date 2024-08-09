#[doc = "Register `FW2HWA_TRIG_1` reader"]
pub type R = crate::R<Fw2hwaTrig1Spec>;
#[doc = "Register `FW2HWA_TRIG_1` writer"]
pub type W = crate::W<Fw2hwaTrig1Spec>;
#[doc = "Field `fw2hwa_trigger_1` reader - 0:0\\]
Software trigger bit 1: This register bit is relevant whenever software triggered mode is used (i.e., TRIGMODE = 111b). The main processor software can set this register bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set. It s a Self clearing bit"]
pub type Fw2hwaTrigger1R = crate::BitReader;
#[doc = "Field `fw2hwa_trigger_1` writer - 0:0\\]
Software trigger bit 1: This register bit is relevant whenever software triggered mode is used (i.e., TRIGMODE = 111b). The main processor software can set this register bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set. It s a Self clearing bit"]
pub type Fw2hwaTrigger1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software trigger bit 1: This register bit is relevant whenever software triggered mode is used (i.e., TRIGMODE = 111b). The main processor software can set this register bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set. It s a Self clearing bit"]
    #[inline(always)]
    pub fn fw2hwa_trigger_1(&self) -> Fw2hwaTrigger1R {
        Fw2hwaTrigger1R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software trigger bit 1: This register bit is relevant whenever software triggered mode is used (i.e., TRIGMODE = 111b). The main processor software can set this register bit, so that the State Machine gets triggered and starts the accelerator operations for that parameter-set. It s a Self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn fw2hwa_trigger_1(&mut self) -> Fw2hwaTrigger1W<Fw2hwaTrig1Spec> {
        Fw2hwaTrigger1W::new(self, 0)
    }
}
#[doc = "FW2HWA_TRIG_1\n\nYou can [`read`](crate::Reg::read) this register and get [`fw2hwa_trig_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fw2hwa_trig_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fw2hwaTrig1Spec;
impl crate::RegisterSpec for Fw2hwaTrig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fw2hwa_trig_1::R`](R) reader structure"]
impl crate::Readable for Fw2hwaTrig1Spec {}
#[doc = "`write(|w| ..)` method takes [`fw2hwa_trig_1::W`](W) writer structure"]
impl crate::Writable for Fw2hwaTrig1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FW2HWA_TRIG_1 to value 0"]
impl crate::Resettable for Fw2hwaTrig1Spec {
    const RESET_VALUE: u32 = 0;
}
