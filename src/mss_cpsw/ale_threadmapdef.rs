#[doc = "Register `ALE_THREADMAPDEF` reader"]
pub type R = crate::R<AleThreadmapdefSpec>;
#[doc = "Register `ALE_THREADMAPDEF` writer"]
pub type W = crate::W<AleThreadmapdefSpec>;
#[doc = "Field `DEFAULT_THREAD_VALUE` reader - 5:0\\]
Default Thread Value - This field specifies the default thread ID value."]
pub type DefaultThreadValueR = crate::FieldReader;
#[doc = "Field `DEFAULT_THREAD_VALUE` writer - 5:0\\]
Default Thread Value - This field specifies the default thread ID value."]
pub type DefaultThreadValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DEFAULT_TREAD_ENABLE` reader - 15:15\\]
Default Tread Enable - When set the switch will use the ~idefthreadval for the host interface thread ID if no classifier is matched. If clear the switch will generate its own thread ID based on port and priority if there is no classifier match."]
pub type DefaultTreadEnableR = crate::BitReader;
#[doc = "Field `DEFAULT_TREAD_ENABLE` writer - 15:15\\]
Default Tread Enable - When set the switch will use the ~idefthreadval for the host interface thread ID if no classifier is matched. If clear the switch will generate its own thread ID based on port and priority if there is no classifier match."]
pub type DefaultTreadEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Default Thread Value - This field specifies the default thread ID value."]
    #[inline(always)]
    pub fn default_thread_value(&self) -> DefaultThreadValueR {
        DefaultThreadValueR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Default Tread Enable - When set the switch will use the ~idefthreadval for the host interface thread ID if no classifier is matched. If clear the switch will generate its own thread ID based on port and priority if there is no classifier match."]
    #[inline(always)]
    pub fn default_tread_enable(&self) -> DefaultTreadEnableR {
        DefaultTreadEnableR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Default Thread Value - This field specifies the default thread ID value."]
    #[inline(always)]
    #[must_use]
    pub fn default_thread_value(&mut self) -> DefaultThreadValueW<AleThreadmapdefSpec> {
        DefaultThreadValueW::new(self, 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Default Tread Enable - When set the switch will use the ~idefthreadval for the host interface thread ID if no classifier is matched. If clear the switch will generate its own thread ID based on port and priority if there is no classifier match."]
    #[inline(always)]
    #[must_use]
    pub fn default_tread_enable(&mut self) -> DefaultTreadEnableW<AleThreadmapdefSpec> {
        DefaultTreadEnableW::new(self, 15)
    }
}
#[doc = "The THREAD Mapping Default Value register is used to set the default thread ID when no classifier is matched,\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_threadmapdef::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_threadmapdef::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleThreadmapdefSpec;
impl crate::RegisterSpec for AleThreadmapdefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_threadmapdef::R`](R) reader structure"]
impl crate::Readable for AleThreadmapdefSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_threadmapdef::W`](W) writer structure"]
impl crate::Writable for AleThreadmapdefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_THREADMAPDEF to value 0"]
impl crate::Resettable for AleThreadmapdefSpec {
    const RESET_VALUE: u32 = 0;
}
