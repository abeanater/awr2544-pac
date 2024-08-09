#[doc = "Register `sec_enable_clr_reg1` reader"]
pub type R = crate::R<SecEnableClrReg1Spec>;
#[doc = "Register `sec_enable_clr_reg1` writer"]
pub type W = crate::W<SecEnableClrReg1Spec>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR_1` reader - 0:0\\]
Interrupt Enable Clear Register for b1tcm1_bank0_pend"]
pub type InterruptEnableClear1R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR_1` writer - 0:0\\]
Interrupt Enable Clear Register for b1tcm1_bank0_pend"]
pub type InterruptEnableClear1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` reader - 1:1\\]
Interrupt Enable Clear Register for b1tcm1_bank1_pend"]
pub type InterruptEnableClearR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` writer - 1:1\\]
Interrupt Enable Clear Register for b1tcm1_bank1_pend"]
pub type InterruptEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for b1tcm1_bank0_pend"]
    #[inline(always)]
    pub fn interrupt_enable_clear_1(&self) -> InterruptEnableClear1R {
        InterruptEnableClear1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for b1tcm1_bank1_pend"]
    #[inline(always)]
    pub fn interrupt_enable_clear(&self) -> InterruptEnableClearR {
        InterruptEnableClearR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for b1tcm1_bank0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear_1(&mut self) -> InterruptEnableClear1W<SecEnableClrReg1Spec> {
        InterruptEnableClear1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for b1tcm1_bank1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear(&mut self) -> InterruptEnableClearW<SecEnableClrReg1Spec> {
        InterruptEnableClearW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Clear Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_clr_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_clr_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnableClrReg1Spec;
impl crate::RegisterSpec for SecEnableClrReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_enable_clr_reg1::R`](R) reader structure"]
impl crate::Readable for SecEnableClrReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_enable_clr_reg1::W`](W) writer structure"]
impl crate::Writable for SecEnableClrReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sec_enable_clr_reg1 to value 0"]
impl crate::Resettable for SecEnableClrReg1Spec {
    const RESET_VALUE: u32 = 0;
}
