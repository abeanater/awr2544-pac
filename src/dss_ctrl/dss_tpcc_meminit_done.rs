#[doc = "Register `DSS_TPCC_MEMINIT_DONE` reader"]
pub type R = crate::R<DssTpccMeminitDoneSpec>;
#[doc = "Register `DSS_TPCC_MEMINIT_DONE` writer"]
pub type W = crate::W<DssTpccMeminitDoneSpec>;
#[doc = "Field `tpcc_a_meminit_done` reader - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
pub type TpccAMeminitDoneR = crate::BitReader;
#[doc = "Field `tpcc_a_meminit_done` writer - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
pub type TpccAMeminitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_meminit_done` reader - 1:1\\]
RESERVED: Dont Use"]
pub type TpccBMeminitDoneR = crate::BitReader;
#[doc = "Field `tpcc_b_meminit_done` writer - 1:1\\]
RESERVED: Dont Use"]
pub type TpccBMeminitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_c_meminit_done` reader - 2:2\\]
RESERVED: Dont Use"]
pub type TpccCMeminitDoneR = crate::BitReader;
#[doc = "Field `tpcc_c_meminit_done` writer - 2:2\\]
RESERVED: Dont Use"]
pub type TpccCMeminitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
    #[inline(always)]
    pub fn tpcc_a_meminit_done(&self) -> TpccAMeminitDoneR {
        TpccAMeminitDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tpcc_b_meminit_done(&self) -> TpccBMeminitDoneR {
        TpccBMeminitDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tpcc_c_meminit_done(&self) -> TpccCMeminitDoneR {
        TpccCMeminitDoneR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_meminit_done(&mut self) -> TpccAMeminitDoneW<DssTpccMeminitDoneSpec> {
        TpccAMeminitDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_meminit_done(&mut self) -> TpccBMeminitDoneW<DssTpccMeminitDoneSpec> {
        TpccBMeminitDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_c_meminit_done(&mut self) -> TpccCMeminitDoneW<DssTpccMeminitDoneSpec> {
        TpccCMeminitDoneW::new(self, 2)
    }
}
#[doc = "DSS_TPCC_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_meminit_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_meminit_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssTpccMeminitDoneSpec;
impl crate::RegisterSpec for DssTpccMeminitDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_tpcc_meminit_done::R`](R) reader structure"]
impl crate::Readable for DssTpccMeminitDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_tpcc_meminit_done::W`](W) writer structure"]
impl crate::Writable for DssTpccMeminitDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_TPCC_MEMINIT_DONE to value 0"]
impl crate::Resettable for DssTpccMeminitDoneSpec {
    const RESET_VALUE: u32 = 0;
}
