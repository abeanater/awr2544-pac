#[doc = "Register `ded_status_reg0` reader"]
pub type R = crate::R<DedStatusReg0Spec>;
#[doc = "Register `ded_status_reg0` writer"]
pub type W = crate::W<DedStatusReg0Spec>;
#[doc = "Field `INTERRUPT_PENDING_STATUS` reader - 0:0\\]
Interrupt Pending Status for mss_l2slv0_pend"]
pub type InterruptPendingStatusR = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS` writer - 0:0\\]
Interrupt Pending Status for mss_l2slv0_pend"]
pub type InterruptPendingStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS` reader - 1:1\\]
Interrupt Pending Status for mss_l2slv1_pend"]
pub type InterruptPendingStatusR = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS` writer - 1:1\\]
Interrupt Pending Status for mss_l2slv1_pend"]
pub type InterruptPendingStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS` reader - 2:2\\]
Interrupt Pending Status for mss_mbox_pend"]
pub type InterruptPendingStatusR = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS` writer - 2:2\\]
Interrupt Pending Status for mss_mbox_pend"]
pub type InterruptPendingStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS` reader - 3:3\\]
Interrupt Pending Status for mss_retram_pend"]
pub type InterruptPendingStatusR = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS` writer - 3:3\\]
Interrupt Pending Status for mss_retram_pend"]
pub type InterruptPendingStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS` reader - 4:4\\]
Interrupt Pending Status for gpadc_pend"]
pub type InterruptPendingStatusR = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS` writer - 4:4\\]
Interrupt Pending Status for gpadc_pend"]
pub type InterruptPendingStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS` reader - 5:5\\]
Interrupt Pending Status for tptc_a0_pend"]
pub type InterruptPendingStatusR = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS` writer - 5:5\\]
Interrupt Pending Status for tptc_a0_pend"]
pub type InterruptPendingStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS` reader - 6:6\\]
Interrupt Pending Status for tptc_a1_pend"]
pub type InterruptPendingStatusR = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS` writer - 6:6\\]
Interrupt Pending Status for tptc_a1_pend"]
pub type InterruptPendingStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_PENDING_STATUS` reader - 7:7\\]
Interrupt Pending Status for mss_l2_bankc_pend"]
pub type InterruptPendingStatusR = crate::BitReader;
#[doc = "Field `INTERRUPT_PENDING_STATUS` writer - 7:7\\]
Interrupt Pending Status for mss_l2_bankc_pend"]
pub type InterruptPendingStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for mss_l2slv0_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status(&self) -> InterruptPendingStatusR {
        InterruptPendingStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for mss_l2slv1_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status(&self) -> InterruptPendingStatusR {
        InterruptPendingStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for mss_mbox_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status(&self) -> InterruptPendingStatusR {
        InterruptPendingStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for mss_retram_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status(&self) -> InterruptPendingStatusR {
        InterruptPendingStatusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for gpadc_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status(&self) -> InterruptPendingStatusR {
        InterruptPendingStatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for tptc_a0_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status(&self) -> InterruptPendingStatusR {
        InterruptPendingStatusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for tptc_a1_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status(&self) -> InterruptPendingStatusR {
        InterruptPendingStatusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for mss_l2_bankc_pend"]
    #[inline(always)]
    pub fn interrupt_pending_status(&self) -> InterruptPendingStatusR {
        InterruptPendingStatusR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for mss_l2slv0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status(&mut self) -> InterruptPendingStatusW<DedStatusReg0Spec> {
        InterruptPendingStatusW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for mss_l2slv1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status(&mut self) -> InterruptPendingStatusW<DedStatusReg0Spec> {
        InterruptPendingStatusW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for mss_mbox_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status(&mut self) -> InterruptPendingStatusW<DedStatusReg0Spec> {
        InterruptPendingStatusW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for mss_retram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status(&mut self) -> InterruptPendingStatusW<DedStatusReg0Spec> {
        InterruptPendingStatusW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for gpadc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status(&mut self) -> InterruptPendingStatusW<DedStatusReg0Spec> {
        InterruptPendingStatusW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for tptc_a0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status(&mut self) -> InterruptPendingStatusW<DedStatusReg0Spec> {
        InterruptPendingStatusW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for tptc_a1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status(&mut self) -> InterruptPendingStatusW<DedStatusReg0Spec> {
        InterruptPendingStatusW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for mss_l2_bankc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_pending_status(&mut self) -> InterruptPendingStatusW<DedStatusReg0Spec> {
        InterruptPendingStatusW::new(self, 7)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_status_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_status_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DedStatusReg0Spec;
impl crate::RegisterSpec for DedStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ded_status_reg0::R`](R) reader structure"]
impl crate::Readable for DedStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ded_status_reg0::W`](W) writer structure"]
impl crate::Writable for DedStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ded_status_reg0 to value 0"]
impl crate::Resettable for DedStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
