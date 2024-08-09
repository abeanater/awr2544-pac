#[doc = "Register `SINGLE_STEP_TRIG` reader"]
pub type R = crate::R<SingleStepTrigSpec>;
#[doc = "Register `SINGLE_STEP_TRIG` writer"]
pub type W = crate::W<SingleStepTrigSpec>;
#[doc = "Field `single_step_trig` reader - 0:0\\]
This is a self clearing sofware trigger bit . When single_step_en is 1 , the state machine executes one parameter-set at a time and wait for the single step trigger every time"]
pub type SingleStepTrigR = crate::BitReader;
#[doc = "Field `single_step_trig` writer - 0:0\\]
This is a self clearing sofware trigger bit . When single_step_en is 1 , the state machine executes one parameter-set at a time and wait for the single step trigger every time"]
pub type SingleStepTrigW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This is a self clearing sofware trigger bit . When single_step_en is 1 , the state machine executes one parameter-set at a time and wait for the single step trigger every time"]
    #[inline(always)]
    pub fn single_step_trig(&self) -> SingleStepTrigR {
        SingleStepTrigR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This is a self clearing sofware trigger bit . When single_step_en is 1 , the state machine executes one parameter-set at a time and wait for the single step trigger every time"]
    #[inline(always)]
    #[must_use]
    pub fn single_step_trig(&mut self) -> SingleStepTrigW<SingleStepTrigSpec> {
        SingleStepTrigW::new(self, 0)
    }
}
#[doc = "SINGLE_STEP_TRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`single_step_trig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`single_step_trig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SingleStepTrigSpec;
impl crate::RegisterSpec for SingleStepTrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`single_step_trig::R`](R) reader structure"]
impl crate::Readable for SingleStepTrigSpec {}
#[doc = "`write(|w| ..)` method takes [`single_step_trig::W`](W) writer structure"]
impl crate::Writable for SingleStepTrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SINGLE_STEP_TRIG to value 0"]
impl crate::Resettable for SingleStepTrigSpec {
    const RESET_VALUE: u32 = 0;
}
