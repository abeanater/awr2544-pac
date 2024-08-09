#[doc = "Register `CPSW_NC_STAT_0_ALE_POL_MATCH_RED` reader"]
pub type R = crate::R<CpswNcStat0AlePolMatchRedSpec>;
#[doc = "Register `CPSW_NC_STAT_0_ALE_POL_MATCH_RED` writer"]
pub type W = crate::W<CpswNcStat0AlePolMatchRedSpec>;
#[doc = "Field `ALE_POLICER_MATCHED` reader - 31:0\\]
ALE Policer Matched and Condition Red"]
pub type AlePolicerMatchedR = crate::FieldReader<u32>;
#[doc = "Field `ALE_POLICER_MATCHED` writer - 31:0\\]
ALE Policer Matched and Condition Red"]
pub type AlePolicerMatchedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Policer Matched and Condition Red"]
    #[inline(always)]
    pub fn ale_policer_matched(&self) -> AlePolicerMatchedR {
        AlePolicerMatchedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Policer Matched and Condition Red"]
    #[inline(always)]
    #[must_use]
    pub fn ale_policer_matched(&mut self) -> AlePolicerMatchedW<CpswNcStat0AlePolMatchRedSpec> {
        AlePolicerMatchedW::new(self, 0)
    }
}
#[doc = "ALE Policer Matched and Condition Red\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_pol_match_red::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_pol_match_red::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat0AlePolMatchRedSpec;
impl crate::RegisterSpec for CpswNcStat0AlePolMatchRedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_0_ale_pol_match_red::R`](R) reader structure"]
impl crate::Readable for CpswNcStat0AlePolMatchRedSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_0_ale_pol_match_red::W`](W) writer structure"]
impl crate::Writable for CpswNcStat0AlePolMatchRedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_0_ALE_POL_MATCH_RED to value 0"]
impl crate::Resettable for CpswNcStat0AlePolMatchRedSpec {
    const RESET_VALUE: u32 = 0;
}
