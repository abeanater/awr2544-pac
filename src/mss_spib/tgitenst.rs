#[doc = "Register `TGITENST` reader"]
pub type R = crate::R<TgitenstSpec>;
#[doc = "Register `TGITENST` writer"]
pub type W = crate::W<TgitenstSpec>;
#[doc = "Field `SETINTENSUS` reader - 15:0\\]
Transfer group interrupt set (enable) when transfer suspended Write: 1 = Enables the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt Interrupt gets generated when Transfer Group x gets suspended. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets suspended. 0 = ΓÇ£The Transfer group x suspendedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets suspended"]
pub type SetintensusR = crate::FieldReader<u16>;
#[doc = "Field `SETINTENSUS` writer - 15:0\\]
Transfer group interrupt set (enable) when transfer suspended Write: 1 = Enables the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt Interrupt gets generated when Transfer Group x gets suspended. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets suspended. 0 = ΓÇ£The Transfer group x suspendedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets suspended"]
pub type SetintensusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SETINTENRDY` reader - 31:16\\]
Transfer group interrupt set (enable) when transfer finished. Write: 1 = Enables the ΓÇ£The Transfer group x completed ΓÇ£ interrupt Interrupt gets generated when Transfer Group x gets completed. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets completed. 0 = ΓÇ£The Transfer group x completedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets completed"]
pub type SetintenrdyR = crate::FieldReader<u16>;
#[doc = "Field `SETINTENRDY` writer - 31:16\\]
Transfer group interrupt set (enable) when transfer finished. Write: 1 = Enables the ΓÇ£The Transfer group x completed ΓÇ£ interrupt Interrupt gets generated when Transfer Group x gets completed. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets completed. 0 = ΓÇ£The Transfer group x completedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets completed"]
pub type SetintenrdyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Transfer group interrupt set (enable) when transfer suspended Write: 1 = Enables the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt Interrupt gets generated when Transfer Group x gets suspended. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets suspended. 0 = ΓÇ£The Transfer group x suspendedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets suspended"]
    #[inline(always)]
    pub fn setintensus(&self) -> SetintensusR {
        SetintensusR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Transfer group interrupt set (enable) when transfer finished. Write: 1 = Enables the ΓÇ£The Transfer group x completed ΓÇ£ interrupt Interrupt gets generated when Transfer Group x gets completed. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets completed. 0 = ΓÇ£The Transfer group x completedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets completed"]
    #[inline(always)]
    pub fn setintenrdy(&self) -> SetintenrdyR {
        SetintenrdyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Transfer group interrupt set (enable) when transfer suspended Write: 1 = Enables the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt Interrupt gets generated when Transfer Group x gets suspended. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets suspended. 0 = ΓÇ£The Transfer group x suspendedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets suspended"]
    #[inline(always)]
    #[must_use]
    pub fn setintensus(&mut self) -> SetintensusW<TgitenstSpec> {
        SetintensusW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Transfer group interrupt set (enable) when transfer finished. Write: 1 = Enables the ΓÇ£The Transfer group x completed ΓÇ£ interrupt Interrupt gets generated when Transfer Group x gets completed. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets completed. 0 = ΓÇ£The Transfer group x completedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets completed"]
    #[inline(always)]
    #[must_use]
    pub fn setintenrdy(&mut self) -> SetintenrdyW<TgitenstSpec> {
        SetintenrdyW::new(self, 16)
    }
}
#[doc = "MibSPI Transfer Group Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tgitenst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgitenst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TgitenstSpec;
impl crate::RegisterSpec for TgitenstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tgitenst::R`](R) reader structure"]
impl crate::Readable for TgitenstSpec {}
#[doc = "`write(|w| ..)` method takes [`tgitenst::W`](W) writer structure"]
impl crate::Writable for TgitenstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TGITENST to value 0"]
impl crate::Resettable for TgitenstSpec {
    const RESET_VALUE: u32 = 0;
}
