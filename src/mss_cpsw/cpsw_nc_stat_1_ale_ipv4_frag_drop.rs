#[doc = "Register `CPSW_NC_STAT_1_ALE_IPV4_FRAG_DROP` reader"]
pub type R = crate::R<CpswNcStat1AleIpv4FragDropSpec>;
#[doc = "Register `CPSW_NC_STAT_1_ALE_IPV4_FRAG_DROP` writer"]
pub type W = crate::W<CpswNcStat1AleIpv4FragDropSpec>;
#[doc = "Field `ALE_IPV4_FRAGMENT` reader - 31:0\\]
ALE IPV4 Fragment drop"]
pub type AleIpv4FragmentR = crate::FieldReader<u32>;
#[doc = "Field `ALE_IPV4_FRAGMENT` writer - 31:0\\]
ALE IPV4 Fragment drop"]
pub type AleIpv4FragmentW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ALE IPV4 Fragment drop"]
    #[inline(always)]
    pub fn ale_ipv4_fragment(&self) -> AleIpv4FragmentR {
        AleIpv4FragmentR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ALE IPV4 Fragment drop"]
    #[inline(always)]
    #[must_use]
    pub fn ale_ipv4_fragment(&mut self) -> AleIpv4FragmentW<CpswNcStat1AleIpv4FragDropSpec> {
        AleIpv4FragmentW::new(self, 0)
    }
}
#[doc = "ALE IPV4 Frag Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_1_ale_ipv4_frag_drop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_1_ale_ipv4_frag_drop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat1AleIpv4FragDropSpec;
impl crate::RegisterSpec for CpswNcStat1AleIpv4FragDropSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_1_ale_ipv4_frag_drop::R`](R) reader structure"]
impl crate::Readable for CpswNcStat1AleIpv4FragDropSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_1_ale_ipv4_frag_drop::W`](W) writer structure"]
impl crate::Writable for CpswNcStat1AleIpv4FragDropSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_1_ALE_IPV4_FRAG_DROP to value 0"]
impl crate::Resettable for CpswNcStat1AleIpv4FragDropSpec {
    const RESET_VALUE: u32 = 0;
}
