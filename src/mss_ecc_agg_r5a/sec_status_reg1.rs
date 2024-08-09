#[doc = "Register `sec_status_reg1` reader"]
pub type R = crate::R<SecStatusReg1Spec>;
#[doc = "Register `sec_status_reg1` writer"]
pub type W = crate::W<SecStatusReg1Spec>;
#[doc = "Field `INTERRUPT_PENDING_STATUS` reader - 0:0\\]
Interrupt Pending Status for b1tcm1_bank0_pend"]
pub type InterruptPendingStatusR = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS` writer - 0:0\\]
Interrupt Pending Status for b1tcm1_bank0_pend"]
pub type InterruptPendingStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS` reader - 1:1\\]
Interrupt Pending Status for b1tcm1_bank1_pend"]
pub type InterruptPendingStatusR = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS` writer - 1:1\\]
Interrupt Pending Status for b1tcm1_bank1_pend"]
pub type InterruptPendingStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for b1tcm1_bank0_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status(&self) -> InterruptPendingStatusR {
        InterruptPendingStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for b1tcm1_bank1_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status(&self) -> InterruptPendingStatusR {
        InterruptPendingStatusR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for b1tcm1_bank0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status(&mut self) -> InterruptPendingStatusW<SecStatusReg1Spec> {
        InterruptPendingStatusW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for b1tcm1_bank1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status(&mut self) -> InterruptPendingStatusW<SecStatusReg1Spec> {
        InterruptPendingStatusW::new(self, 1)
    }
}
#[doc = "Interrupt Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_status_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_status_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecStatusReg1Spec;
impl crate::RegisterSpec for SecStatusReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_status_reg1::R`](R) reader structure"]
impl crate::Readable for SecStatusReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_status_reg1::W`](W) writer structure"]
impl crate::Writable for SecStatusReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sec_status_reg1 to value 0"]
impl crate::Resettable for SecStatusReg1Spec {
    const RESET_VALUE: u32 = 0;
}
