#[doc = "Register `CPSW_NC_EST_TS_DOMAIN_REG` reader"]
pub type R = crate::R<CpswNcEstTsDomainRegSpec>;
#[doc = "Register `CPSW_NC_EST_TS_DOMAIN_REG` writer"]
pub type W = crate::W<CpswNcEstTsDomainRegSpec>;
#[doc = "Field `ENHANCED_SCHEDULED_TRAFFIC` reader - 7:0\\]
Enhanced Scheduled Traffic Host Event Domain"]
pub type EnhancedScheduledTrafficR = crate::FieldReader;
#[doc = "Field `ENHANCED_SCHEDULED_TRAFFIC` writer - 7:0\\]
Enhanced Scheduled Traffic Host Event Domain"]
pub type EnhancedScheduledTrafficW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Enhanced Scheduled Traffic Host Event Domain"]
    #[inline(always)]
    pub fn enhanced_scheduled_traffic(&self) -> EnhancedScheduledTrafficR {
        EnhancedScheduledTrafficR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Enhanced Scheduled Traffic Host Event Domain"]
    #[inline(always)]
    #[must_use]
    pub fn enhanced_scheduled_traffic(
        &mut self,
    ) -> EnhancedScheduledTrafficW<CpswNcEstTsDomainRegSpec> {
        EnhancedScheduledTrafficW::new(self, 0)
    }
}
#[doc = "Enhanced Scheduled Traffic Host Event Domain\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_est_ts_domain_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_est_ts_domain_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEstTsDomainRegSpec;
impl crate::RegisterSpec for CpswNcEstTsDomainRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_est_ts_domain_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEstTsDomainRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_est_ts_domain_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEstTsDomainRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_EST_TS_DOMAIN_REG to value 0"]
impl crate::Resettable for CpswNcEstTsDomainRegSpec {
    const RESET_VALUE: u32 = 0;
}
