#[doc = "Register `Interrupt Enabled Status/Clear` reader"]
pub type R = crate::R<InterruptEnabledStatusClearSpec>;
#[doc = "Register `Interrupt Enabled Status/Clear` writer"]
pub type W = crate::W<InterruptEnabledStatusClearSpec>;
#[doc = "Field `enabled_prot_err` reader - 0:0\\]
Protection violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledProtErrR = crate::BitReader;
#[doc = "Field `enabled_prot_err` writer - 0:0\\]
Protection violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledProtErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enabled_addr_err` reader - 1:1\\]
Addressing violation error. Enabled status is read.Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledAddrErrR = crate::BitReader;
#[doc = "Field `enabled_addr_err` writer - 1:1\\]
Addressing violation error. Enabled status is read.Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledAddrErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn enabled_prot_err(&self) -> EnabledProtErrR {
        EnabledProtErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Enabled status is read.Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn enabled_addr_err(&self) -> EnabledAddrErrR {
        EnabledAddrErrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_prot_err(&mut self) -> EnabledProtErrW<InterruptEnabledStatusClearSpec> {
        EnabledProtErrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Enabled status is read.Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_addr_err(&mut self) -> EnabledAddrErrW<InterruptEnabledStatusClearSpec> {
        EnabledAddrErrW::new(self, 1)
    }
}
#[doc = "Interrupt Enabled Status/Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_enabled_status_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_enabled_status_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptEnabledStatusClearSpec;
impl crate::RegisterSpec for InterruptEnabledStatusClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_enabled_status_clear::R`](R) reader structure"]
impl crate::Readable for InterruptEnabledStatusClearSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt_enabled_status_clear::W`](W) writer structure"]
impl crate::Writable for InterruptEnabledStatusClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Interrupt Enabled Status/Clear to value 0"]
impl crate::Resettable for InterruptEnabledStatusClearSpec {
    const RESET_VALUE: u32 = 0;
}
