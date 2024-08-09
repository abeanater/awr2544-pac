#[doc = "Register `PARAM_DONE_CLR_1` reader"]
pub type R = crate::R<ParamDoneClr1Spec>;
#[doc = "Register `PARAM_DONE_CLR_1` writer"]
pub type W = crate::W<ParamDoneClr1Spec>;
#[doc = "Field `param_done_status_clr_1` reader - 31:0\\]
Status bits in PARAM_DONE_SET_STATUS are not automatically cleared, but they can be individually cleared by writing to PARAM_DONE_CLR. It is a Self clearing bit"]
pub type ParamDoneStatusClr1R = crate::FieldReader<u32>;
#[doc = "Field `param_done_status_clr_1` writer - 31:0\\]
Status bits in PARAM_DONE_SET_STATUS are not automatically cleared, but they can be individually cleared by writing to PARAM_DONE_CLR. It is a Self clearing bit"]
pub type ParamDoneStatusClr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Status bits in PARAM_DONE_SET_STATUS are not automatically cleared, but they can be individually cleared by writing to PARAM_DONE_CLR. It is a Self clearing bit"]
    #[inline(always)]
    pub fn param_done_status_clr_1(&self) -> ParamDoneStatusClr1R {
        ParamDoneStatusClr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Status bits in PARAM_DONE_SET_STATUS are not automatically cleared, but they can be individually cleared by writing to PARAM_DONE_CLR. It is a Self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn param_done_status_clr_1(&mut self) -> ParamDoneStatusClr1W<ParamDoneClr1Spec> {
        ParamDoneStatusClr1W::new(self, 0)
    }
}
#[doc = "PARAM_DONE_CLR_1\n\nYou can [`read`](crate::Reg::read) this register and get [`param_done_clr_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param_done_clr_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamDoneClr1Spec;
impl crate::RegisterSpec for ParamDoneClr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param_done_clr_1::R`](R) reader structure"]
impl crate::Readable for ParamDoneClr1Spec {}
#[doc = "`write(|w| ..)` method takes [`param_done_clr_1::W`](W) writer structure"]
impl crate::Writable for ParamDoneClr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PARAM_DONE_CLR_1 to value 0"]
impl crate::Resettable for ParamDoneClr1Spec {
    const RESET_VALUE: u32 = 0;
}
