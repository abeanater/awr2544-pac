#[doc = "Register `TGITLVST` reader"]
pub type R = crate::R<TgitlvstSpec>;
#[doc = "Register `TGITLVST` writer"]
pub type W = crate::W<TgitlvstSpec>;
#[doc = "Field `SETINTLVLSUS` reader - 15:0\\]
Transfer group suspendedΓÇ¥ interrupt Level set rigester Write: 1 = Sets the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt to line INT1 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT0"]
pub type SetintlvlsusR = crate::FieldReader<u16>;
#[doc = "Field `SETINTLVLSUS` writer - 15:0\\]
Transfer group suspendedΓÇ¥ interrupt Level set rigester Write: 1 = Sets the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt to line INT1 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT0"]
pub type SetintlvlsusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SETINTLVLRDY` reader - 31:16\\]
Transfer group completedΓÇ¥ Interrupt Level set register Write: 1 = Sets the ΓÇ£The Transfer group x completed ΓÇ£ interrupt to line INT1 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT0"]
pub type SetintlvlrdyR = crate::FieldReader<u16>;
#[doc = "Field `SETINTLVLRDY` writer - 31:16\\]
Transfer group completedΓÇ¥ Interrupt Level set register Write: 1 = Sets the ΓÇ£The Transfer group x completed ΓÇ£ interrupt to line INT1 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT0"]
pub type SetintlvlrdyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Transfer group suspendedΓÇ¥ interrupt Level set rigester Write: 1 = Sets the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt to line INT1 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT0"]
    #[inline(always)]
    pub fn setintlvlsus(&self) -> SetintlvlsusR {
        SetintlvlsusR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Transfer group completedΓÇ¥ Interrupt Level set register Write: 1 = Sets the ΓÇ£The Transfer group x completed ΓÇ£ interrupt to line INT1 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT0"]
    #[inline(always)]
    pub fn setintlvlrdy(&self) -> SetintlvlrdyR {
        SetintlvlrdyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Transfer group suspendedΓÇ¥ interrupt Level set rigester Write: 1 = Sets the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt to line INT1 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT0"]
    #[inline(always)]
    #[must_use]
    pub fn setintlvlsus(&mut self) -> SetintlvlsusW<TgitlvstSpec> {
        SetintlvlsusW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Transfer group completedΓÇ¥ Interrupt Level set register Write: 1 = Sets the ΓÇ£The Transfer group x completed ΓÇ£ interrupt to line INT1 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT0"]
    #[inline(always)]
    #[must_use]
    pub fn setintlvlrdy(&mut self) -> SetintlvlrdyW<TgitlvstSpec> {
        SetintlvlrdyW::new(self, 16)
    }
}
#[doc = "MibSPI Transfer Group Interrupt Level Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tgitlvst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgitlvst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TgitlvstSpec;
impl crate::RegisterSpec for TgitlvstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tgitlvst::R`](R) reader structure"]
impl crate::Readable for TgitlvstSpec {}
#[doc = "`write(|w| ..)` method takes [`tgitlvst::W`](W) writer structure"]
impl crate::Writable for TgitlvstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TGITLVST to value 0"]
impl crate::Resettable for TgitlvstSpec {
    const RESET_VALUE: u32 = 0;
}
