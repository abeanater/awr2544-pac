#[doc = "Register `PARAM_DONE_SET_STATUS_0` reader"]
pub type R = crate::R<ParamDoneSetStatus0Spec>;
#[doc = "Register `PARAM_DONE_SET_STATUS_0` writer"]
pub type W = crate::W<ParamDoneSetStatus0Spec>;
#[doc = "Field `param_done_set_status_0` reader - 31:0\\]
Parameter-set done status\\[31:0\\]: This read-only status register can be used by the main processor to see which parameter-sets are complete that led to the interrupt to the main processor. The individual bits in this 64-bit status register indicate which of the 64 parameter-sets have completed."]
pub type ParamDoneSetStatus0R = crate::FieldReader<u32>;
#[doc = "Field `param_done_set_status_0` writer - 31:0\\]
Parameter-set done status\\[31:0\\]: This read-only status register can be used by the main processor to see which parameter-sets are complete that led to the interrupt to the main processor. The individual bits in this 64-bit status register indicate which of the 64 parameter-sets have completed."]
pub type ParamDoneSetStatus0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Parameter-set done status\\[31:0\\]: This read-only status register can be used by the main processor to see which parameter-sets are complete that led to the interrupt to the main processor. The individual bits in this 64-bit status register indicate which of the 64 parameter-sets have completed."]
    #[inline(always)]
    pub fn param_done_set_status_0(&self) -> ParamDoneSetStatus0R {
        ParamDoneSetStatus0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Parameter-set done status\\[31:0\\]: This read-only status register can be used by the main processor to see which parameter-sets are complete that led to the interrupt to the main processor. The individual bits in this 64-bit status register indicate which of the 64 parameter-sets have completed."]
    #[inline(always)]
    #[must_use]
    pub fn param_done_set_status_0(&mut self) -> ParamDoneSetStatus0W<ParamDoneSetStatus0Spec> {
        ParamDoneSetStatus0W::new(self, 0)
    }
}
#[doc = "PARAM_DONE_SET_STATUS_0\n\nYou can [`read`](crate::Reg::read) this register and get [`param_done_set_status_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`param_done_set_status_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamDoneSetStatus0Spec;
impl crate::RegisterSpec for ParamDoneSetStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param_done_set_status_0::R`](R) reader structure"]
impl crate::Readable for ParamDoneSetStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`param_done_set_status_0::W`](W) writer structure"]
impl crate::Writable for ParamDoneSetStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PARAM_DONE_SET_STATUS_0 to value 0"]
impl crate::Resettable for ParamDoneSetStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
