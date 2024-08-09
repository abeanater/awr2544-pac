#[doc = "Register `TGITLVCR` reader"]
pub type R = crate::R<TgitlvcrSpec>;
#[doc = "Register `TGITLVCR` writer"]
pub type W = crate::W<TgitlvcrSpec>;
#[doc = "Field `CLRINTLVLSUS` reader - 15:0\\]
Transfer group suspendedΓÇ¥ interrupt Level clear register Write: 1 = Sets the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt to line INT0 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT0"]
pub type ClrintlvlsusR = crate::FieldReader<u16>;
#[doc = "Field `CLRINTLVLSUS` writer - 15:0\\]
Transfer group suspendedΓÇ¥ interrupt Level clear register Write: 1 = Sets the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt to line INT0 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT0"]
pub type ClrintlvlsusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLRINTLVLRDY` reader - 31:16\\]
Transfer group completedΓÇ¥ Interrupt Level clear register Write: 1 = Sets the ΓÇ£The Transfer group x completed ΓÇ£ interrupt to line INT0 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT0"]
pub type ClrintlvlrdyR = crate::FieldReader<u16>;
#[doc = "Field `CLRINTLVLRDY` writer - 31:16\\]
Transfer group completedΓÇ¥ Interrupt Level clear register Write: 1 = Sets the ΓÇ£The Transfer group x completed ΓÇ£ interrupt to line INT0 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT0"]
pub type ClrintlvlrdyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Transfer group suspendedΓÇ¥ interrupt Level clear register Write: 1 = Sets the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt to line INT0 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT0"]
    #[inline(always)]
    pub fn clrintlvlsus(&self) -> ClrintlvlsusR {
        ClrintlvlsusR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Transfer group completedΓÇ¥ Interrupt Level clear register Write: 1 = Sets the ΓÇ£The Transfer group x completed ΓÇ£ interrupt to line INT0 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT0"]
    #[inline(always)]
    pub fn clrintlvlrdy(&self) -> ClrintlvlrdyR {
        ClrintlvlrdyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Transfer group suspendedΓÇ¥ interrupt Level clear register Write: 1 = Sets the ΓÇ£The Transfer group x suspended ΓÇ£ interrupt to line INT0 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x suspended ΓÇ£ interrupt is set to line INT0"]
    #[inline(always)]
    #[must_use]
    pub fn clrintlvlsus(&mut self) -> ClrintlvlsusW<TgitlvcrSpec> {
        ClrintlvlsusW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Transfer group completedΓÇ¥ Interrupt Level clear register Write: 1 = Sets the ΓÇ£The Transfer group x completed ΓÇ£ interrupt to line INT0 0 = Has no effect. Read: 1 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT1 0 = ΓÇ£The Transfer group x completed ΓÇ£ interrupt is set to line INT0"]
    #[inline(always)]
    #[must_use]
    pub fn clrintlvlrdy(&mut self) -> ClrintlvlrdyW<TgitlvcrSpec> {
        ClrintlvlrdyW::new(self, 16)
    }
}
#[doc = "MibSPI Transfer Group Interrupt Level Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tgitlvcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgitlvcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TgitlvcrSpec;
impl crate::RegisterSpec for TgitlvcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tgitlvcr::R`](R) reader structure"]
impl crate::Readable for TgitlvcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tgitlvcr::W`](W) writer structure"]
impl crate::Writable for TgitlvcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TGITLVCR to value 0"]
impl crate::Resettable for TgitlvcrSpec {
    const RESET_VALUE: u32 = 0;
}
