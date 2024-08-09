#[doc = "Register `FSM_STATE` reader"]
pub type R = crate::R<FsmStateSpec>;
#[doc = "Register `FSM_STATE` writer"]
pub type W = crate::W<FsmStateSpec>;
#[doc = "Field `fsm_state` reader - 2:0\\]
Current state of the state machine"]
pub type FsmStateR = crate::FieldReader;
#[doc = "Field `fsm_state` writer - 2:0\\]
Current state of the state machine"]
pub type FsmStateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Current state of the state machine"]
    #[inline(always)]
    pub fn fsm_state(&self) -> FsmStateR {
        FsmStateR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Current state of the state machine"]
    #[inline(always)]
    #[must_use]
    pub fn fsm_state(&mut self) -> FsmStateW<FsmStateSpec> {
        FsmStateW::new(self, 0)
    }
}
#[doc = "FSM_STATE\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmStateSpec;
impl crate::RegisterSpec for FsmStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_state::R`](R) reader structure"]
impl crate::Readable for FsmStateSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_state::W`](W) writer structure"]
impl crate::Writable for FsmStateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_STATE to value 0"]
impl crate::Resettable for FsmStateSpec {
    const RESET_VALUE: u32 = 0;
}
