#[doc = "Register `sec_enable_set_reg1` reader"]
pub type R = crate::R<SecEnableSetReg1Spec>;
#[doc = "Register `sec_enable_set_reg1` writer"]
pub type W = crate::W<SecEnableSetReg1Spec>;
#[doc = "Field `INTERRUPT_ENABLE_SET` reader - 0:0\\]
Interrupt Enable Set Register for b1tcm1_bank0_pend"]
pub type InterruptEnableSetR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET` writer - 0:0\\]
Interrupt Enable Set Register for b1tcm1_bank0_pend"]
pub type InterruptEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_SET` reader - 1:1\\]
Interrupt Enable Set Register for b1tcm1_bank1_pend"]
pub type InterruptEnableSetR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_SET` writer - 1:1\\]
Interrupt Enable Set Register for b1tcm1_bank1_pend"]
pub type InterruptEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for b1tcm1_bank0_pend"]
    #[inline(always)]
    pub fn interrupt_enable_set(&self) -> InterruptEnableSetR {
        InterruptEnableSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for b1tcm1_bank1_pend"]
    #[inline(always)]
    pub fn interrupt_enable_set(&self) -> InterruptEnableSetR {
        InterruptEnableSetR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for b1tcm1_bank0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set(&mut self) -> InterruptEnableSetW<SecEnableSetReg1Spec> {
        InterruptEnableSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for b1tcm1_bank1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_set(&mut self) -> InterruptEnableSetW<SecEnableSetReg1Spec> {
        InterruptEnableSetW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Set Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_set_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_set_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnableSetReg1Spec;
impl crate::RegisterSpec for SecEnableSetReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_enable_set_reg1::R`](R) reader structure"]
impl crate::Readable for SecEnableSetReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_enable_set_reg1::W`](W) writer structure"]
impl crate::Writable for SecEnableSetReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sec_enable_set_reg1 to value 0"]
impl crate::Resettable for SecEnableSetReg1Spec {
    const RESET_VALUE: u32 = 0;
}
