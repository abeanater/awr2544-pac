#[doc = "Register `REGS_INT_SS_C0_MISC_EN_REG` reader"]
pub type R = crate::R<RegsIntSsC0MiscEnRegSpec>;
#[doc = "Register `REGS_INT_SS_C0_MISC_EN_REG` writer"]
pub type W = crate::W<RegsIntSsC0MiscEnRegSpec>;
#[doc = "Field `CORE_0_MISC_MDIO` reader - 0:0\\]
Core 0 MISC_MDIO userint interrupt enable - OR of bits 1 and 0"]
pub type Core0MiscMdioR = crate::BitReader;
#[doc = "Field `CORE_0_MISC_MDIO` writer - 0:0\\]
Core 0 MISC_MDIO userint interrupt enable - OR of bits 1 and 0"]
pub type Core0MiscMdioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_MISC` reader - 1:1\\]
Core 0 MISC MDIO linkint - OR of bits 1 and 0"]
pub type Core0MiscR = crate::BitReader;
#[doc = "Field `CORE_0_MISC` writer - 1:1\\]
Core 0 MISC MDIO linkint - OR of bits 1 and 0"]
pub type Core0MiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_MISC` reader - 2:2\\]
Core 0 MISC Host Interrupt Enable"]
pub type Core0MiscR = crate::BitReader;
#[doc = "Field `CORE_0_MISC` writer - 2:2\\]
Core 0 MISC Host Interrupt Enable"]
pub type Core0MiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_MISC` reader - 3:3\\]
Core 0 MISC Statistics Interrupt Enable - OR of bits n downto 0"]
pub type Core0MiscR = crate::BitReader;
#[doc = "Field `CORE_0_MISC` writer - 3:3\\]
Core 0 MISC Statistics Interrupt Enable - OR of bits n downto 0"]
pub type Core0MiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_MISC` reader - 4:4\\]
Core 0 MISC CPTS Event Interrupt Enable"]
pub type Core0MiscR = crate::BitReader;
#[doc = "Field `CORE_0_MISC` writer - 4:4\\]
Core 0 MISC CPTS Event Interrupt Enable"]
pub type Core0MiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_MISC` reader - 5:5\\]
Core 0 MISC SEC Memory Protect Error Interrupt Enable"]
pub type Core0MiscR = crate::BitReader;
#[doc = "Field `CORE_0_MISC` writer - 5:5\\]
Core 0 MISC SEC Memory Protect Error Interrupt Enable"]
pub type Core0MiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_MISC` reader - 6:6\\]
Core 0 MISC DED Memory Protect Error Interrupt Enable"]
pub type Core0MiscR = crate::BitReader;
#[doc = "Field `CORE_0_MISC` writer - 6:6\\]
Core 0 MISC DED Memory Protect Error Interrupt Enable"]
pub type Core0MiscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Core 0 MISC_MDIO userint interrupt enable - OR of bits 1 and 0"]
    #[inline(always)]
    pub fn core_0_misc_mdio(&self) -> Core0MiscMdioR {
        Core0MiscMdioR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Core 0 MISC MDIO linkint - OR of bits 1 and 0"]
    #[inline(always)]
    pub fn core_0_misc(&self) -> Core0MiscR {
        Core0MiscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Core 0 MISC Host Interrupt Enable"]
    #[inline(always)]
    pub fn core_0_misc(&self) -> Core0MiscR {
        Core0MiscR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Core 0 MISC Statistics Interrupt Enable - OR of bits n downto 0"]
    #[inline(always)]
    pub fn core_0_misc(&self) -> Core0MiscR {
        Core0MiscR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Core 0 MISC CPTS Event Interrupt Enable"]
    #[inline(always)]
    pub fn core_0_misc(&self) -> Core0MiscR {
        Core0MiscR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Core 0 MISC SEC Memory Protect Error Interrupt Enable"]
    #[inline(always)]
    pub fn core_0_misc(&self) -> Core0MiscR {
        Core0MiscR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Core 0 MISC DED Memory Protect Error Interrupt Enable"]
    #[inline(always)]
    pub fn core_0_misc(&self) -> Core0MiscR {
        Core0MiscR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Core 0 MISC_MDIO userint interrupt enable - OR of bits 1 and 0"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_misc_mdio(&mut self) -> Core0MiscMdioW<RegsIntSsC0MiscEnRegSpec> {
        Core0MiscMdioW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Core 0 MISC MDIO linkint - OR of bits 1 and 0"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_misc(&mut self) -> Core0MiscW<RegsIntSsC0MiscEnRegSpec> {
        Core0MiscW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Core 0 MISC Host Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_misc(&mut self) -> Core0MiscW<RegsIntSsC0MiscEnRegSpec> {
        Core0MiscW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Core 0 MISC Statistics Interrupt Enable - OR of bits n downto 0"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_misc(&mut self) -> Core0MiscW<RegsIntSsC0MiscEnRegSpec> {
        Core0MiscW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Core 0 MISC CPTS Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_misc(&mut self) -> Core0MiscW<RegsIntSsC0MiscEnRegSpec> {
        Core0MiscW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Core 0 MISC SEC Memory Protect Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_misc(&mut self) -> Core0MiscW<RegsIntSsC0MiscEnRegSpec> {
        Core0MiscW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Core 0 MISC DED Memory Protect Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_misc(&mut self) -> Core0MiscW<RegsIntSsC0MiscEnRegSpec> {
        Core0MiscW::new(self, 6)
    }
}
#[doc = "Core 0 Misc Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`regs_int_ss_c0_misc_en_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regs_int_ss_c0_misc_en_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsIntSsC0MiscEnRegSpec;
impl crate::RegisterSpec for RegsIntSsC0MiscEnRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_int_ss_c0_misc_en_reg::R`](R) reader structure"]
impl crate::Readable for RegsIntSsC0MiscEnRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs_int_ss_c0_misc_en_reg::W`](W) writer structure"]
impl crate::Writable for RegsIntSsC0MiscEnRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_INT_SS_C0_MISC_EN_REG to value 0"]
impl crate::Resettable for RegsIntSsC0MiscEnRegSpec {
    const RESET_VALUE: u32 = 0;
}
