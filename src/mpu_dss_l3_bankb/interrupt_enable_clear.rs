#[doc = "Register `Interrupt Enable Clear` reader"]
pub type R = crate::R<InterruptEnableClearSpec>;
#[doc = "Register `Interrupt Enable Clear` writer"]
pub type W = crate::W<InterruptEnableClearSpec>;
#[doc = "Field `prot_err_en_clr` reader - 0:0\\]
Protection violation error enable. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type ProtErrEnClrR = crate::BitReader;
#[doc = "Field `prot_err_en_clr` writer - 0:0\\]
Protection violation error enable. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type ProtErrEnClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `addr_err_en_clr` reader - 1:1\\]
Addressing violation error enable. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type AddrErrEnClrR = crate::BitReader;
#[doc = "Field `addr_err_en_clr` writer - 1:1\\]
Addressing violation error enable. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type AddrErrEnClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn prot_err_en_clr(&self) -> ProtErrEnClrR {
        ProtErrEnClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn addr_err_en_clr(&self) -> AddrErrEnClrR {
        AddrErrEnClrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn prot_err_en_clr(&mut self) -> ProtErrEnClrW<InterruptEnableClearSpec> {
        ProtErrEnClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn addr_err_en_clr(&mut self) -> AddrErrEnClrW<InterruptEnableClearSpec> {
        AddrErrEnClrW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptEnableClearSpec;
impl crate::RegisterSpec for InterruptEnableClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_enable_clear::R`](R) reader structure"]
impl crate::Readable for InterruptEnableClearSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_enable_clear::W`](W) writer structure"]
impl crate::Writable for InterruptEnableClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Interrupt Enable Clear to value 0"]
impl crate::Resettable for InterruptEnableClearSpec {
    const RESET_VALUE: u32 = 0;
}
