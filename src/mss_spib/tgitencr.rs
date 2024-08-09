#[doc = "Register `TGITENCR` reader"]
pub type R = crate::R<TgitencrSpec>;
#[doc = "Register `TGITENCR` writer"]
pub type W = crate::W<TgitencrSpec>;
#[doc = "Field `CLRINTENSUS` reader - 15:0\\]
Transfer group interrupt clear (disable) when transfer suspended Write: 1 = Disables the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt Interrupt does not get generated when Transfer Group x gets suspended. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets suspended. 0 = ΓÇ£The Transfer group x suspendedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets suspended"]
pub type ClrintensusR = crate::FieldReader<u16>;
#[doc = "Field `CLRINTENSUS` writer - 15:0\\]
Transfer group interrupt clear (disable) when transfer suspended Write: 1 = Disables the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt Interrupt does not get generated when Transfer Group x gets suspended. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets suspended. 0 = ΓÇ£The Transfer group x suspendedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets suspended"]
pub type ClrintensusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLRINTENRDY` reader - 31:16\\]
Transfer group interrupt clear (disable) when transfer finished. Write: 1 = Disables the ΓÇ£The Transfer group x completed ΓÇ£ interrupt Interrupt does not get generated when Transfer Group x gets completed. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets completed. 0 = ΓÇ£The Transfer group x completedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets completed"]
pub type ClrintenrdyR = crate::FieldReader<u16>;
#[doc = "Field `CLRINTENRDY` writer - 31:16\\]
Transfer group interrupt clear (disable) when transfer finished. Write: 1 = Disables the ΓÇ£The Transfer group x completed ΓÇ£ interrupt Interrupt does not get generated when Transfer Group x gets completed. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets completed. 0 = ΓÇ£The Transfer group x completedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets completed"]
pub type ClrintenrdyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Transfer group interrupt clear (disable) when transfer suspended Write: 1 = Disables the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt Interrupt does not get generated when Transfer Group x gets suspended. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets suspended. 0 = ΓÇ£The Transfer group x suspendedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets suspended"]
    #[inline(always)]
    pub fn clrintensus(&self) -> ClrintensusR {
        ClrintensusR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Transfer group interrupt clear (disable) when transfer finished. Write: 1 = Disables the ΓÇ£The Transfer group x completed ΓÇ£ interrupt Interrupt does not get generated when Transfer Group x gets completed. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets completed. 0 = ΓÇ£The Transfer group x completedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets completed"]
    #[inline(always)]
    pub fn clrintenrdy(&self) -> ClrintenrdyR {
        ClrintenrdyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Transfer group interrupt clear (disable) when transfer suspended Write: 1 = Disables the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt Interrupt does not get generated when Transfer Group x gets suspended. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets suspended. 0 = ΓÇ£The Transfer group x suspendedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets suspended"]
    #[inline(always)]
    #[must_use]
    pub fn clrintensus(&mut self) -> ClrintensusW<TgitencrSpec> {
        ClrintensusW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Transfer group interrupt clear (disable) when transfer finished. Write: 1 = Disables the ΓÇ£The Transfer group x completed ΓÇ£ interrupt Interrupt does not get generated when Transfer Group x gets completed. 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is enabled Interrupt gets generated when Transfer Group x gets completed. 0 = ΓÇ£The Transfer group x completedΓÇ¥ interrupt is disabled Interrupt does not get generated when Transfer Group x gets completed"]
    #[inline(always)]
    #[must_use]
    pub fn clrintenrdy(&mut self) -> ClrintenrdyW<TgitencrSpec> {
        ClrintenrdyW::new(self, 16)
    }
}
#[doc = "MibSPI Transfer Group Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tgitencr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgitencr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TgitencrSpec;
impl crate::RegisterSpec for TgitencrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tgitencr::R`](R) reader structure"]
impl crate::Readable for TgitencrSpec {}
#[doc = "`write(|w| ..)` method takes [`tgitencr::W`](W) writer structure"]
impl crate::Writable for TgitencrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TGITENCR to value 0"]
impl crate::Resettable for TgitencrSpec {
    const RESET_VALUE: u32 = 0;
}
