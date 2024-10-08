#[doc = "Register `CPSW_NC_SOFT_IDLE_REG` reader"]
pub type R = crate::R<CpswNcSoftIdleRegSpec>;
#[doc = "Register `CPSW_NC_SOFT_IDLE_REG` writer"]
pub type W = crate::W<CpswNcSoftIdleRegSpec>;
#[doc = "Field `SOFTWARE_IDLE` reader - 0:0\\]
Software Idle"]
pub type SoftwareIdleR = crate::BitReader;
#[doc = "Field `SOFTWARE_IDLE` writer - 0:0\\]
Software Idle"]
pub type SoftwareIdleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software Idle"]
    #[inline(always)]
    pub fn software_idle(&self) -> SoftwareIdleR {
        SoftwareIdleR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software Idle"]
    #[inline(always)]
    #[must_use]
    pub fn software_idle(&mut self) -> SoftwareIdleW<CpswNcSoftIdleRegSpec> {
        SoftwareIdleW::new(self, 0)
    }
}
#[doc = "CPSW Software Idle\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_soft_idle_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_soft_idle_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcSoftIdleRegSpec;
impl crate::RegisterSpec for CpswNcSoftIdleRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_soft_idle_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcSoftIdleRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_soft_idle_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcSoftIdleRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_SOFT_IDLE_REG to value 0"]
impl crate::Resettable for CpswNcSoftIdleRegSpec {
    const RESET_VALUE: u32 = 0;
}
