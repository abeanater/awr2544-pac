#[doc = "Register `Interrupt Enable` reader"]
pub type R = crate::R<InterruptEnableSpec>;
#[doc = "Register `Interrupt Enable` writer"]
pub type W = crate::W<InterruptEnableSpec>;
#[doc = "Field `prot_err_en` reader - 0:0\\]
Protection violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type ProtErrEnR = crate::BitReader;
#[doc = "Field `prot_err_en` writer - 0:0\\]
Protection violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type ProtErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `addr_err_en` reader - 1:1\\]
Addressing violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type AddrErrEnR = crate::BitReader;
#[doc = "Field `addr_err_en` writer - 1:1\\]
Addressing violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type AddrErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn prot_err_en(&self) -> ProtErrEnR {
        ProtErrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn addr_err_en(&self) -> AddrErrEnR {
        AddrErrEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn prot_err_en(&mut self) -> ProtErrEnW<InterruptEnableSpec> {
        ProtErrEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn addr_err_en(&mut self) -> AddrErrEnW<InterruptEnableSpec> {
        AddrErrEnW::new(self, 1)
    }
}
#[doc = "Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptEnableSpec;
impl crate::RegisterSpec for InterruptEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_enable::R`](R) reader structure"]
impl crate::Readable for InterruptEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_enable::W`](W) writer structure"]
impl crate::Writable for InterruptEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Interrupt Enable to value 0"]
impl crate::Resettable for InterruptEnableSpec {
    const RESET_VALUE: u32 = 0;
}
