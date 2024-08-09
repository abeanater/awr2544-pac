#[doc = "Register `CPSW_NC_STAT_1_ALE_POL_MATCH_YELLOW` reader"]
pub type R = crate::R<CpswNcStat1AlePolMatchYellowSpec>;
#[doc = "Register `CPSW_NC_STAT_1_ALE_POL_MATCH_YELLOW` writer"]
pub type W = crate::W<CpswNcStat1AlePolMatchYellowSpec>;
#[doc = "Field `ALE_POLICER_MATCHED` reader - 31:0\\]
ALE Policer Matched and Condition Yellow"]
pub type AlePolicerMatchedR = crate::FieldReader<u32>;
#[doc = "Field `ALE_POLICER_MATCHED` writer - 31:0\\]
ALE Policer Matched and Condition Yellow"]
pub type AlePolicerMatchedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Policer Matched and Condition Yellow"]
    #[inline(always)]
    pub fn ale_policer_matched(&self) -> AlePolicerMatchedR {
        AlePolicerMatchedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Policer Matched and Condition Yellow"]
    #[inline(always)]
    #[must_use]
    pub fn ale_policer_matched(&mut self) -> AlePolicerMatchedW<CpswNcStat1AlePolMatchYellowSpec> {
        AlePolicerMatchedW::new(self, 0)
    }
}
#[doc = "ALE Policer Matched and Condition Yellow\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_pol_match_yellow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_pol_match_yellow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat1AlePolMatchYellowSpec;
impl crate::RegisterSpec for CpswNcStat1AlePolMatchYellowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_1_ale_pol_match_yellow::R`](R) reader structure"]
impl crate::Readable for CpswNcStat1AlePolMatchYellowSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_1_ale_pol_match_yellow::W`](W) writer structure"]
impl crate::Writable for CpswNcStat1AlePolMatchYellowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_1_ALE_POL_MATCH_YELLOW to value 0"]
impl crate::Resettable for CpswNcStat1AlePolMatchYellowSpec {
    const RESET_VALUE: u32 = 0;
}
