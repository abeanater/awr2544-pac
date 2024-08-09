#[doc = "Register `MSS_TPCC_MEMINIT_STATUS` reader"]
pub type R = crate::R<MssTpccMeminitStatusSpec>;
#[doc = "Register `MSS_TPCC_MEMINIT_STATUS` writer"]
pub type W = crate::W<MssTpccMeminitStatusSpec>;
#[doc = "Field `tpcc_a_meminit_status` reader - 0:0\\]
1'b0: No initialization is happening for MSS_TPCCA 1'b1: Initialization is in progress for MSS_TPCCB"]
pub type TpccAMeminitStatusR = crate::BitReader;
#[doc = "Field `tpcc_a_meminit_status` writer - 0:0\\]
1'b0: No initialization is happening for MSS_TPCCA 1'b1: Initialization is in progress for MSS_TPCCB"]
pub type TpccAMeminitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for MSS_TPCCA 1'b1: Initialization is in progress for MSS_TPCCB"]
    #[inline(always)]
    pub fn tpcc_a_meminit_status(&self) -> TpccAMeminitStatusR {
        TpccAMeminitStatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for MSS_TPCCA 1'b1: Initialization is in progress for MSS_TPCCB"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_meminit_status(&mut self) -> TpccAMeminitStatusW<MssTpccMeminitStatusSpec> {
        TpccAMeminitStatusW::new(self, 0)
    }
}
#[doc = "MSS_TPCC_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tpcc_meminit_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tpcc_meminit_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssTpccMeminitStatusSpec;
impl crate::RegisterSpec for MssTpccMeminitStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_tpcc_meminit_status::R`](R) reader structure"]
impl crate::Readable for MssTpccMeminitStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_tpcc_meminit_status::W`](W) writer structure"]
impl crate::Writable for MssTpccMeminitStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_TPCC_MEMINIT_STATUS to value 0"]
impl crate::Resettable for MssTpccMeminitStatusSpec {
    const RESET_VALUE: u32 = 0;
}
