#[doc = "Register `ALE_THREADMAPVAL` reader"]
pub type R = crate::R<AleThreadmapvalSpec>;
#[doc = "Register `ALE_THREADMAPVAL` writer"]
pub type W = crate::W<AleThreadmapvalSpec>;
#[doc = "Field `THREAD_VALUE_THIS` reader - 5:0\\]
Thread Value - This field is the thread ID value that is used to map a classifier hit to thread ID for host traffic."]
pub type ThreadValueThisR = crate::FieldReader;
#[doc = "Field `THREAD_VALUE_THIS` writer - 5:0\\]
Thread Value - This field is the thread ID value that is used to map a classifier hit to thread ID for host traffic."]
pub type ThreadValueThisW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `THREAD_ENABLE_WHEN` reader - 15:15\\]
Thread Enable - When set the switch will use the ~ithreadval for the selected classifier match. If clear the the thread ID will be determined by the ~bTHREADMAPDEF register settings."]
pub type ThreadEnableWhenR = crate::BitReader;
#[doc = "Field `THREAD_ENABLE_WHEN` writer - 15:15\\]
Thread Enable - When set the switch will use the ~ithreadval for the selected classifier match. If clear the the thread ID will be determined by the ~bTHREADMAPDEF register settings."]
pub type ThreadEnableWhenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Thread Value - This field is the thread ID value that is used to map a classifier hit to thread ID for host traffic."]
    #[inline(always)]
    pub fn thread_value_this(&self) -> ThreadValueThisR {
        ThreadValueThisR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Thread Enable - When set the switch will use the ~ithreadval for the selected classifier match. If clear the the thread ID will be determined by the ~bTHREADMAPDEF register settings."]
    #[inline(always)]
    pub fn thread_enable_when(&self) -> ThreadEnableWhenR {
        ThreadEnableWhenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Thread Value - This field is the thread ID value that is used to map a classifier hit to thread ID for host traffic."]
    #[inline(always)]
    #[must_use]
    pub fn thread_value_this(&mut self) -> ThreadValueThisW<AleThreadmapvalSpec> {
        ThreadValueThisW::new(self, 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Thread Enable - When set the switch will use the ~ithreadval for the selected classifier match. If clear the the thread ID will be determined by the ~bTHREADMAPDEF register settings."]
    #[inline(always)]
    #[must_use]
    pub fn thread_enable_when(&mut self) -> ThreadEnableWhenW<AleThreadmapvalSpec> {
        ThreadEnableWhenW::new(self, 15)
    }
}
#[doc = "The THREAD Mapping Value register is used to set the thread ID for a particular classifier entry.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_threadmapval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_threadmapval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleThreadmapvalSpec;
impl crate::RegisterSpec for AleThreadmapvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_threadmapval::R`](R) reader structure"]
impl crate::Readable for AleThreadmapvalSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_threadmapval::W`](W) writer structure"]
impl crate::Writable for AleThreadmapvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_THREADMAPVAL to value 0"]
impl crate::Resettable for AleThreadmapvalSpec {
    const RESET_VALUE: u32 = 0;
}
