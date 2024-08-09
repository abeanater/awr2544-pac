#[doc = "Register `RSS_TPCC_MEMINIT_STATUS` reader"]
pub type R = crate::R<RssTpccMeminitStatusSpec>;
#[doc = "Register `RSS_TPCC_MEMINIT_STATUS` writer"]
pub type W = crate::W<RssTpccMeminitStatusSpec>;
#[doc = "Field `tpcc_a_meminit_status` reader - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is in progress."]
pub type TpccAMeminitStatusR = crate::BitReader;
#[doc = "Field `tpcc_a_meminit_status` writer - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is in progress."]
pub type TpccAMeminitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is in progress."]
    #[inline(always)]
    pub fn tpcc_a_meminit_status(&self) -> TpccAMeminitStatusR {
        TpccAMeminitStatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_meminit_status(&mut self) -> TpccAMeminitStatusW<RssTpccMeminitStatusSpec> {
        TpccAMeminitStatusW::new(self, 0)
    }
}
#[doc = "RSS_TPCC_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tpcc_meminit_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tpcc_meminit_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssTpccMeminitStatusSpec;
impl crate::RegisterSpec for RssTpccMeminitStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_tpcc_meminit_status::R`](R) reader structure"]
impl crate::Readable for RssTpccMeminitStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_tpcc_meminit_status::W`](W) writer structure"]
impl crate::Writable for RssTpccMeminitStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_TPCC_MEMINIT_STATUS to value 0"]
impl crate::Resettable for RssTpccMeminitStatusSpec {
    const RESET_VALUE: u32 = 0;
}
