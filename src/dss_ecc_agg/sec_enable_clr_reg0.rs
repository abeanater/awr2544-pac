#[doc = "Register `sec_enable_clr_reg0` reader"]
pub type R = crate::R<SecEnableClrReg0Spec>;
#[doc = "Register `sec_enable_clr_reg0` writer"]
pub type W = crate::W<SecEnableClrReg0Spec>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` reader - 0:0\\]
Interrupt Enable Clear Register for dss_l3ram0_pend"]
pub type InterruptEnableClearR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` writer - 0:0\\]
Interrupt Enable Clear Register for dss_l3ram0_pend"]
pub type InterruptEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` reader - 1:1\\]
Interrupt Enable Clear Register for dss_l3ram1_pend"]
pub type InterruptEnableClearR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` writer - 1:1\\]
Interrupt Enable Clear Register for dss_l3ram1_pend"]
pub type InterruptEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` reader - 2:2\\]
Interrupt Enable Clear Register for dss_l3ram2_pend"]
pub type InterruptEnableClearR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` writer - 2:2\\]
Interrupt Enable Clear Register for dss_l3ram2_pend"]
pub type InterruptEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` reader - 3:3\\]
Interrupt Enable Clear Register for dss_l3ram3_pend"]
pub type InterruptEnableClearR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` writer - 3:3\\]
Interrupt Enable Clear Register for dss_l3ram3_pend"]
pub type InterruptEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` reader - 4:4\\]
Interrupt Enable Clear Register for dss_mailbox_pend"]
pub type InterruptEnableClearR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` writer - 4:4\\]
Interrupt Enable Clear Register for dss_mailbox_pend"]
pub type InterruptEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` reader - 5:5\\]
Interrupt Enable Clear Register for hwacm4_ram_b0_pend"]
pub type InterruptEnableClearR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` writer - 5:5\\]
Interrupt Enable Clear Register for hwacm4_ram_b0_pend"]
pub type InterruptEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` reader - 6:6\\]
Interrupt Enable Clear Register for hwacm4_ram_b1_pend"]
pub type InterruptEnableClearR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` writer - 6:6\\]
Interrupt Enable Clear Register for hwacm4_ram_b1_pend"]
pub type InterruptEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` reader - 7:7\\]
Interrupt Enable Clear Register for hwacm4_ram_b2_pend"]
pub type InterruptEnableClearR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` writer - 7:7\\]
Interrupt Enable Clear Register for hwacm4_ram_b2_pend"]
pub type InterruptEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` reader - 8:8\\]
Interrupt Enable Clear Register for hwacm4_mailbox_pend"]
pub type InterruptEnableClearR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` writer - 8:8\\]
Interrupt Enable Clear Register for hwacm4_mailbox_pend"]
pub type InterruptEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` reader - 9:9\\]
Interrupt Enable Clear Register for dss_tptc_a0_pend"]
pub type InterruptEnableClearR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` writer - 9:9\\]
Interrupt Enable Clear Register for dss_tptc_a0_pend"]
pub type InterruptEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` reader - 10:10\\]
Interrupt Enable Clear Register for dss_tptc_a1_pend"]
pub type InterruptEnableClearR = crate::BitReader;
#[doc = "Field `INTERRUPT_ENABLE_CLEAR` writer - 10:10\\]
Interrupt Enable Clear Register for dss_tptc_a1_pend"]
pub type InterruptEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for dss_l3ram0_pend"]
    #[inline(always)]
    pub fn interrupt_enable_clear(&self) -> InterruptEnableClearR {
        InterruptEnableClearR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for dss_l3ram1_pend"]
    #[inline(always)]
    pub fn interrupt_enable_clear(&self) -> InterruptEnableClearR {
        InterruptEnableClearR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for dss_l3ram2_pend"]
    #[inline(always)]
    pub fn interrupt_enable_clear(&self) -> InterruptEnableClearR {
        InterruptEnableClearR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Clear Register for dss_l3ram3_pend"]
    #[inline(always)]
    pub fn interrupt_enable_clear(&self) -> InterruptEnableClearR {
        InterruptEnableClearR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Clear Register for dss_mailbox_pend"]
    #[inline(always)]
    pub fn interrupt_enable_clear(&self) -> InterruptEnableClearR {
        InterruptEnableClearR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Clear Register for hwacm4_ram_b0_pend"]
    #[inline(always)]
    pub fn interrupt_enable_clear(&self) -> InterruptEnableClearR {
        InterruptEnableClearR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Clear Register for hwacm4_ram_b1_pend"]
    #[inline(always)]
    pub fn interrupt_enable_clear(&self) -> InterruptEnableClearR {
        InterruptEnableClearR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Clear Register for hwacm4_ram_b2_pend"]
    #[inline(always)]
    pub fn interrupt_enable_clear(&self) -> InterruptEnableClearR {
        InterruptEnableClearR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Clear Register for hwacm4_mailbox_pend"]
    #[inline(always)]
    pub fn interrupt_enable_clear(&self) -> InterruptEnableClearR {
        InterruptEnableClearR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Enable Clear Register for dss_tptc_a0_pend"]
    #[inline(always)]
    pub fn interrupt_enable_clear(&self) -> InterruptEnableClearR {
        InterruptEnableClearR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Enable Clear Register for dss_tptc_a1_pend"]
    #[inline(always)]
    pub fn interrupt_enable_clear(&self) -> InterruptEnableClearR {
        InterruptEnableClearR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for dss_l3ram0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear(&mut self) -> InterruptEnableClearW<SecEnableClrReg0Spec> {
        InterruptEnableClearW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for dss_l3ram1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear(&mut self) -> InterruptEnableClearW<SecEnableClrReg0Spec> {
        InterruptEnableClearW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for dss_l3ram2_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear(&mut self) -> InterruptEnableClearW<SecEnableClrReg0Spec> {
        InterruptEnableClearW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Clear Register for dss_l3ram3_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear(&mut self) -> InterruptEnableClearW<SecEnableClrReg0Spec> {
        InterruptEnableClearW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Clear Register for dss_mailbox_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear(&mut self) -> InterruptEnableClearW<SecEnableClrReg0Spec> {
        InterruptEnableClearW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Clear Register for hwacm4_ram_b0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear(&mut self) -> InterruptEnableClearW<SecEnableClrReg0Spec> {
        InterruptEnableClearW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Clear Register for hwacm4_ram_b1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear(&mut self) -> InterruptEnableClearW<SecEnableClrReg0Spec> {
        InterruptEnableClearW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Clear Register for hwacm4_ram_b2_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear(&mut self) -> InterruptEnableClearW<SecEnableClrReg0Spec> {
        InterruptEnableClearW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Clear Register for hwacm4_mailbox_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear(&mut self) -> InterruptEnableClearW<SecEnableClrReg0Spec> {
        InterruptEnableClearW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Enable Clear Register for dss_tptc_a0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear(&mut self) -> InterruptEnableClearW<SecEnableClrReg0Spec> {
        InterruptEnableClearW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Enable Clear Register for dss_tptc_a1_pend"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable_clear(&mut self) -> InterruptEnableClearW<SecEnableClrReg0Spec> {
        InterruptEnableClearW::new(self, 10)
    }
}
#[doc = "Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_enable_clr_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_enable_clr_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEnableClrReg0Spec;
impl crate::RegisterSpec for SecEnableClrReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_enable_clr_reg0::R`](R) reader structure"]
impl crate::Readable for SecEnableClrReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_enable_clr_reg0::W`](W) writer structure"]
impl crate::Writable for SecEnableClrReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sec_enable_clr_reg0 to value 0"]
impl crate::Resettable for SecEnableClrReg0Spec {
    const RESET_VALUE: u32 = 0;
}
