#[doc = "Register `SINGLE_STEP_EN` reader"]
pub type R = crate::R<SingleStepEnSpec>;
#[doc = "Register `SINGLE_STEP_EN` writer"]
pub type W = crate::W<SingleStepEnSpec>;
#[doc = "Field `single_step_en` reader - 0:0\\]
Single step enable 1'b1 : the state machine executes one parameter-set at a time and wait for the single step trigger every time"]
pub type SingleStepEnR = crate::BitReader;
#[doc = "Field `single_step_en` writer - 0:0\\]
Single step enable 1'b1 : the state machine executes one parameter-set at a time and wait for the single step trigger every time"]
pub type SingleStepEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Single step enable 1'b1 : the state machine executes one parameter-set at a time and wait for the single step trigger every time"]
    #[inline(always)]
    pub fn single_step_en(&self) -> SingleStepEnR {
        SingleStepEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Single step enable 1'b1 : the state machine executes one parameter-set at a time and wait for the single step trigger every time"]
    #[inline(always)]
    #[must_use]
    pub fn single_step_en(&mut self) -> SingleStepEnW<SingleStepEnSpec> {
        SingleStepEnW::new(self, 0)
    }
}
#[doc = "SINGLE_STEP_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`single_step_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`single_step_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SingleStepEnSpec;
impl crate::RegisterSpec for SingleStepEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`single_step_en::R`](R) reader structure"]
impl crate::Readable for SingleStepEnSpec {}
#[doc = "`write(|w| ..)` method takes [`single_step_en::W`](W) writer structure"]
impl crate::Writable for SingleStepEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SINGLE_STEP_EN to value 0"]
impl crate::Resettable for SingleStepEnSpec {
    const RESET_VALUE: u32 = 0;
}
