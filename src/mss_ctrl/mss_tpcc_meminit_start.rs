#[doc = "Register `MSS_TPCC_MEMINIT_START` reader"]
pub type R = crate::R<MssTpccMeminitStartSpec>;
#[doc = "Register `MSS_TPCC_MEMINIT_START` writer"]
pub type W = crate::W<MssTpccMeminitStartSpec>;
#[doc = "Field `tpcc_a_meminit_start` reader - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the MSS_TPCCA"]
pub type TpccAMeminitStartR = crate::BitReader;
#[doc = "Field `tpcc_a_meminit_start` writer - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the MSS_TPCCA"]
pub type TpccAMeminitStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the MSS_TPCCA"]
    #[inline(always)]
    pub fn tpcc_a_meminit_start(&self) -> TpccAMeminitStartR {
        TpccAMeminitStartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the MSS_TPCCA"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_meminit_start(&mut self) -> TpccAMeminitStartW<MssTpccMeminitStartSpec> {
        TpccAMeminitStartW::new(self, 0)
    }
}
#[doc = "MSS_TPCC_MEMINIT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tpcc_meminit_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tpcc_meminit_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssTpccMeminitStartSpec;
impl crate::RegisterSpec for MssTpccMeminitStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_tpcc_meminit_start::R`](R) reader structure"]
impl crate::Readable for MssTpccMeminitStartSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_tpcc_meminit_start::W`](W) writer structure"]
impl crate::Writable for MssTpccMeminitStartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_TPCC_MEMINIT_START to value 0"]
impl crate::Resettable for MssTpccMeminitStartSpec {
    const RESET_VALUE: u32 = 0;
}
