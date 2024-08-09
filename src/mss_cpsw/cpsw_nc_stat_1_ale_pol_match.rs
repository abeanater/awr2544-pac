#[doc = "Register `CPSW_NC_STAT_1_ALE_POL_MATCH` reader"]
pub type R = crate::R<CpswNcStat1AlePolMatchSpec>;
#[doc = "Register `CPSW_NC_STAT_1_ALE_POL_MATCH` writer"]
pub type W = crate::W<CpswNcStat1AlePolMatchSpec>;
#[doc = "Field `ALE_POLICER_MATCHED` reader - 31:0\\]
ALE Policer Matched"]
pub type AlePolicerMatchedR = crate::FieldReader<u32>;
#[doc = "Field `ALE_POLICER_MATCHED` writer - 31:0\\]
ALE Policer Matched"]
pub type AlePolicerMatchedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Policer Matched"]
    #[inline(always)]
    pub fn ale_policer_matched(&self) -> AlePolicerMatchedR {
        AlePolicerMatchedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Policer Matched"]
    #[inline(always)]
    #[must_use]
    pub fn ale_policer_matched(&mut self) -> AlePolicerMatchedW<CpswNcStat1AlePolMatchSpec> {
        AlePolicerMatchedW::new(self, 0)
    }
}
#[doc = "ALE Policer Matched\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_pol_match::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_pol_match::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat1AlePolMatchSpec;
impl crate::RegisterSpec for CpswNcStat1AlePolMatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_1_ale_pol_match::R`](R) reader structure"]
impl crate::Readable for CpswNcStat1AlePolMatchSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_1_ale_pol_match::W`](W) writer structure"]
impl crate::Writable for CpswNcStat1AlePolMatchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_1_ALE_POL_MATCH to value 0"]
impl crate::Resettable for CpswNcStat1AlePolMatchSpec {
    const RESET_VALUE: u32 = 0;
}
