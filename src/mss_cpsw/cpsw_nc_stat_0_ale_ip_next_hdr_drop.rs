#[doc = "Register `CPSW_NC_STAT_0_ALE_IP_NEXT_HDR_DROP` reader"]
pub type R = crate::R<CpswNcStat0AleIpNextHdrDropSpec>;
#[doc = "Register `CPSW_NC_STAT_0_ALE_IP_NEXT_HDR_DROP` writer"]
pub type W = crate::W<CpswNcStat0AleIpNextHdrDropSpec>;
#[doc = "Field `ALE_NEXT_HEADER` reader - 31:0\\]
ALE Next Header drop"]
pub type AleNextHeaderR = crate::FieldReader<u32>;
#[doc = "Field `ALE_NEXT_HEADER` writer - 31:0\\]
ALE Next Header drop"]
pub type AleNextHeaderW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Next Header drop"]
    #[inline(always)]
    pub fn ale_next_header(&self) -> AleNextHeaderR {
        AleNextHeaderR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Next Header drop"]
    #[inline(always)]
    #[must_use]
    pub fn ale_next_header(&mut self) -> AleNextHeaderW<CpswNcStat0AleIpNextHdrDropSpec> {
        AleNextHeaderW::new(self, 0)
    }
}
#[doc = "ALE IP Next Header Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_ip_next_hdr_drop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_ip_next_hdr_drop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat0AleIpNextHdrDropSpec;
impl crate::RegisterSpec for CpswNcStat0AleIpNextHdrDropSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_0_ale_ip_next_hdr_drop::R`](R) reader structure"]
impl crate::Readable for CpswNcStat0AleIpNextHdrDropSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_0_ale_ip_next_hdr_drop::W`](W) writer structure"]
impl crate::Writable for CpswNcStat0AleIpNextHdrDropSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_0_ALE_IP_NEXT_HDR_DROP to value 0"]
impl crate::Resettable for CpswNcStat0AleIpNextHdrDropSpec {
    const RESET_VALUE: u32 = 0;
}
